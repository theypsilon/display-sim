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
use web_sys::{WebGl2RenderingContext, Window};

use crate::console;
use crate::web_events::WebEventDispatcher;
use crate::web_utils::{now, window};
use app_error::{AppError, AppResult};
use core::camera::CameraChange;
use core::simulation_context::{ConcreteSimulationContext, RandomGenerator, SimulationContext};
use core::simulation_core_state::{frontend_event, Input, InputEventValue, Resources, VideoInputResources};
use core::simulation_core_ticker::SimulationCoreTicker;
use glow::GlowSafeAdapter;
use render::simulation_draw::SimulationDrawer;
use render::simulation_render_state::{Materials, VideoInputMaterials};

type OwnedClosure = Closure<dyn FnMut(JsValue)>;
pub type StateOwnerPtr = Rc<RefCell<Option<Box<StateOwner>>>>;

pub struct StateOwner {
    pub closures: Vec<OwnedClosure>,
    pub input: Input,
    pub materials: Materials,
    pub event_bus: JsValue,
    pub cancel_id: i32,
}

impl StateOwner {
    pub fn new_ptr(materials: Materials, input: Input, event_bus: JsValue) -> StateOwnerPtr {
        Rc::new(RefCell::new(Some(Box::new(StateOwner {
            closures: Vec::new(),
            materials,
            input,
            cancel_id: 0,
            event_bus,
        }))))
    }
}

pub fn web_entrypoint(
    webgl: JsValue,
    event_bus: JsValue,
    res: Rc<RefCell<Resources>>,
    video_input_resources: VideoInputResources,
    video_input_materials: VideoInputMaterials,
) -> AppResult<StateOwnerPtr> {
    let webgl = webgl.dyn_into::<WebGl2RenderingContext>()?;

    res.borrow_mut().initialize(video_input_resources, now()?);
    let gl = Rc::new(GlowSafeAdapter::new(glow::Context::from_webgl2_context(webgl.clone())));
    let owned_state = StateOwner::new_ptr(Materials::new(gl, video_input_materials)?, Input::new(now()?), event_bus.clone());
    let frame_closure: Closure<dyn FnMut(JsValue)> = {
        let owned_state = Rc::clone(&owned_state);
        let window = window()?;
        let event_bus = event_bus.clone();
        Closure::wrap(Box::new(move |_| {
            if let Some(owned_state) = &mut *owned_state.borrow_mut() {
                let mut ctx = ConcreteSimulationContext::new(WebEventDispatcher::new(webgl.clone(), event_bus.clone()), WebRnd {});
                if let Err(e) = web_entrypoint_iteration(owned_state, &mut *res.borrow_mut(), &window, &mut ctx) {
                    console!(error. "An unexpected error happened during web_entrypoint_iteration.", e);
                    ctx.dispatcher().dispatch_exiting_session();
                    ctx.dispatcher()
                        .dispatch_top_message("Error! Try restarting your browser. Contact me if this problem persists!");
                }
                if let Err(e) = ctx.dispatcher_instance.check_error() {
                    console!(error. "Error dispatching some events: ", e);
                }
            }
        }))
    };
    let owned_state_clone = owned_state.clone();
    if let Some(owned_state) = &mut *owned_state.borrow_mut() {
        owned_state.cancel_id = window()?.request_animation_frame(frame_closure.as_ref().unchecked_ref())?;
        owned_state.closures.push(frame_closure);

        let listeners = set_event_listeners(event_bus, owned_state_clone)?;
        owned_state.closures.extend(listeners);
    }

    Ok(owned_state)
}

