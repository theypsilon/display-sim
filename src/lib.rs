extern crate cfg_if;
extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;
extern crate nalgebra_glm as glm;

mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    Window, console,
    WebGlProgram, WebGl2RenderingContext, 
    WebGlShader, WebGlVertexArrayObject, 
    KeyboardEvent, MouseEvent, WheelEvent, Event, EventTarget
};
use js_sys::{Float32Array};
use std::rc::Rc;
use std::cell::RefCell;
use std::mem::size_of;

#[wasm_bindgen]
pub fn main(gl: JsValue, animation: Animation_Source) {
    match program(gl, &animation) {
        Err(e) => match e {
            Wasm_Error::Js(o) => console::error_2(&"An unexpected error ocurred.".into(), &o),
            Wasm_Error::Str(s) => console::error_2(&"An unexpected error ocurred.".into(), &s.into()),
        },
        Ok(_) => {}
    };
}

#[wasm_bindgen]
pub struct Animation_Source {
    steps: Vec<Float32Array>,
    width: u32,
    height: u32,
    scale_x: f32,
    scale_y: f32,
    screen_width: u32,
    screen_height: u32,
    frame_length: f32
}

#[wasm_bindgen]
impl Animation_Source {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, screen_width: u32, screen_height: u32, frame_length: f32, scale_x: f32, scale_y: f32) -> Animation_Source {
        Animation_Source {
            steps: Vec::new(),
            width: width,
            height: height,
            screen_width: screen_width,
            screen_height: screen_height,
            frame_length: frame_length,
            scale_x: scale_x,
            scale_y: scale_y,
        }
    }

    pub fn add(&mut self, frame: Float32Array) {
        self.steps.push(frame);
    }
}

pub enum Wasm_Error {
    Js(JsValue),
    Str(String)
}

impl Wasm_Error {
    fn to_js(self) -> JsValue {
        match self { Wasm_Error::Js(o) => o, Wasm_Error::Str(s) => s.into()}
    }
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
    toggle_pixels_or_voxels: Boolean_Button,
    showing_pixels_pulse: Boolean_Button,
}

impl Buttons {
    fn new() -> Buttons {
        Buttons {
            speed_up: Boolean_Button::new(),
            speed_down: Boolean_Button::new(),
            mouse_click: Boolean_Button::new(),
            toggle_pixels_or_voxels: Boolean_Button::new(),
            showing_pixels_pulse: Boolean_Button::new()
        }
    }
}

enum Pixels_Or_Voxels {
    Pixels,
    Voxels
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
    pixels_or_voxels: Pixels_Or_Voxels,
    pixels_pulse: f32,
    showing_pixels_pulse: bool,
    pixel_manipulation_speed: f32,
    camera: Camera,
    camera_zoom: f32,
    animation: Animation_Drawable,
    buttons: Buttons,
}

struct Animation_Drawable {
    width: u32,
    height: u32,
    screen_width: u32,
    screen_height: u32,
    frame_length: f32,
    scale_x: f32,
    scale_y: f32,
}

impl Animation_Drawable {
    fn new(source: &Animation_Source) -> Animation_Drawable {
        Animation_Drawable {
            width: source.width,
            height: source.height,
            screen_width: source.screen_width,
            screen_height: source.screen_height,
            frame_length: source.frame_length,
            scale_x: source.scale_x,
            scale_y: source.scale_y,
        }
    }
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
    reset_speeds: bool,
    shift: bool,
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
    toggle_pixels_or_voxels: bool,
    showing_pixels_pulse: bool,
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
            reset_speeds: false,
            shift: false,
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
            toggle_pixels_or_voxels: false,
            showing_pixels_pulse: false,
        })
    }
}

struct State_Owner {
    animation_frame_id: Option<i32>,
    owned_closures: Vec<Option<Closure<FnMut(JsValue)>>>,
    resources: Resources,
}

