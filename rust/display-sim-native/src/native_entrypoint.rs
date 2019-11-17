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

use core::action_bindings::on_button_action;
use core::app_events::AppEventDispatcher;
use core::camera::CameraLockMode;
use core::general_types::Size2D;
use core::internal_resolution::InternalResolution;
use core::pixels_shadow::ShadowShape;
use core::simulation_context::{ConcreteSimulationContext, RandomGenerator};
use core::simulation_core_state::{AnimationStep, Input, Resources, VideoInputResources};
use core::simulation_core_state::{ColorChannels, PixelsGeometryKind, ScreenCurvatureKind, TextureInterpolation};
use core::simulation_core_ticker::SimulationCoreTicker;
use render::error::AppResult;
use render::simulation_draw::SimulationDrawer;
use render::simulation_render_state::{Materials, VideoInputMaterials};

use std::fmt::Display;
use std::rc::Rc;
use std::time::{Duration, Instant};

use glutin::event::{ElementState, Event, MouseButton, MouseScrollDelta, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Fullscreen, WindowBuilder};
use glutin::{ContextBuilder, GlProfile, GlRequest, PossiblyCurrent, Robustness, WindowedContext};

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
        rng.gen_range(0.0, 1.0)
    }
}

fn program() -> AppResult<()> {
    println!("Initializing Window.");
    let el = EventLoop::new();
    let monitor = el.primary_monitor();
    let hidpi = monitor.hidpi_factor();
    let mut window_size = monitor.size().to_logical(hidpi);
    window_size.width *= 0.8;
    window_size.height *= 0.8;

    let wb = WindowBuilder::new()
        .with_inner_size(window_size)
        .with_visible(true)
        .with_decorations(true)
        .with_resizable(true)
        .with_title("Display Sim");

    let windowed_context = ContextBuilder::new()
        .with_gl(GlRequest::Latest)
        .with_gl_profile(GlProfile::Core)
        .with_gl_robustness(Robustness::NotRobust)
        .with_gl_debug_flag(false)
        .with_hardware_acceleration(Some(true))
        .with_vsync(false)
        .with_multisampling(4)
        .with_depth_buffer(24)
        .build_windowed(wb, &el)
        .map_err(|e| format!("{}", e))?;

    let windowed_context = unsafe { windowed_context.make_current().map_err(|e| format!("Context Error: {:?}", e))? };
    let windowed_context = Rc::new(windowed_context);
    let gl_ctx = glow::Context::from_loader_function(|ptr| windowed_context.context().get_proc_address(ptr) as *const _);
    println!("Pixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());

    let img_path = "www/assets/pics/frames/seiken.png";
    println!("Loading image: {}", img_path);
    let img = image::open(img_path).map_err(|e| format!("{}", e))?.to_rgba();
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
            width: (monitor.size().width * 0.8) as u32,
            height: (monitor.size().height * 0.8) as u32,
        },
        pixel_width: 1.0,
        stretch: false,
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
    let mut materials = Materials::new(Rc::new(GlowSafeAdapter::new(gl_ctx)), materials_input)?;

    println!("Preparing input.");
    let mut input = Input::new(0.0);
    println!("Preparing simulation context.");
    let ctx = ConcreteSimulationContext::new(NativeEventDispatcher::new(windowed_context.clone()), NativeRnd {});

    let starting_time = Instant::now();
    let framerate = Duration::from_secs_f64(1.0 / 60.0);
    let mut last_time = starting_time - framerate;

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(size) => {
                    let dpi_factor = windowed_context.window().hidpi_factor();
                    windowed_context.resize(size.to_physical(dpi_factor));

                    println!("Size changed: ({}, {})", size.width, size.height);
                    res.video.viewport_size.width = (size.width * dpi_factor) as u32;
                    res.video.viewport_size.height = (size.height * dpi_factor) as u32;
                }
                WindowEvent::RedrawRequested => {
                    println!("Redraw Requested!!");
                    windowed_context.swap_buffers().unwrap();
                }
                WindowEvent::KeyboardInput { input: keyevent, .. } => {
                    if let Some(key) = keyevent.virtual_keycode {
                        read_key(
                            &mut input,
                            key,
                            match keyevent.state {
                                ElementState::Pressed => true,
                                ElementState::Released => false,
                            },
                        );
                    }
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    if *button == MouseButton::Left {
                        input.mouse_click.input = match state {
                            ElementState::Pressed => true,
                            ElementState::Released => false,
                        };
                        if input.mouse_click.input
                            && match windowed_context.window().fullscreen() {
                                None => true,
                                _ => false,
                            }
                        {
                            windowed_context.window().set_fullscreen(Some(Fullscreen::Borderless(monitor.clone())));
                        }
                    }
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    input.mouse_scroll_y = match delta {
                        MouseScrollDelta::LineDelta(y, ..) => *y,
                        MouseScrollDelta::PixelDelta(position) => position.y as f32,
                    };
                }
                WindowEvent::CursorMoved { position, .. } => {
                    input.mouse_position_x = position.x as i32;
                    input.mouse_position_y = position.y as i32;
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }

        let now = Instant::now();
        if (now - last_time) >= framerate {
            last_time = now;

            match SimulationCoreTicker::new(&ctx, &mut res, &mut input).tick(starting_time.elapsed().as_millis() as f64) {
                Ok(_) => {}
                Err(e) => println!("Tick error: {:?}", e),
            };

            if res.drawable {
                if let Err(e) = SimulationDrawer::new(&ctx, &mut materials, &res).draw() {
                    println!("Draw error: {:?}", e);
                }
            }

            if res.quit {
                println!("User closed the simulation.");
                *control_flow = ControlFlow::Exit;
            }

            windowed_context.swap_buffers().unwrap();
        }
    });
}

