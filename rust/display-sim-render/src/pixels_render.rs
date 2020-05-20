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

use crate::error::AppResult;
use crate::shaders::make_shader;
use crate::simulation_render_state::VideoInputMaterials;
use core::general_types::f32_to_u8;
use core::simulation_core_state::VideoInputResources;
use core::ui_controller::pixel_geometry_kind::PixelGeometryKindOptions;
use core::ui_controller::pixel_shadow_shape_kind::{get_shadows, TEXTURE_SIZE};

use glow::GlowSafeAdapter;
use glow::HasContext;
use std::mem::size_of;
use std::rc::Rc;

pub struct PixelsRender<GL: HasContext> {
    shader: GL::Program,
    vao: Option<GL::VertexArray>,
    colors_vbo: GL::Buffer,
    offsets_vbo: GL::Buffer,
    width: u32,
    height: u32,
    offset_inverse_max_length: f32,
    shadows: Vec<Option<GL::Texture>>,
    video_buffers: Vec<Box<[u8]>>,
    gl: Rc<GlowSafeAdapter<GL>>,
}

pub struct PixelsUniform<'a> {
    pub shadow_kind: usize,
    pub geometry_kind: PixelGeometryKindOptions,
    pub view: &'a [f32; 16],
    pub projection: &'a [f32; 16],
    pub light_pos: &'a [f32; 3],
    pub light_color: &'a [f32; 3],
    pub extra_light: &'a [f32; 3],
    pub ambient_strength: f32,
    pub contrast_factor: f32,
    pub screen_curvature: f32,
    pub pixel_spread: &'a [f32; 2],
    pub pixel_scale: &'a [f32; 3],
    pub pixel_offset: &'a [f32; 3],

    pub rgb_red: &'a [f32; 3],
    pub rgb_green: &'a [f32; 3],
    pub rgb_blue: &'a [f32; 3],
    pub color_gamma: f32,
    pub time: f32,
    pub color_noise: f32,

    pub pixel_pulse: f32,
    pub height_modifier_factor: f32,
}

impl<GL: HasContext> PixelsRender<GL> {
    pub fn new(gl: Rc<GlowSafeAdapter<GL>>, video_materials: VideoInputMaterials) -> AppResult<PixelsRender<GL>> {
        let shader = make_shader(&*gl, PIXEL_VERTEX_SHADER, PIXEL_FRAGMENT_SHADER)?;

        let vao = Some(gl.create_vertex_array()?);
        gl.bind_vertex_array(vao);

        let pixels_vbo = gl.create_buffer()?;
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(pixels_vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, f32_to_u8(&CUBE_GEOMETRY), glow::STATIC_DRAW);

        let a_pos_position = gl.get_attrib_location(shader, "aPos");
        gl.vertex_attrib_pointer_f32(a_pos_position, 3, glow::FLOAT, false, 6 * size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(a_pos_position);

        let a_normal_position = gl.get_attrib_location(shader, "aNormal");
        gl.vertex_attrib_pointer_f32(
            a_normal_position,
            3,
            glow::FLOAT,
            false,
            6 * size_of::<f32>() as i32,
            3 * size_of::<f32>() as i32,
        );
        gl.enable_vertex_attrib_array(a_normal_position);

        let colors_vbo = gl.create_buffer()?;
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(colors_vbo));

        let a_color_position = gl.get_attrib_location(shader, "aColor");
        gl.enable_vertex_attrib_array(a_color_position);
        gl.vertex_attrib_pointer_f32(a_color_position, 1, glow::FLOAT, false, size_of::<f32>() as i32, 0);
        gl.vertex_attrib_divisor(a_color_position, 1);

        let offsets_vbo = gl.create_buffer()?;
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(offsets_vbo));

        let a_offset_position = gl.get_attrib_location(shader, "aOffset");
        gl.enable_vertex_attrib_array(a_offset_position);
        gl.vertex_attrib_pointer_f32(a_offset_position, 2, glow::FLOAT, false, 2 * size_of::<f32>() as i32, 0);
        gl.vertex_attrib_divisor(a_offset_position, 1);

