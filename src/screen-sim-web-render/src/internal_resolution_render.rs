use crate::web::{WebGl2RenderingContext, WebGlProgram, WebGlVertexArrayObject, WebGlTexture};

use crate::shaders::{make_quad_vao, make_shader, TEXTURE_FRAGMENT_SHADER, TEXTURE_VERTEX_SHADER};
use crate::error::WebResult;

pub struct InternalResolutionRender {
    vao: Option<WebGlVertexArrayObject>,
    shader: WebGlProgram,
    gl: WebGl2RenderingContext,
}

impl InternalResolutionRender {
    pub fn new(gl: &WebGl2RenderingContext) -> WebResult<InternalResolutionRender> {
        let shader = make_shader(gl, TEXTURE_VERTEX_SHADER, TEXTURE_FRAGMENT_SHADER)?;
        let vao = make_quad_vao(gl, &shader)?;
        Ok(InternalResolutionRender { vao, shader, gl: gl.clone() })
    }

    pub fn render(&self, texture: Option<&WebGlTexture>) {
        self.gl.use_program(Some(&self.shader));
        self.gl.bind_vertex_array(self.vao.as_ref());
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture);
        self.gl.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, 6, WebGl2RenderingContext::UNSIGNED_INT, 0);
    }
}
