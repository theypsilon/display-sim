/* Copyright (c) 2019 José manuel Barroso Galindo <theypsilon@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use crate::web::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlTexture, WebGlVertexArrayObject};

use crate::error::WebResult;
use crate::shaders::make_shader;
use crate::simulation_render_state::VideoInputMaterials;
use core::general_types::f32_to_u8;
use core::pixels_shadow::{get_shadows, TEXTURE_SIZE};
use core::simulation_core_state::{PixelsGeometryKind, VideoInputResources};

use std::mem::size_of;

pub struct PixelsRender {
    shader: WebGlProgram,
    vao: Option<WebGlVertexArrayObject>,
    colors_vbo: WebGlBuffer,
    offsets_vbo: WebGlBuffer,
    width: u32,
    height: u32,
    offset_inverse_max_length: f32,
    shadows: Vec<Option<WebGlTexture>>,
    video_buffers: Vec<Box<[u8]>>,
    gl: WebGl2RenderingContext,
}

pub struct PixelsUniform<'a> {
    pub shadow_kind: usize,
    pub geometry_kind: PixelsGeometryKind,
    pub view: &'a [f32],
    pub projection: &'a [f32],
    pub light_pos: &'a [f32],
    pub light_color: &'a [f32],
    pub extra_light: &'a [f32],
    pub ambient_strength: f32,
    pub contrast_factor: f32,
    pub screen_curvature: f32,
    pub pixel_spread: &'a [f32],
    pub pixel_scale: &'a [f32],
    pub pixel_offset: &'a [f32],
    pub pixel_pulse: f32,
    pub height_modifier_factor: f32,
}

impl PixelsRender {
    pub fn new(gl: &WebGl2RenderingContext, video_materials: VideoInputMaterials) -> WebResult<PixelsRender> {
        let shader = make_shader(&gl, PIXEL_VERTEX_SHADER, PIXEL_FRAGMENT_SHADER)?;

        let vao = gl.create_vertex_array();
        gl.bind_vertex_array(vao.as_ref());

        let pixels_vbo = gl.create_buffer().ok_or("cannot create pixels_vbo")?;
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&pixels_vbo));
        gl.buffer_data_with_u8_array(
            WebGl2RenderingContext::ARRAY_BUFFER,
            f32_to_u8(&CUBE_GEOMETRY),
            WebGl2RenderingContext::STATIC_DRAW,
        );

        let a_pos_position = gl.get_attrib_location(&shader, "aPos") as u32;
        gl.vertex_attrib_pointer_with_i32(a_pos_position, 3, WebGl2RenderingContext::FLOAT, false, 6 * size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(a_pos_position);

        let a_normal_position = gl.get_attrib_location(&shader, "aNormal") as u32;
        gl.vertex_attrib_pointer_with_i32(
            a_normal_position,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            6 * size_of::<f32>() as i32,
            3 * size_of::<f32>() as i32,
        );
        gl.enable_vertex_attrib_array(a_normal_position);

        let colors_vbo = gl.create_buffer().ok_or("cannot create colors_vbo")?;
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&colors_vbo));

        let a_color_position = gl.get_attrib_location(&shader, "aColor") as u32;
        gl.enable_vertex_attrib_array(a_color_position);
        gl.vertex_attrib_pointer_with_i32(a_color_position, 1, WebGl2RenderingContext::FLOAT, false, size_of::<f32>() as i32, 0);
        gl.vertex_attrib_divisor(a_color_position, 1);

        let offsets_vbo = gl.create_buffer().ok_or("cannot create offsets_vbo")?;
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&offsets_vbo));

        let a_offset_position = gl.get_attrib_location(&shader, "aOffset") as u32;
        gl.enable_vertex_attrib_array(a_offset_position);
        gl.vertex_attrib_pointer_with_i32(a_offset_position, 2, WebGl2RenderingContext::FLOAT, false, 2 * size_of::<f32>() as i32, 0);
        gl.vertex_attrib_divisor(a_offset_position, 1);

        let shadows = get_shadows()
            .iter()
            .map(|closure| Self::create_shadow_texture(gl, &**closure))
            .collect::<WebResult<Vec<Option<WebGlTexture>>>>()?;

        Ok(PixelsRender {
            video_buffers: video_materials.buffers,
            vao,
            shader,
            offsets_vbo,
            colors_vbo,
            width: 0,
            height: 0,
            offset_inverse_max_length: 0.0,
            shadows,
            gl: gl.clone(),
        })
    }

    fn create_shadow_texture(gl: &WebGl2RenderingContext, weight: &dyn Fn(usize, usize) -> f64) -> WebResult<Option<WebGlTexture>> {
        let mut texture: [u8; 4 * TEXTURE_SIZE * TEXTURE_SIZE] = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
        {
            for i in TEXTURE_SIZE / 2..TEXTURE_SIZE {
                for j in TEXTURE_SIZE / 2..TEXTURE_SIZE {
                    let mut value = weight(i, j);
                    if value > 255.0 {
                        value = 255.0;
                    }
                    let value = value as u8;
                    //let value = 255;
                    texture[(i * TEXTURE_SIZE + j) * 4 + 0] = 255;
                    texture[(i * TEXTURE_SIZE + j) * 4 + 1] = 255;
                    texture[(i * TEXTURE_SIZE + j) * 4 + 2] = 255;
                    texture[(i * TEXTURE_SIZE + j) * 4 + 3] = value;
                    texture[((i + 1) * TEXTURE_SIZE - j - 1) * 4 + 0] = 255;
                    texture[((i + 1) * TEXTURE_SIZE - j - 1) * 4 + 1] = 255;
                    texture[((i + 1) * TEXTURE_SIZE - j - 1) * 4 + 2] = 255;
                    texture[((i + 1) * TEXTURE_SIZE - j - 1) * 4 + 3] = value;
                    texture[((TEXTURE_SIZE - i - 1) * TEXTURE_SIZE + j) * 4 + 0] = 255;
                    texture[((TEXTURE_SIZE - i - 1) * TEXTURE_SIZE + j) * 4 + 1] = 255;
                    texture[((TEXTURE_SIZE - i - 1) * TEXTURE_SIZE + j) * 4 + 2] = 255;
                    texture[((TEXTURE_SIZE - i - 1) * TEXTURE_SIZE + j) * 4 + 3] = value;
                    texture[((TEXTURE_SIZE - i) * TEXTURE_SIZE - j - 1) * 4 + 0] = 255;
                    texture[((TEXTURE_SIZE - i) * TEXTURE_SIZE - j - 1) * 4 + 1] = 255;
                    texture[((TEXTURE_SIZE - i) * TEXTURE_SIZE - j - 1) * 4 + 2] = 255;
                    texture[((TEXTURE_SIZE - i) * TEXTURE_SIZE - j - 1) * 4 + 3] = value;
                }
            }
        }

        /*
        for i in 0 .. TEXTURE_SIZE {
            let mut line = "".to_string();
            for j in 0 .. TEXTURE_SIZE {
                let weight = texture[i * TEXTURE_SIZE * 4 + j * 4 + 3] as i32;
                line += &format!("{} ", (weight));
            }
            console!(log. line);
        }*/

        let pixel_shadow_texture = gl.create_texture();
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, pixel_shadow_texture.as_ref());
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            TEXTURE_SIZE as i32,
            TEXTURE_SIZE as i32,
            0,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            Some(&texture),
        )?;
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);

        Ok(pixel_shadow_texture)
    }

    pub fn load_image(&mut self, video_res: &VideoInputResources) {
        if video_res.image_size.width != self.width || video_res.image_size.height != self.height {
            self.width = video_res.image_size.width;
            self.height = video_res.image_size.height;
            self.offset_inverse_max_length = 1.0 / ((self.width as f32 * 0.5).powi(2) + (self.height as f32 * 0.5).powi(2)).sqrt();
            self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.offsets_vbo));
            let offsets = calculate_offsets(self.width, self.height);
            self.gl
                .buffer_data_with_u8_array(WebGl2RenderingContext::ARRAY_BUFFER, f32_to_u8(&offsets), WebGl2RenderingContext::STATIC_DRAW);
        }
        self.gl.bind_vertex_array(self.vao.as_ref());
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.colors_vbo));

        self.gl.buffer_data_with_u8_array(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &self.video_buffers[video_res.current_frame],
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    pub fn render(&self, uniforms: PixelsUniform) {
        self.gl.use_program(Some(&self.shader));
        if uniforms.shadow_kind >= self.shadows.len() {
            panic!("Bug on shadow_kind!")
        }
        self.gl
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, self.shadows[uniforms.shadow_kind].as_ref());
        self.gl
            .uniform_matrix4fv_with_f32_array(self.gl.get_uniform_location(&self.shader, "view").as_ref(), false, uniforms.view);
        self.gl
            .uniform_matrix4fv_with_f32_array(self.gl.get_uniform_location(&self.shader, "projection").as_ref(), false, uniforms.projection);
        self.gl
            .uniform3fv_with_f32_array(self.gl.get_uniform_location(&self.shader, "lightPos").as_ref(), uniforms.light_pos);
        self.gl
            .uniform3fv_with_f32_array(self.gl.get_uniform_location(&self.shader, "lightColor").as_ref(), uniforms.light_color);
        self.gl
            .uniform3fv_with_f32_array(self.gl.get_uniform_location(&self.shader, "extraLight").as_ref(), uniforms.extra_light);
        self.gl.uniform1f(
            self.gl.get_uniform_location(&self.shader, "ambientStrength").as_ref(),
            uniforms.ambient_strength,
        );
        self.gl
            .uniform1f(self.gl.get_uniform_location(&self.shader, "contrastFactor").as_ref(), uniforms.contrast_factor);
        self.gl.uniform1f(
            self.gl.get_uniform_location(&self.shader, "offset_inverse_max_length").as_ref(),
            self.offset_inverse_max_length,
        );
        self.gl.uniform1f(
            self.gl.get_uniform_location(&self.shader, "screen_curvature").as_ref(),
            uniforms.screen_curvature,
        );
        self.gl
            .uniform2fv_with_f32_array(self.gl.get_uniform_location(&self.shader, "pixel_spread").as_ref(), uniforms.pixel_spread);
        self.gl
            .uniform3fv_with_f32_array(self.gl.get_uniform_location(&self.shader, "pixel_scale").as_ref(), uniforms.pixel_scale);
        self.gl
            .uniform3fv_with_f32_array(self.gl.get_uniform_location(&self.shader, "pixel_offset").as_ref(), uniforms.pixel_offset);
        self.gl
            .uniform1f(self.gl.get_uniform_location(&self.shader, "pixel_pulse").as_ref(), uniforms.pixel_pulse);
        self.gl.uniform1f(
            self.gl.get_uniform_location(&self.shader, "heightModifierFactor").as_ref(),
            uniforms.height_modifier_factor,
        );

        self.gl.bind_vertex_array(self.vao.as_ref());
        self.gl.draw_arrays_instanced(
            WebGl2RenderingContext::TRIANGLES,
            0,
            match uniforms.geometry_kind {
                PixelsGeometryKind::Squares => 6,
                PixelsGeometryKind::Cubes => 36,
            },
            (self.width * self.height) as i32,
        );
    }
}

