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

use crate::background_render::BackgroundRender;
use crate::blur_render::BlurRender;
use crate::error::WebResult;
use crate::internal_resolution_render::InternalResolutionRender;
use crate::pixels_render::PixelsRender;
use crate::render_types::TextureBufferStack;
use crate::rgb_render::RgbRender;

use glow::Context;
use std::rc::Rc;

#[derive(Default)]
pub struct VideoInputMaterials {
    pub buffers: Vec<Box<[u8]>>,
}

// Rendering Materials
pub struct Materials {
    pub gl: Rc<Context>,
    pub main_buffer_stack: TextureBufferStack<Context>,
    pub bg_buffer_stack: TextureBufferStack<Context>,
    pub pixels_render: PixelsRender<Context>,
    pub blur_render: BlurRender<Context>,
    pub background_render: BackgroundRender<Context>,
    pub internal_resolution_render: InternalResolutionRender<Context>,
    pub rgb_render: RgbRender<Context>,
    pub screenshot_pixels: Option<Box<[u8]>>,
}

impl Materials {
    pub fn new(gl: Rc<Context>, video: VideoInputMaterials) -> WebResult<Materials> {
        Ok(Materials {
            main_buffer_stack: TextureBufferStack::new(gl.clone()),
            bg_buffer_stack: TextureBufferStack::new(gl.clone()),
            pixels_render: PixelsRender::new(gl.clone(), video)?,
            blur_render: BlurRender::new(gl.clone())?,
            internal_resolution_render: InternalResolutionRender::new(gl.clone())?,
            rgb_render: RgbRender::new(gl.clone())?,
            background_render: BackgroundRender::new(gl.clone())?,
            screenshot_pixels: None,
            gl,
        })
    }
}
