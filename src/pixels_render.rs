use js_sys::{Float32Array, ArrayBuffer};
use std::mem::size_of;
use web_sys::{
    WebGl2RenderingContext, WebGlVertexArrayObject, WebGlProgram, WebGlBuffer,
};

use wasm_error::WasmResult;
use shaders::{
    make_shader,
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
    offset_vbo: WebGlBuffer,
    width: usize,
    height: usize,
}

pub struct PixelsUniform<'a> {
    pub view: &'a mut [f32],
    pub projection: &'a mut [f32],
    pub light_pos: &'a mut [f32],
    pub light_color: &'a mut [f32],
    pub extra_light: &'a mut [f32],
    pub ambient_strength: f32,
    pub contrast_factor: f32,
    pub pixel_gap: &'a mut [f32],
    pub pixel_scale: &'a mut [f32],
    pub pixel_offset: &'a mut [f32],
    pub pixel_pulse: f32,
}

impl PixelsRender {
    pub fn new(gl: &WebGl2RenderingContext, width: usize, height: usize) -> WasmResult<PixelsRender> {

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

        let offsets = calculate_offsets(width, height);

        let a_offset_position = gl.get_attrib_location(&shader, "aOffset") as u32;
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&offset_vbo));
        gl.buffer_data_with_opt_array_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&offsets.buffer()),
            WebGl2RenderingContext::STATIC_DRAW,
        );
        gl.enable_vertex_attrib_array(a_offset_position);
        gl.vertex_attrib_pointer_with_i32(a_offset_position, 2, WebGl2RenderingContext::FLOAT, false, 2 * size_of::<f32>() as i32, 0);
        gl.vertex_attrib_divisor(a_offset_position, 1);
        
        Ok(PixelsRender {vao, shader, offset_vbo, colors_vbo, width, height})
    }

    pub fn apply_colors(&self, gl: &WebGl2RenderingContext, buffer: &ArrayBuffer) {
        gl.bind_vertex_array(self.vao.as_ref());
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
        gl.uniform1f(gl.get_uniform_location(&self.shader, "contrastFactor").as_ref(), uniforms.contrast_factor);
        gl.uniform2fv_with_f32_array(gl.get_uniform_location(&self.shader, "pixel_gap").as_ref(), uniforms.pixel_gap);
        gl.uniform3fv_with_f32_array(gl.get_uniform_location(&self.shader, "pixel_scale").as_ref(), uniforms.pixel_scale);
        gl.uniform2fv_with_f32_array(gl.get_uniform_location(&self.shader, "pixel_offset").as_ref(), uniforms.pixel_offset);
        gl.uniform1f(gl.get_uniform_location(&self.shader, "pixel_pulse").as_ref(), uniforms.pixel_pulse);

        gl.bind_vertex_array(self.vao.as_ref());
        gl.draw_arrays_instanced(
            WebGl2RenderingContext::TRIANGLES,
            0,
            match pixels_render_kind { PixelsRenderKind::Squares => 6, PixelsRenderKind::Cubes => 36 },
            (self.width * self.height) as i32
        );
    }
}

fn calculate_offsets(width: usize, height: usize) -> Float32Array {
    let pixels_total = width * height;
    let offsets = Float32Array::new(&wasm_bindgen::JsValue::from(pixels_total as u32 * 2));
    {
        let half_width: f32 = width as f32 / 2.0;
        let half_height: f32 = height as f32 / 2.0;
        let center_dx = if width % 2 == 0 {0.5} else {0.0};
        let center_dy = if height % 2 == 0 {0.5} else {0.0};
        for i in 0..width {
            for j in 0..height {
                let index = (pixels_total - width - j * width + i) as u32;
                let x = i as f32 - half_width + center_dx;
                let y = j as f32 - half_height + center_dy;
                offsets.fill(x, index * 2 + 0, index * 2 + 1);
                offsets.fill(y, index * 2 + 1, index * 2 + 2);
            }
        }
    }
    offsets
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

pub const PIXEL_VERTEX_SHADER: &str = r#"#version 300 es
precision highp float;

in vec3 aPos;
in vec3 aNormal;
in float aColor;
in vec2 aOffset;

out vec3 FragPos;
out vec3 Normal;
out vec4 ObjectColor;

uniform mat4 view;
uniform mat4 projection;

uniform vec2 pixel_gap;
uniform vec3 pixel_scale;
uniform float pixel_pulse;
uniform vec2 pixel_offset;

const float COLOR_FACTOR = 1.0/255.0;
const uint hex_FF = uint(0xFF);

void main()
{
    vec3 pos = aPos / pixel_scale + vec3(aOffset * pixel_gap, 0);
    if (pixel_pulse > 0.0) {
        float radius = length(aOffset);
        pos += vec3(0, 0, sin(pixel_pulse + sin(pixel_pulse / 10.0) * radius / 4.0) * 2.0);
    }
    if (pixel_offset.x != 0.0 || pixel_offset.y != 0.0) {
        pos += vec3(pixel_offset, 0.0);
    }
    FragPos = pos;
    Normal = aNormal;

    uint color = floatBitsToUint(aColor);
    float r = float((color >>  0) & hex_FF);
    float g = float((color >>  8) & hex_FF);
    float b = float((color >> 16) & hex_FF);
    float a = float((color >> 24) & hex_FF);

    ObjectColor = vec4(r * COLOR_FACTOR, g * COLOR_FACTOR, b * COLOR_FACTOR, a * COLOR_FACTOR);
    
    gl_Position = projection * view * vec4(FragPos, 1.0);
}
"#;

pub const PIXEL_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;

out vec4 FragColor;

in vec3 Normal;  
in vec3 FragPos;
in vec4 ObjectColor;

uniform vec3 lightColor;
uniform vec3 extraLight;
uniform vec3 lightPos;
uniform float ambientStrength;
uniform float contrastFactor;

void main()
{
    if (ObjectColor.a == 0.0) {
        discard;
    }

    vec4 result;
    if (ambientStrength == 1.0) {
        result = ObjectColor * vec4(lightColor, 1.0);
        float contrastUmbral = 0.5;
        result.r = (result.r - contrastUmbral) * contrastFactor + contrastFactor * contrastUmbral;
        result.g = (result.g - contrastUmbral) * contrastFactor + contrastFactor * contrastUmbral;
        result.b = (result.b - contrastUmbral) * contrastFactor + contrastFactor * contrastUmbral;
    } else {
        vec3 norm = normalize(Normal);
        vec3 lightDir = normalize(lightPos - FragPos);
        
        vec3 ambient = ambientStrength * lightColor;

        float diff = max(dot(norm, lightDir), 0.0);
        vec3 diffuse = diff * lightColor;
        
        result = ObjectColor * vec4(ambient + diffuse * (1.0 - ambientStrength), 1.0);
    }
    FragColor = result + vec4(extraLight, 0.0);
} 
"#;
