/* Copyright (c) 2019-2024 José manuel Barroso Galindo <theypsilon@gmail.com>
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
use crate::render_types::{TextureBuffer, TextureBufferStack};
use crate::shaders::{make_quad_vao, make_shader, TEXTURE_VERTEX_SHADER};

use glow::GlowSafeAdapter;
use glow::HasContext;
use std::rc::Rc;

pub struct BlurRender<GL: HasContext> {
    shader: GL::Program,
    vao: Option<GL::VertexArray>,
    gl: Rc<GlowSafeAdapter<GL>>,
}

impl<GL: HasContext> BlurRender<GL> {
    pub fn new(gl: Rc<GlowSafeAdapter<GL>>) -> AppResult<BlurRender<GL>> {
        let shader = make_shader(&*gl, TEXTURE_VERTEX_SHADER, BLUR_FRAGMENT_SHADER)?;
        let vao = make_quad_vao(&*gl, &shader)?;
        Ok(BlurRender { shader, vao, gl })
    }

    pub fn render(&self, stack: &mut TextureBufferStack<GL>, source: &TextureBuffer<GL>, target: &TextureBuffer<GL>, passes: usize) -> AppResult<()> {
        if passes < 1 {
            panic!("Should not be called when passes < 1!");
        }

        stack.push()?;
        stack.push()?;

        let texture_buffers = [stack.get_nth(0)?, stack.get_nth(-1)?];

        let blur_iteration = |texture: Option<GL::Texture>, tb: &TextureBuffer<GL>, horizontal: bool| {
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, tb.framebuffer());
            self.gl.viewport(0, 0, tb.width, tb.height);
            self.gl.bind_texture(glow::TEXTURE_2D, texture);
            self.gl
                .uniform_1_i32(self.gl.get_uniform_location(self.shader, "horizontal"), i32::from(horizontal));
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            self.gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
        };

        self.gl.use_program(Some(self.shader));
        self.gl.bind_vertex_array(self.vao);

        blur_iteration(source.texture(), texture_buffers[0], true);
        for i in 1..passes {
            let buffer_index = i % 2;
            let texture_index = (i + 1) % 2;
            blur_iteration(texture_buffers[texture_index].texture(), texture_buffers[buffer_index], buffer_index == 0);
        }
        let buffer_index = passes % 2;
        let texture_index = (passes + 1) % 2;
        blur_iteration(texture_buffers[texture_index].texture(), target, buffer_index == 0);
        self.gl.bind_vertex_array(None);
        self.gl.bind_texture(glow::TEXTURE_2D, None);
        stack.pop()?;
        stack.pop()?;
        Ok(())
    }
}

pub const BLUR_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;

out vec4 FragColor;
in vec2 TexCoord;

uniform sampler2D image;
uniform int horizontal;
const float weight[5] = float[] (0.2270270270, 0.1945945946, 0.1216216216, 0.0540540541, 0.0162162162);

void main()
{
    vec2 tex_offset = vec2(1.0, 1.0) / float(textureSize(image, 0)); // gets size of single texel
    vec3 result = texture(image, TexCoord).rgb * weight[0];
    if(horizontal == 1)
    {
        for(int i = 1; i < 5; ++i)
        {
            result += texture(image, TexCoord + vec2(tex_offset.x * float(i), 0.0)).rgb * weight[i % 5];
            result += texture(image, TexCoord - vec2(tex_offset.x * float(i), 0.0)).rgb * weight[i % 5];
        }
    }
    else
    {
        for(int i = 1; i < 5; ++i)
        {
            result += texture(image, TexCoord + vec2(0.0, tex_offset.y * float(i))).rgb * weight[i % 5];
            result += texture(image, TexCoord - vec2(0.0, tex_offset.y * float(i))).rgb * weight[i % 5];
        }
    }
    FragColor = vec4(result, 1.0);
}
"#;
