extern crate cfg_if;
extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;
extern crate nalgebra_glm as glm;

mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader, WebGlVertexArrayObject, KeyboardEvent, MouseEvent, WheelEvent, Event, EventTarget};
use std::rc::Rc;
use std::cell::RefCell;
use std::mem::size_of;

pub enum Wasm_Error {
    Js(wasm_bindgen::JsValue),
    Str(String)
}

impl From<std::string::String> for Wasm_Error {
    fn from(string: std::string::String) -> Self {
        Wasm_Error::Str(string)
    }
}

impl<'a> From<&'a str> for Wasm_Error {
    fn from(string: &'a str) -> Self {
        Wasm_Error::Str(string.into())
    }
}

impl From<wasm_bindgen::JsValue> for Wasm_Error {
    fn from(o: wasm_bindgen::JsValue) -> Self {
        Wasm_Error::Js(o)
    }
}

type Result<T> = std::result::Result<T, Wasm_Error>;

#[wasm_bindgen]
pub fn main(gl: JsValue) {
    match program(gl) {
        Err(e) => match e {
            Wasm_Error::Js(o) => web_sys::console::error_2(&"An unexpected error ocurred.".into(), &o),
            Wasm_Error::Str(s) => web_sys::console::error_2(&"An unexpected error ocurred.".into(), &s.into()),
        },
        Ok(_) => {}
    };
}

struct Boolean_Button {
    activated: bool,
    just_pressed: bool,
    just_released: bool
}

impl Boolean_Button {
    fn new() -> Boolean_Button {
        Boolean_Button {
            activated: false,
            just_pressed: false,
            just_released: false
        }
    }

    fn track(&mut self, pushed: bool) {
        self.just_pressed = false;
        self.just_released = false;
        if pushed == false && self.activated {
            self.just_released = true;
        } else if pushed && self.activated == false {
            self.just_pressed = true;
        }
        self.activated = pushed;
    }
}

struct Buttons {
    speed_up: Boolean_Button,
    speed_down: Boolean_Button,
    mouse_click: Boolean_Button,
}

impl Buttons {
    fn new() -> Buttons {
        Buttons {
            speed_up: Boolean_Button::new(),
            speed_down: Boolean_Button::new(),
            mouse_click: Boolean_Button::new(),
        }
    }
}

pub struct Resources {
    pixel_shader: WebGlProgram,
    pixel_vao: Option<WebGlVertexArrayObject>,
    frame_count: u32,
    last_time: f64,
    last_second: f64,
    last_mouse_x: i32,
    last_mouse_y: i32,
    cur_pixel_scale_x: f32,
    cur_pixel_scale_y: f32,
    cur_pixel_gap: f32,
    pixel_manipulation_speed: f32,
    camera: Camera,
    camera_zoom: f32,
    buttons: Buttons,
}

#[derive(Clone)]
pub struct Input {
    now: f64,
    walk_left: bool,
    walk_right: bool,
    walk_up: bool,
    walk_down: bool,
    walk_forward: bool,
    walk_backward: bool,
    turn_left: bool,
    turn_right: bool,
    turn_up: bool,
    turn_down: bool,
    rotate_left: bool,
    rotate_right: bool,
    speed_up: bool,
    speed_down: bool,
    ctrl: bool,
    alt: bool,
    space: bool,
    mouse_left_click: bool,
    mouse_position_x: i32,
    mouse_position_y: i32,
    mouse_scroll_y: f32,
    increase_pixel_scale_y: bool,
    decrease_pixel_scale_y: bool,
    increase_pixel_scale_x: bool,
    decrease_pixel_scale_x: bool,
    increase_pixel_gap: bool,
    decrease_pixel_gap: bool,
}

