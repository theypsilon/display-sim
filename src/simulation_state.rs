use js_sys::ArrayBuffer;
use wasm_bindgen::prelude::{Closure, JsValue};

use num_derive::FromPrimitive;
use variant_count::VariantCount;

use std::cell::RefCell;
use std::rc::Rc;

use crate::background_render::BackgroundRender;
use crate::blur_render::BlurRender;
use crate::boolean_button::BooleanButton;
use crate::camera::Camera;
use crate::internal_resolution_render::InternalResolutionRender;
use crate::pixels_render::{PixelsGeometryKind, PixelsRender};
use crate::render_types::TextureBufferStack;
use crate::rgb_render::RgbRender;
use crate::wasm_error::WasmResult;
use crate::web_utils::now;

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
    pub screen_curvature_kind: ScreenCurvatureKind,
    pub showing_solid_background: bool,
    pub showing_diffuse_foreground: bool,
    pub solid_color_weight: f32,
    pub pixel_shadow_kind: usize,
    pub layering_kind: ScreenLayeringKind,
}

#[derive(FromPrimitive, Copy, Clone, VariantCount)]
pub enum ScreenLayeringKind {
    ShadowOnly = 0,
    SolidOnly = 1,
    ShadowWithSolidBackground75 = 2,
    ShadowWithSolidBackground50 = 3,
    ShadowWithSolidBackground25 = 4,
}

#[derive(FromPrimitive, Copy, Clone, VariantCount)]
pub enum ScreenCurvatureKind {
    Flat = 0,
    Curved = 1,
    Pulse = 2,
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
            change_speed,
            pixels_pulse: 0.0,
            pixels_geometry_kind: PixelsGeometryKind::Squares,
            pixel_shadow_kind: 1,
            color_channels: ColorChannels::Combined,
            screen_curvature_kind: ScreenCurvatureKind::Curved,
            showing_diffuse_foreground: true,
            showing_solid_background: true,
            solid_color_weight: 0.75,
            layering_kind: ScreenLayeringKind::ShadowOnly,
        }
    }
}

pub struct CustomInputEvent {
    pub value: JsValue,
    pub kind: String,
}

impl Default for CustomInputEvent {
    fn default() -> Self {
        CustomInputEvent {
            value: JsValue::undefined(),
            kind: String::new(),
        }
    }
}

#[derive(Clone, Default)]
pub struct IncDec<T> {
    pub increase: T,
    pub decrease: T,
}

pub trait DefaultReset {
    fn reset(&mut self)
    where
        Self: std::marker::Sized + std::default::Default,
    {
        std::mem::swap(self, &mut Self::default());
    }
}

impl<T> DefaultReset for IncDec<T> where T: std::marker::Sized + std::default::Default {}

#[derive(Default)]
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
    pub camera_zoom: IncDec<bool>,
    pub reset_speeds: bool,
    pub reset_position: bool,
    pub reset_filters: bool,
    pub shift: bool,
    pub alt: bool,
    pub input_focused: bool,
    pub mouse_position_x: i32,
    pub mouse_position_y: i32,
    pub mouse_scroll_y: f32,
    pub pixel_scale_y: IncDec<bool>,
    pub pixel_scale_x: IncDec<bool>,
    pub pixel_width: IncDec<bool>,
    pub pixel_gap: IncDec<bool>,
    pub bright: IncDec<bool>,
    pub contrast: IncDec<bool>,
    pub toggle_pixels_shadow_kind: BooleanButton,
    pub translation_speed: IncDec<BooleanButton>,
    pub turn_speed: IncDec<BooleanButton>,
    pub filter_speed: IncDec<BooleanButton>,
    pub mouse_click: BooleanButton,
    pub blur: IncDec<BooleanButton>,
    pub lpp: IncDec<BooleanButton>,
    pub next_color_representation_kind: BooleanButton,
    pub next_pixel_geometry_kind: BooleanButton,
    pub next_layering_kind: BooleanButton,
    pub next_screen_curvature_type: BooleanButton,
    pub esc: BooleanButton,
    pub space: BooleanButton,
    pub screenshot: BooleanButton,
}

impl Input {
    pub fn new() -> WasmResult<Input> {
        let mut input: Input = Input::default();
        input.now = now()?;
        Ok(input)
    }
}
