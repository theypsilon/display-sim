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

use crate::web::{WebGl2RenderingContext, WebGlProgram, WebGlTexture, WebGlVertexArrayObject};

use crate::error::WebResult;
use crate::shaders::{make_quad_vao, make_shader, TEXTURE_FRAGMENT_SHADER, TEXTURE_VERTEX_SHADER};

pub struct InternalResolutionRender {
    vao: Option<WebGlVertexArrayObject>,
    shader: WebGlProgram,
    gl: WebGl2RenderingContext,
}

impl InternalResolutionRender {
    pub fn new(gl: &WebGl2RenderingContext) -> WebResult<InternalResolutionRender> {
        let shader = make_shader(gl, TEXTURE_VERTEX_SHADER, TEXTURE_FRAGMENT_SHADER)?;
        let vao = make_quad_vao(gl, &shader)?;
        Ok(InternalResolutionRender { vao, shader, gl: gl.clone() })
    }

    pub fn render(&self, texture: Option<&WebGlTexture>) {
        self.gl.use_program(Some(&self.shader));
        self.gl.bind_vertex_array(self.vao.as_ref());
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture);
        self.gl
            .draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, 6, WebGl2RenderingContext::UNSIGNED_INT, 0);
    }
}