impl Input {
    pub fn new() -> Result<Input> {
        Ok(Input {
            now: now()?,
            walk_left: false,
            walk_right: false,
            walk_up: false,
            walk_down: false,
            walk_forward: false,
            walk_backward: false,
            turn_left: false,
            turn_right: false,
            turn_up: false,
            turn_down: false,
            rotate_left: false,
            rotate_right: false,
            speed_up: false,
            speed_down: false,
            ctrl: false,
            alt: false,
            space: false,
            mouse_left_click: false,
            mouse_position_x: -1,
            mouse_position_y: -1,
            mouse_scroll_y: 0.0,
            increase_pixel_scale_y: false,
            decrease_pixel_scale_y: false,
            increase_pixel_scale_x: false,
            decrease_pixel_scale_x: false,
            increase_pixel_gap: false,
            decrease_pixel_gap: false,
        })
    }
}

struct Render_Loop {
    animation_frame_id: Option<i32>,
    pub animation_frame_closure: Option<Closure<FnMut()>>,
    pub keyboard_down_closure: Option<Closure<FnMut(JsValue)>>,
    pub keyboard_up_closure: Option<Closure<FnMut(JsValue)>>,
    pub mouse_position_closure: Option<Closure<FnMut(JsValue)>>,
    pub mouse_wheel_closure: Option<Closure<FnMut(JsValue)>>,
    resources: Resources,
}

enum Camera_Direction{Down, Up, Left, Right, Forward, Backward}

struct Camera {
    position: glm::Vec3,
    position_delta: glm::Vec3,
    direction: glm::Vec3,
    axis_up: glm::Vec3,
    axis_right: glm::Vec3,
    pitch: f32,
    heading: f32,
    rotate: f32,
    movement_speed: f32,
    turning_speed: f32
}

impl Camera {
    fn new() -> Camera {
        Camera {
            position: glm::vec3 (0.0, 0.0, 270.0),
            position_delta: glm::vec3 (0.0, 0.0, 0.0),
            direction: glm::vec3 (0.0, 0.0, -1.0),
            axis_up: glm::vec3 (0.0, 1.0, 0.0),
            axis_right: glm::vec3 (1.0, 0.0, 0.0),
            pitch: 0.0,
            heading: 0.0,
            rotate: 0.0,
            movement_speed: 10.0,
            turning_speed: 1.0
        }
    }

    fn advance(&mut self, direction: Camera_Direction, dt: f32) {
        let velocity = self.movement_speed * dt;
        self.position_delta += match direction {
            Camera_Direction::Up => self.axis_up * velocity,
            Camera_Direction::Down => - self.axis_up * velocity,
            Camera_Direction::Left => - self.axis_right * velocity,
            Camera_Direction::Right => self.axis_right * velocity,
            Camera_Direction::Forward => self.direction * velocity,
            Camera_Direction::Backward => - self.direction * velocity,
        };
    }

    fn turn(&mut self, direction: Camera_Direction, dt: f32) {
        let velocity = 20.0 * dt * 0.003 * self.turning_speed;
        match direction {
            Camera_Direction::Up => self.heading += velocity,
            Camera_Direction::Down => self.heading -= velocity,
            Camera_Direction::Left => self.pitch += velocity,
            Camera_Direction::Right => self.pitch -= velocity,
            _ => unreachable!()
        };
    }

    fn rotate(&mut self, direction: Camera_Direction, dt: f32) {
        let velocity = 60.0 * dt * 0.001 * self.turning_speed;
        match direction {
            Camera_Direction::Left => self.rotate += velocity,
            Camera_Direction::Right => self.rotate -= velocity,
            _ => unreachable!()
        };
    }

    fn drag(&mut self, xoffset: i32, yoffset: i32) {
        self.pitch = self.pitch - xoffset as f32 * 0.0003;
        self.heading = self.heading - yoffset as f32 * 0.0003;
    }

