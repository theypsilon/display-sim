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
use core::general_types::{f32_to_u8, i32_to_u8};
use glow::GlowSafeAdapter;
use glow::HasContext;
use std::mem::size_of;

pub fn make_shader<GL: HasContext>(gl: &GlowSafeAdapter<GL>, vertex_shader: &str, fragment_shader: &str) -> AppResult<GL::Program> {
    let vert_shader = compile_shader(gl, glow::VERTEX_SHADER, vertex_shader)?;
    let frag_shader = compile_shader(gl, glow::FRAGMENT_SHADER, fragment_shader)?;
    link_shader(gl, [vert_shader, frag_shader].iter())
}

fn compile_shader<GL: HasContext>(gl: &GlowSafeAdapter<GL>, shader_type: u32, source: &str) -> AppResult<GL::Shader> {
    let shader = gl.create_shader(shader_type)?;
    gl.shader_source(shader, source);
    gl.compile_shader(shader);

    if gl.get_shader_compile_status(shader) {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(shader).into())
    }
}

fn link_shader<'a, GL: HasContext + 'a, T: IntoIterator<Item = &'a GL::Shader>>(gl: &GlowSafeAdapter<GL>, shaders: T) -> AppResult<GL::Program> {
    let program = gl.create_program()?;
    for shader in shaders {
        gl.attach_shader(program, *shader)
    }
    gl.link_program(program);

    if gl.get_program_link_status(program) {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(program).into())
    }
}

pub fn make_quad_vao<GL: HasContext>(gl: &GlowSafeAdapter<GL>, shader: &GL::Program) -> AppResult<Option<GL::VertexArray>> {
    let vao = gl.create_vertex_array()?;
    gl.bind_vertex_array(Some(vao));

    let quad_vbo = gl.create_buffer()?;
    let quad_ebo = gl.create_buffer()?;
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(quad_vbo));
    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, f32_to_u8(&QUAD_GEOMETRY), glow::STATIC_DRAW);
    gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(quad_ebo));
    gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, i32_to_u8(&QUAD_INDICES), glow::STATIC_DRAW);

    let q_pos_position = gl.get_attrib_location(*shader, "qPos") as u32;
    let q_texture_position = gl.get_attrib_location(*shader, "qTexCoords") as u32;

    gl.enable_vertex_attrib_array(q_pos_position);
    gl.enable_vertex_attrib_array(q_texture_position);

    gl.vertex_attrib_pointer_f32(q_pos_position, 3, glow::FLOAT, false, 5 * size_of::<f32>() as i32, 0);
    gl.vertex_attrib_pointer_f32(
        q_texture_position,
        2,
        glow::FLOAT,
        false,
        5 * size_of::<f32>() as i32,
        3 * size_of::<f32>() as i32,
    );
    Ok(Some(vao))
}

#[rustfmt::skip]
pub const QUAD_GEOMETRY : [f32; 20] = [
    1.0,  1.0, 0.0,   1.0, 1.0,
    1.0, -1.0, 0.0,   1.0, 0.0,
    -1.0, -1.0, 0.0,   0.0, 0.0,
    -1.0,  1.0, 0.0,   0.0, 1.0
];

#[rustfmt::skip]
pub const QUAD_INDICES: [i32; 6] = [
    0, 1, 3,
    1, 2, 3,
];

pub const TEXTURE_VERTEX_SHADER: &str = r#"#version 300 es
precision highp float;

layout (location = 0) in vec3 qPos;
layout (location = 1) in vec2 qTexCoords;

out vec2 TexCoord;

void main()
{
    TexCoord = qTexCoords;
    gl_Position = vec4(qPos, 1.0);
}
"#;

pub const TEXTURE_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;

out vec4 FragColor;
in vec2 TexCoord;

uniform sampler2D image;

void main()
{
    FragColor = texture(image, TexCoord);
} 
"#;
