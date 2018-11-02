use std::mem::size_of;
use web_sys::{
    WebGl2RenderingContext, WebGlVertexArrayObject, WebGlProgram, WebGlFramebuffer, WebGlTexture,
};

use wasm_error::WasmResult;
use shaders::{
    make_shader,
};
use web_utils::{js_f32_array, js_i32_array};


pub struct BlurRender {
    shader: WebGlProgram,
    vao: Option<WebGlVertexArrayObject>,
    framebuffers: [Option<WebGlFramebuffer>; 2],
    textures: [Option<WebGlTexture>; 2],
}

impl BlurRender {
    pub fn new(gl: &WebGl2RenderingContext, width: i32, height: i32) -> WasmResult<BlurRender> {
            
        let framebuffers = [
            gl.create_framebuffer(),
            gl.create_framebuffer(),
        ];

        let textures = [
            gl.create_texture(),
            gl.create_texture(),
        ];

        for i in 0..=1 {
            gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, framebuffers[i].as_ref());
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, textures[i].as_ref());
            gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                WebGl2RenderingContext::TEXTURE_2D, 0, WebGl2RenderingContext::RGBA as i32, width, height, 0, WebGl2RenderingContext::RGBA, WebGl2RenderingContext::UNSIGNED_BYTE, None
            )?;
            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR as i32);
            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR as i32);
            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::REPEAT as i32);
            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::REPEAT as i32);
            gl.framebuffer_texture_2d(WebGl2RenderingContext::FRAMEBUFFER, WebGl2RenderingContext::COLOR_ATTACHMENT0, WebGl2RenderingContext::TEXTURE_2D, textures[i].as_ref(), 0);
        }

        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);

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

        let shader = make_shader(&gl, BLUR_VERTEX_SHADER, BLUR_FRAGMENT_SHADER)?;
        gl.use_program(Some(&shader));
        gl.uniform1i(gl.get_uniform_location(&shader, "image").as_ref(), 0);

        let q_pos_position = gl.get_attrib_location(&shader, "qPos") as u32;
        let q_texture_position = gl.get_attrib_location(&shader, "qTexCoords") as u32;

        gl.enable_vertex_attrib_array(q_pos_position);
        gl.enable_vertex_attrib_array(q_texture_position);

        gl.vertex_attrib_pointer_with_i32(q_pos_position, 3, WebGl2RenderingContext::FLOAT, false, 5 * size_of::<f32>() as i32, 0);
        gl.vertex_attrib_pointer_with_i32(q_texture_position, 2, WebGl2RenderingContext::FLOAT, false, 5 * size_of::<f32>() as i32, 3 * size_of::<f32>() as i32);

        Ok(BlurRender {shader, vao, framebuffers, textures})
    }

    pub fn pre_render(&self, gl: &WebGl2RenderingContext) {
        for i in 0..=1 {
            gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, self.framebuffers[i].as_ref());
            gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT|WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        }
    }

    pub fn render(&self, gl: &WebGl2RenderingContext, passes: usize) {
        gl.use_program(Some(&self.shader));
        gl.bind_vertex_array(self.vao.as_ref());
        for i in 0 ..= passes {
            let buffer_index = i % 2;
            let texture_index = (i + 1) % 2;

            gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, if i < passes { self.framebuffers[buffer_index].as_ref() } else { None });
            if i == passes {
                gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT|WebGl2RenderingContext::DEPTH_BUFFER_BIT);
            }
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, self.textures[texture_index].as_ref());
            gl.uniform1i(gl.get_uniform_location(&self.shader, "horizontal").as_ref(), buffer_index as i32);
            gl.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, 6, WebGl2RenderingContext::UNSIGNED_INT, 0);
        }
        gl.bind_vertex_array(None);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
    }
}

const QUAD_GEOMETRY : [f32; 20] = [
    1.0,  1.0, 0.0,   1.0, 1.0,
    1.0, -1.0, 0.0,   1.0, 0.0,
    -1.0, -1.0, 0.0,   0.0, 0.0,
    -1.0,  1.0, 0.0,   0.0, 1.0
];

const QUAD_INDICES: [i32; 6] = [
    0, 1, 3,
    1, 2, 3,
];


pub const BLUR_VERTEX_SHADER: &str = r#"#version 300 es
precision highp float;

layout (location = 0) in vec3 qPos;
layout (location = 1) in vec2 qTexCoords;

out vec2 TexCoord;

void main()
{
    TexCoord = qTexCoords;
    gl_Position = vec4(qPos, 1.0);
}
"#;

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