    fn update_position(&mut self) {
        let pitch_quat = glm::quat_angle_axis(self.pitch, &self.axis_up);
        let heading_quat = glm::quat_angle_axis(self.heading, &self.axis_right);
        let rotate_quat = glm::quat_angle_axis(self.rotate, &self.direction);

        let temp = glm::quat_cross(&glm::quat_cross(&pitch_quat, &heading_quat), &rotate_quat);

        self.direction = glm::quat_cross_vec(&temp, &self.direction);
        self.axis_up = glm::quat_cross_vec(&temp, &self.axis_up);
        self.axis_right = glm::quat_cross_vec(&temp, &self.axis_right);
        
        self.heading *= 0.5;
        self.pitch *= 0.5;
        self.rotate *= 0.5;
        
        self.position += self.position_delta;
        self.position_delta = glm::vec3 (0.0, 0.0, 0.0);
    }

    fn get_view(&self) -> glm::TMat4<f32> {
        glm::look_at(&self.position, &(self.position + self.direction), &self.axis_up)
    }
}

const ratio_4_3: f64 = 4.0 / 3.0;
const ratio_256_224: f64 = 256.0 / 224.0;

const snes_factor_horizontal: f64 = ratio_4_3 / ratio_256_224;
const pixel_manipulation_base_speed: f32 = 20.0;
const turning_base_speed: f32 = 1.0;
const movement_speed_factor: f32 = 50.0;

