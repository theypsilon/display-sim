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

use core::app_events::AppEventDispatcher;
use core::camera::CameraLockMode;
use core::general_types::Size2D;
use core::input_types::{Input, InputEventValue, Pressed};
use core::simulation_context::{ConcreteSimulationContext, RandomGenerator};
use core::simulation_core_state::ScalingMethod;
use core::simulation_core_state::{AnimationStep, Resources, VideoInputResources};
use core::simulation_core_ticker::SimulationCoreTicker;
use render::error::AppResult;
use render::simulation_draw::{present_to_default_framebuffer, SimulationDrawer};
use render::simulation_render_state::{Materials, VideoInputMaterials};
use sim_ui::{shared_panel_events, SharedPanelEvents, SimPanel, SimPanelSection};

use std::cell::Cell;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use glutin::dpi::LogicalSize;
use glutin::event::{DeviceEvent, ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::monitor::MonitorHandle;
use glutin::window::{Fullscreen, WindowBuilder};
use glutin::{ContextBuilder, GlProfile, GlRequest, PossiblyCurrent, Robustness, WindowedContext};

use glow::GlowSafeAdapter;

use crate::simulation_input::{browser_wheel_delta, SimulationKeyboardInput, SimulationPointerInput};
use crate::winit_egui::WinitEguiInput;

pub fn main() {
    if let Err(e) = program() {
        println!("Error: {:?}", e);
        std::process::exit(-1);
    }
}

struct NativeRnd {}

impl RandomGenerator for NativeRnd {
    fn next(&self) -> f32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(0.0..=1.0)
    }
}