pub fn stop_frame_loop(owner: StateOwnerPtr) -> AppResult<()> {
    if let Some(owner) = &mut *owner.borrow_mut() {
        window()?.cancel_animation_frame(owner.cancel_id)?;
        owner.cancel_id = 0;

        if let Some(onfrontendevent) = owner.closures.last() {
            let unsubscribe = js_sys::Reflect::get(&owner.event_bus, &"unsubscribe".into())?.dyn_into::<js_sys::Function>()?;
            let args = js_sys::Array::new();
            args.push(onfrontendevent.as_ref().unchecked_ref());
            unsubscribe.apply(&owner.event_bus, &args)?;
        }
    }
    let _ = owner.borrow_mut().take();
    Ok(())
}

pub fn print_error(e: AppError) {
    console!(error. "An unexpected error ocurred.", e);
}

fn web_entrypoint_iteration(owned_state: &mut StateOwner, resources: &mut Resources, window: &Window, ctx: &mut dyn SimulationContext) -> AppResult<()> {
    match tick(ctx, &mut owned_state.input, resources, &mut owned_state.materials) {
        Ok(true) => {
            if owned_state.cancel_id != 0 {
                owned_state.cancel_id = window.request_animation_frame(owned_state.closures[0].as_ref().as_ref().unchecked_ref())?;
            }
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

fn tick(ctx: &dyn SimulationContext, input: &mut Input, res: &mut Resources, materials: &mut Materials) -> AppResult<bool> {
    SimulationCoreTicker::new(ctx, res, input).tick(now()?)?;
    if res.quit {
        return Ok(false);
    }
    if res.drawable {
        SimulationDrawer::new(ctx, materials, res).draw()?;
    }
    Ok(true)
}

fn set_event_listeners(event_bus: JsValue, state_owner: StateOwnerPtr) -> AppResult<Vec<OwnedClosure>> {
    let onfrontendevent: Closure<dyn FnMut(JsValue)> = {
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Some(state_owner) = &mut *state_owner.borrow_mut() {
                let mut input = &mut state_owner.input;
                if let Err(e) = read_frontend_event(&mut input, event) {
                    console!(error. "Could not read custom event.", e);
                }
            }
        }))
    };
    let subscribe = js_sys::Reflect::get(&event_bus, &"subscribe".into())?.dyn_into::<js_sys::Function>()?;
    let args = js_sys::Array::new();
    args.push(onfrontendevent.as_ref().unchecked_ref());
    subscribe.apply(&event_bus, &args)?;
    Ok(vec![onfrontendevent])
}