impl State_Owner {
    fn new(resources: Resources) -> State_Owner {
        State_Owner {
            animation_frame_id: None,
            owned_closures: Vec::new(),
            resources: resources,
        }
    }
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
            movement_speed: movement_base_speed,
            turning_speed: turning_base_speed
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

const pixel_manipulation_base_speed: f32 = 20.0;
const turning_base_speed: f32 = 1.0;
const movement_base_speed: f32 = 10.0;
const movement_speed_factor: f32 = 50.0;

const cube_geometry : [f32; 216] = [
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

pub fn program(gl: JsValue, animation: &Animation_Source) -> Result<()> {
    let gl = gl.dyn_into::<WebGl2RenderingContext>()?;
    gl.enable(WebGl2RenderingContext::DEPTH_TEST);
    let render_loop = Rc::new(RefCell::new(State_Owner::new(load_resources(&gl, &animation)?)));
    let input = Rc::new(RefCell::new( Input::new().ok().expect("cannot create input")));
    let frame_closure: Closure<FnMut(JsValue)> = {
        let render_loop = render_loop.clone();
        let mut input = Rc::clone(&input);
        let window = window()?;
        Closure::wrap(Box::new(move |_| {
            let mut render_loop = render_loop.borrow_mut();
                let mut input = input.borrow_mut();
                input.now = now().unwrap_or(render_loop.resources.last_time);
                let update_status = update(&mut render_loop.resources, &input);

                input.mouse_scroll_y = 0.0;
                input.mouse_position_x = 0;
                input.mouse_position_y = 0;

                if let Err(e) = update_status {
                    console::error_2(&"An unexpected error happened during update.".into(), &e.to_js());
                    return;
                }

                if let Err(e) = draw(&gl, &render_loop.resources) {
                    console::error_2(&"An unexpected error happened during draw.".into(), &e.to_js());
                    return;
                }

                let mut frame_id = None;
                if let Some(ref frame_closure) = render_loop.owned_closures[0] {
                    if let Ok(id) = window.request_animation_frame(frame_closure.as_ref().unchecked_ref()) {
                        frame_id = Some(id);
                    }
                }
                render_loop.animation_frame_id = frame_id
        }))
    };
    let mut render_loop = render_loop.borrow_mut();
    render_loop.animation_frame_id = Some(window()?.request_animation_frame(frame_closure.as_ref().unchecked_ref())?);
    render_loop.owned_closures.push(Some(frame_closure));

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
                    "t" => input.reset_speeds = true,
                    "u" => input.increase_pixel_scale_x = true,
                    "i" => input.decrease_pixel_scale_x = true,
                    "j" => input.increase_pixel_scale_y = true,
                    "k" => input.decrease_pixel_scale_y = true,
                    "n" => input.increase_pixel_gap = true,
                    "m" => input.decrease_pixel_gap = true,
                    "o" => input.toggle_pixels_or_voxels = true,
                    "p" => input.showing_pixels_pulse = true,
                    "shift" => input.shift = true,
                    "alt" => input.alt = true,
                    " " => input.space = true,
                    _ => console::log_2(&"down".into(), &e.key().into())
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
                    "t" => input.reset_speeds = false,
                    "u" => input.increase_pixel_scale_x = false,
                    "i" => input.decrease_pixel_scale_x = false,
                    "j" => input.increase_pixel_scale_y = false,
                    "k" => input.decrease_pixel_scale_y = false,
                    "n" => input.increase_pixel_gap = false,
                    "m" => input.decrease_pixel_gap = false,
                    "o" => input.toggle_pixels_or_voxels = false,
                    "p" => input.showing_pixels_pulse = false,
                    "shift" => input.shift = false,
                    "alt" => input.alt = false,
                    " " => input.space = false,
                    _ => console::log_2(&"up".into(), &e.key().into())
                }
            }
        }))
    };

    let onmousedown: Closure<FnMut(JsValue)> = {
        let mut input = Rc::clone(&input);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                let mut input = input.borrow_mut();
                input.mouse_left_click = e.buttons() == 1;
            }
        }))
    };

    let onmouseup: Closure<FnMut(JsValue)> = {
        let mut input = Rc::clone(&input);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(_) = event.dyn_into::<MouseEvent>() {
                let mut input = input.borrow_mut();
                input.mouse_left_click = false;
            }
        }))
    };

    let onmousemove: Closure<FnMut(JsValue)> = {
        let mut input = Rc::clone(&input);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                let mut input = input.borrow_mut();
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
    document.set_onmousedown(Some(onmousedown.as_ref().unchecked_ref()));
    document.set_onmouseup(Some(onmouseup.as_ref().unchecked_ref()));
    document.set_onmousemove(Some(onmousemove.as_ref().unchecked_ref()));
    document.set_onwheel(Some(onmousewheel.as_ref().unchecked_ref()));

    render_loop.owned_closures.push(Some(onkeydown));
    render_loop.owned_closures.push(Some(onkeyup));
    render_loop.owned_closures.push(Some(onmousedown));
    render_loop.owned_closures.push(Some(onmouseup));
    render_loop.owned_closures.push(Some(onmousemove));
    render_loop.owned_closures.push(Some(onmousewheel));

    Ok(())
}

