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

use crate::error::WebResult;
use crate::shaders::{make_quad_vao, make_shader, TEXTURE_VERTEX_SHADER};

use glow::GlowSafeAdapter;
use glow::HasContext;
use std::rc::Rc;

pub struct BackgroundRender<GL: HasContext> {
    vao: Option<GL::VertexArray>,
    shader: GL::Program,
    gl: Rc<GlowSafeAdapter<GL>>,
}

impl<GL: HasContext> BackgroundRender<GL> {
    pub fn new(gl: Rc<GlowSafeAdapter<GL>>) -> WebResult<BackgroundRender<GL>> {
        let shader = make_shader(&*gl, TEXTURE_VERTEX_SHADER, BACKGROUND_FRAGMENT_SHADER)?;
        let vao = make_quad_vao(&*gl, &shader)?;
        Ok(BackgroundRender { vao, shader, gl })
    }

    pub fn render(&self) {
        self.gl.bind_vertex_array(self.vao);
        self.gl.use_program(Some(self.shader));
        self.gl.uniform_1_i32(self.gl.get_uniform_location(self.shader, "foregroundImage"), 0);
        self.gl.uniform_1_i32(self.gl.get_uniform_location(self.shader, "backgroundImage"), 1);
        self.gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
    }
}

pub const BACKGROUND_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;

out vec4 FragColor;
in vec2 TexCoord;

uniform sampler2D foregroundImage;
uniform sampler2D backgroundImage;

void main()
{
    vec4 foregroundColor = texture(foregroundImage, TexCoord);
    float foregroundWeight = (foregroundColor.r + foregroundColor.g + foregroundColor.b + foregroundColor.a) / 4.0;
    vec4 backgroundColor = texture(backgroundImage, TexCoord);
    float backgroundWeight = (backgroundColor.r + backgroundColor.g + backgroundColor.b + backgroundColor.a) / 4.0;
    vec4 result1 = foregroundColor.a * foregroundColor + (1.0 - foregroundColor.a) * backgroundColor;
    float weight1 = (result1.r + result1.g + result1.b + result1.a) / 4.0;
    if (foregroundWeight <= 0.26 && backgroundWeight > foregroundWeight) {
        weight1 = 0.0;
    }
    float factor = weight1 / (weight1 + backgroundWeight * 0.1);
    FragColor = result1 * factor + (1.0 - factor) * backgroundColor;
} 
"#;
