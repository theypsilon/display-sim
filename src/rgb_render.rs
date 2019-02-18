use web_sys::{WebGl2RenderingContext, WebGlVertexArrayObject, WebGlProgram};

use crate::wasm_error::WasmResult;
use crate::render_types::TextureBuffer;
use crate::shaders::{make_shader, TEXTURE_VERTEX_SHADER, QUAD_GEOMETRY, QUAD_INDICES};
use crate::web_utils::{js_f32_array, js_i32_array};
use std::mem::size_of;

pub struct RgbRender {
    texture_buffers: [TextureBuffer; 3],
    vao: Option<WebGlVertexArrayObject>,
    shader: WebGlProgram,
    width: i32, 
    height: i32,
}

impl RgbRender {
    pub fn new(gl: &WebGl2RenderingContext, width: i32, height: i32) -> WasmResult<RgbRender> {
        let texture_buffers = [
            TextureBuffer::new_with_depthbuffer(gl, width, height)?,
            TextureBuffer::new_with_depthbuffer(gl, width, height)?,
            TextureBuffer::new_with_depthbuffer(gl, width, height)?,
        ];

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

        let shader = make_shader(&gl, TEXTURE_VERTEX_SHADER, RGB_FRAGMENT_SHADER)?;
        let q_pos_position = gl.get_attrib_location(&shader, "qPos") as u32;
        let q_texture_position = gl.get_attrib_location(&shader, "qTexCoords") as u32;

        gl.enable_vertex_attrib_array(q_pos_position);
        gl.enable_vertex_attrib_array(q_texture_position);

        gl.vertex_attrib_pointer_with_i32(q_pos_position, 3, WebGl2RenderingContext::FLOAT, false, 5 * size_of::<f32>() as i32, 0);
        gl.vertex_attrib_pointer_with_i32(q_texture_position, 2, WebGl2RenderingContext::FLOAT, false, 5 * size_of::<f32>() as i32, 3 * size_of::<f32>() as i32);

        Ok(RgbRender{ texture_buffers, vao, shader, width, height })
    }

    pub fn bind_framebuffer_for_color(&self, gl: &WebGl2RenderingContext, i: usize) {
        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, self.texture_buffers[i].framebuffer());
        gl.viewport(0, 0, self.width, self.height);

        gl.active_texture(WebGl2RenderingContext::TEXTURE0 + i as u32);
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT|WebGl2RenderingContext::DEPTH_BUFFER_BIT);
    }

    pub fn render(&self, gl: &WebGl2RenderingContext) {
        gl.bind_vertex_array(self.vao.as_ref());
        gl.use_program(Some(&self.shader));
        
        gl.uniform1i(gl.get_uniform_location(&self.shader, "redImage").as_ref(), 0);
        gl.uniform1i(gl.get_uniform_location(&self.shader, "greenImage").as_ref(), 1);
        gl.uniform1i(gl.get_uniform_location(&self.shader, "blueImage").as_ref(), 2);

        gl.active_texture(WebGl2RenderingContext::TEXTURE0 + 0);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, self.texture_buffers[0].texture());
        gl.active_texture(WebGl2RenderingContext::TEXTURE0 + 1);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, self.texture_buffers[1].texture());
        gl.active_texture(WebGl2RenderingContext::TEXTURE0 + 2);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, self.texture_buffers[2].texture());
        gl.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, 6, WebGl2RenderingContext::UNSIGNED_INT, 0);
        gl.active_texture(WebGl2RenderingContext::TEXTURE0 + 0);
    }
}

pub const RGB_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;

out vec4 FragColor;
in vec2 TexCoord;

uniform sampler2D redImage;
uniform sampler2D greenImage;
uniform sampler2D blueImage;

void main()
{
    FragColor = texture(redImage, TexCoord) + texture(greenImage, TexCoord) + texture(blueImage, TexCoord);
} 
"#;
