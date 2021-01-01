/* Copyright (c) 2019-2021 José manuel Barroso Galindo <theypsilon@gmail.com>
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
use crate::shaders::{make_quad_vao, make_shader, TEXTURE_VERTEX_SHADER};

use glow::GlowSafeAdapter;
use glow::HasContext;
use std::rc::Rc;

pub struct RgbRender<GL: HasContext> {
    vao: Option<GL::VertexArray>,
    shader: GL::Program,
    gl: Rc<GlowSafeAdapter<GL>>,
}

impl<GL: HasContext> RgbRender<GL> {
    pub fn new(gl: Rc<GlowSafeAdapter<GL>>) -> AppResult<RgbRender<GL>> {
        let shader = make_shader(&*gl, TEXTURE_VERTEX_SHADER, RGB_FRAGMENT_SHADER)?;
        let vao = make_quad_vao(&*gl, &shader)?;
        Ok(RgbRender { vao, shader, gl })
    }

    pub fn render(&self) {
        self.gl.bind_vertex_array(self.vao);
        self.gl.use_program(Some(self.shader));

        self.gl.uniform_1_i32(self.gl.get_uniform_location(self.shader, "redImage"), 0);
        self.gl.uniform_1_i32(self.gl.get_uniform_location(self.shader, "greenImage"), 1);
        self.gl.uniform_1_i32(self.gl.get_uniform_location(self.shader, "blueImage"), 2);

        self.gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
    }
}

pub const RGB_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;

out vec4 FragColor;
in vec2 TexCoord;

uniform sampler2D redImage;
uniform sampler2D greenImage;
uniform sampler2D blueImage;

void main()
{
    vec4 red = texture(redImage, TexCoord);
    vec4 green = texture(greenImage, TexCoord);
    vec4 blue = texture(blueImage, TexCoord);
    FragColor = red * red.a + green * green.a + blue * blue.a;
} 
"#;