fn program() -> AppResult<()> {
    println!("Initializing Window.");
    let winit_loop = EventLoop::new();
    let monitor = winit_loop.primary_monitor().or_else(|| winit_loop.available_monitors().next());
    let window_size = if let Some(monitor) = &monitor {
        let mut size = monitor.size().to_logical::<f64>(monitor.scale_factor());
        size.width *= 0.8;
        size.height *= 0.8;
        size
    } else {
        println!("No monitor metadata available; using a 1280x720 window.");
        LogicalSize::new(1280.0, 720.0)
    };

    let wb = WindowBuilder::new()
        .with_inner_size(window_size)
        .with_visible(true)
        .with_decorations(true)
        .with_resizable(true)
        .with_title("Display Sim");

    let windowed_ctx = ContextBuilder::new()
        .with_gl(GlRequest::Latest)
        .with_gl_profile(GlProfile::Core)
        .with_gl_robustness(Robustness::NotRobust)
        .with_gl_debug_flag(false)
        .with_hardware_acceleration(Some(true))
        .with_vsync(false)
        .with_multisampling(4)
        .with_depth_buffer(24)
        .build_windowed(wb, &winit_loop)
        .map_err(|e| format!("{}", e))?;

    let windowed_ctx = unsafe { windowed_ctx.make_current().map_err(|e| format!("Context Error: {:?}", e))? };
    let windowed_ctx = Rc::new(windowed_ctx);
    let gl_ctx = Arc::new(unsafe { glow::Context::from_loader_function(|ptr| windowed_ctx.context().get_proc_address(ptr) as *const _) });
    println!("Pixel format of the window's GL context: {:?}", windowed_ctx.get_pixel_format());

    let img_path = "www/assets/pics/frames/seiken.png";
    println!("Loading image: {}", img_path);
    let img = image::open(img_path).map_err(|e| format!("{}", e))?.to_rgba8();
    let img_size = img.dimensions();
    let pixels = img.into_vec().into_boxed_slice();

    let physical_size = windowed_ctx.window().inner_size();
    let res_input = VideoInputResources {
        steps: vec![AnimationStep { delay: 16 }],
        max_texture_size: std::i32::MAX,
        image_size: Size2D {
            width: img_size.0,
            height: img_size.1,
        },
        background_size: Size2D {
            width: img_size.0,
            height: img_size.1,
        },
        viewport_size: Size2D {
            width: physical_size.width,
            height: physical_size.height,
        },
        current_frame: 0,
        preset: None,
        last_frame_change: 0.0,
        needs_buffer_data_load: true,
        drawing_activation: true,
    };
    let materials_input = VideoInputMaterials { buffers: vec![pixels] };

    println!("Preparing resources.");
    let mut res = Resources::default();
    res.initialize(res_input, 0.0);
    println!("Preparing materials.");
    let adapter = Rc::new(GlowSafeAdapter::from_shared(Arc::clone(&gl_ctx)));
    let materials = Materials::new(adapter.clone(), materials_input)?;
    let painter = egui_glow::Painter::new(gl_ctx, "", None, false).map_err(|error| format!("Could not create egui painter: {error}"))?;

    println!("Preparing input.");
    let input = Input::new(0.0);
    let egui_input = WinitEguiInput::new(windowed_ctx.window());
    let mut panel = SimPanel::new();
    if std::env::var_os("DISPLAY_SIM_CAPTURE_UI").is_some() {
        let section = std::env::var("DISPLAY_SIM_UI_SECTION").ok().and_then(|name| match name.as_str() {
            "presets" => Some(SimPanelSection::Presets),
            "image-scaling" => Some(SimPanelSection::ImageScaling),
            "performance" => Some(SimPanelSection::Performance),
            "colors" => Some(SimPanelSection::Colors),
            "geometry-and-textures" => Some(SimPanelSection::GeometryAndTextures),
            "camera" => Some(SimPanelSection::Camera),
            "command-modifiers" => Some(SimPanelSection::CommandModifiers),
            "webgl-settings" => Some(SimPanelSection::WebGlSettings),
            "extra" => Some(SimPanelSection::Extra),
            _ => None,
        });
        if let Some(section) = section {
            panel.open_only(section);
        }
    }
    let panel_events = shared_panel_events();
    println!("Preparing simulation context.");
    let sim_ctx = ConcreteSimulationContext::new(NativeEventDispatcher::new(windowed_ctx.clone(), adapter, panel_events.clone()), NativeRnd {});

    let timings = Timings::new(Instant::now(), Duration::from_secs_f64(1.0 / 60.0));

    let mut state = NativeSimulationState::new(
        sim_ctx,
        windowed_ctx,
        monitor,
        res,
        input,
        materials,
        timings,
        panel,
        panel_events,
        egui_input,
        painter,
    );

    winit_loop.run(move |event, _, control_flow| match state.iteration(event, control_flow) {
        Ok(()) => {}
        Err(e) => {
            println!("Main iteration error: {}", e);
            *control_flow = ControlFlow::Exit;
        }
    });
}

struct NativeSimulationState {
    sim_ctx: ConcreteSimulationContext<NativeEventDispatcher, NativeRnd>,
    windowed_ctx: Rc<WindowedContext<PossiblyCurrent>>,
    monitor: Option<MonitorHandle>,
    res: Resources,
    input: Input,
    materials: Materials,
    timings: Timings,
    panel: SimPanel,
    panel_events: SharedPanelEvents,
    egui_input: WinitEguiInput,
    painter: Option<egui_glow::Painter>,
    ui_capture_path: Option<PathBuf>,
    has_simulation_frame: bool,
    simulation_pointer: SimulationPointerInput,
    canvas_focused: bool,
    input_focused: bool,
    window_focused: bool,
    interactions_reset: bool,
    simulation_keyboard: SimulationKeyboardInput,
}

struct Timings {
    starting_time: Instant,
    framerate: Duration,
    last_time: Instant,
}

impl Timings {
    pub fn new(starting_time: Instant, framerate: Duration) -> Self {
        Timings {
            starting_time,
            framerate,
            last_time: starting_time - framerate,
        }
    }
}

