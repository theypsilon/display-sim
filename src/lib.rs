extern crate cfg_if;
extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;

mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader};
use std::rc::Rc;
use std::cell::RefCell;

pub enum WasmError {
    Js(wasm_bindgen::JsValue),
    Str(String)
}

impl From<std::string::String> for WasmError {
    fn from(string: std::string::String) -> Self {
        WasmError::Str(string)
    }
}

impl<'a> From<&'a str> for WasmError {
    fn from(string: &'a str) -> Self {
        WasmError::Str(string.into())
    }
}

impl From<wasm_bindgen::JsValue> for WasmError {
    fn from(o: wasm_bindgen::JsValue) -> Self {
        WasmError::Js(o)
    }
}

type Result<T> = std::result::Result<T, WasmError>;

#[wasm_bindgen]
pub fn main(gl: JsValue) {
    match program(gl) {
        Err(e) => match e {
            WasmError::Js(o) => web_sys::console::error_2(&"An unexpected error ocurred.".into(), &o),
            WasmError::Str(s) => web_sys::console::error_2(&"An unexpected error ocurred.".into(), &s.into()),
        },
        Ok(_) => {}
    };
}

pub struct Resources {
    pixel_shader: WebGlProgram,
    vertices_len: i32,
    frame_count: u32,
    last_time: f64,
    last_second: f64
}

#[derive(Clone)]
pub struct Input {
    now: f64
}

struct RenderLoop {
    animation_frame_id: Option<i32>,
    pub animation_frame_closure: Option<Closure<FnMut()>>,
    resources: Resources,
}

pub fn program(gl: JsValue) -> Result<()> {
    let gl = gl.dyn_into::<WebGl2RenderingContext>()?;
    let render_loop = Rc::new(RefCell::new(RenderLoop {
        animation_frame_id: None,
        animation_frame_closure : None,
        resources: load_resources(&gl)?
    }));
    let closure: Closure<FnMut()> = {
        let render_loop = render_loop.clone();
        let window = window()?;
        Closure::wrap(Box::new(move || {
            let mut render_loop = render_loop.borrow_mut();
            let input = Input{ now: now().unwrap_or(render_loop.resources.last_time) };

            if update(&gl, &mut render_loop.resources, &input).unwrap_or(false) == false {
                return;
            }
            
            let mut frame_id = None;
            if let Some(ref closure) = render_loop.animation_frame_closure {
                if let Ok(id) = window.request_animation_frame(closure.as_ref().unchecked_ref()) {
                    frame_id = Some(id);
                }
            }
            render_loop.animation_frame_id = frame_id
        }))
    };
    let mut render_loop = render_loop.borrow_mut();
    render_loop.animation_frame_id = Some(window()?.request_animation_frame(closure.as_ref().unchecked_ref())?);
    render_loop.animation_frame_closure = Some(closure);
    Ok(())
}

pub fn window() -> Result<web_sys::Window> {
    Ok(web_sys::window().ok_or("cannot access window")?)
}

pub fn now() -> Result<f64> {
    Ok(window()?.performance().ok_or("cannot access performance")?.now())
}

pub fn update(gl: &WebGl2RenderingContext, res: &mut Resources, input: &Input) -> Result<bool> {
    let dt = (input.now - res.last_time) / 1000.0;
    let ellapsed = input.now - res.last_second;
    res.last_time = input.now;

    if ellapsed >= 10_000.0 {
        let fps = res.frame_count as f32 * 0.1;
        web_sys::console::log_2(&fps.into(), &"FPS".into());
        res.last_second = input.now;
        res.frame_count = 0;
    } else {
        res.frame_count += 1;
    }

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    gl.use_program(Some(&res.pixel_shader));
    gl.draw_arrays(
        WebGl2RenderingContext::TRIANGLES,
        0,
        res.vertices_len,
    );
    Ok(true)
}

const pixel_vertex_shader: &str = r#"
    attribute vec4 position;
    void main() {
        gl_Position = position;
    }
"#;

const pixel_fragment_shader: &str = r#"
    void main() {
        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    }
"#;

pub fn make_shader(gl: &WebGl2RenderingContext, vertex_shader: &str, fragment_shader: &str) -> Result<WebGlProgram> {
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

pub fn js_f32_array(data: &[f32]) -> js_sys::Float32Array {
    let array = js_sys::Float32Array::new(&wasm_bindgen::JsValue::from(data.len() as u32));
    for (i, f) in data.iter().enumerate() {
        array.fill(*f, i as u32, (i + 1) as u32);
    }
    array
}

pub fn load_resources(gl: &WebGl2RenderingContext) -> Result<Resources> {
    let program = make_shader(&gl, pixel_vertex_shader, pixel_fragment_shader)?;
    let vertices = js_f32_array(&[-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0]);

    let buffer = gl.create_buffer().ok_or("cannot create buffer")?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_opt_array_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        Some(&vertices.buffer()),
        WebGl2RenderingContext::STATIC_DRAW,
    );
    gl.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    let now = now()?;

    Ok(Resources {
        pixel_shader: program,
        vertices_len: (9 / 3) as i32,
        frame_count: 0,
        last_time: now,
        last_second: now
    })
}

pub fn compile_shader(gl: &WebGl2RenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or("Unable to create shader object")?;
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

pub fn link_shader<'a, T: IntoIterator<Item = &'a WebGlShader>>(gl: &WebGl2RenderingContext, shaders: T) -> Result<WebGlProgram> {
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
