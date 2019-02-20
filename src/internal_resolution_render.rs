use web_sys::{WebGl2RenderingContext, WebGlVertexArrayObject, WebGlProgram};

use crate::wasm_error::WasmResult;
use crate::shaders::{make_shader, make_quad_vao, TEXTURE_FRAGMENT_SHADER, TEXTURE_VERTEX_SHADER};
use web_sys::{WebGlTexture};

pub struct InternalResolutionRender {
    vao: Option<WebGlVertexArrayObject>,
    shader: WebGlProgram,
}

impl InternalResolutionRender {
    pub fn new(gl: &WebGl2RenderingContext) -> WasmResult<InternalResolutionRender> {
        let shader = make_shader(gl, TEXTURE_VERTEX_SHADER, TEXTURE_FRAGMENT_SHADER)?;
        let vao = make_quad_vao(gl, &shader)?;
        Ok(InternalResolutionRender{vao, shader })
    }

    pub fn render(&self, gl: &WebGl2RenderingContext, texture: Option<&WebGlTexture>) {
        gl.use_program(Some(&self.shader));
        gl.bind_vertex_array(self.vao.as_ref());
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture);
        gl.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, 6, WebGl2RenderingContext::UNSIGNED_INT, 0);
    }
}