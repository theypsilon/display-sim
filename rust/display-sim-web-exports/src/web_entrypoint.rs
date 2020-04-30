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
use web_sys::WebGl2RenderingContext;

use crate::console;
use crate::web_events::WebEventDispatcher;
use crate::web_utils::now;
use app_error::{AppError, AppResult};
use core::camera::CameraChange;
use core::input_types::{frontend_event, Input, InputEventValue, Pressed, RgbChange};
use core::simulation_context::{ConcreteSimulationContext, RandomGenerator, SimulationContext};
use core::simulation_core_state::{Resources, VideoInputResources};
use core::simulation_core_ticker::SimulationCoreTicker;
use glow::GlowSafeAdapter;
use render::simulation_draw::SimulationDrawer;
use render::simulation_render_state::{Materials, VideoInputMaterials};

type OwnedClosure = Closure<dyn FnMut(JsValue)>;

pub(crate) struct InputOutput {
    event_bus_subscriber: OwnedClosure,
    input: Input,
    materials: Materials,
    event_bus: JsValue,
    webgl: WebGl2RenderingContext,
    events: Rc<RefCell<Vec<JsValue>>>,
}

pub(crate) fn web_load(
    res: &mut Resources,
    webgl: JsValue,
    event_bus: JsValue,
    input_resources: VideoInputResources,
    input_materials: VideoInputMaterials,
) -> AppResult<InputOutput> {
    let webgl = webgl.dyn_into::<WebGl2RenderingContext>()?;
    let gl = Rc::new(GlowSafeAdapter::new(glow::Context::from_webgl2_context(webgl.clone())));

    res.initialize(input_resources, now()?);
    let (events, event_bus_subscriber) = set_event_listeners(event_bus.clone())?;
    Ok(InputOutput {
        input: Input::new(now()?),
        materials: Materials::new(gl, input_materials)?,
        event_bus,
        webgl,
        event_bus_subscriber,
        events,
    })
}

pub(crate) fn web_unload(io: InputOutput) -> AppResult<()> {
    let unsubscribe = js_sys::Reflect::get(&io.event_bus, &"unsubscribe".into())?.dyn_into::<js_sys::Function>()?;
    let args = js_sys::Array::new();
    args.push(io.event_bus_subscriber.as_ref().unchecked_ref());
    unsubscribe.apply(&io.event_bus, &args)?;
    Ok(())
}

pub(crate) fn web_run_frame(res: &mut Resources, io: &mut InputOutput) -> AppResult<bool> {
    for event in io.events.borrow_mut().drain(0..) {
        read_frontend_event(&mut io.input, event)?;
    }
    let ctx = ConcreteSimulationContext::new(WebEventDispatcher::new(io.webgl.clone(), io.event_bus.clone()), WebRnd {});
    let condition = tick(&ctx, &mut io.input, res, &mut io.materials)?;
    ctx.dispatcher_instance.check_error()?;
    Ok(condition)
}

