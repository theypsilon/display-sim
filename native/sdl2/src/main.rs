use webgl_to_sdl2::{WebGl2RenderingContext, WebResult};

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

fn main() {
    program().unwrap()
}

fn program() -> WebResult<()> {
    let res_input = VideoInputResources {
        steps: vec![AnimationStep{ delay: 60 }],
        max_texture_size: 16000,
        image_size: Size2D{ width: 256, height: 224 },
        background_size: Size2D{ width: 256, height: 224 },
        viewport_size: Size2D{ width: 1920, height: 1080 },
        pixel_width: 1.0,
        stretch: false,
        current_frame: 0,
        last_frame_change: 0.0,
        needs_buffer_data_load: true,
    };
    let materials_input = VideoInputMaterials {
        buffers: vec![Box::new([0; 256 * 224 * 4 * 4])]
    };

    let mut res = Resources::default();
    init_resources(&mut res, res_input, 0.0);
    let gl = WebGl2RenderingContext{};
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
    let mut input = Input::new(0.0);
    let mut ctx : SimulationContext<NativeEventDispatcher> = SimulationContext::default();

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Screen Sim", res.video.viewport_size.width, res.video.viewport_size.height)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        if !SimulationUpdater::new(&mut ctx, &mut res, &input).update() {
            println!("User closed the simulation.");
            return Ok(());
        }

        if res.launch_screenshot || res.screenshot_delay <= 0 {
            SimulationDrawer::new(&mut ctx, &mut materials, &res).draw()?;
        }

        input.now += 0.01;
        window.gl_swap_window();
    }
    Ok(())
}

#[derive(Default)]
struct NativeEventDispatcher {}

impl AppEventDispatcher for NativeEventDispatcher {
    fn dispatch_camera_update(&self, _: &glm::Vec3, _: &glm::Vec3, _: &glm::Vec3) {}
    fn dispatch_change_pixel_horizontal_gap(&self, _: f32) {}
    fn dispatch_change_pixel_vertical_gap(&self, _: f32) {}
    fn dispatch_change_pixel_width(&self, _: f32) {}
    fn dispatch_change_pixel_spread(&self, _: f32) {}
    fn dispatch_change_pixel_brightness(&self, _: &Resources) {}
    fn dispatch_change_pixel_contrast(&self, _: &Resources) {}
    fn dispatch_change_light_color(&self, _: &Resources) {}
    fn dispatch_change_brightness_color(&self, _: &Resources) {}
    fn dispatch_change_camera_zoom(&self, _: f32) {}
    fn dispatch_change_blur_level(&self, _: &Resources) {}
    fn dispatch_change_lines_per_pixel(&self, _: &Resources) {}
    fn dispatch_color_representation(&self, _: &Resources) {}
    fn dispatch_pixel_geometry(&self, _: &Resources) {}
    fn dispatch_pixel_shadow_shape(&self, _: &Resources) {}
    fn dispatch_pixel_shadow_height(&self, _: &Resources) {}
    fn dispatch_screen_layering_type(&self, _: &Resources) {}
    fn dispatch_screen_curvature(&self, _: &Resources) {}
    fn dispatch_internal_resolution(&self, _: &Resources) {}
    fn dispatch_texture_interpolation(&self, _: &Resources) {}
    fn dispatch_change_pixel_speed(&self, _: f32) {}
    fn dispatch_change_turning_speed(&self, _: f32) {}
    fn dispatch_change_movement_speed(&self, _: f32) {}
    fn dispatch_exiting_session(&self) {}
    fn dispatch_toggle_info_panel(&self) {}
    fn dispatch_fps(&self, _: f32) {}
    fn dispatch_request_pointer_lock(&self) {}
    fn dispatch_exit_pointer_lock(&self) {}
    fn dispatch_screenshot(&self, _: &[u8], _: f64) {}
    fn dispatch_top_message(&self, _: &str) {}
}