const cube_geometry : [f32; 216] = [
    // cube coordinates       cube normals
    -0.5, -0.5, -0.5,      0.0,  0.0, -1.0,
     0.5, -0.5, -0.5,      0.0,  0.0, -1.0,
     0.5,  0.5, -0.5,      0.0,  0.0, -1.0,
     0.5,  0.5, -0.5,      0.0,  0.0, -1.0,
    -0.5,  0.5, -0.5,      0.0,  0.0, -1.0,
    -0.5, -0.5, -0.5,      0.0,  0.0, -1.0,

    -0.5, -0.5,  0.5,      0.0,  0.0,  1.0,
     0.5, -0.5,  0.5,      0.0,  0.0,  1.0,
     0.5,  0.5,  0.5,      0.0,  0.0,  1.0,
     0.5,  0.5,  0.5,      0.0,  0.0,  1.0,
    -0.5,  0.5,  0.5,      0.0,  0.0,  1.0,
    -0.5, -0.5,  0.5,      0.0,  0.0,  1.0,

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

const square_geometry : [f32; 36] = [
    -0.5, -0.5,  0.5,      0.0,  0.0,  1.0,
     0.5, -0.5,  0.5,      0.0,  0.0,  1.0,
     0.5,  0.5,  0.5,      0.0,  0.0,  1.0,
     0.5,  0.5,  0.5,      0.0,  0.0,  1.0,
    -0.5,  0.5,  0.5,      0.0,  0.0,  1.0,
    -0.5, -0.5,  0.5,      0.0,  0.0,  1.0,
];

pub fn program(gl: JsValue) -> Result<()> {
    let gl = gl.dyn_into::<WebGl2RenderingContext>()?;
    gl.enable(WebGl2RenderingContext::DEPTH_TEST);
    let render_loop = Rc::new(RefCell::new(Render_Loop {
        animation_frame_id: None,
        animation_frame_closure : None,
        keyboard_down_closure: None,
        keyboard_up_closure: None,
        mouse_position_closure: None,
        mouse_wheel_closure: None,
        resources: load_resources(&gl)?
    }));
    let mut input = Rc::new(RefCell::new( Input::new().ok().expect("cannot create input")));
    let frame_closure: Closure<FnMut()> = {
        let render_loop = render_loop.clone();
        let mut input = Rc::clone(&input);
        let window = window()?;
        Closure::wrap(Box::new(move || {
            let mut render_loop = render_loop.borrow_mut();
                let mut input = input.borrow_mut();
                input.now = now().unwrap_or(render_loop.resources.last_time);
                let update_status = update(&mut render_loop.resources, &input);
                if let Err(e) = update_status {
                    web_sys::console::error_2(&"An unexpected error happened during update.".into(), &match e { Wasm_Error::Js(o) => o, Wasm_Error::Str(s) => s.into()});
                    return;
                }

                input.mouse_scroll_y = 0.0;

                draw(&gl, &render_loop.resources);

                let mut frame_id = None;
                if let Some(ref frame_closure) = render_loop.animation_frame_closure {
                    if let Ok(id) = window.request_animation_frame(frame_closure.as_ref().unchecked_ref()) {
                        frame_id = Some(id);
                    }
                }
                render_loop.animation_frame_id = frame_id
        }))
    };
    let mut render_loop = render_loop.borrow_mut();
    render_loop.animation_frame_id = Some(window()?.request_animation_frame(frame_closure.as_ref().unchecked_ref())?);
    render_loop.animation_frame_closure = Some(frame_closure);

    let onkeydown: Closure<FnMut(JsValue)> = {
        let mut input = Rc::clone(&input);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<KeyboardEvent>() {
                let mut input = input.borrow_mut();
                match e.key().to_lowercase().as_ref() {
                    "a" => input.walk_left = true,
                    "d" => input.walk_right = true,
                    "w" => input.walk_forward = true,
                    "s" => input.walk_backward = true,
                    "q" => input.walk_up = true,
                    "e" => input.walk_down = true,
                    "arrowleft" => input.turn_left = true,
                    "arrowright" => input.turn_right = true,
                    "arrowup" => input.turn_up = true,
                    "arrowdown" => input.turn_down = true,
                    "+" => input.rotate_left = true,
                    "-" => input.rotate_right = true,
                    "f" => input.speed_up = true,
                    "r" => input.speed_down = true,
                    "u" => input.increase_pixel_scale_x = true,
                    "i" => input.decrease_pixel_scale_x = true,
                    "j" => input.increase_pixel_scale_y = true,
                    "k" => input.decrease_pixel_scale_y = true,
                    "n" => input.increase_pixel_gap = true,
                    "m" => input.decrease_pixel_gap = true,
                    "control" => input.ctrl = true,
                    "alt" => input.alt = true,
                    " " => input.space = true,
                    _ => web_sys::console::log_2(&"down".into(), &e.key().into())
                }
            }
        }))
    };

    let onkeyup: Closure<FnMut(JsValue)> = {
        let mut input = Rc::clone(&input);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<KeyboardEvent>() {
                let mut input = input.borrow_mut();
                match e.key().to_lowercase().as_ref() {
                    "a" => input.walk_left = false,
                    "d" => input.walk_right = false,
                    "w" => input.walk_forward = false,
                    "s" => input.walk_backward = false,
                    "q" => input.walk_up = false,
                    "e" => input.walk_down = false,
                    "arrowleft" => input.turn_left = false,
                    "arrowright" => input.turn_right = false,
                    "arrowup" => input.turn_up = false,
                    "arrowdown" => input.turn_down = false,
                    "+" => input.rotate_left = false,
                    "-" => input.rotate_right = false,
                    "f" => input.speed_up = false,
                    "r" => input.speed_down = false,
                    "u" => input.increase_pixel_scale_x = false,
                    "i" => input.decrease_pixel_scale_x = false,
                    "j" => input.increase_pixel_scale_y = false,
                    "k" => input.decrease_pixel_scale_y = false,
                    "n" => input.increase_pixel_gap = false,
                    "m" => input.decrease_pixel_gap = false,
                    "control" => input.ctrl = false,
                    "alt" => input.alt = false,
                    " " => input.space = false,
                    _ => web_sys::console::log_2(&"up".into(), &e.key().into())
                }
            }
        }))
    };

    let onmousemove: Closure<FnMut(JsValue)> = {
        let mut input = Rc::clone(&input);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                let mut input = input.borrow_mut();
                input.mouse_left_click = e.buttons() == 1;
                input.mouse_position_x = e.movement_x();
                input.mouse_position_y = e.movement_y();
            }
        }))
    };

    let onmousewheel: Closure<FnMut(JsValue)> = {
        let mut input = Rc::clone(&input);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<WheelEvent>() {
                let mut input = input.borrow_mut();
                input.mouse_scroll_y = e.delta_y() as f32;
            }
        }))
    };

    let document = window()?.document().ok_or("cannot access document")?;
    document.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));
    document.set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));
    document.set_onmousemove(Some(onmousemove.as_ref().unchecked_ref()));
    document.set_onwheel(Some(onmousewheel.as_ref().unchecked_ref()));

    render_loop.keyboard_down_closure = Some(onkeydown);
    render_loop.keyboard_up_closure = Some(onkeyup);
    render_loop.mouse_position_closure = Some(onmousemove);
    render_loop.mouse_wheel_closure = Some(onmousewheel);

    Ok(())
}

