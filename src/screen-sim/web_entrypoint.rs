use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{CustomEvent, EventTarget, KeyboardEvent, MouseEvent, WebGl2RenderingContext, WheelEvent, Window};

use crate::action_bindings::on_button_action;
use crate::app_events::{dispatch_exiting_session, dispatch_top_message};
use crate::console;
use crate::internal_resolution::InternalResolution;
use crate::simulation_main_functions::{init_resources, load_materials, simulation_tick};
use crate::simulation_state::{Input, Materials, Resources, VideoInputMaterials, VideoInputResources};
use crate::wasm_error::{WasmError, WasmResult};
use crate::web_utils::window;

pub type OwnedClosure = Option<Closure<FnMut(JsValue)>>;

pub struct StateOwner {
    pub closures: RefCell<Vec<OwnedClosure>>,
    pub resources: Rc<RefCell<Resources>>,
    pub input: RefCell<Input>,
    pub materials: RefCell<Materials>,
}

impl StateOwner {
    pub fn new_rc(resources: Rc<RefCell<Resources>>, materials: Materials, input: Input) -> Rc<StateOwner> {
        Rc::new(StateOwner {
            closures: RefCell::new(Vec::new()),
            resources,
            materials: RefCell::new(materials),
            input: RefCell::new(input),
        })
    }
}

pub fn web_entrypoint(
    gl: JsValue,
    res: Rc<RefCell<Resources>>,
    video_input_resources: VideoInputResources,
    video_input_materials: VideoInputMaterials,
) -> WasmResult<()> {
    let gl = gl.dyn_into::<WebGl2RenderingContext>()?;
    init_resources(&mut res.borrow_mut(), video_input_resources)?;
    let owned_state = StateOwner::new_rc(res, load_materials(gl, video_input_materials)?, Input::new()?);
    let frame_closure: Closure<FnMut(JsValue)> = {
        let owned_state = Rc::clone(&owned_state);
        let window = window()?;
        Closure::wrap(Box::new(move |_| {
            if let Err(e) = web_entrypoint_iteration(&owned_state, &window) {
                console!(error. "An unexpected error happened during web_entrypoint_iteration.", e.to_js());
                dispatch_exiting_session()
                    .and_then(|_| dispatch_top_message("Error! Wild guess... Did you try a too big resolution?".into()))
                    .ok()
                    .expect("Can't exit properly.");
            }
        }))
    };
    window()?.request_animation_frame(frame_closure.as_ref().unchecked_ref())?;
    let mut closures = owned_state.closures.borrow_mut();
    closures.push(Some(frame_closure));

    let listeners = set_event_listeners(&owned_state)?;
    closures.extend(listeners);

    Ok(())
}

pub fn print_error(e: WasmError) {
    match e {
        WasmError::Js(o) => console!(error. "An unexpected error ocurred.", o),
        WasmError::Str(s) => console!(error. "An unexpected error ocurred.", s),
    };
}

fn web_entrypoint_iteration(owned_state: &StateOwner, window: &Window) -> WasmResult<()> {
    let mut input = owned_state.input.borrow_mut();
    let mut resources = owned_state.resources.borrow_mut();
    let mut materials = owned_state.materials.borrow_mut();
    let closures = owned_state.closures.borrow();
    match simulation_tick(&mut input, &mut resources, &mut materials) {
        Ok(true) => {
            window.request_animation_frame(closures[0].as_ref().ok_or("Wrong closure.")?.as_ref().unchecked_ref())?;
        }
        Ok(false) => {}
        Err(e) => {
            resources.filters.internal_resolution = InternalResolution::new(1.0);
            return Err(e);
        }
    };
    Ok(())
}

fn set_event_listeners(state_owner: &Rc<StateOwner>) -> WasmResult<Vec<OwnedClosure>> {
    let onblur: Closure<FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |_: JsValue| {
            let mut input = state_owner.input.borrow_mut();
            *input = Input::new().ok().unwrap();
        }))
    };

    let onkeydown: Closure<FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<KeyboardEvent>() {
                let mut input = state_owner.input.borrow_mut();
                on_button_action(&mut input, e.key().to_lowercase().as_ref(), true);
            }
        }))
    };

    let onkeyup: Closure<FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<KeyboardEvent>() {
                let mut input = state_owner.input.borrow_mut();
                on_button_action(&mut input, e.key().to_lowercase().as_ref(), false);
            }
        }))
    };

    let onmousedown: Closure<FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                let mut input = state_owner.input.borrow_mut();
                input.mouse_click.input = e.buttons() == 1;
            }
        }))
    };

    let onmouseup: Closure<FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if event.dyn_into::<MouseEvent>().is_ok() {
                let mut input = state_owner.input.borrow_mut();
                input.mouse_click.input = false;
            }
        }))
    };

    let onmousemove: Closure<FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                let mut input = state_owner.input.borrow_mut();
                input.mouse_position_x = e.movement_x();
                input.mouse_position_y = e.movement_y();
            }
        }))
    };

    let onmousewheel: Closure<FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<WheelEvent>() {
                let mut input = state_owner.input.borrow_mut();
                input.mouse_scroll_y = e.delta_y() as f32;
            }
        }))
    };

    let oncustominputevent: Closure<FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<CustomEvent>() {
                let mut input = state_owner.input.borrow_mut();
                let object = e.detail();
                if let Ok(value) = js_sys::Reflect::get(&object, &"value".into()) {
                    input.custom_event.value = value;
                }
                if let Ok(value) = js_sys::Reflect::get(&object, &"kind".into()) {
                    if let Some(js_kind) = value.as_string() {
                        input.custom_event.kind = js_kind;
                    }
                }
            }
        }))
    };

    let window = window()?;
    window.set_onblur(Some(onblur.as_ref().unchecked_ref()));

    let document = window.document().ok_or("cannot access document")?;
    let canvas = document
        .get_element_by_id("gl-canvas")
        .ok_or("Could not get gl-canvas")?
        .dyn_into::<EventTarget>()
        .map_err(|_| "Could not cast gl-canvas")?;
    document.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));
    document.set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));
    canvas.add_event_listener_with_callback("mousedown", onmousedown.as_ref().unchecked_ref())?;
    canvas.add_event_listener_with_callback("mouseup", onmouseup.as_ref().unchecked_ref())?;
    canvas.add_event_listener_with_callback("mousemove", onmousemove.as_ref().unchecked_ref())?;
    document.set_onwheel(Some(onmousewheel.as_ref().unchecked_ref()));
    EventTarget::from(window).add_event_listener_with_callback("app-event.custom_input_event", oncustominputevent.as_ref().unchecked_ref())?;

    let mut closures: Vec<OwnedClosure> = vec![];
    closures.push(Some(onblur));
    closures.push(Some(onkeydown));
    closures.push(Some(onkeyup));
    closures.push(Some(onmousedown));
    closures.push(Some(onmouseup));
    closures.push(Some(onmousemove));
    closures.push(Some(onmousewheel));
    closures.push(Some(oncustominputevent));

    Ok(closures)
}
