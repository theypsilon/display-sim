use web_sys::{
    WebGlProgram, WebGl2RenderingContext, 
    WebGlShader,
};

use wasm_error::{WasmError, WasmResult};

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

const float COLOR_FACTOR = 1.0/255.0;
const uint hex_FF = uint(0xFF);

void main()
{
    float radius = length(aOffset);
    FragPos = aPos / pixel_scale + vec3(aOffset * pixel_gap, 0) + vec3(0, 0, sin(pixel_pulse + sin(pixel_pulse / 10.0) * radius / 4.0) * 2.0);
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

void main()
{
    if (ObjectColor.a == 0.0) {
        discard;
    }

    vec4 result;
    if (ambientStrength == 1.0) {
        result = ObjectColor * vec4(lightColor, 1.0);
    } else {
        vec3 norm = normalize(Normal);
        vec3 lightDir = normalize(lightPos - FragPos);
        
        vec3 ambient = ambientStrength * lightColor;

        float diff = max(dot(norm, lightDir), 0.0);
        vec3 diffuse = diff * lightColor;
        
        result = ObjectColor * vec4(ambient + diffuse * (1.0 - ambientStrength), 1.0);
    }
    FragColor = result + vec4(extraLight, 1.0);
} 
"#;

pub const BLOOM_VERTEX_SHADER: &str = r#"#version 300 es
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

pub const BLOOM_FRAGMENT_SHADER: &str = r#"#version 300 es
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

pub fn make_shader(gl: &WebGl2RenderingContext, vertex_shader: &str, fragment_shader: &str) -> WasmResult<WebGlProgram> {
    let vert_shader = compile_shader(
        &gl,
        WebGl2RenderingContext::VERTEX_SHADER,
        vertex_shader,
    )?;
    let frag_shader = compile_shader(
        &gl,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        fragment_shader,
    )?;
    link_shader(&gl, [vert_shader, frag_shader].iter())
}

fn compile_shader(gl: &WebGl2RenderingContext, shader_type: u32, source: &str) -> WasmResult<WebGlShader> {
    let shader = gl.create_shader(shader_type).ok_or("Unable to create shader object")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(WasmError::Str(gl
            .get_shader_info_log(&shader)
            .ok_or("Unknown error creating shader")?)
        )
    }
}

fn link_shader<'a, T: IntoIterator<Item = &'a WebGlShader>>(gl: &WebGl2RenderingContext, shaders: T) -> WasmResult<WebGlProgram> {
    let program = gl.create_program().ok_or("Unable to create shader object")?;
    for shader in shaders {
        gl.attach_shader(&program, shader)
    }
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(WasmError::Str(gl.get_program_info_log(&program).ok_or("cannot get program info log")?))
    }
}