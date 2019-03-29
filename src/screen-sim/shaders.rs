use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlVertexArrayObject};

use crate::wasm_error::{WasmError, WasmResult};
use crate::web_utils::{js_f32_array, js_i32_array};
use std::mem::size_of;

pub fn make_shader(gl: &WebGl2RenderingContext, vertex_shader: &str, fragment_shader: &str) -> WasmResult<WebGlProgram> {
    let vert_shader = compile_shader(&gl, WebGl2RenderingContext::VERTEX_SHADER, vertex_shader)?;
    let frag_shader = compile_shader(&gl, WebGl2RenderingContext::FRAGMENT_SHADER, fragment_shader)?;
    link_shader(&gl, [vert_shader, frag_shader].iter())
}

fn compile_shader(gl: &WebGl2RenderingContext, shader_type: u32, source: &str) -> WasmResult<WebGlShader> {
    let shader = gl.create_shader(shader_type).ok_or("Unable to create shader object")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
        Ok(shader)
    } else {
        Err(WasmError::Str(gl.get_shader_info_log(&shader).ok_or("Unknown error creating shader")?))
    }
}

fn link_shader<'a, T: IntoIterator<Item = &'a WebGlShader>>(gl: &WebGl2RenderingContext, shaders: T) -> WasmResult<WebGlProgram> {
    let program = gl.create_program().ok_or("Unable to create shader object")?;
    for shader in shaders {
        gl.attach_shader(&program, shader)
    }
    gl.link_program(&program);

    if gl.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        Err(WasmError::Str(gl.get_program_info_log(&program).ok_or("cannot get program info log")?))
    }
}

pub fn make_quad_vao(gl: &WebGl2RenderingContext, shader: &WebGlProgram) -> WasmResult<Option<WebGlVertexArrayObject>> {
    let vao = gl.create_vertex_array();
    gl.bind_vertex_array(vao.as_ref());

    let quad_vbo = gl.create_buffer().ok_or("cannot create quad_vbo")?;
    let quad_ebo = gl.create_buffer().ok_or("cannot create quad_ebo")?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&quad_vbo));
    gl.buffer_data_with_opt_array_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&js_f32_array(&QUAD_GEOMETRY).buffer()), WebGl2RenderingContext::STATIC_DRAW);
    gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&quad_ebo));
    gl.buffer_data_with_opt_array_buffer(
        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
        Some(&js_i32_array(&QUAD_INDICES).buffer()),
        WebGl2RenderingContext::STATIC_DRAW,
    );

    let q_pos_position = gl.get_attrib_location(shader, "qPos") as u32;
    let q_texture_position = gl.get_attrib_location(shader, "qTexCoords") as u32;

    gl.enable_vertex_attrib_array(q_pos_position);
    gl.enable_vertex_attrib_array(q_texture_position);

    gl.vertex_attrib_pointer_with_i32(q_pos_position, 3, WebGl2RenderingContext::FLOAT, false, 5 * size_of::<f32>() as i32, 0);
    gl.vertex_attrib_pointer_with_i32(q_texture_position, 2, WebGl2RenderingContext::FLOAT, false, 5 * size_of::<f32>() as i32, 3 * size_of::<f32>() as i32);
    Ok(vao)
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