pub fn read_key(input: &mut Input, key: VirtualKeyCode, pressed: bool) {
    let used = on_button_action(input, &format!("{:?}", key).to_lowercase(), pressed);
    if !used {
        println!("Not used: {:?}", key);
    }
}

struct NativeEventDispatcher {
    video_ctx: Rc<WindowedContext<PossiblyCurrent>>,
}

impl NativeEventDispatcher {
    pub fn new(video_ctx: Rc<WindowedContext<PossiblyCurrent>>) -> Self {
        NativeEventDispatcher {
            video_ctx
        }
    }
}

impl AppEventDispatcher for NativeEventDispatcher {
    fn enable_extra_messages(&self, _: bool) {}
    fn dispatch_camera_update(&self, a: &glm::Vec3, b: &glm::Vec3, c: &glm::Vec3) {
        println!("camera_update {}, {}, {}", a, b, c);
    }
    fn dispatch_change_pixel_horizontal_gap(&self, size: f32) {
        println!("ixel_horizontal_gap: {}", size);
    }
    fn dispatch_change_pixel_vertical_gap(&self, size: f32) {
        println!("change_pixel_vertical_gap: {}", size);
    }
    fn dispatch_change_pixel_width(&self, size: f32) {
        println!("change_pixel_width: {}", size);
    }
    fn dispatch_change_pixel_spread(&self, size: f32) {
        println!("change_pixel_spread: {}", size);
    }
    fn dispatch_change_pixel_brightness(&self, res: f32) {
        println!("change_pixel_brightness: {}", res);
    }
    fn dispatch_change_pixel_contrast(&self, res: f32) {
        println!("change_pixel_contrast: {}", res);
    }
    fn dispatch_change_light_color(&self, res: i32) {
        println!("change_light_color: {}", res);
    }
    fn dispatch_change_brightness_color(&self, res: i32) {
        println!("change_brightness_color: {}", res);
    }
    fn dispatch_change_camera_zoom(&self, zoom: f32) {
        println!("change_camera_zoom: {}", zoom);
    }
    fn dispatch_change_blur_level(&self, res: usize) {
        println!("change_blur_level: {}", res);
    }
    fn dispatch_change_vertical_lpp(&self, res: usize) {
        println!("change_vertical_lpp: {}", res);
    }
    fn dispatch_change_horizontal_lpp(&self, res: usize) {
        println!("change_horizontal_lpp: {}", res);
    }
    fn dispatch_color_representation(&self, res: ColorChannels) {
        println!("color_representation: {}", res);
    }
    fn dispatch_pixel_geometry(&self, res: PixelsGeometryKind) {
        println!("pixel_geometry: {}", res);
    }
    fn dispatch_pixel_shadow_shape(&self, res: ShadowShape) {
        println!("pixel_shadow_shape: {}", res);
    }
    fn dispatch_pixel_shadow_height(&self, res: f32) {
        println!("pixel_shadow_height: {}", res);
    }
    fn dispatch_backlight_presence(&self, res: f32) {
        println!("backlight_presence: {}", res);
    }
    fn dispatch_screen_curvature(&self, res: ScreenCurvatureKind) {
        println!("screen_curvature: {}", res);
    }
    fn dispatch_internal_resolution(&self, res: &InternalResolution) {
        println!("internal_resolution: {}", res);
    }
    fn dispatch_texture_interpolation(&self, res: TextureInterpolation) {
        println!("texture_interpolation: {}", res);
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
    fn dispatch_new_frame(&self) {}
    fn dispatch_request_pointer_lock(&self) {
        println!("request_pointer_lock");
        self.video_ctx.window().set_cursor_visible(false);
    }
    fn dispatch_exit_pointer_lock(&self) {
        println!("exit_pointer_lock");
        self.video_ctx.window().set_cursor_visible(true);
    }
    fn dispatch_change_preset_selected(&self, preset_name: &str) {
        println!("dispatch_change_preset_selected: {}", preset_name);
    }
    fn fire_screenshot(&self, _: i32, _: i32, _: &mut [u8], _: f64) {}
    fn dispatch_screenshot(&self, _: &[u8], _: f64) {}
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
