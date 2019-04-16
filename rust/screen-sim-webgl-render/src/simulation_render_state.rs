use crate::web::WebGl2RenderingContext;

use crate::background_render::BackgroundRender;
use crate::blur_render::BlurRender;
use crate::error::WebResult;
use crate::internal_resolution_render::InternalResolutionRender;
use crate::pixels_render::PixelsRender;
use crate::render_types::TextureBufferStack;
use crate::rgb_render::RgbRender;

#[derive(Default)]
pub struct VideoInputMaterials {
    pub buffers: Vec<Box<[u8]>>,
}

// Rendering Materials
pub struct Materials {
    pub gl: WebGl2RenderingContext,
    pub main_buffer_stack: TextureBufferStack,
    pub bg_buffer_stack: TextureBufferStack,
    pub pixels_render: PixelsRender,
    pub blur_render: BlurRender,
    pub background_render: BackgroundRender,
    pub internal_resolution_render: InternalResolutionRender,
    pub rgb_render: RgbRender,
    pub screenshot_pixels: Option<Box<[u8]>>,
}

impl Materials {
    pub fn new(gl: WebGl2RenderingContext, video: VideoInputMaterials) -> WebResult<Materials> {
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
}
