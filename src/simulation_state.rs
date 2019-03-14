use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::{Closure, JsValue};
use js_sys::{ArrayBuffer};

use crate::wasm_error::{WasmResult};
use crate::boolean_button::BooleanButton;
use crate::camera::{Camera};
use crate::web_utils::{now};
use crate::pixels_render::{PixelsRender, PixelsGeometryKind};
use crate::blur_render::BlurRender;
use crate::internal_resolution_render::InternalResolutionRender;
use crate::rgb_render::RgbRender;
use crate::background_render::BackgroundRender;
use crate::render_types::{TextureBufferStack};

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
    pub background_render: BackgroundRender,
    pub internal_resolution_render: InternalResolutionRender,
    pub internal_resolution_multiplier: i32,
    pub rgb_render: RgbRender,
    pub texture_buffer_stack: std::cell::RefCell<TextureBufferStack>,
    pub timers: SimulationTimers,
    pub initial_parameters: InitialParameters,
    pub launch_screenshot: bool,
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
    pub lines_per_pixel: usize,
    pub light_color: i32,
    pub brightness_color: i32,
    pub extra_bright: f32,
    pub extra_contrast: f32,
    pub cur_pixel_width: f32,
    pub cur_pixel_scale_x: f32,
    pub cur_pixel_scale_y: f32,
    pub cur_pixel_gap: f32,
    pub change_speed: f32,
    pub pixels_pulse: f32,
    pub pixels_geometry_kind: PixelsGeometryKind,
    pub color_channels: ColorChannels,
    pub showing_pixels_pulse: bool,
    pub showing_solid_background: bool,
    pub showing_diffuse_foreground: bool,
    pub pixel_shadow_kind: usize,
}

pub enum ColorChannels {
    Combined,
    Overlapping,
    SplitHorizontal,
    SplitVertical,
}

impl CrtFilters {
    pub fn new(change_speed: f32) -> CrtFilters {
        CrtFilters {
            blur_passes: 0,
            lines_per_pixel: 1,
            light_color: 0x00FF_FFFF,
            brightness_color: 0x00FF_FFFF,
            extra_bright: 0.0,
            extra_contrast: 1.0,
            cur_pixel_width: 1.0,
            cur_pixel_scale_x: 0.0,
            cur_pixel_scale_y: 0.0,
            cur_pixel_gap: 0.0,
            change_speed: change_speed,
            pixels_pulse: 0.0,
            pixels_geometry_kind: PixelsGeometryKind::Squares,
            pixel_shadow_kind: 1,
            color_channels: ColorChannels::Combined,
            showing_pixels_pulse: false,
            showing_diffuse_foreground: true,
            showing_solid_background: true,
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
    pub increase_camera_zoom: bool,
    pub decrease_camera_zoom: bool,
    pub reset_speeds: bool,
    pub reset_position: bool,
    pub reset_filters: bool,
    pub shift: bool,
    pub alt: bool,
    pub input_focused: bool,
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
    pub increase_contrast: bool,
    pub decrease_contrast: bool,
    pub toggle_pixels_shadow_kind: BooleanButton,
    pub speed_up: BooleanButton,
    pub speed_down: BooleanButton,
    pub mouse_click: BooleanButton,
    pub increase_blur: BooleanButton,
    pub decrease_blur: BooleanButton,
    pub increase_lpp: BooleanButton, // lines per pixel
    pub decrease_lpp: BooleanButton, // lines per pixel
    pub toggle_split_colors: BooleanButton,
    pub toggle_pixels_geometry_kind: BooleanButton,
    pub toggle_diffuse_foreground: BooleanButton,
    pub toggle_solid_background: BooleanButton,
    pub showing_pixels_pulse: BooleanButton,
    pub esc: BooleanButton,
    pub space: BooleanButton,
    pub screenshot: BooleanButton,
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
            increase_camera_zoom: false,
            decrease_camera_zoom: false,
            turn_left: false,
            turn_right: false,
            turn_up: false,
            turn_down: false,
            rotate_left: false,
            rotate_right: false,
            reset_speeds: false,
            reset_position: false,
            reset_filters: false,
            shift: false,
            alt: false,
            input_focused: false,
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
            increase_contrast: false,
            decrease_contrast: false,
            toggle_pixels_shadow_kind: BooleanButton::new(),
            speed_up: BooleanButton::new(),
            speed_down: BooleanButton::new(),
            mouse_click: BooleanButton::new(),
            increase_blur: BooleanButton::new(),
            decrease_blur: BooleanButton::new(),
            increase_lpp: BooleanButton::new(),
            decrease_lpp: BooleanButton::new(),
            toggle_split_colors: BooleanButton::new(),
            toggle_pixels_geometry_kind: BooleanButton::new(),
            toggle_diffuse_foreground: BooleanButton::new(),
            toggle_solid_background: BooleanButton::new(),
            showing_pixels_pulse: BooleanButton::new(),
            esc: BooleanButton::new(),
            space: BooleanButton::new(),
            screenshot: BooleanButton::new(),
        })
    }
}