use js_sys::{Float32Array, ArrayBuffer};
use super::glm;
use std::mem::size_of;
use web_sys::{
    WebGl2RenderingContext, WebGlVertexArrayObject, WebGlProgram, WebGlBuffer,
};

use wasm_error::Result;
use shaders::{
    make_shader,
    PIXEL_VERTEX_SHADER, PIXEL_FRAGMENT_SHADER
};
use web_utils::{js_f32_array};

pub struct PixelsRender {
    pub shader: WebGlProgram,
    pub vao: Option<WebGlVertexArrayObject>,
    pub colors_vbo: WebGlBuffer,
}

impl PixelsRender {
    pub fn new(gl: &WebGl2RenderingContext, offsets: &Float32Array) -> Result<PixelsRender> {
        let shader = make_shader(&gl, PIXEL_VERTEX_SHADER, PIXEL_FRAGMENT_SHADER)?;

        let vao = gl.create_vertex_array();
        gl.bind_vertex_array(vao.as_ref());

        let pixels_vbo = gl.create_buffer().ok_or("cannot create pixels_vbo")?;
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&pixels_vbo));
        gl.buffer_data_with_opt_array_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&js_f32_array(&CUBE_GEOMETRY).buffer()),
            WebGl2RenderingContext::STATIC_DRAW,
        );

        let a_pos_position = gl.get_attrib_location(&shader, "aPos") as u32;
        gl.vertex_attrib_pointer_with_i32(a_pos_position, 3, WebGl2RenderingContext::FLOAT, false, 6 * size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(a_pos_position);

        let a_normal_position = gl.get_attrib_location(&shader, "aNormal") as u32;
        gl.vertex_attrib_pointer_with_i32(a_normal_position, 3, WebGl2RenderingContext::FLOAT, false, 6 * size_of::<f32>() as i32, 3 * size_of::<f32>() as i32);
        gl.enable_vertex_attrib_array(a_normal_position);

        let colors_vbo = gl.create_buffer().ok_or("cannot create colors_vbo")?;
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&colors_vbo));

        let a_color_position = gl.get_attrib_location(&shader, "aColor") as u32;
        gl.enable_vertex_attrib_array(a_color_position);
        gl.vertex_attrib_pointer_with_i32(a_color_position, 1, WebGl2RenderingContext::FLOAT, false, size_of::<f32>() as i32, 0);
        gl.vertex_attrib_divisor(a_color_position, 1);

        let offset_vbo = gl.create_buffer().ok_or("cannot create offset_vbo")?;
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&offset_vbo));
        gl.buffer_data_with_opt_array_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&offsets.buffer()),
            WebGl2RenderingContext::STATIC_DRAW,
        );

        let a_offset_position = gl.get_attrib_location(&shader, "aOffset") as u32;
        gl.enable_vertex_attrib_array(a_offset_position);
        gl.vertex_attrib_pointer_with_i32(a_offset_position, 2, WebGl2RenderingContext::FLOAT, false, size_of::<glm::Vec2>() as i32, 0);
        gl.vertex_attrib_divisor(a_offset_position, 1);
        
        Ok(PixelsRender {vao, shader, colors_vbo})
    }

    pub fn apply_colors(&self, gl: &WebGl2RenderingContext, buffer: &ArrayBuffer) {
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.colors_vbo));
        gl.buffer_data_with_opt_array_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&buffer),
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
}

const CUBE_GEOMETRY : [f32; 216] = [
    // cube coordinates       cube normals
    -0.5, -0.5,  0.5,      0.0,  0.0,  1.0,
     0.5, -0.5,  0.5,      0.0,  0.0,  1.0,
     0.5,  0.5,  0.5,      0.0,  0.0,  1.0,
     0.5,  0.5,  0.5,      0.0,  0.0,  1.0,
    -0.5,  0.5,  0.5,      0.0,  0.0,  1.0,
    -0.5, -0.5,  0.5,      0.0,  0.0,  1.0,

    -0.5, -0.5, -0.5,      0.0,  0.0, -1.0,
     0.5, -0.5, -0.5,      0.0,  0.0, -1.0,
     0.5,  0.5, -0.5,      0.0,  0.0, -1.0,
     0.5,  0.5, -0.5,      0.0,  0.0, -1.0,
    -0.5,  0.5, -0.5,      0.0,  0.0, -1.0,
    -0.5, -0.5, -0.5,      0.0,  0.0, -1.0,

    -0.5,  0.5,  0.5,      -1.0,  0.0,  0.0,
    -0.5,  0.5, -0.5,      -1.0,  0.0,  0.0,
    -0.5, -0.5, -0.5,      -1.0,  0.0,  0.0,
    -0.5, -0.5, -0.5,      -1.0,  0.0,  0.0,
    -0.5, -0.5,  0.5,      -1.0,  0.0,  0.0,
    -0.5,  0.5,  0.5,      -1.0,  0.0,  0.0,

     0.5,  0.5,  0.5,      1.0,  0.0,  0.0,
     0.5,  0.5, -0.5,      1.0,  0.0,  0.0,
     0.5, -0.5, -0.5,      1.0,  0.0,  0.0,
     0.5, -0.5, -0.5,      1.0,  0.0,  0.0,
     0.5, -0.5,  0.5,      1.0,  0.0,  0.0,
     0.5,  0.5,  0.5,      1.0,  0.0,  0.0,

    -0.5, -0.5, -0.5,      0.0, -1.0,  0.0,
     0.5, -0.5, -0.5,      0.0, -1.0,  0.0,
     0.5, -0.5,  0.5,      0.0, -1.0,  0.0,
     0.5, -0.5,  0.5,      0.0, -1.0,  0.0,
    -0.5, -0.5,  0.5,      0.0, -1.0,  0.0,
    -0.5, -0.5, -0.5,      0.0, -1.0,  0.0,

    -0.5,  0.5, -0.5,      0.0,  1.0,  0.0,
     0.5,  0.5, -0.5,      0.0,  1.0,  0.0,
     0.5,  0.5,  0.5,      0.0,  1.0,  0.0,
     0.5,  0.5,  0.5,      0.0,  1.0,  0.0,
    -0.5,  0.5,  0.5,      0.0,  1.0,  0.0,
    -0.5,  0.5, -0.5,      0.0,  1.0,  0.0,
];