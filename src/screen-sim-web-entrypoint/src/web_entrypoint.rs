use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{CustomEvent, EventTarget, KeyboardEvent, MouseEvent, WebGl2RenderingContext, WheelEvent, Window};

use crate::action_bindings::on_button_action;
use crate::console;
use crate::simulation_entrypoint::{init_resources, load_materials, SimulationTicker};
use crate::web_events::WebEventDispatcher;
use core::app_events::AppEventDispatcher;
use core::internal_resolution::InternalResolution;
use core::simulation_context::SimulationContext;
use core::simulation_core_state::{Input, InputEventValue, Resources, VideoInputResources};
use render::simulation_render_state::{Materials, VideoInputMaterials};
use web_error::{WebError, WebResult};
use crate::web_utils::{now, window};

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
) -> WebResult<()> {
    let gl = gl.dyn_into::<WebGl2RenderingContext>()?;
    init_resources(&mut res.borrow_mut(), video_input_resources)?;
    let owned_state = StateOwner::new_rc(res, load_materials(gl, video_input_materials)?, Input::new(now()?));
    let frame_closure: Closure<FnMut(JsValue)> = {
        let owned_state = Rc::clone(&owned_state);
        let window = window()?;
        Closure::wrap(Box::new(move |_| {
            let mut ctx: SimulationContext<WebEventDispatcher> = SimulationContext::default();
            if let Err(e) = web_entrypoint_iteration(&owned_state, &window, &mut ctx) {
                console!(error. "An unexpected error happened during web_entrypoint_iteration.", e.to_js());
                ctx.dispatcher.dispatch_exiting_session();
                ctx.dispatcher
                    .dispatch_top_message("Error! Try restarting your browser. Contact me if this problem persists!");
            }
            if let Err(e) = ctx.dispatcher.check_error() {
                console!(error. "Error dispatching some events: ", e.to_js());
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

pub fn print_error(e: WebError) {
    match e {
        WebError::Js(o) => console!(error. "An unexpected error ocurred.", o),
        WebError::Str(s) => console!(error. "An unexpected error ocurred.", s),
    };
}

fn web_entrypoint_iteration<T: AppEventDispatcher + Default>(owned_state: &StateOwner, window: &Window, ctx: &mut SimulationContext<T>) -> WebResult<()> {
    let mut input = owned_state.input.borrow_mut();
    let mut resources = owned_state.resources.borrow_mut();
    let mut materials = owned_state.materials.borrow_mut();
    let closures = owned_state.closures.borrow();
    match SimulationTicker::new(ctx, &mut input, &mut resources, &mut materials).tick() {
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

fn set_event_listeners(state_owner: &Rc<StateOwner>) -> WebResult<Vec<OwnedClosure>> {
    let onblur: Closure<FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |_: JsValue| {
            let mut input = state_owner.input.borrow_mut();
            *input = Input::new(now().unwrap_or(0.0));
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
            let mut input = state_owner.input.borrow_mut();
            if let Err(e) = read_custom_event(&mut input, event) {
                console!(error. "Could not read custom event.", e.to_js());
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

pub fn read_custom_event(input: &mut Input, event: JsValue) -> WebResult<()> {
    let event = event.dyn_into::<CustomEvent>()?;
    let object = event.detail();
    let value = js_sys::Reflect::get(&object, &"value".into())?;
    let kind = js_sys::Reflect::get(&object, &"kind".into())?
        .as_string()
        .ok_or_else(|| WebError::Str("Could not get kind".into()))?;
    input.custom_event.value = match kind.as_ref() as &str {
        "event_kind:pixel_brightness" => InputEventValue::PixelBrighttness(value.as_f64().ok_or("it should be a number")? as f32),
        "event_kind:pixel_contrast" => InputEventValue::PixelContrast(value.as_f64().ok_or("it should be a number")? as f32),
        "event_kind:light_color" => InputEventValue::LightColor(value.as_f64().ok_or("it should be a number")? as i32),
        "event_kind:brightness_color" => InputEventValue::BrightnessColor(value.as_f64().ok_or("it should be a number")? as i32),
        "event_kind:blur_level" => InputEventValue::BlurLevel(value.as_f64().ok_or("it should be a number")? as usize),
        "event_kind:lines_per_pixel" => InputEventValue::LinersPerPixel(value.as_f64().ok_or("it should be a number")? as usize),
        "event_kind:pixel_shadow_height" => InputEventValue::PixelShadowHeight(value.as_f64().ok_or("it should be a number")? as f32),
        "event_kind:pixel_vertical_gap" => InputEventValue::PixelVerticalGap(value.as_f64().ok_or("it should be a number")? as f32),
        "event_kind:pixel_horizontal_gap" => InputEventValue::PixelHorizontalGap(value.as_f64().ok_or("it should be a number")? as f32),
        "event_kind:pixel_width" => InputEventValue::PixelWidth(value.as_f64().ok_or("it should be a number")? as f32),
        "event_kind:pixel_spread" => InputEventValue::PixelSpread(value.as_f64().ok_or("it should be a number")? as f32),
        "event_kind:camera_zoom" => InputEventValue::CameraZoom(value.as_f64().ok_or("it should be a number")? as f32),
        "event_kind:camera_pos_x" => InputEventValue::CameraPosX(value.as_f64().ok_or("it should be a number")? as f32),
        "event_kind:camera_pos_y" => InputEventValue::CameraPosY(value.as_f64().ok_or("it should be a number")? as f32),
        "event_kind:camera_pos_z" => InputEventValue::CameraPosZ(value.as_f64().ok_or("it should be a number")? as f32),
        "event_kind:camera_axis_up_x" => InputEventValue::CameraAxisUpX(value.as_f64().ok_or("it should be a number")? as f32),
        "event_kind:camera_axis_up_y" => InputEventValue::CameraAxisUpY(value.as_f64().ok_or("it should be a number")? as f32),
        "event_kind:camera_axis_up_z" => InputEventValue::CameraAxisUpZ(value.as_f64().ok_or("it should be a number")? as f32),
        "event_kind:camera_direction_x" => InputEventValue::CameraDirectionX(value.as_f64().ok_or("it should be a number")? as f32),
        "event_kind:camera_direction_y" => InputEventValue::CameraDirectionY(value.as_f64().ok_or("it should be a number")? as f32),
        "event_kind:camera_direction_z" => InputEventValue::CameraDirectionZ(value.as_f64().ok_or("it should be a number")? as f32),
        _ => InputEventValue::None,
    };
    input.custom_event.kind = kind;
    Ok(())
}