fn calculate_offsets(width: u32, height: u32) -> Vec<f32> {
    let pixels_total = width * height;
    let mut offsets: Vec<f32> = vec![0.0; pixels_total as usize * 2];
    {
        let half_width: f32 = width as f32 / 2.0;
        let half_height: f32 = height as f32 / 2.0;
        let center_dx = if width % 2 == 0 { 0.5 } else { 0.0 };
        let center_dy = if height % 2 == 0 { 0.5 } else { 0.0 };
        for i in 0..width {
            for j in 0..height {
                let index = (pixels_total - width - j * width + i) as usize;
                let x = i as f32 - half_width + center_dx;
                let y = j as f32 - half_height + center_dy;
                offsets[index * 2 + 0] = x;
                offsets[index * 2 + 1] = y;
            }
        }
    }
    offsets
}

#[rustfmt::skip]
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

pub const PIXEL_VERTEX_SHADER: &str = r#"#version 300 es
precision highp float;

in vec3 aPos;
in vec3 aNormal;
in float aColor;
in vec2 aOffset;

out vec3 FragPos;
out vec3 Normal;
out vec4 ObjectColor;
out vec2 ImagePos;

uniform mat4 view;
uniform mat4 projection;

uniform float offset_inverse_max_length;
uniform float screen_curvature;
uniform vec2 pixel_spread;
uniform vec3 pixel_scale;
uniform float pixel_pulse;
uniform vec3 pixel_offset;
uniform float heightModifierFactor;

