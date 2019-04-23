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
