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

use render::opengl_hooks::{WebGl2RenderingContext, WebResult};

use core::action_bindings::on_button_action;
use core::app_events::AppEventDispatcher;
use core::camera::CameraLockMode;
use core::general_types::Size2D;
use core::internal_resolution::InternalResolution;
use core::pixels_shadow::ShadowShape;
use core::simulation_context::{ConcreteSimulationContext, RandomGenerator};
use core::simulation_core_state::{AnimationStep, FiltersPreset, Input, Resources, VideoInputResources};
use core::simulation_core_state::{ColorChannels, PixelsGeometryKind, ScreenCurvatureKind, TextureInterpolation};
use core::simulation_core_ticker::SimulationCoreTicker;
use render::simulation_draw::SimulationDrawer;
use render::simulation_render_state::{Materials, VideoInputMaterials};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::FullscreenType;
use sdl2::Sdl;
use std::fmt::Display;
use std::time::SystemTime;

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

fn program() -> WebResult<()> {
    println!("Initializing SDL.");
    let sdl = sdl2::init().unwrap();
    println!("Initializing Video Subsystem.");
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 3);
    let display_mode = video_subsystem.current_display_mode(0)?;

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
            width: (display_mode.w as f32 * 0.8) as u32,
            height: (display_mode.h as f32 * 0.8) as u32,
        },
        pixel_width: 1.0,
        stretch: false,
        current_frame: 0,
        preset: FiltersPreset::default(),
        last_frame_change: 0.0,
        needs_buffer_data_load: true,
    };
    let materials_input = VideoInputMaterials { buffers: vec![pixels] };

    println!("Opening window.");
    let mut window = video_subsystem
        .window("Display Sim", res_input.viewport_size.width, res_input.viewport_size.height)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    println!("Creating GL Context.");
    let _gl_context = window.gl_create_context().unwrap();
    println!("Loading GL on Video Subsystem.");
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    println!("Preparing resources.");
    let starting_time = SystemTime::now();
    let mut res = Resources::default();
    res.initialize(res_input, get_millis_since(&starting_time)?);
    println!("Preparing materials.");
    let mut materials = Materials::new(WebGl2RenderingContext::default(), materials_input)?;

    println!("Preparing input.");
    let mut input = Input::new(get_millis_since(&starting_time)?);
    println!("Preparing simulation context.");
    let mut ctx = ConcreteSimulationContext::new(NativeEventDispatcher::default(), NativeRnd {});
    ctx.dispatcher_instance.sdl_ctx = Some(&sdl);

    let mut event_pump = sdl.event_pump().unwrap();
    println!("Main loop.");
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                Event::KeyDown { keycode: Some(key), .. } => read_key(&mut input, key, true),
                Event::KeyUp { keycode: Some(key), .. } => read_key(&mut input, key, false),
                Event::MouseButtonDown {
                    mouse_btn: sdl2::mouse::MouseButton::Left,
                    ..
                } => {
                    input.mouse_click.input = true;
                    if let FullscreenType::Off = window.fullscreen_state() {
                        window.set_fullscreen(FullscreenType::Desktop)?;
                    }
                }
                Event::MouseButtonUp {
                    mouse_btn: sdl2::mouse::MouseButton::Left,
                    ..
                } => {
                    input.mouse_click.input = false;
                }
                Event::MouseMotion { xrel, yrel, .. } => {
                    input.mouse_position_x = xrel;
                    input.mouse_position_y = yrel;
                }
                Event::MouseWheel { y, .. } => {
                    input.mouse_scroll_y = y as f32;
                }
                Event::Window {
                    win_event: sdl2::event::WindowEvent::SizeChanged(width, height),
                    ..
                } => {
                    println!("Size changed: ({}, {})", width, height);
                    res.video.viewport_size.width = width as u32;
                    res.video.viewport_size.height = height as u32;
                }
                _ => {}
            }
        }

        SimulationCoreTicker::new(&ctx, &mut res, &mut input).tick(get_millis_since(&starting_time)?);
        if res.quit {
            println!("User closed the simulation.");
            return Ok(());
        }
        if res.drawable {
            SimulationDrawer::new(&ctx, &mut materials, &res).draw()?;
        }

        window.gl_swap_window();
    }
    Ok(())
}

fn get_millis_since(time: &SystemTime) -> Result<f64, String> {
    Ok(time.elapsed().map_err(|e| format!("{}", e))?.as_millis() as f64)
}

pub fn read_key(input: &mut Input, key: Keycode, pressed: bool) {
    let used = on_button_action(input, &format!("{}", key).to_lowercase(), pressed);
    if !used {
        println!("Not used: {}", key);
    }
}

#[derive(Default)]
struct NativeEventDispatcher<'a> {
    sdl_ctx: Option<&'a Sdl>,
}

impl<'a> AppEventDispatcher for NativeEventDispatcher<'a> {
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
    fn dispatch_request_pointer_lock(&self) {
        println!("request_pointer_lock");
        self.sdl_ctx.unwrap().mouse().show_cursor(false);
    }
    fn dispatch_exit_pointer_lock(&self) {
        println!("exit_pointer_lock");
        self.sdl_ctx.unwrap().mouse().show_cursor(true);
    }
    fn dispatch_change_preset_selected(&self, preset_name: &str) {
        println!("dispatch_change_preset_selected: {}", preset_name);
    }
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