const float COLOR_FACTOR = 1.0/255.0;
const uint hex_FF = uint(0xFF);

void main()
{
    uint color = floatBitsToUint(aColor);
    float r = float((color >>  0) & hex_FF);
    float g = float((color >>  8) & hex_FF);
    float b = float((color >> 16) & hex_FF);
    float a = float((color >> 24) & hex_FF);

    vec4 vecColor = vec4(r * COLOR_FACTOR, g * COLOR_FACTOR, b * COLOR_FACTOR, a * COLOR_FACTOR);

    float height_mod = 0.5 * ((vecColor.r + vecColor.g + vecColor.b) / 4.0 + 0.25) + 0.5 * (max(max(vecColor.r, vecColor.g), vecColor.b) / 1.33 + 0.25);

    ObjectColor = (1.0 - heightModifierFactor) * vecColor + heightModifierFactor * (vecColor * 0.5 +  0.5 * (vecColor / height_mod));

    vec3 modPos = (1.0 - heightModifierFactor) * aPos + heightModifierFactor * vec3(aPos.x, aPos.y * height_mod, aPos.z);

    vec3 pos = modPos / pixel_scale + vec3(aOffset * pixel_spread, 0);

    if (pixel_pulse > 0.0) {
        float radius = length(aOffset);
        pos += vec3(0, 0, sin(pixel_pulse + sin(pixel_pulse * 0.1) * radius * 0.25) * 2.0);
    }
    if (screen_curvature > 0.0) {
        float radius = length(aOffset);
        float normalized = radius * offset_inverse_max_length;
        pos.z -= sin(normalized) * screen_curvature * 100.0;
    }
    if (pixel_offset.x != 0.0 || pixel_offset.y != 0.0 || pixel_offset.z != 0.0) {
        pos += pixel_offset;
    }

    FragPos = pos;
    Normal = aNormal;
    
    gl_Position = projection * view * vec4(FragPos, 1.0);

    ImagePos = aPos.xy + 0.5;
}
"#;