pub fn window() -> Result<web_sys::Window> {
    Ok(web_sys::window().ok_or("cannot access window")?)
}

pub fn now() -> Result<f64> {
    Ok(window()?.performance().ok_or("cannot access performance")?.now())
}

pub fn update(res: &mut Resources, input: &Input) -> Result<bool> {
    let dt: f32 = ((input.now - res.last_time) / 1000.0) as f32;
    let ellapsed = input.now - res.last_second;
    res.last_time = input.now;

    if ellapsed >= 5_000.0 {
        let fps = res.frame_count as f32 * 0.2;
        web_sys::console::log_2(&fps.into(), &"FPS".into());
        res.last_second = input.now;
        res.frame_count = 0;
    } else {
        res.frame_count += 1;
    }

    res.buttons.speed_up.track(input.speed_up);
    res.buttons.speed_down.track(input.speed_down);
    if input.alt {
        if res.buttons.speed_up.just_pressed { res.camera.turning_speed *= 2.0; }
        if res.buttons.speed_down.just_pressed { res.camera.turning_speed /= 2.0; }
    } else if input.ctrl {
        if res.buttons.speed_up.just_pressed { res.pixel_manipulation_speed *= 2.0; }
        if res.buttons.speed_down.just_pressed { res.pixel_manipulation_speed /= 2.0; }
    } else {
        if res.buttons.speed_up.just_pressed { res.camera.movement_speed *= 2.0; }
        if res.buttons.speed_down.just_pressed { res.camera.movement_speed /= 2.0; }
    }

    if res.camera.movement_speed > 10000.0 {
        res.camera.movement_speed = 10000.0; }
    if res.camera.movement_speed < 0.1 {
        res.camera.movement_speed = 0.1; }

    if res.camera.turning_speed > 10000.0 {
        res.camera.turning_speed = 10000.0;
    }
    if res.camera.turning_speed < 0.1 {
        res.camera.turning_speed = 0.1;
    }

    if input.walk_left { res.camera.advance(Camera_Direction::Left, dt); }
    if input.walk_right { res.camera.advance(Camera_Direction::Right, dt); }
    if input.walk_up { res.camera.advance(Camera_Direction::Up, dt); }
    if input.walk_down { res.camera.advance(Camera_Direction::Down, dt); }
    if input.walk_forward { res.camera.advance(Camera_Direction::Forward, dt); }
    if input.walk_backward { res.camera.advance(Camera_Direction::Backward, dt); }

    if input.turn_left { res.camera.turn(Camera_Direction::Left, dt); }
    if input.turn_right { res.camera.turn(Camera_Direction::Right, dt); }
    if input.turn_up { res.camera.turn(Camera_Direction::Up, dt); }
    if input.turn_down { res.camera.turn(Camera_Direction::Down, dt); }

    if input.rotate_left { res.camera.rotate(Camera_Direction::Left, dt); }
    if input.rotate_right { res.camera.rotate(Camera_Direction::Right, dt); }

    res.buttons.mouse_click.track(input.mouse_left_click || input.space);
    if res.buttons.mouse_click.just_pressed {
        res.last_mouse_x = input.mouse_position_x;
        res.last_mouse_y = input.mouse_position_y;

        window()?.dyn_into::<EventTarget>().ok().ok_or("cannot have even target")?.dispatch_event(&Event::new("request_pointer_lock")?);
    } else if res.buttons.mouse_click.activated {
        res.camera.drag(input.mouse_position_x, input.mouse_position_y);
    } else if res.buttons.mouse_click.just_released {
        window()?.dyn_into::<EventTarget>().ok().ok_or("cannot have even target")?.dispatch_event(&Event::new("exit_pointer_lock")?);
    }

    if input.mouse_scroll_y != 0.0 {
        if res.camera_zoom >= 1.0 && res.camera_zoom <= 45.0 {
            res.camera_zoom -= input.mouse_scroll_y * 0.1;
        }
        if res.camera_zoom <= 1.0 {
            res.camera_zoom = 1.0;
        }
        if res.camera_zoom >= 45.0 {
            res.camera_zoom = 45.0;
        }
    }

    res.camera.update_position();

    if input.increase_pixel_scale_x {
        res.cur_pixel_scale_x += 0.005 * dt * res.pixel_manipulation_speed; }
    if input.decrease_pixel_scale_x {
        res.cur_pixel_scale_x -= 0.005 * dt * res.pixel_manipulation_speed; }
    if res.cur_pixel_scale_x <= 0.0 {
        res.cur_pixel_scale_x = 0.0; }

    if input.increase_pixel_scale_y {
        res.cur_pixel_scale_y += 0.005 * dt * res.pixel_manipulation_speed; }
    if input.decrease_pixel_scale_y {
        res.cur_pixel_scale_y -= 0.005 * dt * res.pixel_manipulation_speed; }
    if res.cur_pixel_scale_y <= 0.0 {
        res.cur_pixel_scale_y = 0.0; }

    if input.increase_pixel_gap {
        res.cur_pixel_gap += 0.005 * dt * res.pixel_manipulation_speed; }
    if input.decrease_pixel_gap {
        res.cur_pixel_gap -= 0.005 * dt * res.pixel_manipulation_speed; }
    if res.cur_pixel_gap <= 0.0 {
        res.cur_pixel_gap = 0.0; }

    Ok(true)
}