pub fn read_frontend_event(input: &mut Input, event: JsValue) -> AppResult<()> {
    let value = js_sys::Reflect::get(&event, &"message".into())?;
    let frontend_event: AppResult<String> = js_sys::Reflect::get(&event, &"type".into())?.as_string().ok_or("Could not get kind".into());
    let frontend_event = frontend_event?;
    let event_value = match frontend_event.as_ref() as &str {
        frontend_event::KEYBOARD => {
            let pressed = js_sys::Reflect::get(&value, &"pressed".into())?.as_bool().ok_or("it should be a bool")?;
            let key = js_sys::Reflect::get(&value, &"key".into())?
                .as_string()
                .ok_or_else(|| format!("it should be a string, but was {:?}", value))?;
            InputEventValue::Keyboard { pressed, key }
        }
        frontend_event::MOUSE_CLICK => InputEventValue::MouseClick(value.as_bool().ok_or("it should be a bool")?),
        frontend_event::MOUSE_MOVE => {
            let x = js_sys::Reflect::get(&value, &"x".into())?.as_f64().ok_or("it should be a number")? as i32;
            let y = js_sys::Reflect::get(&value, &"y".into())?.as_f64().ok_or("it should be a number")? as i32;
            InputEventValue::MouseMove { x, y }
        }
        frontend_event::MOUSE_WHEEL => InputEventValue::MouseWheel(value.as_f64().ok_or("it should be a number")? as f32),
        frontend_event::BLURRED_WINDOW => InputEventValue::BlurredWindow,
        frontend_event::FILTER_PRESET => InputEventValue::FilterPreset(value.as_string().ok_or("it should be a string")?),
        frontend_event::PIXEL_BRIGHTNESS => InputEventValue::PixelBrighttness(value.as_f64().ok_or("it should be a number")? as f32),
        frontend_event::PIXEL_CONTRAST => InputEventValue::PixelContrast(value.as_f64().ok_or("it should be a number")? as f32),
        frontend_event::LIGHT_COLOR => InputEventValue::LightColor(value.as_f64().ok_or("it should be a number")? as i32),
        frontend_event::BRIGHTNESS_COLOR => InputEventValue::BrightnessColor(value.as_f64().ok_or("it should be a number")? as i32),
        frontend_event::BLUR_LEVEL => InputEventValue::BlurLevel(value.as_f64().ok_or("it should be a number")? as usize),
        frontend_event::VERTICAL_LPP => InputEventValue::VerticalLpp(value.as_f64().ok_or("it should be a number")? as usize),
        frontend_event::HORIZONTAL_LPP => InputEventValue::HorizontalLpp(value.as_f64().ok_or("it should be a number")? as usize),
        frontend_event::PIXEL_SHADOW_HEIGHT => InputEventValue::PixelShadowHeight(value.as_f64().ok_or("it should be a number")? as f32),
        frontend_event::BACKLIGHT_PERCENT => InputEventValue::BacklightPercent(value.as_f64().ok_or("it should be a number")? as f32),
        frontend_event::PIXEL_VERTICAL_GAP => InputEventValue::PixelVerticalGap(value.as_f64().ok_or("it should be a number")? as f32),
        frontend_event::PIXEL_HORIZONTAL_GAP => InputEventValue::PixelHorizontalGap(value.as_f64().ok_or("it should be a number")? as f32),
        frontend_event::PIXEL_WIDTH => InputEventValue::PixelWidth(value.as_f64().ok_or("it should be a number")? as f32),
        frontend_event::PIXEL_SPREAD => InputEventValue::PixelSpread(value.as_f64().ok_or("it should be a number")? as f32),
        frontend_event::CAMERA_ZOOM => InputEventValue::Camera(CameraChange::Zoom(value.as_f64().ok_or("it should be a number")? as f32)),
        frontend_event::CAMERA_POS_X => InputEventValue::Camera(CameraChange::PosX(value.as_f64().ok_or("it should be a number")? as f32)),
        frontend_event::CAMERA_POS_Y => InputEventValue::Camera(CameraChange::PosY(value.as_f64().ok_or("it should be a number")? as f32)),
        frontend_event::CAMERA_POS_Z => InputEventValue::Camera(CameraChange::PosZ(value.as_f64().ok_or("it should be a number")? as f32)),
        frontend_event::CAMERA_AXIS_UP_X => InputEventValue::Camera(CameraChange::AxisUpX(value.as_f64().ok_or("it should be a number")? as f32)),
        frontend_event::CAMERA_AXIS_UP_Y => InputEventValue::Camera(CameraChange::AxisUpY(value.as_f64().ok_or("it should be a number")? as f32)),
        frontend_event::CAMERA_AXIS_UP_Z => InputEventValue::Camera(CameraChange::AxisUpZ(value.as_f64().ok_or("it should be a number")? as f32)),
        frontend_event::CAMERA_DIRECTION_X => InputEventValue::Camera(CameraChange::DirectionX(value.as_f64().ok_or("it should be a number")? as f32)),
        frontend_event::CAMERA_DIRECTION_Y => InputEventValue::Camera(CameraChange::DirectionY(value.as_f64().ok_or("it should be a number")? as f32)),
        frontend_event::CAMERA_DIRECTION_Z => InputEventValue::Camera(CameraChange::DirectionZ(value.as_f64().ok_or("it should be a number")? as f32)),
        _ => return Err(format!("Can't read frontend_event: {}", frontend_event).into()),
    };
    input.custom_event.add_value(event_value);
    Ok(())
}
