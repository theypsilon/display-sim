/* Copyright (c) 2019-2024 José manuel Barroso Galindo <theypsilon@gmail.com>
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
use std::sync::Arc;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::WebGl2RenderingContext;

use crate::console;
use crate::dispatch_event::dispatch_event_with;
use crate::web_egui_input::WebEguiInput;
use crate::web_events::WebEventDispatcher;
use crate::web_utils::now;
use app_util::{AppError, AppResult};
use core::camera::CameraChange;
use core::input_types::Input;
use core::simulation_command::{ControllerValue, Pressed, SimulationCommand, SimulationCommandBus};
use core::simulation_context::{ConcreteSimulationContext, RandomGenerator, SimulationContext};
use core::simulation_core_state::{KeyEventKind, Resources, VideoInputResources};
use core::simulation_core_ticker::SimulationCoreTicker;
use glow::GlowSafeAdapter;
use render::simulation_draw::{present_to_default_framebuffer, SimulationDrawer};
use render::simulation_render_state::{Materials, VideoInputMaterials};
use sim_ui::{shared_panel_events, SharedPanelEvents, SimPanel};

type OwnedClosure = Closure<dyn FnMut(JsValue)>;

pub(crate) struct InputOutput {
    event_bus_subscriber: OwnedClosure,
    input: Input,
    commands: SimulationCommandBus,
    materials: Materials,
    event_bus: JsValue,
    events: Rc<RefCell<Vec<JsValue>>>,
    panel: SimPanel,
    panel_events: SharedPanelEvents,
    egui_input: WebEguiInput,
    painter: egui_glow::Painter,
    input_focused: bool,
    last_cursor: Option<egui::CursorIcon>,
    last_ime: Option<egui::output::IMEOutput>,
    has_simulation_frame: bool,
    panel_enabled: bool,
    panel_clear_pending: bool,
    toast_was_painted: bool,
}

pub(crate) fn web_load(
    res: &mut Resources,
    webgl: JsValue,
    event_bus: JsValue,
    input_resources: VideoInputResources,
    input_materials: VideoInputMaterials,
) -> AppResult<InputOutput> {
    let webgl = webgl.dyn_into::<WebGl2RenderingContext>()?;
    let gl_context = Arc::new(glow::Context::from_webgl2_context(webgl));
    let gl = Rc::new(GlowSafeAdapter::from_shared(Arc::clone(&gl_context)));
    let painter = egui_glow::Painter::new(gl_context, "", None, false).map_err(|error| format!("Could not create web egui painter: {error}"))?;
    let panel_events = shared_panel_events();

    res.initialize(input_resources, now()?);
    let (events, event_bus_subscriber) = set_event_listeners(event_bus.clone())?;
    Ok(InputOutput {
        input: Input::new(now()?),
        commands: SimulationCommandBus::default(),
        materials: Materials::new(gl, input_materials)?,
        event_bus,
        event_bus_subscriber,
        events,
        panel: SimPanel::new(),
        panel_events,
        egui_input: WebEguiInput::default(),
        painter,
        input_focused: false,
        last_cursor: None,
        last_ime: None,
        has_simulation_frame: false,
        panel_enabled: true,
        panel_clear_pending: false,
        toast_was_painted: false,
    })
}

pub(crate) fn web_unload(mut io: InputOutput) -> AppResult<()> {
    let unsubscribe = js_sys::Reflect::get(&io.event_bus, &"unsubscribe".into())?.dyn_into::<js_sys::Function>()?;
    let args = js_sys::Array::new();
    args.push(io.event_bus_subscriber.as_ref().unchecked_ref());
    unsubscribe.apply(&io.event_bus, &args)?;
    io.painter.destroy();
    Ok(())
}

pub(crate) fn web_run_frame(res: &mut Resources, io: &mut InputOutput) -> AppResult<bool> {
    for event in io.events.borrow_mut().drain(0..) {
        read_frontend_event(&mut io.commands, res, event)?;
    }
    let frame_now = now()?;
    let raw_input = io.egui_input.take_input(frame_now / 1000.0);
    let mut egui_output = io.panel.run_with_controls(raw_input, res, &mut io.commands, &io.panel_events, io.panel_enabled);
    let toast_visible = io.panel.has_active_toast();
    if io.panel_enabled {
        let input_focused = io.panel.context().egui_wants_keyboard_input();
        if input_focused != io.input_focused {
            io.input_focused = input_focused;
            io.commands.emit(SimulationCommand::Keyboard {
                pressed: Pressed::from_bool(input_focused),
                key: "input_focused".into(),
            });
        }
        handle_platform_output(io, &mut egui_output.platform_output)?;
    }

    let ctx = ConcreteSimulationContext::new(
        WebEventDispatcher::new(io.materials.gl.clone(), io.event_bus.clone(), io.panel_events.clone()),
        WebRnd {},
    );
    let (condition, drew_simulation) = tick(&ctx, &mut io.input, &mut io.commands, res, &mut io.materials)?;
    io.has_simulation_frame |= drew_simulation;
    ctx.dispatcher_instance.check_error()?;

    if condition {
        if io.panel_clear_pending || toast_visible || io.toast_was_painted {
            if !drew_simulation {
                if io.has_simulation_frame {
                    present_to_default_framebuffer(&mut io.materials, res)?;
                } else {
                    io.materials.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
                    io.materials.gl.disable(glow::SCISSOR_TEST);
                    io.materials
                        .gl
                        .viewport(0, 0, res.video.viewport_size.width as i32, res.video.viewport_size.height as i32);
                    io.materials.gl.clear_color(0.0, 0.0, 0.0, 1.0);
                    io.materials.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                }
            }
            io.panel_clear_pending = false;
        }
        if res.quit && io.has_simulation_frame {
            present_to_default_framebuffer(&mut io.materials, res)?;
        }
        let width = res.video.viewport_size.width;
        let height = res.video.viewport_size.height;
        io.materials.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        io.materials.gl.viewport(0, 0, width as i32, height as i32);
        if res.quit && !io.has_simulation_frame {
            io.materials.gl.clear_color(0.0, 0.0, 0.0, 1.0);
            io.materials.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
        let primitives = io
            .panel
            .context()
            .tessellate(std::mem::take(&mut egui_output.shapes), egui_output.pixels_per_point);
        io.painter
            .paint_and_update_textures([width, height], egui_output.pixels_per_point, &primitives, &mut egui_output.textures_delta);
        io.materials.gl.disable(glow::BLEND);
        io.materials.gl.enable(glow::DEPTH_TEST);
        io.toast_was_painted = toast_visible;
    } else {
        egui_output.drop_without_applying_deltas();
    }
    Ok(condition)
}

pub(crate) fn web_set_ui_metrics(io: &mut InputOutput, width: u32, height: u32, pixels_per_point: f32) {
    io.egui_input.set_metrics(width, height, pixels_per_point);
}

pub(crate) fn web_set_panel_enabled(io: &mut InputOutput, enabled: bool) {
    if io.panel_enabled == enabled {
        return;
    }
    if !enabled {
        io.panel.release_all(&mut io.commands);
        io.panel_clear_pending = true;
        if io.input_focused {
            io.input_focused = false;
            io.commands.emit(SimulationCommand::Keyboard {
                pressed: Pressed::No,
                key: "input_focused".into(),
            });
        }
        io.last_cursor = None;
        io.last_ime = None;
    }
    io.egui_input.set_panel_enabled(enabled);
    io.panel_enabled = enabled;
}

pub(crate) fn web_ui_event(io: &mut InputOutput, kind: &str, value: &JsValue) -> AppResult<()> {
    if io.panel_enabled || kind == "focus" {
        io.egui_input.on_event(kind, value, io.panel.panel_rect())
    } else {
        Ok(())
    }
}

pub(crate) fn web_ui_captures_pointer(io: &InputOutput) -> bool {
    io.panel_enabled && (io.egui_input.pointer_is_captured(io.panel.panel_rect()) || io.panel.context().egui_is_using_pointer())
}

pub(crate) fn web_ui_wants_keyboard(io: &InputOutput) -> bool {
    io.panel_enabled && io.panel.context().egui_wants_keyboard_input()
}

pub(crate) fn web_ui_message(io: &mut InputOutput, message: &str) {
    io.panel_events.borrow_mut().push_message(message);
}

fn handle_platform_output(io: &mut InputOutput, output: &mut egui::PlatformOutput) -> AppResult<()> {
    for command in std::mem::take(&mut output.commands) {
        if let egui::OutputCommand::CopyText(text) = command {
            dispatch_event_with(&io.event_bus, "back2front:ui-copy", &text.into())?;
        }
    }

    if io.last_cursor != Some(output.cursor_icon) {
        io.last_cursor = Some(output.cursor_icon);
        dispatch_event_with(&io.event_bus, "back2front:ui-cursor", &css_cursor(output.cursor_icon).into())?;
    }

    if io.last_ime != output.ime {
        io.last_ime = output.ime;
        let message = js_sys::Object::new();
        if let Some(ime) = output.ime {
            js_sys::Reflect::set(&message, &"active".into(), &true.into())?;
            js_sys::Reflect::set(&message, &"x".into(), &(ime.cursor_rect.left() as f64).into())?;
            js_sys::Reflect::set(&message, &"y".into(), &(ime.cursor_rect.bottom() as f64).into())?;
        } else {
            js_sys::Reflect::set(&message, &"active".into(), &false.into())?;
        }
        dispatch_event_with(&io.event_bus, "back2front:ui-ime", &message)?;
    }
    Ok(())
}

fn css_cursor(icon: egui::CursorIcon) -> &'static str {
    use egui::CursorIcon as C;
    match icon {
        C::Default => "default",
        C::ContextMenu => "context-menu",
        C::Help => "help",
        C::PointingHand => "pointer",
        C::Progress => "progress",
        C::Wait => "wait",
        C::Cell => "cell",
        C::Crosshair => "crosshair",
        C::Text => "text",
        C::VerticalText => "vertical-text",
        C::Alias => "alias",
        C::Copy => "copy",
        C::Move => "move",
        C::NoDrop => "no-drop",
        C::NotAllowed => "not-allowed",
        C::Grab => "grab",
        C::Grabbing => "grabbing",
        C::AllScroll => "all-scroll",
        C::ResizeHorizontal | C::ResizeColumn => "ew-resize",
        C::ResizeVertical | C::ResizeRow => "ns-resize",
        C::ResizeNeSw => "nesw-resize",
        C::ResizeNwSe => "nwse-resize",
        C::ResizeEast => "e-resize",
        C::ResizeSouthEast => "se-resize",
        C::ResizeSouth => "s-resize",
        C::ResizeSouthWest => "sw-resize",
        C::ResizeWest => "w-resize",
        C::ResizeNorthWest => "nw-resize",
        C::ResizeNorth => "n-resize",
        C::ResizeNorthEast => "ne-resize",
        C::ZoomIn => "zoom-in",
        C::ZoomOut => "zoom-out",
        C::None => "none",
    }
}

pub(crate) fn print_error(e: AppError) {
    console!(error. "An unexpected error ocurred.", e);
}

struct WebRnd {}

impl RandomGenerator for WebRnd {
    fn next(&self) -> f32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(0.0..=1.0)
    }
}

fn tick(
    ctx: &dyn SimulationContext,
    input: &mut Input,
    commands: &mut SimulationCommandBus,
    res: &mut Resources,
    materials: &mut Materials,
) -> AppResult<(bool, bool)> {
    if !res.quit {
        SimulationCoreTicker::new(ctx, res, input, commands).tick(now()?)?;
    }
    if res.quit {
        return Ok((true, false));
    }
    if res.drawable {
        let drew_simulation = res.video.drawing_activation;
        SimulationDrawer::new(ctx, materials, res).draw()?;
        return Ok((true, drew_simulation));
    }
    Ok((true, false))
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

fn read_frontend_event(commands: &mut SimulationCommandBus, res: &Resources, event: JsValue) -> AppResult<()> {
    let value = js_sys::Reflect::get(&event, &"message".into())?;
    let frontend_event: AppResult<String> = js_sys::Reflect::get(&event, &"type".into())?.as_string().ok_or("Could not get kind".into());
    let frontend_event = frontend_event?;
    if matches!(res.controller_events.get(frontend_event.as_str()), Some((KeyEventKind::Set, _))) {
        let value = if let Some(number) = value.as_f64() {
            ControllerValue::Number(number)
        } else if let Some(text) = value.as_string() {
            ControllerValue::Text(text)
        } else {
            return Err(format!("controller event {frontend_event} requires a number or string").into());
        };
        commands.emit(SimulationCommand::controller_set(frontend_event, value));
        return Ok(());
    }
    let event_value = match frontend_event.as_ref() as &str {
        "front2back:keyboard" => {
            let pressed = js_sys::Reflect::get(&value, &"pressed".into())?.as_bool().ok_or("it should be a bool")?;
            let pressed = if pressed { Pressed::Yes } else { Pressed::No };
            let key = js_sys::Reflect::get(&value, &"key".into())?
                .as_string()
                .ok_or_else(|| format!("it should be a string, but was {:?}", value))?;
            SimulationCommand::Keyboard { pressed, key }
        }
        "front2back:mouse-click" => {
            let pressed = value.as_bool().ok_or("it should be a bool")?;
            let pressed = if pressed { Pressed::Yes } else { Pressed::No };
            SimulationCommand::MouseClick(pressed)
        }
        "front2back:mouse-move" => {
            let x = js_sys::Reflect::get(&value, &"x".into())?.as_f64().ok_or("it should be a number")? as i32;
            let y = js_sys::Reflect::get(&value, &"y".into())?.as_f64().ok_or("it should be a number")? as i32;
            SimulationCommand::MouseMove { x, y }
        }
        "front2back:mouse-wheel" => SimulationCommand::MouseWheel(value.as_f64().ok_or("it should be a number")? as f32),
        "front2back:blurred-window" => SimulationCommand::BlurredWindow,
        "front2back:pixel-width" => SimulationCommand::PixelWidth(value.as_f64().ok_or("it should be a number")? as f32),
        "front2back:camera_zoom" => SimulationCommand::Camera(CameraChange::Zoom(value.as_f64().ok_or("it should be a number")? as f32)),
        "front2back:camera-pos-x" => SimulationCommand::Camera(CameraChange::PosX(value.as_f64().ok_or("it should be a number")? as f32)),
        "front2back:camera-pos-y" => SimulationCommand::Camera(CameraChange::PosY(value.as_f64().ok_or("it should be a number")? as f32)),
        "front2back:camera-pos-z" => SimulationCommand::Camera(CameraChange::PosZ(value.as_f64().ok_or("it should be a number")? as f32)),
        "front2back:camera-axis-up-x" => SimulationCommand::Camera(CameraChange::AxisUpX(value.as_f64().ok_or("it should be a number")? as f32)),
        "front2back:camera-axis-up-y" => SimulationCommand::Camera(CameraChange::AxisUpY(value.as_f64().ok_or("it should be a number")? as f32)),
        "front2back:camera-axis-up-z" => SimulationCommand::Camera(CameraChange::AxisUpZ(value.as_f64().ok_or("it should be a number")? as f32)),
        "front2back:camera-dir-x" => SimulationCommand::Camera(CameraChange::DirectionX(value.as_f64().ok_or("it should be a number")? as f32)),
        "front2back:camera-dir-y" => SimulationCommand::Camera(CameraChange::DirectionY(value.as_f64().ok_or("it should be a number")? as f32)),
        "front2back:camera-dir-z" => SimulationCommand::Camera(CameraChange::DirectionZ(value.as_f64().ok_or("it should be a number")? as f32)),
        "front2back:custom-scaling-resolution-width" => SimulationCommand::CustomScalingResolutionWidth(value.as_f64().ok_or("it should be a number")? as f32),
        "front2back:custom-scaling-resolution-height" => {
            SimulationCommand::CustomScalingResolutionHeight(value.as_f64().ok_or("it should be a number")? as f32)
        }
        "front2back:custom-scaling-aspect-ratio-x" => SimulationCommand::CustomScalingAspectRatioX(value.as_f64().ok_or("it should be a number")? as f32),
        "front2back:custom-scaling-aspect-ratio-y" => SimulationCommand::CustomScalingAspectRatioY(value.as_f64().ok_or("it should be a number")? as f32),
        "front2back:custom-scaling-stretch-nearest" => SimulationCommand::CustomScalingStretchNearest(value.as_bool().ok_or("it should be a bool")?),
        "front2back:viewport-resize" => SimulationCommand::ViewportResize(
            js_sys::Reflect::get(&value, &"width".into())?.as_f64().ok_or("it should contain width")? as u32,
            js_sys::Reflect::get(&value, &"height".into())?.as_f64().ok_or("it should contain height")? as u32,
        ),
        _ => return Err(format!("Can't read frontend_event: {}", frontend_event).into()),
    };
    commands.emit(event_value);
    Ok(())
}
