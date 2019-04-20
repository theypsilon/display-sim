use core::app_events::AppEventDispatcher;
use core::general_types::Size2D;
use core::internal_resolution::InternalResolution;
use core::pixels_shadow::ShadowShape;
use core::simulation_context::SimulationContext;
use core::simulation_core_state::{AnimationStep, Input, Resources, VideoInputResources};
use core::simulation_core_state::{ColorChannels, PixelsGeometryKind, ScreenCurvatureKind, ScreenLayeringKind, TextureInterpolation};
use core::simulation_core_ticker::SimulationCoreTicker;
use render::background_render::BackgroundRender;
use render::blur_render::BlurRender;
use render::internal_resolution_render::InternalResolutionRender;
use render::pixels_render::PixelsRender;
use render::render_types::TextureBufferStack;
use render::rgb_render::RgbRender;
use render::simulation_draw::SimulationDrawer;
use render::simulation_render_state::{Materials, VideoInputMaterials};
use render::stubs::{WebGl2RenderingContext, WebResult};
use std::fmt::Display;
use std::time::SystemTime;

pub struct FakeVideoInput(VideoInputResources, VideoInputMaterials);

impl Default for FakeVideoInput {
    fn default() -> FakeVideoInput {
        FakeVideoInput(
            VideoInputResources {
                steps: vec![AnimationStep { delay: 60 }],
                max_texture_size: 16000,
                image_size: Size2D { width: 256, height: 240 },
                background_size: Size2D { width: 256, height: 240 },
                viewport_size: Size2D { width: 256, height: 240 },
                pixel_width: 1.0,
                stretch: false,
                current_frame: 0,
                last_frame_change: 0.0,
                needs_buffer_data_load: true,
            },
            VideoInputMaterials {
                buffers: vec![Box::new([0; 256 * 224 * 4 * 4])],
            },
        )
    }
}

impl FakeVideoInput {
    pub fn iterate_times(self, times: u128) -> WebResult<()> {
        let mut res = Resources::default();
        res.initialize(self.0, 0.0);
        let gl = WebGl2RenderingContext {};
        let mut materials = Materials {
            main_buffer_stack: TextureBufferStack::new(&gl),
            bg_buffer_stack: TextureBufferStack::new(&gl),
            pixels_render: PixelsRender::new(&gl, self.1)?,
            blur_render: BlurRender::new(&gl)?,
            internal_resolution_render: InternalResolutionRender::new(&gl)?,
            rgb_render: RgbRender::new(&gl)?,
            background_render: BackgroundRender::new(&gl)?,
            screenshot_pixels: None,
            gl,
        };

        let now = SystemTime::now();
        let mut input = Input::new(0.0);
        let mut ctx: SimulationContext<FakeEventDispatcher> = SimulationContext::default();
        for _ in 0..times {
            SimulationCoreTicker::new(&mut ctx, &mut res, &mut input).tick(now.elapsed().map_err(|e| e.to_string())?.as_millis() as f64 * 0.05);
            if res.quit {
                println!("User closed the simulation.");
                return Ok(());
            }
            if !res.drawable {
                continue;
            }
            SimulationDrawer::new(&mut ctx, &mut materials, &res).draw()?;
        }
        Ok(())
    }
}

#[derive(Default)]
struct FakeEventDispatcher {}

impl AppEventDispatcher for FakeEventDispatcher {
    fn enable_extra_messages(&self, _: bool) {}
    fn dispatch_camera_update(&self, _: &glm::Vec3, _: &glm::Vec3, _: &glm::Vec3) {}
    fn dispatch_change_pixel_horizontal_gap(&self, _: f32) {}
    fn dispatch_change_pixel_vertical_gap(&self, _: f32) {}
    fn dispatch_change_pixel_width(&self, _: f32) {}
    fn dispatch_change_pixel_spread(&self, _: f32) {}
    fn dispatch_change_pixel_brightness(&self, _: f32) {}
    fn dispatch_change_pixel_contrast(&self, _: f32) {}
    fn dispatch_change_light_color(&self, _: i32) {}
    fn dispatch_change_brightness_color(&self, _: i32) {}
    fn dispatch_change_camera_zoom(&self, _: f32) {}
    fn dispatch_change_blur_level(&self, _: usize) {}
    fn dispatch_change_lines_per_pixel(&self, _: usize) {}
    fn dispatch_color_representation(&self, _: &ColorChannels) {}
    fn dispatch_pixel_geometry(&self, _: &PixelsGeometryKind) {}
    fn dispatch_pixel_shadow_shape(&self, _: &ShadowShape) {}
    fn dispatch_pixel_shadow_height(&self, _: f32) {}
    fn dispatch_screen_layering_type(&self, _: &ScreenLayeringKind) {}
    fn dispatch_screen_curvature(&self, _: &ScreenCurvatureKind) {}
    fn dispatch_internal_resolution(&self, _: &InternalResolution) {}
    fn dispatch_texture_interpolation(&self, _: &TextureInterpolation) {}
    fn dispatch_change_pixel_speed(&self, _: f32) {}
    fn dispatch_change_turning_speed(&self, _: f32) {}
    fn dispatch_change_movement_speed(&self, _: f32) {}
    fn dispatch_exiting_session(&self) {}
    fn dispatch_toggle_info_panel(&self) {}
    fn dispatch_fps(&self, fps: f32) {
        println!("frames in 20 seconds: {}", fps);
    }
    fn dispatch_request_pointer_lock(&self) {}
    fn dispatch_exit_pointer_lock(&self) {}
    fn dispatch_screenshot(&self, _: &[u8], _: f64) {}
    fn dispatch_change_camera_movement_mode(&self, _: bool) {}
    fn dispatch_top_message(&self, _: &str) {}
    fn dispatch_minimum_value<T: Display>(&self, _: &T) {}
    fn dispatch_maximum_value<T: Display>(&self, _: &T) {}
}
