// Copyright 2013 The gl-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate gl;
extern crate sdl2;
extern crate libc;

use std::ffi::CString;
use std::ptr;
use std::str;
use std::mem;
use libc::c_void;

use gl::types::{GLfloat, GLenum, GLuint, GLint, GLchar, GLsizeiptr};
use gl::types::{GLboolean};

use sdl2::video::{Window, GLAttr, OPENGL, WindowPos};
use sdl2::event::Event;
use sdl2::keycode::KeyCode;

fn main() {
    sdl2::init(::sdl2::INIT_VIDEO);

    sdl2::video::gl_set_attribute(GLAttr::GLContextMajorVersion, 3);
    sdl2::video::gl_set_attribute(GLAttr::GLContextMinorVersion, 2);

    sdl2::video::gl_set_attribute(GLAttr::GLDoubleBuffer, 1);
    sdl2::video::gl_set_attribute(GLAttr::GLDepthSize, 24);

    let window = match Window::new("gb-rs",
                                   WindowPos::PosCentered,
                                   WindowPos::PosCentered,
                                   800, 600,
                                   OPENGL) {
        Ok(window) => window,
        Err(err)   => panic!("failed to create SDL2 window: {}", err)
    };

    let context = window.gl_create_context().unwrap();

    gl::load_with(|s| sdl2::video::gl_get_proc_address(s).unwrap() as *const c_void);

    sdl2::video::gl_set_swap_interval(1);

    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);

    let program = link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&VERTEX_DATA[0]),
                       gl::STATIC_DRAW);

        // Use shader program
        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0,
                                 CString::from_slice(b"out_color").as_ptr());

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(program,
                                             CString::from_slice(b"position").as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT,
                                gl::FALSE as GLboolean, 0, ptr::null());
    }


    'mainloop: loop {
        loop {
            match sdl2::event::poll_event() {
                Event::None => break,
                Event::KeyDown { keycode: KeyCode::Escape, .. } => break 'mainloop,
                Event::Quit { .. } => break 'mainloop,
                _ => (),
            }
        }

        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Draw a triangle from the 3 vertices
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.gl_swap_window();
    }

    sdl2::quit();
}

// Vertex data
static VERTEX_DATA: [GLfloat; 6] = [
    0.0,  0.5,
    0.5, -0.5,
    -0.5, -0.5
        ];

// Shader sources
static VS_SRC: &'static str =
    "#version 150\n\
    in vec2 position;\n\
    void main() {\n\
    gl_Position = vec4(position, 0.0, 1.0);\n\
    }";

static FS_SRC: &'static str =
   "#version 150\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = vec4(1.0, 1.0, 1.0, 1.0);\n\
    }";

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::from_slice(src.as_bytes());
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::new();
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(buf.as_slice()).ok().expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::new();
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(buf.as_slice()).ok().expect("ProgramInfoLog not valid utf8"));
        }
        program
    }
}