use web_sys::WebGl2RenderingContext;

use crate::console;
use crate::web_utils::now;
use core::app_events::AppEventDispatcher;
use core::simulation_context::SimulationContext;
use core::simulation_core_state::{Input, Resources};
use core::simulation_update::SimulationUpdater;
use derive_new::new;
use render::background_render::BackgroundRender;
use render::blur_render::BlurRender;
use render::internal_resolution_render::InternalResolutionRender;
use render::pixels_render::PixelsRender;
use render::render_types::TextureBufferStack;
use render::rgb_render::RgbRender;
use render::simulation_draw::SimulationDrawer;
use render::simulation_render_state::{Materials, VideoInputMaterials};
use web_error::WebResult;

#[derive(new)]
pub struct SimulationTicker<'a, T: AppEventDispatcher> {
    ctx: &'a mut SimulationContext<T>,
    input: &'a mut Input,
    resources: &'a mut Resources,
    materials: &'a mut Materials,
}

impl<'a, T: AppEventDispatcher> SimulationTicker<'a, T> {
    pub fn tick(&mut self) -> WebResult<bool> {
        self.pre_process_input()?;

        if !SimulationUpdater::new(self.ctx, self.resources, self.input).update() {
            console!(log. "User closed the simulation.");
            return Ok(false);
        }
        self.post_process_input();
        if self.resources.launch_screenshot || self.resources.screenshot_delay <= 0 {
            SimulationDrawer::new(self.ctx, self.materials, self.resources).draw()?;
        }
        Ok(true)
    }

    fn pre_process_input(&mut self) -> WebResult<()> {
        self.input.now = now()?;
        let mut my_iter = self.input.get_mut_fields_booleanbutton().into_iter();
        self.input
            .get_mut_fields_incdec_booleanbutton_()
            .iter_mut()
            .for_each(|incdec| incdec.get_mut_fields_t().iter_mut().for_each(|button| my_iter.chain(button)))
        /*self.input.get_mut_fields_booleanbutton().iter_mut().for_each(|button| button.track_input());
        self.input
            .get_mut_fields_incdec_booleanbutton_()
            .iter_mut()
            .for_each(|incdec| incdec.get_mut_fields_t().iter_mut().for_each(|button| button.track_input()));*/
        Ok(())
    }

    fn post_process_input(&mut self) {
        self.input.mouse_scroll_y = 0.0;
        self.input.mouse_position_x = 0;
        self.input.mouse_position_y = 0;
        self.input.custom_event.kind = String::new();
    }
}

pub fn load_materials(gl: WebGl2RenderingContext, video: VideoInputMaterials) -> WebResult<Materials> {
    Ok(Materials {
        main_buffer_stack: TextureBufferStack::new(&gl),
        bg_buffer_stack: TextureBufferStack::new(&gl),
        pixels_render: PixelsRender::new(&gl, video)?,
        blur_render: BlurRender::new(&gl)?,
        internal_resolution_render: InternalResolutionRender::new(&gl)?,
        rgb_render: RgbRender::new(&gl)?,
        background_render: BackgroundRender::new(&gl)?,
        screenshot_pixels: None,
        gl,
    })
}
