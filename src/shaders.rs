use web_sys::{
    WebGlProgram, WebGl2RenderingContext, 
    WebGlShader,
};

use wasm_error::{WasmError, WasmResult};

pub fn make_shader(gl: &WebGl2RenderingContext, vertex_shader: &str, fragment_shader: &str) -> WasmResult<WebGlProgram> {
    let vert_shader = compile_shader(
        &gl,
        WebGl2RenderingContext::VERTEX_SHADER,
        vertex_shader,
    )?;
    let frag_shader = compile_shader(
        &gl,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        fragment_shader,
    )?;
    link_shader(&gl, [vert_shader, frag_shader].iter())
}

fn compile_shader(gl: &WebGl2RenderingContext, shader_type: u32, source: &str) -> WasmResult<WebGlShader> {
    let shader = gl.create_shader(shader_type).ok_or("Unable to create shader object")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(WasmError::Str(gl
            .get_shader_info_log(&shader)
            .ok_or("Unknown error creating shader")?)
        )
    }
}

fn link_shader<'a, T: IntoIterator<Item = &'a WebGlShader>>(gl: &WebGl2RenderingContext, shaders: T) -> WasmResult<WebGlProgram> {
    let program = gl.create_program().ok_or("Unable to create shader object")?;
    for shader in shaders {
        gl.attach_shader(&program, shader)
    }
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(WasmError::Str(gl.get_program_info_log(&program).ok_or("cannot get program info log")?))
    }
}