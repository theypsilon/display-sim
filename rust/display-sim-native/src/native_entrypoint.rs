/* Copyright (c) 2019-2021 José manuel Barroso Galindo <theypsilon@gmail.com>
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
use render::simulation_draw::SimulationDrawer;
use render::simulation_render_state::{Materials, VideoInputMaterials};

use std::fmt::Display;
use std::rc::Rc;
use std::time::{Duration, Instant};

use glutin::event::{ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::monitor::MonitorHandle;
use glutin::window::{Fullscreen, WindowBuilder};
use glutin::{ContextBuilder, ContextError, GlProfile, GlRequest, PossiblyCurrent, Robustness, WindowedContext};

use glow::GlowSafeAdapter;

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
    let monitor = winit_loop.primary_monitor().unwrap();
    let hidpi = monitor.scale_factor();
    let mut window_size = monitor.size().to_logical::<f64>(hidpi);
    window_size.width *= 0.8;
    window_size.height *= 0.8;

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
    let gl_ctx = unsafe { glow::Context::from_loader_function(|ptr| windowed_ctx.context().get_proc_address(ptr) as *const _) };
    println!("Pixel format of the window's GL context: {:?}", windowed_ctx.get_pixel_format());

    let img_path = "www/assets/pics/frames/seiken.png";
    println!("Loading image: {}", img_path);
    let img = image::open(img_path).map_err(|e| format!("{}", e))?.to_rgba8();
    let img_size = img.dimensions();
    let pixels = img.into_vec().into_boxed_slice();

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
            width: (monitor.size().width as f64 * 0.8) as u32,
            height: (monitor.size().height as f64 * 0.8) as u32,
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
    let materials = Materials::new(Rc::new(GlowSafeAdapter::new(gl_ctx)), materials_input)?;

    println!("Preparing input.");
    let input = Input::new(0.0);
    println!("Preparing simulation context.");
    let sim_ctx = ConcreteSimulationContext::new(NativeEventDispatcher::new(windowed_ctx.clone()), NativeRnd {});

    let timings = Timings::new(Instant::now(), Duration::from_secs_f64(1.0 / 60.0));

    let mut state = NativeSimulationState::new(sim_ctx, windowed_ctx, monitor, res, input, materials, timings);

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
    monitor: MonitorHandle,
    res: Resources,
    input: Input,
    materials: Materials,
    timings: Timings,
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
        monitor: MonitorHandle,
        res: Resources,
        input: Input,
        materials: Materials,
        timings: Timings,
    ) -> Self {
        NativeSimulationState {
            sim_ctx,
            windowed_ctx,
            monitor,
            res,
            input,
            materials,
            timings,
        }
    }

    pub fn iteration(&mut self, event: Event<()>, control_flow: &mut ControlFlow) -> Result<(), ContextError> {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return Ok(()),
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(size) => {
                    let dpi_factor = self.windowed_ctx.window().scale_factor();
                    //self.windowed_ctx.resize(size.to_physical::<f64>(dpi_factor));
                    self.windowed_ctx.resize(*size);

                    println!("Size changed: ({}, {})", size.width, size.height);
                    self.res.video.viewport_size.width = (size.width as f64 * dpi_factor) as u32;
                    self.res.video.viewport_size.height = (size.height as f64 * dpi_factor) as u32;
                }
                WindowEvent::KeyboardInput { input: keyevent, .. } => {
                    if let Some(key) = keyevent.virtual_keycode {
                        self.input.push_event(InputEventValue::Keyboard {
                            pressed: match keyevent.state {
                                ElementState::Pressed => Pressed::Yes,
                                ElementState::Released => Pressed::No,
                            },
                            key: format!("{:?}", key),
                        });
                    }
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    if *button == MouseButton::Left {
                        let pressed = match state {
                            ElementState::Pressed => Pressed::Yes,
                            ElementState::Released => Pressed::No,
                        };
                        self.input.push_event(InputEventValue::MouseClick(pressed));
                        if pressed == Pressed::Yes && matches!(self.windowed_ctx.window().fullscreen(), None)
                        {
                            self.windowed_ctx.window().set_fullscreen(Some(Fullscreen::Borderless(Some(self.monitor.clone()))));
                        }
                    }
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    let mouse_wheel = match delta {
                        MouseScrollDelta::LineDelta(y, ..) => *y,
                        MouseScrollDelta::PixelDelta(position) => position.y as f32,
                    };
                    self.input.push_event(InputEventValue::MouseWheel(mouse_wheel));
                }
                WindowEvent::CursorMoved { position, .. } => {
                    self.input.push_event(InputEventValue::MouseMove {
                        x: position.x as i32,
                        y: position.y as i32,
                    });
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }

        let now = Instant::now();
        if (now - self.timings.last_time) >= self.timings.framerate {
            self.timings.last_time = now;

            match SimulationCoreTicker::new(&self.sim_ctx, &mut self.res, &mut self.input).tick(self.timings.starting_time.elapsed().as_millis() as f64) {
                Ok(_) => {}
                Err(e) => println!("Tick error: {:?}", e),
            };

            if self.res.drawable {
                if let Err(e) = SimulationDrawer::new(&self.sim_ctx, &mut self.materials, &self.res).draw() {
                    println!("Draw error: {:?}", e);
                }
            }

            if self.res.quit {
                println!("User closed the simulation.");
                *control_flow = ControlFlow::Exit;
            }

            self.windowed_ctx.swap_buffers()?;
        }
        Ok(())
    }
}

struct NativeEventDispatcher {
    video_ctx: Rc<WindowedContext<PossiblyCurrent>>,
}

impl NativeEventDispatcher {
    pub fn new(video_ctx: Rc<WindowedContext<PossiblyCurrent>>) -> Self {
        NativeEventDispatcher { video_ctx }
    }
}

impl AppEventDispatcher for NativeEventDispatcher {
    fn enable_extra_messages(&self, _: bool) {}
    fn are_extra_messages_enabled(&self) -> bool {
        false
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
    }
    fn dispatch_fps(&self, fps: f32) {
        println!("frames in 20 seconds: {}", fps);
    }
    fn dispatch_request_fullscreen(&self) {
        println!("request_fullscreen");
    }
    fn dispatch_request_pointer_lock(&self) {
        println!("request_pointer_lock");
        self.video_ctx.window().set_cursor_visible(false);
    }
    fn dispatch_exit_pointer_lock(&self) {
        println!("exit_pointer_lock");
        self.video_ctx.window().set_cursor_visible(true);
    }
    fn dispatch_screenshot(&self, _: i32, _: i32, _: &mut [u8]) -> AppResult<()> {
        Ok(())
    }
    fn dispatch_change_camera_movement_mode(&self, locked_mode: CameraLockMode) {
        println!("change_camera_movement_mode: {}", locked_mode);
    }
    fn dispatch_top_message(&self, message: &str) {
        println!("top_message: {}", message);
    }
    fn dispatch_minimum_value(&self, value: &dyn Display) {
        println!("minimum: {}", value);
    }
    fn dispatch_maximum_value(&self, value: &dyn Display) {
        println!("maximum: {}", value);
    }
}