const pixel_vertex_shader: &str = r#"#version 300 es
precision highp float;

in vec3 aPos;
in vec3 aNormal;
in vec4 aColor;
in vec2 aOffset;

out vec3 FragPos;
out vec3 Normal;
out vec4 ObjectColor;

uniform mat4 view;
uniform mat4 projection;

uniform vec2 voxel_gap;
uniform vec3 voxel_scale;

void main()
{
    FragPos = aPos / voxel_scale + vec3(aOffset * voxel_gap, 0);
    Normal = aNormal;  
    ObjectColor = aColor;
    
    gl_Position = projection * view * vec4(FragPos, 1.0);
}
"#;

const pixel_fragment_shader: &str = r#"#version 300 es
precision highp float;

out vec4 FragColor;

in vec3 Normal;  
in vec3 FragPos;
in vec4 ObjectColor;

uniform vec3 lightColor;
uniform vec3 lightPos;
uniform float ambientStrength;

void main()
{
    // ambient
    vec3 ambient = ambientStrength * lightColor;

    // diffuse 
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;
      
    FragColor = vec4(ambient + diffuse * (1.0 - ambientStrength), 1.0) * ObjectColor;
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

pub fn js_vec2_array(data: &[glm::Vec2]) -> js_sys::Float32Array {
    let array = js_sys::Float32Array::new(&wasm_bindgen::JsValue::from(data.len() as u32 * 2));
    for (i, f) in data.iter().enumerate() {
        array.fill(f[0], (i * 2 + 0) as u32, (i * 2 + 1) as u32);
        array.fill(f[1], (i * 2 + 1) as u32, (i * 2 + 2) as u32);
    }
    array
}


pub fn js_vec4_array(data: &[glm::Vec4]) -> js_sys::Float32Array {
    let array = js_sys::Float32Array::new(&wasm_bindgen::JsValue::from(data.len() as u32 * 4));
    for (i, f) in data.iter().enumerate() {
        array.fill(f[0], (i * 4 + 0) as u32, (i * 4 + 1) as u32);
        array.fill(f[1], (i * 4 + 1) as u32, (i * 4 + 2) as u32);
        array.fill(f[2], (i * 4 + 2) as u32, (i * 4 + 3) as u32);
        array.fill(f[3], (i * 4 + 3) as u32, (i * 4 + 4) as u32);
    }
    array
}

const WIDTH: usize = 256;
const HEIGHT: usize = 224;

pub fn check_error(gl: &WebGl2RenderingContext, line: u32) -> Result<()> {
    let error = gl.get_error();
    if error != WebGl2RenderingContext::NO_ERROR {
        return Err(Wasm_Error::Str(error.to_string() + " on line: " + &line.to_string()));
    }
    Ok(())
}

pub fn load_resources(gl: &WebGl2RenderingContext) -> Result<Resources> {
    const HALF_WIDTH: f32 = WIDTH as f32 / 2.0;
    const HALF_HEIGHT: f32 = HEIGHT as f32 / 2.0;
    let mut offsets = vec![glm::vec2(0.0, 0.0); WIDTH*HEIGHT];
    let mut colors = vec![glm::vec4(0.0, 0.0, 0.0, 0.0); WIDTH*HEIGHT];
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let x = i as f32 - HALF_WIDTH;
            let y = j as f32 - HALF_HEIGHT;
            offsets[j * WIDTH + i] = glm::vec2(x as f32, y as f32);
            colors[j * WIDTH + i] = glm::vec4(
                if j % 2 == 0 {0.8} else {0.2}, 
                if i % 2 == 0 {0.8} else {0.2}, 
                if (j + i) % 2 == 0 {0.8} else {0.2}, 
                1.0
            );
        }
    }

    let program = make_shader(&gl, pixel_vertex_shader, pixel_fragment_shader)?;

    let pixel_vao = gl.create_vertex_array();
    gl.bind_vertex_array(pixel_vao.as_ref());

    check_error(&gl, line!())?;

    let pixel_vbo = gl.create_buffer().ok_or("cannot create pixel_vbo")?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&pixel_vbo));
    gl.buffer_data_with_opt_array_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        Some(&js_f32_array(&cube_geometry).buffer()),
        WebGl2RenderingContext::STATIC_DRAW,
    );

    check_error(&gl, line!())?;

    let aPos_position = gl.get_attrib_location(&program, "aPos") as u32;
    gl.vertex_attrib_pointer_with_i32(aPos_position, 3, WebGl2RenderingContext::FLOAT, false, 6 * size_of::<f32>() as i32, 0);
    gl.enable_vertex_attrib_array(aPos_position);

    let aNormal_position = gl.get_attrib_location(&program, "aNormal") as u32;
    gl.vertex_attrib_pointer_with_i32(aNormal_position, 3, WebGl2RenderingContext::FLOAT, false, 6 * size_of::<f32>() as i32, 3 * size_of::<f32>() as i32);
    gl.enable_vertex_attrib_array(aNormal_position);

    check_error(&gl, line!())?;

    let colors_vbo = gl.create_buffer().ok_or("cannot create colors_vbo")?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&colors_vbo));
    gl.buffer_data_with_opt_array_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        Some(&js_vec4_array(&colors).buffer()),
        WebGl2RenderingContext::STATIC_DRAW,
    );

    check_error(&gl, line!())?;

    let aColor_position = gl.get_attrib_location(&program, "aColor") as u32;
    gl.enable_vertex_attrib_array(aColor_position);
    gl.vertex_attrib_pointer_with_i32(aColor_position, 4, WebGl2RenderingContext::FLOAT, false, size_of::<glm::Vec4>() as i32, 0);
    gl.vertex_attrib_divisor(aColor_position, 1);

    check_error(&gl, line!())?;

    let offset_vbo = gl.create_buffer().ok_or("cannot create offset_vbo")?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&offset_vbo));
    gl.buffer_data_with_opt_array_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        Some(&js_vec2_array(&offsets).buffer()),
        WebGl2RenderingContext::STATIC_DRAW,
    );

    check_error(&gl, line!())?;

    let aOffset_position = gl.get_attrib_location(&program, "aOffset") as u32;
    gl.enable_vertex_attrib_array(aOffset_position);
    gl.vertex_attrib_pointer_with_i32(aOffset_position, 2, WebGl2RenderingContext::FLOAT, false, size_of::<glm::Vec2>() as i32, 0);
    gl.vertex_attrib_divisor(aOffset_position, 1);

    gl.bind_vertex_array(None);

    check_error(&gl, line!())?;

    let now = now()?;

    Ok(Resources {
        pixel_shader: program,
        pixel_vao: pixel_vao,
        frame_count: 0,
        last_time: now,
        last_second: now,
        last_mouse_x: -1,
        last_mouse_y: -1,
        pixel_manipulation_speed: pixel_manipulation_base_speed,
        cur_pixel_scale_x: 0.0,
        cur_pixel_scale_y: 0.0,
        cur_pixel_gap: 0.0,
        camera: Camera::new(),
        camera_zoom: 45.0,
        buttons: Buttons::new()
    })
}

