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

#[wasm_bindgen]
pub fn main() {
    match program() {
        Err(e) => {
            web_sys::console::error_2(&"An unexpected error ocurred.".into(), &e);
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

pub fn program() -> Result<(), JsValue> {
    let gl = make_gl_context()?;
    let render_loop = Rc::new(RefCell::new(RenderLoop {
        animation_frame_id: None,
        animation_frame_closure : None,
        resources: load_resources(&gl)?
    }));
    let closure: Closure<FnMut()> = {
        let render_loop = render_loop.clone();
        Closure::wrap(Box::new(move || {
            let mut render_loop = render_loop.borrow_mut();
            let input = Input{ now: now() };
            if update(&gl, &mut render_loop.resources, &input) {
                render_loop.animation_frame_id = if let Some(ref closure) = render_loop.animation_frame_closure {
                    Some(web_sys::window().unwrap().request_animation_frame(closure.as_ref().unchecked_ref()).expect("cannot set animation frame"))
                } else {
                    None
                }
            }
        }))
    };
    let mut render_loop = render_loop.borrow_mut();
    render_loop.animation_frame_id = Some(web_sys::window().unwrap().request_animation_frame(closure.as_ref().unchecked_ref()).expect("cannot set animation frame"));
    render_loop.animation_frame_closure = Some(closure);
    Ok(())
}

pub fn now() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now()
}

pub fn update(gl: &WebGl2RenderingContext, res: &mut Resources, input: &Input) -> bool {
    let dt = (input.now - res.last_time) / 1000.0;
    let ellapsed = (input.now - res.last_second);
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
    gl.draw_arrays(
        WebGl2RenderingContext::TRIANGLES,
        0,
        res.vertices_len,
    );
    gl.use_program(Some(&res.pixel_shader));
    true
}

pub fn make_gl_context() -> Result<WebGl2RenderingContext, JsValue> {
    let window =  web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("1-canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let screen = window.screen().unwrap();
    let width: i32 = screen.width().unwrap();
    let height: i32 = screen.height().unwrap();
    let dpi: f64 = window.device_pixel_ratio();
    
    canvas.set_width((width as f64 * dpi).round() as u32);
    canvas.set_height((height as f64 * dpi).round() as u32);

    let style = (canvas.as_ref() as &web_sys::HtmlElement).style();
    style.set_property("width", &(width.to_string() + "px")).unwrap();
    style.set_property("height", &(height.to_string() + "px")).unwrap();

    let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
    Ok(gl)
}

pub fn load_resources(gl: &WebGl2RenderingContext) -> Result<Resources, JsValue> {
    let vert_shader = compile_shader(
        &gl,
        WebGl2RenderingContext::VERTEX_SHADER,
        r#"
        attribute vec4 position;
        void main() {
            gl_Position = position;
        }
    "#,
    ).unwrap();
    let frag_shader = compile_shader(
        &gl,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r#"
        void main() {
            gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#,
    ).unwrap();
    let program = link_shader(&gl, [vert_shader, frag_shader].iter()).unwrap();
    let vertices = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
    let vert_array = js_sys::Float32Array::new(&wasm_bindgen::JsValue::from(vertices.len() as u32));
    for (i, f) in vertices.iter().enumerate() {
        vert_array.fill(*f, i as u32, (i + 1) as u32);
    }

    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_opt_array_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        Some(&vert_array.buffer()),
        WebGl2RenderingContext::STATIC_DRAW,
    );
    gl.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    let now = now();

    Ok(Resources {
        pixel_shader: program,
        vertices_len: (vertices.len() / 3) as i32,
        frame_count: 0,
        last_time: now,
        last_second: now
    })
}

pub fn compile_shader(
    gl: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".into()))
    }
}

pub fn link_shader<'a, T: IntoIterator<Item = &'a WebGlShader>>(
    gl: &WebGl2RenderingContext,
    shaders: T,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;
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
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program object".into()))
    }
}