pub const PIXEL_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;

out vec4 FragColor;

in vec3 Normal;  
in vec3 FragPos;
in vec4 ObjectColor;
in vec2 ImagePos;

uniform vec3 lightColor;
uniform vec3 extraLight;
uniform vec3 lightPos;
uniform float ambientStrength;
uniform float contrastFactor;

uniform sampler2D image;

void main()
{
    if (ObjectColor.a == 0.0) {
        discard;
    }

    vec4 result;
    if (ambientStrength == 1.0) {
        result = ObjectColor * vec4(lightColor, 1.0) * texture(image, ImagePos);
    } else {
        vec3 norm = normalize(Normal);
        vec3 lightDir = normalize(lightPos - FragPos);
        
        vec3 ambient = ambientStrength * lightColor;

        float diff = max(dot(norm, lightDir), 0.0);
        vec3 diffuse = diff * lightColor;
        
        result = ObjectColor * vec4(ambient + diffuse * (1.0 - ambientStrength), 1.0) * texture(image, ImagePos);
    }
    float contrastUmbral = 0.5;
    result.r = (result.r - contrastUmbral) * contrastFactor + contrastFactor * contrastUmbral;
    result.g = (result.g - contrastUmbral) * contrastFactor + contrastFactor * contrastUmbral;
    result.b = (result.b - contrastUmbral) * contrastFactor + contrastFactor * contrastUmbral;
    FragColor = result + vec4(extraLight, 0.0);
} 
"#;