impl NativeSimulationState {
    pub fn new(
        sim_ctx: ConcreteSimulationContext<NativeEventDispatcher, NativeRnd>,
        windowed_ctx: Rc<WindowedContext<PossiblyCurrent>>,
        monitor: Option<MonitorHandle>,
        res: Resources,
        input: Input,
        materials: Materials,
        timings: Timings,
        panel: SimPanel,
        panel_events: SharedPanelEvents,
        egui_input: WinitEguiInput,
        painter: egui_glow::Painter,
    ) -> Self {
        NativeSimulationState {
            sim_ctx,
            windowed_ctx,
            monitor,
            res,
            input,
            materials,
            timings,
            panel,
            panel_events,
            egui_input,
            painter: Some(painter),
            ui_capture_path: std::env::var_os("DISPLAY_SIM_CAPTURE_UI").map(PathBuf::from),
            has_simulation_frame: false,
            simulation_pointer: SimulationPointerInput::default(),
            canvas_focused: false,
            input_focused: false,
            window_focused: true,
            interactions_reset: false,
            simulation_keyboard: SimulationKeyboardInput::default(),
        }
    }

    fn set_canvas_focused(&mut self, focused: bool) {
        if self.canvas_focused == focused {
            return;
        }
        self.canvas_focused = focused;
        self.input.push_event(InputEventValue::Keyboard {
            pressed: Pressed::from_bool(focused),
            key: "canvas_focused".into(),
        });
    }

    fn set_input_focused(&mut self, focused: bool) {
        if self.input_focused == focused {
            return;
        }
        self.input_focused = focused;
        self.input.push_event(InputEventValue::Keyboard {
            pressed: Pressed::from_bool(focused),
            key: "input_focused".into(),
        });
    }

    fn release_sim_pointer(&mut self, emit_release: bool) {
        let was_active = self.simulation_pointer.is_down() || self.sim_ctx.dispatcher_instance.cursor_hidden.get();
        if emit_release {
            if let Some(event) = self.simulation_pointer.release() {
                self.input.push_event(event);
            }
        } else {
            self.simulation_pointer.clear();
        }
        if !was_active {
            return;
        }
        self.sim_ctx.dispatcher_instance.cursor_hidden.set(false);
        if let Err(error) = self.windowed_ctx.window().set_cursor_grab(false) {
            println!("Could not release cursor grab: {error}");
        }
        self.windowed_ctx.window().set_cursor_visible(true);
    }

    fn reset_interactions(&mut self) {
        if self.interactions_reset {
            return;
        }
        self.interactions_reset = true;
        self.panel.release_all(&mut self.input);
        self.input.push_event(InputEventValue::BlurredWindow);
        self.simulation_keyboard.clear();
        self.canvas_focused = false;
        self.input_focused = false;
        self.release_sim_pointer(false);
    }

