/* Copyright (c) 2019 José manuel Barroso Galindo <theypsilon@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{CustomEvent, EventTarget, KeyboardEvent, MouseEvent, WebGl2RenderingContext, WheelEvent, Window};

use crate::console;
use crate::web_events::WebEventDispatcher;
use crate::web_utils::{now, window};
use core::action_bindings::on_button_action;
use core::camera::CameraChange;
use core::simulation_context::{ConcreteSimulationContext, RandomGenerator, SimulationContext};
use core::simulation_core_state::{event_kind, Input, InputEventValue, Resources, VideoInputResources};
use core::simulation_core_ticker::SimulationCoreTicker;
use glow::GlowSafeAdapter;
use render::simulation_draw::SimulationDrawer;
use render::simulation_render_state::{Materials, VideoInputMaterials};
use web_error::{WebError, WebResult};

pub type OwnedClosure = Option<Closure<dyn FnMut(JsValue)>>;

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
    let webgl = gl.dyn_into::<WebGl2RenderingContext>()?;
    let gl = Rc::new(GlowSafeAdapter::new(glow::Context::from_webgl2_context(webgl.clone())));
    res.borrow_mut().initialize(video_input_resources, now()?);
    let owned_state = StateOwner::new_rc(res, Materials::new(gl, video_input_materials)?, Input::new(now()?));
    let frame_closure: Closure<dyn FnMut(JsValue)> = {
        let owned_state = Rc::clone(&owned_state);
        let window = window()?;
        Closure::wrap(Box::new(move |_| {
            let mut ctx = ConcreteSimulationContext::new(WebEventDispatcher::new(webgl.clone()), WebRnd {});
            if let Err(e) = web_entrypoint_iteration(&owned_state, &window, &mut ctx) {
                console!(error. "An unexpected error happened during web_entrypoint_iteration.", e.into_js());
                ctx.dispatcher().dispatch_exiting_session();
                ctx.dispatcher()
                    .dispatch_top_message("Error! Try restarting your browser. Contact me if this problem persists!");
            }
            if let Err(e) = ctx.dispatcher_instance.check_error() {
                console!(error. "Error dispatching some events: ", e.into_js());
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
    console!(error. "An unexpected error ocurred.", e.into_js());
}

fn web_entrypoint_iteration(owned_state: &StateOwner, window: &Window, ctx: &mut dyn SimulationContext) -> WebResult<()> {
    let mut input = owned_state.input.borrow_mut();
    let mut resources = owned_state.resources.borrow_mut();
    let mut materials = owned_state.materials.borrow_mut();
    let closures = owned_state.closures.borrow();
    match tick(ctx, &mut input, &mut resources, &mut materials) {
        Ok(true) => {
            window.request_animation_frame(closures[0].as_ref().ok_or("Wrong closure.")?.as_ref().unchecked_ref())?;
        }
        Ok(false) => {}
        Err(e) => return Err(e),
    };
    Ok(())
}

struct WebRnd {}

impl RandomGenerator for WebRnd {
    fn next(&self) -> f32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(0.0, 1.0)
    }
}

fn tick(ctx: &dyn SimulationContext, input: &mut Input, res: &mut Resources, materials: &mut Materials) -> WebResult<bool> {
    SimulationCoreTicker::new(ctx, res, input).tick(now()?);
    if res.quit {
        console!(log. "User closed the simulation.");
        return Ok(false);
    }
    if res.drawable {
        SimulationDrawer::new(ctx, materials, res).draw()?;
    }
    Ok(true)
}

fn set_event_listeners(state_owner: &Rc<StateOwner>) -> WebResult<Vec<OwnedClosure>> {
    let onblur: Closure<dyn FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |_: JsValue| {
            let mut input = state_owner.input.borrow_mut();
            *input = Input::new(now().unwrap_or(0.0));
        }))
    };

    let onkeydown: Closure<dyn FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<KeyboardEvent>() {
                let mut input = state_owner.input.borrow_mut();
                let used = on_button_action(&mut input, e.key().to_lowercase().as_ref(), true);
                if !used {
                    console!(log. format!("Ignored keydown: {}", e.key()));
                }
            }
        }))
    };

    let onkeyup: Closure<dyn FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<KeyboardEvent>() {
                let mut input = state_owner.input.borrow_mut();
                let used = on_button_action(&mut input, e.key().to_lowercase().as_ref(), false);
                if !used {
                    console!(log. format!("Ignored keyup: {}", e.key()));
                }
            }
        }))
    };

    let onmousedown: Closure<dyn FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                let mut input = state_owner.input.borrow_mut();
                input.mouse_click.input = e.buttons() == 1;
            }
        }))
    };

    let onmouseup: Closure<dyn FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if event.dyn_into::<MouseEvent>().is_ok() {
                let mut input = state_owner.input.borrow_mut();
                input.mouse_click.input = false;
            }
        }))
    };

    let onmousemove: Closure<dyn FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                let mut input = state_owner.input.borrow_mut();
                input.mouse_position_x = e.movement_x();
                input.mouse_position_y = e.movement_y();
            }
        }))
    };

    let onmousewheel: Closure<dyn FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<WheelEvent>() {
                let mut input = state_owner.input.borrow_mut();
                if input.canvas_focused {
                    input.mouse_scroll_y = e.delta_y() as f32;
                }
            }
        }))
    };

    let oncustominputevent: Closure<dyn FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            let mut input = state_owner.input.borrow_mut();
            if let Err(e) = read_custom_event(&mut input, event) {
                console!(error. "Could not read custom event.", e.into_js());
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
    document.add_event_listener_with_callback("keydown", onkeydown.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("keyup", onkeyup.as_ref().unchecked_ref())?;
    canvas.add_event_listener_with_callback("mousedown", onmousedown.as_ref().unchecked_ref())?;
    canvas.add_event_listener_with_callback("mouseup", onmouseup.as_ref().unchecked_ref())?;
    canvas.add_event_listener_with_callback("mousemove", onmousemove.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("mousewheel", onmousewheel.as_ref().unchecked_ref())?;
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
    let event_kind = js_sys::Reflect::get(&object, &"kind".into())?
        .as_string()
        .ok_or_else(|| WebError::Str("Could not get kind".into()))?;
    let event_value = match event_kind.as_ref() as &str {
        event_kind::FILTER_PRESET => InputEventValue::FilterPreset(value.as_string().ok_or("it should be a string")?),
        event_kind::PIXEL_BRIGHTNESS => InputEventValue::PixelBrighttness(value.as_f64().ok_or("it should be a number")? as f32),
        event_kind::PIXEL_CONTRAST => InputEventValue::PixelContrast(value.as_f64().ok_or("it should be a number")? as f32),
        event_kind::LIGHT_COLOR => InputEventValue::LightColor(value.as_f64().ok_or("it should be a number")? as i32),
        event_kind::BRIGHTNESS_COLOR => InputEventValue::BrightnessColor(value.as_f64().ok_or("it should be a number")? as i32),
        event_kind::BLUR_LEVEL => InputEventValue::BlurLevel(value.as_f64().ok_or("it should be a number")? as usize),
        event_kind::VERTICAL_LPP => InputEventValue::VerticalLpp(value.as_f64().ok_or("it should be a number")? as usize),
        event_kind::HORIZONTAL_LPP => InputEventValue::HorizontalLpp(value.as_f64().ok_or("it should be a number")? as usize),
        event_kind::PIXEL_SHADOW_HEIGHT => InputEventValue::PixelShadowHeight(value.as_f64().ok_or("it should be a number")? as f32),
        event_kind::BACKLIGHT_PERCENT => InputEventValue::BacklightPercent(value.as_f64().ok_or("it should be a number")? as f32),
        event_kind::PIXEL_VERTICAL_GAP => InputEventValue::PixelVerticalGap(value.as_f64().ok_or("it should be a number")? as f32),
        event_kind::PIXEL_HORIZONTAL_GAP => InputEventValue::PixelHorizontalGap(value.as_f64().ok_or("it should be a number")? as f32),
        event_kind::PIXEL_WIDTH => InputEventValue::PixelWidth(value.as_f64().ok_or("it should be a number")? as f32),
        event_kind::PIXEL_SPREAD => InputEventValue::PixelSpread(value.as_f64().ok_or("it should be a number")? as f32),
        event_kind::CAMERA_ZOOM => InputEventValue::Camera(CameraChange::Zoom(value.as_f64().ok_or("it should be a number")? as f32)),
        event_kind::CAMERA_POS_X => InputEventValue::Camera(CameraChange::PosX(value.as_f64().ok_or("it should be a number")? as f32)),
        event_kind::CAMERA_POS_Y => InputEventValue::Camera(CameraChange::PosY(value.as_f64().ok_or("it should be a number")? as f32)),
        event_kind::CAMERA_POS_Z => InputEventValue::Camera(CameraChange::PosZ(value.as_f64().ok_or("it should be a number")? as f32)),
        event_kind::CAMERA_AXIS_UP_X => InputEventValue::Camera(CameraChange::AxisUpX(value.as_f64().ok_or("it should be a number")? as f32)),
        event_kind::CAMERA_AXIS_UP_Y => InputEventValue::Camera(CameraChange::AxisUpY(value.as_f64().ok_or("it should be a number")? as f32)),
        event_kind::CAMERA_AXIS_UP_Z => InputEventValue::Camera(CameraChange::AxisUpZ(value.as_f64().ok_or("it should be a number")? as f32)),
        event_kind::CAMERA_DIRECTION_X => InputEventValue::Camera(CameraChange::DirectionX(value.as_f64().ok_or("it should be a number")? as f32)),
        event_kind::CAMERA_DIRECTION_Y => InputEventValue::Camera(CameraChange::DirectionY(value.as_f64().ok_or("it should be a number")? as f32)),
        event_kind::CAMERA_DIRECTION_Z => InputEventValue::Camera(CameraChange::DirectionZ(value.as_f64().ok_or("it should be a number")? as f32)),
        _ => InputEventValue::None,
    };
    input.custom_event.add_value(event_kind, event_value);
    Ok(())
}
