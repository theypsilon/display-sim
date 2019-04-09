use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlVertexArrayObject};

use crate::shaders::{make_quad_vao, make_shader, TEXTURE_VERTEX_SHADER};
use web_error::WebResult;

pub struct RgbRender {
    vao: Option<WebGlVertexArrayObject>,
    shader: WebGlProgram,
    gl: WebGl2RenderingContext,
}

impl RgbRender {
    pub fn new(gl: &WebGl2RenderingContext) -> WebResult<RgbRender> {
        let shader = make_shader(gl, TEXTURE_VERTEX_SHADER, RGB_FRAGMENT_SHADER)?;
        let vao = make_quad_vao(gl, &shader)?;
        Ok(RgbRender { vao, shader, gl: gl.clone() })
    }

    pub fn render(&self) {
        self.gl.bind_vertex_array(self.vao.as_ref());
        self.gl.use_program(Some(&self.shader));

        self.gl.uniform1i(self.gl.get_uniform_location(&self.shader, "redImage").as_ref(), 0);
        self.gl.uniform1i(self.gl.get_uniform_location(&self.shader, "greenImage").as_ref(), 1);
        self.gl.uniform1i(self.gl.get_uniform_location(&self.shader, "blueImage").as_ref(), 2);

        self.gl.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, 6, WebGl2RenderingContext::UNSIGNED_INT, 0);
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
    vec4 red = texture(redImage, TexCoord);
    vec4 green = texture(greenImage, TexCoord);
    vec4 blue = texture(blueImage, TexCoord);
    FragColor = red * red.a + green * green.a + blue * blue.a;
} 
"#;