pub fn window() -> Result<Window> {
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
        console::log_2(&fps.into(), &"FPS".into());
        res.last_second = input.now;
        res.frame_count = 0;
    } else {
        res.frame_count += 1;
    }

    res.buttons.showing_pixels_pulse.track(input.showing_pixels_pulse);
    if res.buttons.showing_pixels_pulse.just_pressed {
        res.showing_pixels_pulse = !res.showing_pixels_pulse;
    }

    if res.showing_pixels_pulse {
        res.pixels_pulse += dt * 0.3;
    } else {
        res.pixels_pulse = 0.0;
    }

    res.buttons.toggle_pixels_or_voxels.track(input.toggle_pixels_or_voxels);
    if res.buttons.toggle_pixels_or_voxels.just_released {
        res.pixels_or_voxels = match res.pixels_or_voxels {
            Pixels_Or_Voxels::Pixels => Pixels_Or_Voxels::Voxels,
            Pixels_Or_Voxels::Voxels => Pixels_Or_Voxels::Pixels
        };
    }

    res.buttons.speed_up.track(input.speed_up);
    res.buttons.speed_down.track(input.speed_down);
    if input.alt {
        if res.buttons.speed_up.just_pressed { res.camera.turning_speed *= 2.0; }
        if res.buttons.speed_down.just_pressed { res.camera.turning_speed /= 2.0; }
    } else if input.shift {
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

    if input.reset_speeds {
        res.camera.turning_speed = turning_base_speed;
        res.camera.movement_speed = movement_base_speed;
        res.pixel_manipulation_speed = pixel_manipulation_base_speed;
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

    res.buttons.mouse_click.track(input.mouse_left_click);
    if res.buttons.mouse_click.just_pressed {
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
    vec3 ambient = ambientStrength * lightColor;

    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;
      
    FragColor = ObjectColor * vec4(ambient + diffuse * (1.0 - ambientStrength), 1.0);
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

pub fn js_f32_array(data: &[f32]) -> Float32Array {
    let array = Float32Array::new(&wasm_bindgen::JsValue::from(data.len() as u32));
    for (i, f) in data.iter().enumerate() {
        array.fill(*f, i as u32, (i + 1) as u32);
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

pub fn load_resources(gl: &WebGl2RenderingContext, animation: &Animation_Source) -> Result<Resources> {
    console::log_2(&now()?.into(), &"load_resources".into());
    let width = animation.width as usize;
    let height = animation.height as usize;
    let half_width: f32 = width as f32 / 2.0;
    let half_height: f32 = height as f32 / 2.0;
    let pixels_total = width * height;
    let channels = 4;
    let colors = &animation.steps[0];
    let offsets = Float32Array::new(&wasm_bindgen::JsValue::from(pixels_total as u32 * 2)); // js_vec2_array
    console::log_2(&now()?.into(), &"for loop begin".into());
    for i in 0..width {
        for j in 0..height {
            let index = (pixels_total - width - j * width + i) as u32;
            let x = i as f32 - half_width;
            let y = j as f32 - half_height;
            offsets.fill(x, index * 2 + 0, index * 2 + 1);
            offsets.fill(y, index * 2 + 1, index * 2 + 2);
        }
    }
    console::log_2(&now()?.into(), &"for loop end".into());

    let program = make_shader(&gl, pixel_vertex_shader, pixel_fragment_shader)?;

    let pixel_vao = gl.create_vertex_array();
    gl.bind_vertex_array(pixel_vao.as_ref());

    let pixel_vbo = gl.create_buffer().ok_or("cannot create pixel_vbo")?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&pixel_vbo));
    gl.buffer_data_with_opt_array_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        Some(&js_f32_array(&cube_geometry).buffer()),
        WebGl2RenderingContext::STATIC_DRAW,
    );

    let a_pos_position = gl.get_attrib_location(&program, "aPos") as u32;
    gl.vertex_attrib_pointer_with_i32(a_pos_position, 3, WebGl2RenderingContext::FLOAT, false, 6 * size_of::<f32>() as i32, 0);
    gl.enable_vertex_attrib_array(a_pos_position);

    let a_normal_position = gl.get_attrib_location(&program, "aNormal") as u32;
    gl.vertex_attrib_pointer_with_i32(a_normal_position, 3, WebGl2RenderingContext::FLOAT, false, 6 * size_of::<f32>() as i32, 3 * size_of::<f32>() as i32);
    gl.enable_vertex_attrib_array(a_normal_position);

    let colors_vbo = gl.create_buffer().ok_or("cannot create colors_vbo")?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&colors_vbo));
    console::log_2(&now()?.into(), &"buffer colors".into());
    gl.buffer_data_with_opt_array_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        Some(&colors.buffer()),
        WebGl2RenderingContext::STATIC_DRAW,
    );

    let a_color_position = gl.get_attrib_location(&program, "aColor") as u32;
    gl.enable_vertex_attrib_array(a_color_position);
    gl.vertex_attrib_pointer_with_i32(a_color_position, 1, WebGl2RenderingContext::FLOAT, false, size_of::<f32>() as i32, 0);
    gl.vertex_attrib_divisor(a_color_position, 1);

    let offset_vbo = gl.create_buffer().ok_or("cannot create offset_vbo")?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&offset_vbo));
    console::log_2(&now()?.into(), &"buffer offsets".into());
    gl.buffer_data_with_opt_array_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        Some(&offsets.buffer()),
        WebGl2RenderingContext::STATIC_DRAW,
    );

    let a_offset_position = gl.get_attrib_location(&program, "aOffset") as u32;
    gl.enable_vertex_attrib_array(a_offset_position);
    gl.vertex_attrib_pointer_with_i32(a_offset_position, 2, WebGl2RenderingContext::FLOAT, false, size_of::<glm::Vec2>() as i32, 0);
    gl.vertex_attrib_divisor(a_offset_position, 1);

    let now = now()?;

    let far_away_position = {
        let far_factor: f64 = 112.0 / 270.0;
        animation.height as f64 / 2.0 / far_factor 
    } as f32;

    let mut camera = Camera::new();
    camera.position = glm::vec3(0.0, 0.0, far_away_position);
    camera.movement_speed *= far_away_position / movement_speed_factor;

    check_error(&gl, line!())?;
    console::log_2(&now.into(), &"load_resources end".into());
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
        pixels_or_voxels: Pixels_Or_Voxels::Pixels,
        pixels_pulse: 0.0,
        showing_pixels_pulse: false,
        animation: Animation_Drawable::new(animation),
        camera: camera,
        camera_zoom: 45.0,
        buttons: Buttons::new()
    })
}

