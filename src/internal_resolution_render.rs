use web_sys::{WebGl2RenderingContext, WebGlVertexArrayObject, WebGlProgram};

use crate::wasm_error::WasmResult;
use crate::render_types::TextureBuffer;
use crate::shaders::{make_shader, TEXTURE_FRAGMENT_SHADER, TEXTURE_VERTEX_SHADER, QUAD_GEOMETRY, QUAD_INDICES};
use crate::web_utils::{js_f32_array, js_i32_array};
use std::mem::size_of;

pub struct InternalResolutionRender {
    pub texture_buffer: TextureBuffer,
    vao: Option<WebGlVertexArrayObject>,
    shader: WebGlProgram,
    width: i32,
    height: i32,
}

impl InternalResolutionRender {
    pub fn new(gl: &WebGl2RenderingContext, width: i32, height: i32) -> WasmResult<InternalResolutionRender> {
        let texture_buffer = TextureBuffer::new_with_depthbuffer(gl, width, height)?;

        let vao = gl.create_vertex_array();
        gl.bind_vertex_array(vao.as_ref());

        let quad_vbo = gl.create_buffer().ok_or("cannot create quad_vbo")?;
        let quad_ebo = gl.create_buffer().ok_or("cannot create quad_ebo")?;
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&quad_vbo));
        gl.buffer_data_with_opt_array_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&js_f32_array(&QUAD_GEOMETRY).buffer()),
            WebGl2RenderingContext::STATIC_DRAW,
        );
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&quad_ebo));
        gl.buffer_data_with_opt_array_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&js_i32_array(&QUAD_INDICES).buffer()), WebGl2RenderingContext::STATIC_DRAW);

        let shader = make_shader(&gl, TEXTURE_VERTEX_SHADER, TEXTURE_FRAGMENT_SHADER)?;
        gl.use_program(Some(&shader));
        gl.uniform1i(gl.get_uniform_location(&shader, "image").as_ref(), 0);

        let q_pos_position = gl.get_attrib_location(&shader, "qPos") as u32;
        let q_texture_position = gl.get_attrib_location(&shader, "qTexCoords") as u32;

        gl.enable_vertex_attrib_array(q_pos_position);
        gl.enable_vertex_attrib_array(q_texture_position);

        gl.vertex_attrib_pointer_with_i32(q_pos_position, 3, WebGl2RenderingContext::FLOAT, false, 5 * size_of::<f32>() as i32, 0);
        gl.vertex_attrib_pointer_with_i32(q_texture_position, 2, WebGl2RenderingContext::FLOAT, false, 5 * size_of::<f32>() as i32, 3 * size_of::<f32>() as i32);        

        Ok(InternalResolutionRender{ texture_buffer, vao, shader, width, height })
    }

    pub fn bind_framebuffer(&self, gl: &WebGl2RenderingContext) {
        gl.enable(WebGl2RenderingContext::DEPTH_TEST);
        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, self.texture_buffer.framebuffer());
        gl.viewport(0, 0, self.width, self.height);
        gl.clear_color(0.05, 0.05, 0.05, 0.0);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT|WebGl2RenderingContext::DEPTH_BUFFER_BIT);
    }

    pub fn render(&self, gl: &WebGl2RenderingContext, target_width: i32, target_height: i32) {
        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
        gl.viewport(0, 0, target_width, target_height);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        gl.use_program(Some(&self.shader));
        gl.bind_vertex_array(self.vao.as_ref());
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, self.texture_buffer.texture());
        gl.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, 6, WebGl2RenderingContext::UNSIGNED_INT, 0);
    }
}