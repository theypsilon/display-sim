use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::{Closure, JsValue};
use js_sys::{ArrayBuffer};

use wasm_error::{WasmResult};
use boolean_button::BooleanButton;
use camera::{Camera};
use web_utils::{now};
use pixels_render::{PixelsRender, PixelsRenderKind};
use blur_render::BlurRender;

pub struct AnimationData {
    pub steps: Vec<ArrayBuffer>,
    pub image_width: u32,
    pub image_height: u32,
    pub background_width: u32,
    pub background_height: u32,
    pub viewport_width: u32,
    pub viewport_height: u32,
    pub pixel_width: f32,
    pub stretch: bool,
    pub frame_length: f32,
    pub current_frame: usize,
    pub last_frame_change: f64,
    pub needs_buffer_data_load: bool,
}

pub type OwnedClosure = Option<Closure<FnMut(JsValue)>>;

pub struct StateOwner {
    pub closures: RefCell<Vec<OwnedClosure>>,
    pub resources: RefCell<Resources>,
    pub input: RefCell<Input>,
}

impl StateOwner {
    pub fn new_rc(resources: Resources, input: Input) -> Rc<StateOwner> {
        Rc::new(StateOwner {
            closures: RefCell::new(Vec::new()),
            resources: RefCell::new(resources),
            input: RefCell::new(input),
        })
    }
}

pub struct Resources {
    pub animation: AnimationData,
    pub camera: Camera,
    pub crt_filters: CrtFilters,
    pub pixels_render: PixelsRender,
    pub blur_render: BlurRender,
    pub timers: SimulationTimers,
    pub initial_parameters: InitialParameters,
    pub buttons: Buttons,
}

pub struct SimulationTimers {
    pub frame_count: u32,
    pub last_time: f64,
    pub last_second: f64,
}

pub struct InitialParameters {
    pub initial_movement_speed: f32,
    pub initial_position_z: f32,
    pub initial_pixel_width: f32,
}

pub struct CrtFilters {
    pub blur_passes: usize,
    pub light_color: i32,
    pub brightness_color: i32,
    pub extra_bright: f32,
    pub cur_pixel_width: f32,
    pub cur_pixel_scale_x: f32,
    pub cur_pixel_scale_y: f32,
    pub cur_pixel_gap: f32,
    pub change_speed: f32,
    pub pixels_pulse: f32,
    pub pixels_render_kind: PixelsRenderKind,
    pub showing_split_colors: bool,
    pub showing_pixels_pulse: bool,
}

impl CrtFilters {
    pub fn new(change_speed: f32) -> CrtFilters {
        CrtFilters {
            blur_passes: 0,
            light_color: 0x00FF_FFFF,
            brightness_color: 0x00FF_FFFF,
            extra_bright: 0.0,
            cur_pixel_width: 1.0,
            cur_pixel_scale_x: 0.0,
            cur_pixel_scale_y: 0.0,
            cur_pixel_gap: 0.0,
            change_speed: change_speed,
            pixels_pulse: 0.0,
            pixels_render_kind: PixelsRenderKind::Squares,
            showing_split_colors: false,
            showing_pixels_pulse: false,
        }
    }
}

pub struct Buttons {
    pub speed_up: BooleanButton,
    pub speed_down: BooleanButton,
    pub mouse_click: BooleanButton,
    pub increase_blur: BooleanButton,
    pub decrease_blur: BooleanButton,
    pub toggle_split_colors: BooleanButton,
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
            increase_blur: BooleanButton::new(),
            decrease_blur: BooleanButton::new(),
            toggle_split_colors: BooleanButton::new(),
            toggle_pixels_render_kind: BooleanButton::new(),
            showing_pixels_pulse: BooleanButton::new(),
            esc: BooleanButton::new(),
            space: BooleanButton::new(),
        }
    }
}

pub struct CustomInputEvent {
    pub value: JsValue,
    pub kind: String,
}

impl CustomInputEvent {
    pub fn new() -> CustomInputEvent {
        CustomInputEvent {
            value: JsValue::undefined(),
            kind: String::new(),
        }
    }
}

pub struct Input {
    pub now: f64,
    pub custom_event: CustomInputEvent,
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
    pub reset_position: bool,
    pub reset_filters: bool,
    pub increase_blur: bool,
    pub decrease_blur: bool,
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
    pub toggle_split_colors: bool,
    pub toggle_pixels_render_kind: bool,
    pub showing_pixels_pulse: bool,
}

impl Input {
    pub fn new() -> WasmResult<Input> {
        Ok(Input {
            now: now()?,
            custom_event: CustomInputEvent::new(),
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
            reset_position: false,
            reset_filters: false,
            increase_blur: false,
            decrease_blur: false,
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
            toggle_split_colors: false,
            toggle_pixels_render_kind: false,
            showing_pixels_pulse: false,
        })
    }
}