pub fn radians(grad: f32) -> f32 {
    let pi: f32 = glm::pi();
    grad * pi / 180.0
}

pub fn draw(gl: &WebGl2RenderingContext, res: &Resources) -> Result<()> {
    let screen_width = res.animation.screen_width as f32;
    let screen_height = res.animation.screen_height as f32;

    let mut projection = glm::perspective::<f32>(screen_width / screen_height, radians(res.camera_zoom), 0.01, 10000.0);
    let mut view = res.camera.get_view();

    gl.clear_color(0.05, 0.05, 0.05, 1.0);  // Clear to black, fully opaque
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT|WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    let mut pixel_scale : &mut [f32] = &mut [
        res.cur_pixel_scale_x + 1.0,
        res.cur_pixel_scale_y + 1.0,
        (res.cur_pixel_scale_x + res.cur_pixel_scale_x)/2.0 + 1.0
    ];

    let mut pixel_gap : &mut [f32] = &mut [1.0 + res.cur_pixel_gap, 1.0 + res.cur_pixel_gap];

    if res.animation.scale_x != 1.0 {
        pixel_scale[0] /= res.animation.scale_x;
        pixel_gap[0] *= res.animation.scale_x;
    }
    if res.animation.scale_y != 1.0 {
        pixel_scale[1] /= res.animation.scale_y;
        pixel_gap[1] *= res.animation.scale_y;
    }

    gl.use_program(Some(&res.pixel_shader));
    gl.uniform_matrix4fv_with_f32_array(gl.get_uniform_location(&res.pixel_shader, "view").as_ref(), false, view.as_mut_slice());
    gl.uniform_matrix4fv_with_f32_array(gl.get_uniform_location(&res.pixel_shader, "projection").as_ref(), false, projection.as_mut_slice());
    gl.uniform3fv_with_f32_array(gl.get_uniform_location(&res.pixel_shader, "lightPos").as_ref(), &mut [screen_width / 2.0, screen_height / 2.0, 10.0]);
    gl.uniform3fv_with_f32_array(gl.get_uniform_location(&res.pixel_shader, "lightColor").as_ref(), &mut [1.0, 1.0, 1.0]);
    gl.uniform1f(gl.get_uniform_location(&res.pixel_shader, "ambientStrength").as_ref(), 0.75);
    gl.uniform2fv_with_f32_array(gl.get_uniform_location(&res.pixel_shader, "pixel_gap").as_ref(), &mut pixel_gap);
    gl.uniform3fv_with_f32_array(gl.get_uniform_location(&res.pixel_shader, "pixel_scale").as_ref(), &mut pixel_scale);
    gl.uniform1f(gl.get_uniform_location(&res.pixel_shader, "pixel_pulse").as_ref(), res.pixels_pulse);

    gl.bind_vertex_array(res.pixel_vao.as_ref());
    gl.draw_arrays_instanced(
        WebGl2RenderingContext::TRIANGLES,
        0,
        match res.pixels_or_voxels { Pixels_Or_Voxels::Pixels => 6, Pixels_Or_Voxels::Voxels => 36 },
        (res.animation.width * res.animation.height) as i32
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
