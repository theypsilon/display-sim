use js_sys::{Float32Array, ArrayBuffer};
use super::glm;
use std::mem::size_of;
use web_sys::{
    WebGl2RenderingContext, WebGlVertexArrayObject, WebGlProgram, WebGlBuffer,
};

use wasm_error::WasmResult;
use shaders::{
    make_shader,
    PIXEL_VERTEX_SHADER, PIXEL_FRAGMENT_SHADER
};
use web_utils::{js_f32_array};

pub enum PixelsRenderKind {
    Squares,
    Cubes
}

pub struct PixelsRender {
    shader: WebGlProgram,
    vao: Option<WebGlVertexArrayObject>,
    colors_vbo: WebGlBuffer,
    element_quantity: i32,
}

pub struct PixelsUniform<'a> {
    pub view: &'a mut [f32],
    pub projection: &'a mut [f32],
    pub light_pos: &'a mut [f32],
    pub light_color: &'a mut [f32],
    pub extra_light: &'a mut [f32],
    pub ambient_strength: f32,
    pub pixel_gap: &'a mut [f32],
    pub pixel_scale: &'a mut [f32],
    pub pixel_pulse: f32,
}

impl PixelsRender {
    pub fn new(gl: &WebGl2RenderingContext, offsets: &Float32Array) -> WasmResult<PixelsRender> {
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
        
        Ok(PixelsRender {vao, shader, colors_vbo, element_quantity: offsets.length() as i32 / 2})
    }

    pub fn apply_colors(&self, gl: &WebGl2RenderingContext, buffer: &ArrayBuffer) {
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.colors_vbo));
        gl.buffer_data_with_opt_array_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&buffer),
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    pub fn render(&self, gl: &WebGl2RenderingContext, pixels_render_kind: &PixelsRenderKind, uniforms: PixelsUniform) {
        gl.use_program(Some(&self.shader));
        gl.uniform_matrix4fv_with_f32_array(gl.get_uniform_location(&self.shader, "view").as_ref(), false, uniforms.view);
        gl.uniform_matrix4fv_with_f32_array(gl.get_uniform_location(&self.shader, "projection").as_ref(), false, uniforms.projection);
        gl.uniform3fv_with_f32_array(gl.get_uniform_location(&self.shader, "lightPos").as_ref(), uniforms.light_pos);
        gl.uniform3fv_with_f32_array(gl.get_uniform_location(&self.shader, "lightColor").as_ref(), uniforms.light_color);
        gl.uniform3fv_with_f32_array(gl.get_uniform_location(&self.shader, "extraLight").as_ref(), uniforms.extra_light);
        gl.uniform1f(gl.get_uniform_location(&self.shader, "ambientStrength").as_ref(), uniforms.ambient_strength);
        gl.uniform2fv_with_f32_array(gl.get_uniform_location(&self.shader, "pixel_gap").as_ref(), uniforms.pixel_gap);
        gl.uniform3fv_with_f32_array(gl.get_uniform_location(&self.shader, "pixel_scale").as_ref(), uniforms.pixel_scale);
        gl.uniform1f(gl.get_uniform_location(&self.shader, "pixel_pulse").as_ref(), uniforms.pixel_pulse);

        gl.bind_vertex_array(self.vao.as_ref());
        gl.draw_arrays_instanced(
            WebGl2RenderingContext::TRIANGLES,
            0,
            match pixels_render_kind { PixelsRenderKind::Squares => 6, PixelsRenderKind::Cubes => 36 },
            self.element_quantity
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