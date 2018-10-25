use wasm_bindgen::prelude::*;
use js_sys::{ArrayBuffer};

use wasm_error::{Result};
use boolean_button::BooleanButton;
use camera::{Camera};
use web_utils::{now};
use pixels_render::{PixelsRender, PixelsRenderKind};
use blur_render::BlurRender;

pub struct AnimationData {
    pub steps: Vec<ArrayBuffer>,
    pub width: u32,
    pub height: u32,
    pub scale_x: f32,
    pub stretch: bool,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub frame_length: f32,
    pub current_frame: usize,
    pub last_frame_change: f64,
    pub needs_buffer_data_load: bool,
}

pub struct StateOwner {
    pub animation_frame_id: Option<i32>,
    pub closures: Vec<Option<Closure<FnMut(JsValue)>>>,
    pub resources: Resources,
}

impl StateOwner {
    pub fn new(resources: Resources) -> StateOwner {
        StateOwner {
            animation_frame_id: None,
            closures: Vec::new(),
            resources,
        }
    }
}

pub struct Resources {
    pub pixels_render: PixelsRender,
    pub blur_render: BlurRender,
    pub bloom_passes: usize,
    pub light_color: i32,
    pub brightness_color: i32,
    pub extra_bright: f32,
    pub frame_count: u32,
    pub last_time: f64,
    pub last_second: f64,
    pub translation_base_speed: f32,
    pub cur_pixel_scale_x: f32,
    pub cur_pixel_scale_y: f32,
    pub cur_pixel_gap: f32,
    pub pixels_render_kind: PixelsRenderKind,
    pub pixels_pulse: f32,
    pub showing_pixels_pulse: bool,
    pub pixel_manipulation_speed: f32,
    pub camera: Camera,
    pub camera_zoom: f32,
    pub animation: AnimationData,
    pub buttons: Buttons,
}

pub struct Buttons {
    pub speed_up: BooleanButton,
    pub speed_down: BooleanButton,
    pub mouse_click: BooleanButton,
    pub increase_bloom: BooleanButton,
    pub decrease_bloom: BooleanButton,
    pub toggle_pixels_render_kind: BooleanButton,
    pub showing_pixels_pulse: BooleanButton,
    pub esc: BooleanButton,
    pub space: BooleanButton,
}

impl Buttons {
    pub fn new() -> Buttons {
        Buttons {
            speed_up: BooleanButton::new(),
            speed_down: BooleanButton::new(),
            mouse_click: BooleanButton::new(),
            increase_bloom: BooleanButton::new(),
            decrease_bloom: BooleanButton::new(),
            toggle_pixels_render_kind: BooleanButton::new(),
            showing_pixels_pulse: BooleanButton::new(),
            esc: BooleanButton::new(),
            space: BooleanButton::new(),
        }
    }
}

#[derive(Clone)]
pub struct Input {
    pub now: f64,
    pub color_value: i32,
    pub color_kind: i32,
    pub walk_left: bool,
    pub walk_right: bool,
    pub walk_up: bool,
    pub walk_down: bool,
    pub walk_forward: bool,
    pub walk_backward: bool,
    pub turn_left: bool,
    pub turn_right: bool,
    pub turn_up: bool,
    pub turn_down: bool,
    pub rotate_left: bool,
    pub rotate_right: bool,
    pub speed_up: bool,
    pub speed_down: bool,
    pub reset_speeds: bool,
    pub increase_bloom: bool,
    pub decrease_bloom: bool,
    pub shift: bool,
    pub alt: bool,
    pub space: bool,
    pub esc: bool,
    pub mouse_left_click: bool,
    pub mouse_position_x: i32,
    pub mouse_position_y: i32,
    pub mouse_scroll_y: f32,
    pub increase_pixel_scale_y: bool,
    pub decrease_pixel_scale_y: bool,
    pub increase_pixel_scale_x: bool,
    pub decrease_pixel_scale_x: bool,
    pub increase_pixel_gap: bool,
    pub decrease_pixel_gap: bool,
    pub increase_bright: bool,
    pub decrease_bright: bool,
    pub reset_brightness: bool,
    pub toggle_pixels_render_kind: bool,
    pub showing_pixels_pulse: bool,
}

impl Input {
    pub fn new() -> Result<Input> {
        Ok(Input {
            now: now()?,
            color_value: 0x00FF_FFFF,
            color_kind: 1,
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
            increase_bloom: false,
            decrease_bloom: false,
            shift: false,
            alt: false,
            space: false,
            esc: false,
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
            increase_bright: false,
            decrease_bright: false,
            reset_brightness: false,
            toggle_pixels_render_kind: false,
            showing_pixels_pulse: false,
        })
    }
}