        let shadows = get_shadows()
            .iter()
            .map(|closure| Self::create_shadow_texture(&*gl, &**closure))
            .collect::<AppResult<Vec<Option<GL::Texture>>>>()?;

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
            gl,
        })
    }

    fn create_shadow_texture(gl: &GlowSafeAdapter<GL>, weight: &dyn Fn(usize, usize) -> f64) -> AppResult<Option<GL::Texture>> {
        let mut texture: Vec<u8> = vec![0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
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
        let pixel_shadow_texture = Some(gl.create_texture()?);
        gl.bind_texture(glow::TEXTURE_2D, pixel_shadow_texture);
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGBA as i32,
            TEXTURE_SIZE as i32,
            TEXTURE_SIZE as i32,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            Some(&texture),
        );
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
        gl.bind_texture(glow::TEXTURE_2D, None);

        Ok(pixel_shadow_texture)
    }

    pub fn load_image(&mut self, video_res: &VideoInputResources) {
        if video_res.image_size.width != self.width || video_res.image_size.height != self.height {
            self.width = video_res.image_size.width;
            self.height = video_res.image_size.height;
            self.offset_inverse_max_length = 1.0 / ((self.width as f32 * 0.5).powi(2) + (self.height as f32 * 0.5).powi(2)).sqrt();
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.offsets_vbo));
            let offsets = calculate_offsets(self.width, self.height);
            self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, f32_to_u8(&offsets), glow::STATIC_DRAW);
        }
        self.gl.bind_vertex_array(self.vao);
        self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.colors_vbo));

        self.gl
            .buffer_data_u8_slice(glow::ARRAY_BUFFER, &self.video_buffers[video_res.current_frame], glow::STATIC_DRAW);
    }

    pub fn render(&self, uniforms: PixelsUniform) {
        let gl = &self.gl;
        let shader = self.shader;

        gl.use_program(Some(shader));
        if uniforms.shadow_kind >= self.shadows.len() {
            panic!("Bug on shadow_kind!")
        }
        gl.bind_texture(glow::TEXTURE_2D, self.shadows[uniforms.shadow_kind]);
        gl.uniform_matrix_4_f32_slice(gl.get_uniform_location(shader, "view"), false, uniforms.view);
        gl.uniform_matrix_4_f32_slice(gl.get_uniform_location(shader, "projection"), false, uniforms.projection);
        gl.uniform_3_f32_slice(gl.get_uniform_location(shader, "lightPos"), uniforms.light_pos);
        gl.uniform_3_f32_slice(gl.get_uniform_location(shader, "lightColor"), uniforms.light_color);
        gl.uniform_3_f32_slice(gl.get_uniform_location(shader, "extraLight"), uniforms.extra_light);
        gl.uniform_1_f32(gl.get_uniform_location(shader, "ambientStrength"), uniforms.ambient_strength);
        gl.uniform_1_f32(gl.get_uniform_location(shader, "contrastFactor"), uniforms.contrast_factor);
        gl.uniform_1_f32(gl.get_uniform_location(shader, "offset_inverse_max_length"), self.offset_inverse_max_length);
        gl.uniform_1_f32(gl.get_uniform_location(shader, "screen_curvature"), uniforms.screen_curvature);
        gl.uniform_2_f32_slice(gl.get_uniform_location(shader, "pixel_spread"), uniforms.pixel_spread);
        gl.uniform_3_f32_slice(gl.get_uniform_location(shader, "pixel_scale"), uniforms.pixel_scale);
        gl.uniform_3_f32_slice(gl.get_uniform_location(shader, "pixel_offset"), uniforms.pixel_offset);
        gl.uniform_1_f32(gl.get_uniform_location(shader, "pixel_pulse"), uniforms.pixel_pulse);
        gl.uniform_1_f32(gl.get_uniform_location(shader, "heightModifierFactor"), uniforms.height_modifier_factor);

        gl.uniform_3_f32_slice(gl.get_uniform_location(shader, "red"), uniforms.rgb_red);
        gl.uniform_3_f32_slice(gl.get_uniform_location(shader, "green"), uniforms.rgb_green);
        gl.uniform_3_f32_slice(gl.get_uniform_location(shader, "blue"), uniforms.rgb_blue);
        gl.uniform_1_f32(gl.get_uniform_location(shader, "gamma"), uniforms.color_gamma);
        gl.uniform_1_f32(gl.get_uniform_location(shader, "time"), uniforms.time);
        gl.uniform_1_f32(gl.get_uniform_location(shader, "color_noise"), uniforms.color_noise);

        gl.bind_vertex_array(self.vao);
        gl.draw_arrays_instanced(
            glow::TRIANGLES,
            0,
            match uniforms.geometry_kind {
                PixelGeometryKindOptions::Squares => 6,
                PixelGeometryKindOptions::Cubes => 36,
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

uniform vec3 red;
uniform vec3 green;
uniform vec3 blue;

uniform float gamma;

uniform vec3 lightColor;
uniform vec3 extraLight;
uniform vec3 lightPos;
uniform float ambientStrength;
uniform float contrastFactor;

uniform sampler2D image;
uniform float time;
uniform float color_noise;

uint hash( uint x ) {
    x += ( x << 10u );
    x ^= ( x >>  6u );
    x += ( x <<  3u );
    x ^= ( x >> 11u );
    x += ( x << 15u );
    return x;
}

uint hash( uvec3 v ) { return hash( v.x ^ hash(v.y) ^ hash(v.z)             ); }

float floatConstruct( uint m ) {
    const uint ieeeMantissa = 0x007FFFFFu; // binary32 mantissa bitmask
    const uint ieeeOne      = 0x3F800000u; // 1.0 in IEEE binary32

    m &= ieeeMantissa;                     // Keep only mantissa bits (fractional part)
    m |= ieeeOne;                          // Add fractional part to 1.0

    float  f = uintBitsToFloat( m );       // Range [1:2]
    return f - 1.0;                        // Range [0:1]
}

float random( vec3  v ) { return floatConstruct(hash(floatBitsToUint(v))); }

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
    result.r = (result.r - contrastUmbral) * contrastFactor + contrastFactor * contrastUmbral - color_noise/2.0 + color_noise * random(vec3(ImagePos, time * 0.5));
    result.g = (result.g - contrastUmbral) * contrastFactor + contrastFactor * contrastUmbral - color_noise/2.0 + color_noise * random(vec3(ImagePos, time));
    result.b = (result.b - contrastUmbral) * contrastFactor + contrastFactor * contrastUmbral - color_noise/2.0 + color_noise * random(vec3(ImagePos, time * 2.0));
    result = result.r * vec4(red, result.a) + result.g * vec4(green, result.a) + result.b * vec4(blue, result.a) + vec4(extraLight, 0.0);
    FragColor = vec4(pow(result.r, gamma), pow(result.g, gamma), pow(result.b, gamma), result.a);
} 
"#;
