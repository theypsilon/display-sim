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

use crate::error::AppResult;
use crate::shaders::{make_quad_vao, make_shader, TEXTURE_FRAGMENT_SHADER, TEXTURE_VERTEX_SHADER};

use glow::GlowSafeAdapter;
use glow::HasContext;
use std::rc::Rc;

pub struct InternalResolutionRender<GL: HasContext> {
    vao: Option<GL::VertexArray>,
    shader: GL::Program,
    gl: Rc<GlowSafeAdapter<GL>>,
}

impl<GL: HasContext> InternalResolutionRender<GL> {
    pub fn new(gl: Rc<GlowSafeAdapter<GL>>) -> AppResult<InternalResolutionRender<GL>> {
        let shader = make_shader(&*gl, TEXTURE_VERTEX_SHADER, TEXTURE_FRAGMENT_SHADER)?;
        let vao = make_quad_vao(&*gl, &shader)?;
        Ok(InternalResolutionRender { vao, shader, gl })
    }

    pub fn render(&self, texture: Option<GL::Texture>) {
        self.gl.use_program(Some(self.shader));
        self.gl.bind_vertex_array(self.vao);
        self.gl.bind_texture(glow::TEXTURE_2D, texture);
        self.gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
    }
}