pub fn radians(grad: f32) -> f32 {
    let pi: f32 = glm::pi();
    grad * pi / 180.0
}

pub fn draw(gl: &WebGl2RenderingContext, res: &Resources) -> Result<()> {
    let screen_width = 3920.0;
    let screen_height = 2160.0;

    let mut projection = glm::perspective::<f32>(screen_width / screen_height, radians(res.camera_zoom), 0.01, 10000.0);
    let mut view = res.camera.get_view();

    gl.clear_color(0.05, 0.05, 0.05, 1.0);  // Clear to black, fully opaque
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT|WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    check_error(&gl, line!())?;

    let mut voxel_scale : &mut [f32] = &mut [
        res.cur_pixel_scale_x + 1.0,
        res.cur_pixel_scale_y + 1.0,
        (res.cur_pixel_scale_x + res.cur_pixel_scale_x)/2.0 + 1.0
    ];

    let mut voxel_gap : &mut [f32] = &mut [1.0 + res.cur_pixel_gap, 1.0 + res.cur_pixel_gap];

    gl.use_program(Some(&res.pixel_shader));
    gl.uniform_matrix4fv_with_f32_array(gl.get_uniform_location(&res.pixel_shader, "view").as_ref(), false, view.as_mut_slice());
    gl.uniform_matrix4fv_with_f32_array(gl.get_uniform_location(&res.pixel_shader, "projection").as_ref(), false, projection.as_mut_slice());
    gl.uniform3fv_with_f32_array(gl.get_uniform_location(&res.pixel_shader, "lightPos").as_ref(), &mut [screen_width / 2.0, screen_height / 2.0, 400.0]);
    gl.uniform3fv_with_f32_array(gl.get_uniform_location(&res.pixel_shader, "lightColor").as_ref(), &mut [1.0, 1.0, 1.0]);
    gl.uniform1f(gl.get_uniform_location(&res.pixel_shader, "ambientStrength").as_ref(), 0.5);
    gl.uniform2fv_with_f32_array(gl.get_uniform_location(&res.pixel_shader, "voxel_gap").as_ref(), &mut voxel_gap);
    gl.uniform3fv_with_f32_array(gl.get_uniform_location(&res.pixel_shader, "voxel_scale").as_ref(), &mut voxel_scale);

    check_error(&gl, line!())?;

    gl.bind_vertex_array(res.pixel_vao.as_ref());
    gl.draw_arrays_instanced(
        WebGl2RenderingContext::TRIANGLES,
        0,
        36,
        (WIDTH * HEIGHT) as i32
    );

    check_error(&gl, line!())?;

    Ok(())
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
        Err(Wasm_Error::Str(gl
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
        Err(Wasm_Error::Str(gl.get_program_info_log(&program).ok_or("cannot get program info log")?))
    }
}