    pub fn iteration(&mut self, event: Event<()>, control_flow: &mut ControlFlow) -> AppResult<()> {
        *control_flow = ControlFlow::Poll;
        let frame_boundary = matches!(&event, Event::MainEventsCleared);

        match event {
            Event::LoopDestroyed => {
                if let Some(mut painter) = self.painter.take() {
                    painter.destroy();
                }
                return Ok(());
            }
            Event::WindowEvent { ref event, .. } => {
                if let WindowEvent::Focused(focused) = event {
                    self.window_focused = *focused;
                }
                if self.window_focused {
                    self.interactions_reset = false;
                }
                let panel_rect = self.panel.panel_rect();
                self.egui_input.on_window_event(event, panel_rect);
                let pointer_captured = self.egui_input.pointer_is_captured(panel_rect) || self.panel.context().egui_is_using_pointer();
                match event {
                    WindowEvent::Resized(size) => {
                        self.windowed_ctx.resize(*size);
                        println!("Size changed: ({}, {})", size.width, size.height);
                        self.input.push_event(InputEventValue::ViewportResize(size.width, size.height));
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        self.windowed_ctx.resize(**new_inner_size);
                        self.input
                            .push_event(InputEventValue::ViewportResize(new_inner_size.width, new_inner_size.height));
                    }
                    WindowEvent::KeyboardInput { input: keyevent, .. } => {
                        // The web frontend has a window-level keyboard
                        // listener, so simulation hotkeys continue to see
                        // both presses and releases while an input is being
                        // edited. Printable keys are paired with the following
                        // ReceivedCharacter event so their values also follow
                        // the active operating-system keyboard layout.
                        if self.window_focused {
                            let ui_owns_activation = self.panel.context().egui_wants_keyboard_input()
                                && matches!(
                                    keyevent.virtual_keycode,
                                    Some(VirtualKeyCode::Return | VirtualKeyCode::NumpadEnter | VirtualKeyCode::Space)
                                );
                            for input_event in self.simulation_keyboard.on_keyboard_input_routed(keyevent, !ui_owns_activation) {
                                self.input.push_event(input_event);
                            }
                        }
                    }
                    WindowEvent::ReceivedCharacter(character) => {
                        if self.window_focused {
                            for input_event in self.simulation_keyboard.on_received_character(*character) {
                                self.input.push_event(input_event);
                            }
                        }
                    }
                    WindowEvent::ModifiersChanged(modifiers) => {
                        if self.window_focused {
                            self.simulation_keyboard.on_modifiers_changed(*modifiers);
                        }
                    }
                    WindowEvent::MouseInput { button, state, .. } => {
                        if *button == MouseButton::Left {
                            let pressed = *state == ElementState::Pressed;
                            if let Some(input_event) = self.simulation_pointer.on_primary_button(pressed, pointer_captured || !self.window_focused) {
                                let started = matches!(input_event, InputEventValue::MouseClick(Pressed::Yes));
                                self.input.push_event(input_event);
                                if started && matches!(self.windowed_ctx.window().fullscreen(), None) {
                                    self.windowed_ctx.window().set_fullscreen(Some(Fullscreen::Borderless(self.monitor.clone())));
                                }
                            }
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        if self.window_focused && !pointer_captured {
                            self.set_canvas_focused(true);
                            self.input.push_event(InputEventValue::MouseWheel(browser_wheel_delta(delta)));
                        }
                    }
                    WindowEvent::CursorMoved { .. } => {
                        if self.window_focused {
                            self.set_canvas_focused(!pointer_captured);
                        }
                    }
                    WindowEvent::CursorLeft { .. } => {
                        self.set_canvas_focused(false);
                        self.panel.release_all(&mut self.input);
                        self.release_sim_pointer(true);
                    }
                    WindowEvent::Focused(false) => {
                        self.reset_interactions();
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                }
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                // `movementX/Y` in the web frontend are relative deltas. Raw
                // device motion is the native equivalent and remains usable
                // while the cursor is grabbed at a window edge.
                if self.simulation_pointer.is_down() {
                    self.input.push_event(InputEventValue::MouseMove {
                        x: delta.0 as i32,
                        y: delta.1 as i32,
                    });
                }
            }
            Event::Suspended => {
                self.window_focused = false;
                self.egui_input.on_suspended();
                self.reset_interactions();
            }
            Event::MainEventsCleared => {
                // KeyboardInput and ReceivedCharacter are separate winit
                // events. Flush keys for which the platform emitted no text
                // only after the current operating-system event batch.
                for input_event in self.simulation_keyboard.flush_pending() {
                    self.input.push_event(input_event);
                }
            }
            _ => (),
        }

        let now = Instant::now();
        // Like requestAnimationFrame in the web frontend, process a complete
        // operating-system event batch before running one simulation/UI frame.
        if frame_boundary && (now - self.timings.last_time) >= self.timings.framerate {
            self.timings.last_time = now;
            // Preserve sub-millisecond frame timing. Integer milliseconds
            // make 60 Hz motion alternate between coarse 16/17 ms steps.
            let elapsed_ms = (now - self.timings.starting_time).as_secs_f64() * 1000.0;
            let was_panel_visible = self.panel.is_visible();
            let raw_input = self.egui_input.take_input(elapsed_ms / 1000.0);
            let mut egui_output = self.panel.run(raw_input, &mut self.res, &mut self.input, &self.panel_events)?;
            self.set_input_focused(self.panel.context().egui_wants_keyboard_input());
            if !was_panel_visible && self.panel.is_visible() {
                self.panel.release_all(&mut self.input);
                self.release_sim_pointer(true);
            }
            self.egui_input.handle_platform_output(
                self.windowed_ctx.window(),
                std::mem::take(&mut egui_output.platform_output),
                self.sim_ctx.dispatcher_instance.cursor_hidden.get(),
            );

            if !self.res.quit {
                match SimulationCoreTicker::new(&self.sim_ctx, &mut self.res, &mut self.input).tick(elapsed_ms) {
                    Ok(_) => {}
                    Err(e) => println!("Tick error: {:?}", e),
                };
            }

            let screenshot_frame = self.res.screenshot_trigger.is_triggered;
            let reuse_simulation_for_ui_capture = self.ui_capture_path.is_some() && self.has_simulation_frame;
            if self.res.drawable && !reuse_simulation_for_ui_capture {
                if let Err(e) = SimulationDrawer::new(&self.sim_ctx, &mut self.materials, &self.res).draw() {
                    println!("Draw error: {:?}", e);
                } else if self.res.video.drawing_activation {
                    self.has_simulation_frame = true;
                }
            }

            // Screenshot rendering leaves the offscreen target bound. During
            // screenshot cooldown no simulation draw happens at all. In both
            // cases redraw the latest clean simulation image before egui.
            if self.has_simulation_frame && (screenshot_frame || !self.res.drawable || reuse_simulation_for_ui_capture || self.res.quit) {
                present_to_default_framebuffer(&mut self.materials, &self.res)?;
            }

            let width = self.res.video.viewport_size.width;
            let height = self.res.video.viewport_size.height;
            self.materials.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            self.materials.gl.viewport(0, 0, width as i32, height as i32);
            if self.res.quit && !self.has_simulation_frame {
                self.materials.gl.clear_color(0.0, 0.0, 0.0, 1.0);
                self.materials.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            }
            let primitives = self
                .panel
                .context()
                .tessellate(std::mem::take(&mut egui_output.shapes), egui_output.pixels_per_point);
            if self.ui_capture_path.is_some() {
                println!("Composited UI capture contains {} egui paint primitives", primitives.len());
            }
            self.painter
                .as_mut()
                .expect("egui painter was destroyed before event-loop shutdown")
                .paint_and_update_textures([width, height], egui_output.pixels_per_point, &primitives, &mut egui_output.textures_delta);
            // egui can intentionally emit an empty first pass while it adopts
            // the window's native scale factor. Capture the first populated
            // pass so this developer hook always includes the overlay.
            if !primitives.is_empty() {
                if let Some(path) = self.ui_capture_path.take() {
                    match capture_composited_frame(&self.materials.gl, width, height, &path) {
                        Ok(()) => println!("Composited UI frame saved to {}", path.display()),
                        Err(error) => println!("Composited UI frame capture failed: {error}"),
                    }
                }
            }
            self.materials.gl.disable(glow::BLEND);
            self.materials.gl.enable(glow::DEPTH_TEST);

            self.windowed_ctx.swap_buffers().map_err(|error| format!("Swap buffers failed: {error}"))?;
        }
        Ok(())
    }
}

struct NativeEventDispatcher {
    video_ctx: Rc<WindowedContext<PossiblyCurrent>>,
    gl: Rc<GlowSafeAdapter<glow::Context>>,
    panel_events: SharedPanelEvents,
    extra_messages_enabled: Cell<bool>,
    cursor_hidden: Cell<bool>,
}

impl NativeEventDispatcher {
    pub fn new(video_ctx: Rc<WindowedContext<PossiblyCurrent>>, gl: Rc<GlowSafeAdapter<glow::Context>>, panel_events: SharedPanelEvents) -> Self {
        NativeEventDispatcher {
            video_ctx,
            gl,
            panel_events,
            extra_messages_enabled: Cell::new(false),
            cursor_hidden: Cell::new(false),
        }
    }
}

impl AppEventDispatcher for NativeEventDispatcher {
    fn enable_extra_messages(&self, enabled: bool) {
        self.extra_messages_enabled.set(enabled);
    }
    fn are_extra_messages_enabled(&self) -> bool {
        self.extra_messages_enabled.get()
    }
    fn dispatch_log(&self, msg: String) {
        println!("log: {}", msg);
    }
    fn dispatch_string_event(&self, event_id: &'static str, message: &str) {
        println!("{} {}", event_id, message);
    }
    fn dispatch_camera_update(&self, a: &glm::Vec3, b: &glm::Vec3, c: &glm::Vec3) {
        println!("camera_update {}, {}, {}", a, b, c);
    }
    fn dispatch_change_pixel_width(&self, size: f32) {
        println!("change_pixel_width: {}", size);
    }
    fn dispatch_change_camera_zoom(&self, zoom: f32) {
        println!("change_camera_zoom: {}", zoom);
    }
    fn dispatch_scaling_method(&self, method: ScalingMethod) {
        println!("scaling_method: {}", method);
    }
    fn dispatch_scaling_resolution_width(&self, value: u32) {
        println!("scaling_resolution_width: {}", value);
    }
    fn dispatch_scaling_resolution_height(&self, value: u32) {
        println!("scaling_resolution_height: {}", value);
    }
    fn dispatch_scaling_aspect_ratio_x(&self, value: f32) {
        println!("scaling_aspect_ratio_x: {}", value);
    }
    fn dispatch_scaling_aspect_ratio_y(&self, value: f32) {
        println!("custom_aspect_ratio_y: {}", value);
    }
    fn dispatch_custom_scaling_stretch_nearest(&self, value: bool) {
        println!("custom_scaling_stretch_nearest: {}", value);
    }
    fn dispatch_change_pixel_speed(&self, speed: f32) {
        println!("change_pixel_speed: {}", speed);
    }
    fn dispatch_change_turning_speed(&self, speed: f32) {
        println!("change_turning_speed: {}", speed);
    }
    fn dispatch_change_movement_speed(&self, speed: f32) {
        println!("change_movement_speed: {}", speed);
    }
    fn dispatch_exiting_session(&self) {
        println!("exiting_session");
    }
    fn dispatch_toggle_info_panel(&self) {
        println!("toggle_info_panel");
        self.panel_events.borrow_mut().request_toggle();
    }
    fn dispatch_fps(&self, fps: f32) {
        println!("frames in 20 seconds: {}", fps);
        self.panel_events.borrow_mut().set_fps(fps);
    }
    fn dispatch_request_fullscreen(&self) {
        println!("request_fullscreen");
    }
    fn dispatch_request_pointer_lock(&self) {
        println!("request_pointer_lock");
        if let Err(error) = self.video_ctx.window().set_cursor_grab(true) {
            println!("Could not grab cursor: {error}");
        }
        self.cursor_hidden.set(true);
        self.video_ctx.window().set_cursor_visible(false);
    }
    fn dispatch_exit_pointer_lock(&self) {
        println!("exit_pointer_lock");
        if let Err(error) = self.video_ctx.window().set_cursor_grab(false) {
            println!("Could not release cursor grab: {error}");
        }
        self.cursor_hidden.set(false);
        self.video_ctx.window().set_cursor_visible(true);
    }
    fn dispatch_screenshot(&self, width: i32, height: i32, pixels: &mut [u8]) -> AppResult<()> {
        let result = (|| {
            if width <= 0 || height <= 0 {
                return Err(format!("invalid screenshot size {width}x{height}").into());
            }
            self.gl.read_pixels(0, 0, width, height, glow::RGBA, glow::UNSIGNED_BYTE, pixels);
            flip_rgba_rows(pixels, width as usize, height as usize)?;
            let path = next_screenshot_path(Path::new("."));
            image::save_buffer(&path, pixels, width as u32, height as u32, image::ColorType::Rgba8)
                .map_err(|error| format!("could not save {}: {error}", path.display()))?;
            Ok(path)
        })();

        match result {
            Ok(path) => {
                println!("Screenshot saved to {}", path.display());
                self.panel_events.borrow_mut().push_message("Screenshot saved.");
                Ok(())
            }
            Err(error) => {
                let message = format!("Screenshot failed: {error}");
                println!("{message}");
                self.panel_events.borrow_mut().push_message(message);
                Err(error)
            }
        }
    }
    fn dispatch_change_camera_movement_mode(&self, locked_mode: CameraLockMode) {
        println!("change_camera_movement_mode: {}", locked_mode);
    }
    fn dispatch_top_message(&self, message: &str) {
        println!("top_message: {}", message);
        self.panel_events.borrow_mut().push_message(message);
    }
    fn dispatch_minimum_value(&self, value: &dyn Display) {
        println!("minimum: {}", value);
        self.panel_events.borrow_mut().push_message(format!("Minimum: {value}"));
    }
    fn dispatch_maximum_value(&self, value: &dyn Display) {
        println!("maximum: {}", value);
        self.panel_events.borrow_mut().push_message(format!("Maximum: {value}"));
    }
}

fn flip_rgba_rows(pixels: &mut [u8], width: usize, height: usize) -> AppResult<()> {
    let row_len = width.checked_mul(4).ok_or_else(|| "screenshot row size overflow".to_string())?;
    let expected = row_len.checked_mul(height).ok_or_else(|| "screenshot buffer size overflow".to_string())?;
    if pixels.len() != expected {
        return Err(format!("screenshot buffer has {} bytes, expected {expected}", pixels.len()).into());
    }
    for y in 0..height / 2 {
        let opposite = height - 1 - y;
        for x in 0..row_len {
            pixels.swap(y * row_len + x, opposite * row_len + x);
        }
    }
    Ok(())
}

fn next_screenshot_path(directory: &Path) -> PathBuf {
    for index in 1_u64.. {
        let path = directory.join(format!("screenshot-{index}.png"));
        if !path.exists() {
            return path;
        }
    }
    unreachable!("the screenshot index space was exhausted")
}

fn capture_composited_frame(gl: &GlowSafeAdapter<glow::Context>, width: u32, height: u32, path: &Path) -> AppResult<()> {
    if width == 0 || height == 0 {
        return Err(format!("invalid UI capture size {width}x{height}").into());
    }
    let len = (width as usize)
        .checked_mul(height as usize)
        .and_then(|pixels| pixels.checked_mul(4))
        .ok_or_else(|| "UI capture buffer size overflow".to_string())?;
    let mut pixels = vec![0; len];
    gl.read_pixels(0, 0, width as i32, height as i32, glow::RGBA, glow::UNSIGNED_BYTE, &mut pixels);
    flip_rgba_rows(&mut pixels, width as usize, height as usize)?;
    image::save_buffer(path, &pixels, width, height, image::ColorType::Rgba8).map_err(|error| format!("could not save {}: {error}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flips_screenshot_rows() {
        let mut pixels = vec![1, 2, 3, 4, 5, 6, 7, 8];
        flip_rgba_rows(&mut pixels, 1, 2).unwrap();
        assert_eq!(pixels, vec![5, 6, 7, 8, 1, 2, 3, 4]);
    }

    #[test]
    fn screenshot_names_do_not_collide() {
        let directory = std::env::temp_dir().join(format!("display-sim-screenshot-test-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let first = directory.join("screenshot-1.png");
        std::fs::write(&first, []).unwrap();
        assert_eq!(next_screenshot_path(&directory), directory.join("screenshot-2.png"));
        std::fs::remove_file(first).unwrap();
        std::fs::remove_dir(directory).unwrap();
    }
}
