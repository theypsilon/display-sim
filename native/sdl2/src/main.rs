use webgl_to_sdl2::{WebGl2RenderingContext, WebResult};

use core::action_bindings::on_button_action;
use core::app_events::AppEventDispatcher;
use core::simulation_context::SimulationContext;
use core::simulation_core_state::{
    init_resources, AnimationStep, Input, Resources, VideoInputResources,
};
use core::simulation_update::SimulationUpdater;
use core::general_types::Size2D;
use render::background_render::BackgroundRender;
use render::blur_render::BlurRender;
use render::internal_resolution_render::InternalResolutionRender;
use render::pixels_render::PixelsRender;
use render::render_types::TextureBufferStack;
use render::rgb_render::RgbRender;
use render::simulation_draw::SimulationDrawer;
use render::simulation_render_state::{Materials, VideoInputMaterials};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{SystemTime};

fn main() {
    if let Err(e) = program() {
        println!("Error: {:?}", e);
        std::process::exit(-1);
    }
}

fn program() -> WebResult<()> {

    let img = image::open("../www/assets/pics/frames/seiken.png").map_err(|e| format!("{}", e))?.to_rgba();
    let img_size = img.dimensions();
    let pixels = img.into_vec().into_boxed_slice();

    let res_input = VideoInputResources {
        steps: vec![AnimationStep{ delay: 16 }],
        max_texture_size: std::i32::MAX,
        image_size: Size2D{ width: img_size.0, height: img_size.1 },
        background_size: Size2D{ width: img_size.0, height: img_size.1 },
        viewport_size: Size2D{ width: 1920, height: 1080 },
        pixel_width: 1.0,
        stretch: false,
        current_frame: 0,
        last_frame_change: 0.0,
        needs_buffer_data_load: true,
    };
    let materials_input = VideoInputMaterials {
        buffers: vec![pixels]
    };

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Screen Sim", res_input.viewport_size.width, res_input.viewport_size.height)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let now = SystemTime::now();
    let mut res = Resources::default();
    init_resources(&mut res, res_input, now.elapsed().map_err(|e| format!("{}", e))?.as_millis() as f64);
    let gl = WebGl2RenderingContext::default();
    let mut materials = Materials {
        main_buffer_stack: TextureBufferStack::new(&gl),
        bg_buffer_stack: TextureBufferStack::new(&gl),
        pixels_render: PixelsRender::new(&gl, materials_input)?,
        blur_render: BlurRender::new(&gl)?,
        internal_resolution_render: InternalResolutionRender::new(&gl)?,
        rgb_render: RgbRender::new(&gl)?,
        background_render: BackgroundRender::new(&gl)?,
        screenshot_pixels: None,
        gl,
    };

    let mut input = Input::new(now.elapsed().map_err(|e| format!("{}", e))?.as_millis() as f64);
    let mut ctx : SimulationContext<NativeEventDispatcher> = SimulationContext::default();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main,
                Event::KeyDown { keycode: Some(key), .. } => read_key(&mut input, key, true),
                Event::KeyUp { keycode: Some(key), .. } => read_key(&mut input, key, false),
                _ => {}
            }
        }

        input.now = now.elapsed().map_err(|e| format!("{}", e))?.as_millis() as f64;

        if !SimulationUpdater::new(&mut ctx, &mut res, &input).update() {
            println!("User closed the simulation.");
            return Ok(());
        }

        if res.launch_screenshot || res.screenshot_delay <= 0 {
            SimulationDrawer::new(&mut ctx, &mut materials, &res).draw()?;
        }

        window.gl_swap_window();
    }
    Ok(())
}

pub fn read_key(input: &mut Input, key: Keycode, pressed: bool) {
    let used = on_button_action(input, &format!("{}", key).to_lowercase(), pressed);
    if !used {
        println!("Not used: {}", key);
    }
}

#[derive(Default)]
struct NativeEventDispatcher {}

impl AppEventDispatcher for NativeEventDispatcher {
    fn dispatch_camera_update(&self, a: &glm::Vec3, b: &glm::Vec3, c: &glm::Vec3) {
        println!("camera_update {}, {}, {}", a, b, c);
    }
    fn dispatch_change_pixel_horizontal_gap(&self, a: f32) {
        println!("change_pixel_horizontal_gap {}", a);
    }
    fn dispatch_change_pixel_vertical_gap(&self, a: f32) {
        println!("change_pixel_vertical_gap {}", a);
    }
    fn dispatch_change_pixel_width(&self, a: f32) {
        println!("change_pixel_width {}", a);
    }
    fn dispatch_change_pixel_spread(&self, a: f32) {
        println!("change_pixel_spread {}", a);
    }
    fn dispatch_change_pixel_brightness(&self, _: &Resources) {
        println!("change_pixel_brightness");
    }
    fn dispatch_change_pixel_contrast(&self, _: &Resources) {
        println!("change_pixel_contrast");
    }
    fn dispatch_change_light_color(&self, _: &Resources) {
        println!("change_light_color");
    }
    fn dispatch_change_brightness_color(&self, _: &Resources) {
        println!("change_brightness_color");
    }
    fn dispatch_change_camera_zoom(&self, a: f32) {
        println!("change_camera_zoom {}", a);
    }
    fn dispatch_change_blur_level(&self, _: &Resources) {
        println!("change_blur_level");
    }
    fn dispatch_change_lines_per_pixel(&self, _: &Resources) {
        println!("change_lines_per_pixel");
    }
    fn dispatch_color_representation(&self, _: &Resources) {
        println!("color_representation");
    }
    fn dispatch_pixel_geometry(&self, _: &Resources) {
        println!("pixel_geometry");
    }
    fn dispatch_pixel_shadow_shape(&self, _: &Resources) {
        println!("pixel_shadow_shape");
    }
    fn dispatch_pixel_shadow_height(&self, _: &Resources) {
        println!("pixel_shadow_height");
    }
    fn dispatch_screen_layering_type(&self, _: &Resources) {
        println!("screen_layering_type");
    }
    fn dispatch_screen_curvature(&self, _: &Resources) {
        println!("screen_curvature");
    }
    fn dispatch_internal_resolution(&self, _: &Resources) {
        println!("internal_resolution");
    }
    fn dispatch_texture_interpolation(&self, _: &Resources) {
        println!("texture_interpolation");
    }
    fn dispatch_change_pixel_speed(&self, a: f32) {
        println!("change_pixel_speed {}", a);
    }
    fn dispatch_change_turning_speed(&self, a: f32) {
        println!("change_turning_speed {}", a);
    }
    fn dispatch_change_movement_speed(&self, a: f32) {
        println!("change_movement_speed {}", a);
    }
    fn dispatch_exiting_session(&self) {
        println!("exiting_session");
    }
    fn dispatch_toggle_info_panel(&self) {
        println!("toggle_info_panel");
    }
    fn dispatch_fps(&self, a: f32) {
        println!("fps {}", a);
    }
    fn dispatch_request_pointer_lock(&self) {
        println!("request_pointer_lock");
    }
    fn dispatch_exit_pointer_lock(&self) {
        println!("exit_pointer_lock");
    }
    fn dispatch_screenshot(&self, _: &[u8], _: f64) {
        println!("screenshot");
    }
    fn dispatch_top_message(&self, msg: &str) {
        println!("top_message: {}", msg);
    }
}