pub(crate) fn print_error(e: AppError) {
    console!(error. "An unexpected error ocurred.", e);
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

fn set_event_listeners(event_bus: JsValue) -> AppResult<(Rc<RefCell<Vec<JsValue>>>, OwnedClosure)> {
    let events = Rc::new(RefCell::new(vec![]));
    let onfrontendevent: Closure<dyn FnMut(JsValue)> = {
        let events = events.clone();
        Closure::wrap(Box::new(move |event: JsValue| {
            events.borrow_mut().push(event);
        }))
    };
    let subscribe = js_sys::Reflect::get(&event_bus, &"subscribe".into())?.dyn_into::<js_sys::Function>()?;
    let args = js_sys::Array::new();
    args.push(onfrontendevent.as_ref().unchecked_ref());
    subscribe.apply(&event_bus, &args)?;
    Ok((events, onfrontendevent))
}

fn read_frontend_event(input: &mut Input, event: JsValue) -> AppResult<()> {
    let value = js_sys::Reflect::get(&event, &"message".into())?;
    let frontend_event: AppResult<String> = js_sys::Reflect::get(&event, &"type".into())?.as_string().ok_or("Could not get kind".into());
    let frontend_event = frontend_event?;
    let event_value = match frontend_event.as_ref() as &str {
        frontend_event::KEYBOARD => {
            let pressed = js_sys::Reflect::get(&value, &"pressed".into())?.as_bool().ok_or("it should be a bool")?;
            let pressed = if pressed { Pressed::Yes } else { Pressed::No };
            let key = js_sys::Reflect::get(&value, &"key".into())?
                .as_string()
                .ok_or_else(|| format!("it should be a string, but was {:?}", value))?;
            InputEventValue::Keyboard { pressed, key }
        }
        frontend_event::MOUSE_CLICK => {
            let pressed = value.as_bool().ok_or("it should be a bool")?;
            let pressed = if pressed { Pressed::Yes } else { Pressed::No };
            InputEventValue::MouseClick(pressed)
        }
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
        frontend_event::RGB_RED_R => InputEventValue::Rgb(RgbChange::RedR(value.as_f64().ok_or("it should be a number")? as f32)),
        frontend_event::RGB_RED_G => InputEventValue::Rgb(RgbChange::RedG(value.as_f64().ok_or("it should be a number")? as f32)),
        frontend_event::RGB_RED_B => InputEventValue::Rgb(RgbChange::RedB(value.as_f64().ok_or("it should be a number")? as f32)),
        frontend_event::RGB_GREEN_R => InputEventValue::Rgb(RgbChange::GreenR(value.as_f64().ok_or("it should be a number")? as f32)),
        frontend_event::RGB_GREEN_G => InputEventValue::Rgb(RgbChange::GreenG(value.as_f64().ok_or("it should be a number")? as f32)),
        frontend_event::RGB_GREEN_B => InputEventValue::Rgb(RgbChange::GreenB(value.as_f64().ok_or("it should be a number")? as f32)),
        frontend_event::RGB_BLUE_R => InputEventValue::Rgb(RgbChange::BlueR(value.as_f64().ok_or("it should be a number")? as f32)),
        frontend_event::RGB_BLUE_G => InputEventValue::Rgb(RgbChange::BlueG(value.as_f64().ok_or("it should be a number")? as f32)),
        frontend_event::RGB_BLUE_B => InputEventValue::Rgb(RgbChange::BlueB(value.as_f64().ok_or("it should be a number")? as f32)),
        frontend_event::COLOR_GAMMA => InputEventValue::ColorGamma(value.as_f64().ok_or("it should be a number")? as f32),
        frontend_event::COLOR_NOISE => InputEventValue::ColorNoise(value.as_f64().ok_or("it should be a number")? as f32),
        frontend_event::CUSTOM_SCALING_RESOLUTION_WIDTH => InputEventValue::CustomScalingResolutionWidth(value.as_f64().ok_or("it should be a number")? as f32),
        frontend_event::CUSTOM_SCALING_RESOLUTION_HEIGHT => {
            InputEventValue::CustomScalingResolutionHeight(value.as_f64().ok_or("it should be a number")? as f32)
        }
        frontend_event::CUSTOM_SCALING_ASPECT_RATIO_X => InputEventValue::CustomScalingAspectRatioX(value.as_f64().ok_or("it should be a number")? as f32),
        frontend_event::CUSTOM_SCALING_ASPECT_RATIO_Y => InputEventValue::CustomScalingAspectRatioY(value.as_f64().ok_or("it should be a number")? as f32),
        frontend_event::CUSTOM_SCALING_STRETCH_NEAREST => InputEventValue::CustomScalingStretchNearest(value.as_bool().ok_or("it should be a bool")?),
        frontend_event::VIEWPORT_RESIZE => InputEventValue::ViewportResize(
            js_sys::Reflect::get(&value, &"width".into())?.as_f64().ok_or("it should contain width")? as u32,
            js_sys::Reflect::get(&value, &"height".into())?.as_f64().ok_or("it should contain height")? as u32,
        ),
        _ => return Err(format!("Can't read frontend_event: {}", frontend_event).into()),
    };
    input.push_event(event_value);
    Ok(())
}
