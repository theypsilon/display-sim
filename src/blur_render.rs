use std::mem::size_of;
use web_sys::{
    WebGl2RenderingContext, WebGlVertexArrayObject, WebGlProgram, WebGlFramebuffer, WebGlTexture,
};

use crate::wasm_error::WasmResult;
use crate::shaders::{
    make_shader, QUAD_GEOMETRY, QUAD_INDICES, TEXTURE_VERTEX_SHADER
};
use crate::web_utils::{js_f32_array, js_i32_array};
use crate::render_types::TextureBuffer;

pub struct BlurRender {
    shader: WebGlProgram,
    vao: Option<WebGlVertexArrayObject>,
    texture_buffers: [TextureBuffer; 2],
}

impl BlurRender {
    pub fn new(gl: &WebGl2RenderingContext, width: i32, height: i32) -> WasmResult<BlurRender> {

        let texture_buffers = [
            TextureBuffer::new(gl, width, height)?, TextureBuffer::new(gl, width, height)?,
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

        let shader = make_shader(&gl, TEXTURE_VERTEX_SHADER, BLUR_FRAGMENT_SHADER)?;
        gl.use_program(Some(&shader));
        gl.uniform1i(gl.get_uniform_location(&shader, "image").as_ref(), 0);

        let q_pos_position = gl.get_attrib_location(&shader, "qPos") as u32;
        let q_texture_position = gl.get_attrib_location(&shader, "qTexCoords") as u32;

        gl.enable_vertex_attrib_array(q_pos_position);
        gl.enable_vertex_attrib_array(q_texture_position);

        gl.vertex_attrib_pointer_with_i32(q_pos_position, 3, WebGl2RenderingContext::FLOAT, false, 5 * size_of::<f32>() as i32, 0);
        gl.vertex_attrib_pointer_with_i32(q_texture_position, 2, WebGl2RenderingContext::FLOAT, false, 5 * size_of::<f32>() as i32, 3 * size_of::<f32>() as i32);

        Ok(BlurRender {shader, vao, texture_buffers})
    }

    pub fn render(&self, gl: &WebGl2RenderingContext, passes: usize, target: &TextureBuffer) {
        if passes < 1 {
            panic!("Should not be called when passes < 1!");
        }

        for i in 0..=1 {
            gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, self.texture_buffers[i].framebuffer());
            gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT|WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        }

        let blur_iteration = |texture: Option<&WebGlTexture>, framebuffer: Option<&WebGlFramebuffer>, horizontal: i32| {
            gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, framebuffer);
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture);
            gl.uniform1i(gl.get_uniform_location(&self.shader, "horizontal").as_ref(), horizontal);
            gl.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, 6, WebGl2RenderingContext::UNSIGNED_INT, 0);
        };

        gl.use_program(Some(&self.shader));
        gl.bind_vertex_array(self.vao.as_ref());
        blur_iteration(target.texture(), self.texture_buffers[0].framebuffer(), 0);
        for i in 1 .. passes {
            let buffer_index = i % 2;
            let texture_index = (i + 1) % 2;
            blur_iteration(self.texture_buffers[texture_index].texture(), self.texture_buffers[buffer_index].framebuffer(), buffer_index as i32);
        }

        let buffer_index = passes % 2;
        let texture_index = (passes + 1) % 2;
        blur_iteration(self.texture_buffers[texture_index].texture(), target.framebuffer(), buffer_index as i32);
        gl.bind_vertex_array(None);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
    }
}

pub const BLUR_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;

out vec4 FragColor;
in vec2 TexCoord;

uniform sampler2D image;
uniform int horizontal;
const float weight[5] = float[] (0.2270270270, 0.1945945946, 0.1216216216, 0.0540540541, 0.0162162162);

void main()
{
    vec2 tex_offset = vec2(1.0, 1.0) / float(textureSize(image, 0)); // gets size of single texel
    vec3 result = texture(image, TexCoord).rgb * weight[0];
    if(horizontal == 1)
    {
        for(int i = 1; i < 5; ++i)
        {
            result += texture(image, TexCoord + vec2(tex_offset.x * float(i), 0.0)).rgb * weight[i % 5];
            result += texture(image, TexCoord - vec2(tex_offset.x * float(i), 0.0)).rgb * weight[i % 5];
        }
    }
    else
    {
        for(int i = 1; i < 5; ++i)
        {
            result += texture(image, TexCoord + vec2(0.0, tex_offset.y * float(i))).rgb * weight[i % 5];
            result += texture(image, TexCoord - vec2(0.0, tex_offset.y * float(i))).rgb * weight[i % 5];
        }
    }
    FragColor = vec4(result, 1.0);
}
"#;
