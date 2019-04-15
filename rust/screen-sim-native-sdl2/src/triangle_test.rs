use render::opengl_hooks::{WebGl2RenderingContext, WebGlShader, WebGlProgram, WebResult};
use core::general_types::f32_to_u8;
use std::mem::size_of;

pub fn main() {
    if let Err(e) = program() {
        println!("Error: {:?}", e);
        std::process::exit(-1);
    }
}

pub fn program() -> WebResult<()> {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);
    let window = video_subsystem
        .window("Triangle Test", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);


    let gl = WebGl2RenderingContext::default();

    let vertex_shader = shader_from_source(&gl, TRIANGLE_VERTEX, gl::VERTEX_SHADER).unwrap();
    let fragment_shader = shader_from_source(&gl, TRIANGLE_FRAGMENT, gl::FRAGMENT_SHADER).unwrap();
    let program = load_program_from_shaders(&gl, &vertex_shader, &fragment_shader).unwrap();

    let vertices: Vec<f32> = vec![
        // positions      // colors
        0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom left
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top
    ];

    let vbo = gl.create_buffer();

    gl.bind_buffer(gl::ARRAY_BUFFER, vbo.as_ref());
    gl.buffer_data_with_u8_array(gl::ARRAY_BUFFER, f32_to_u8(&vertices), gl::STATIC_DRAW);
    gl.bind_buffer(gl::ARRAY_BUFFER, None);

    let vao = gl.create_vertex_array();
    gl.bind_vertex_array(vao.as_ref());
    gl.bind_buffer(gl::ARRAY_BUFFER, vbo.as_ref());
    gl.enable_vertex_attrib_array(0);
    // @TODO maybe this impl of 'vertex_attrib_pointer_with_i32' is bad
    gl.vertex_attrib_pointer_with_i32(0, 3, gl::FLOAT, false, (6 * size_of::<f32>()) as gl::types::GLint, 0 * size_of::<f32>() as i32);
    gl.enable_vertex_attrib_array(1);
    gl.vertex_attrib_pointer_with_i32(1, 3, gl::FLOAT, false, (6 * size_of::<f32>()) as gl::types::GLint, 3 * size_of::<f32>() as i32);
    gl.bind_buffer(gl::ARRAY_BUFFER, None);
    gl.bind_vertex_array(None);

    gl.viewport(0, 0, 800, 600);
    gl.clear_color(0.3, 0.3, 0.5, 1.0);

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        gl.clear(gl::COLOR_BUFFER_BIT);

        // draw triangle

        gl.use_program(program.as_ref());
        
        
        gl.bind_vertex_array(vao.as_ref());
        gl.draw_arrays(gl::TRIANGLES, 0, 3);

        window.gl_swap_window();
    };

    Ok(())
}

fn load_program_from_shaders(gl: &WebGl2RenderingContext, vertex_shader: &Option<WebGlShader>, fragment_shader: &Option<WebGlShader>) -> Result<Option<WebGlProgram>, String> {
    let program = gl.create_program();
    {
        let vertex_shader = vertex_shader.as_ref().unwrap();
        let fragment_shader = fragment_shader.as_ref().unwrap();
        let program = program.as_ref().unwrap();

        gl.attach_shader(program, vertex_shader);
        gl.attach_shader(program, fragment_shader);

        gl.link_program(program);

        let success = gl.get_program_parameter(program, gl::LINK_STATUS).as_number();
        if success == 0 {
            return Err(gl.get_program_info_log(program).unwrap());
        }
    }
    Ok(program)
}

fn shader_from_source(
    gl: &WebGl2RenderingContext,
    source: &str,
    kind: gl::types::GLenum,
) -> Result<Option<WebGlShader>, String> {
    let shader = gl.create_shader(kind);
    {
        let shader = shader.as_ref().unwrap();
        gl.shader_source(shader, source);
        gl.compile_shader(shader);

        let success = gl.get_shader_parameter(shader, gl::COMPILE_STATUS).as_number();

        if success == 0 {
            return Err(gl.get_shader_info_log(shader).unwrap());
        }
    }
    Ok(shader)
}

const TRIANGLE_VERTEX: &'static str = "
#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;

out VS_OUTPUT {
    vec3 Color;
} OUT;

void main()
{
    gl_Position = vec4(Position, 1.0);
    OUT.Color = Color;
}
";

const TRIANGLE_FRAGMENT: &'static str = "
#version 330 core

in VS_OUTPUT {
    vec3 Color;
} IN;

out vec4 Color;

void main()
{
    Color = vec4(IN.Color, 1.0f);
}
";