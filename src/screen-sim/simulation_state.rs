use js_sys::ArrayBuffer;
use wasm_bindgen::prelude::JsValue;
use web_sys::WebGl2RenderingContext;

use enum_len_derive::EnumLen;
use getters_by_type::GettersMutByType;
use num_derive::{FromPrimitive, ToPrimitive};

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

pub const PIXEL_MANIPULATION_BASE_SPEED: f32 = 20.0;
pub const TURNING_BASE_SPEED: f32 = 3.0;
pub const MOVEMENT_BASE_SPEED: f32 = 10.0;
pub const MOVEMENT_SPEED_FACTOR: f32 = 50.0;

#[derive(Default)]
pub struct AnimationData {
    pub steps: Vec<AnimationStep>,
    pub image_width: u32,
    pub image_height: u32,
    pub background_width: u32,
    pub background_height: u32,
    pub viewport_width: u32,
    pub viewport_height: u32,
    pub pixel_width: f32,
    pub stretch: bool,
    pub current_frame: usize,
    pub last_frame_change: f64,
    pub needs_buffer_data_load: bool,
}

pub struct AnimationStep {
    pub buffer: ArrayBuffer,
    pub delay: u32,
}

// Simulation Resources
pub struct Resources {
    pub animation: AnimationData,
    pub camera: Camera,
    pub crt_filters: CrtFilters,
    pub timers: SimulationTimers,
    pub initial_parameters: InitialParameters,
    pub launch_screenshot: bool,
    pub screenshot_delay: i32,
    pub resetted: bool,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            resetted: true,
            initial_parameters: InitialParameters::default(),
            timers: SimulationTimers::default(),
            animation: AnimationData::default(),
            camera: Camera::new(MOVEMENT_BASE_SPEED / MOVEMENT_SPEED_FACTOR, TURNING_BASE_SPEED),
            crt_filters: CrtFilters::new(PIXEL_MANIPULATION_BASE_SPEED),
            launch_screenshot: false,
            screenshot_delay: 0,
        }
    }
}

// Rendering Materials
pub struct Materials {
    pub gl: WebGl2RenderingContext,
    pub main_buffer_stack: TextureBufferStack,
    pub bg_buffer_stack: TextureBufferStack,
    pub pixels_render: PixelsRender,
    pub blur_render: BlurRender,
    pub background_render: BackgroundRender,
    pub internal_resolution_render: InternalResolutionRender,
    pub rgb_render: RgbRender,
}

#[derive(Default)]
pub struct SimulationTimers {
    pub frame_count: u32,
    pub last_time: f64,
    pub last_second: f64,
}

#[derive(Default)]
pub struct InitialParameters {
    pub initial_movement_speed: f32,
    pub initial_position_z: f32,
    pub initial_pixel_width: f32,
}

pub struct CrtFilters {
    pub internal_resolution: InternalResolution,
    pub texture_interpolation: TextureInterpolation,
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
    pub pixel_shadow_height_factor: f32,
    pub pixels_geometry_kind: PixelsGeometryKind,
    pub color_channels: ColorChannels,
    pub screen_curvature_kind: ScreenCurvatureKind,
    pub showing_solid_background: bool,
    pub showing_diffuse_foreground: bool,
    pub solid_color_weight: f32,
    pub pixel_shadow_shape_kind: usize,
    pub layering_kind: ScreenLayeringKind,
}

#[derive(FromPrimitive, ToPrimitive, EnumLen, Copy, Clone)]
pub enum ScreenLayeringKind {
    ShadowOnly,
    ShadowWithSolidBackground25,
    ShadowWithSolidBackground50,
    ShadowWithSolidBackground75,
    DiffuseOnly,
    SolidOnly,
}

impl std::fmt::Display for ScreenLayeringKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ScreenLayeringKind::ShadowOnly => write!(f, "Lines only"),
            ScreenLayeringKind::ShadowWithSolidBackground75 => write!(f, "+75% BL"),
            ScreenLayeringKind::ShadowWithSolidBackground50 => write!(f, "+50% BL"),
            ScreenLayeringKind::ShadowWithSolidBackground25 => write!(f, "+25% BL"),
            ScreenLayeringKind::DiffuseOnly => write!(f, "Backlight only"),
            ScreenLayeringKind::SolidOnly => write!(f, "Disabled"),
        }
    }
}

pub struct InternalResolution {
    pub multiplier: f64,
    pub minimum_reached: bool,
    backup_multiplier: f64,
}

impl InternalResolution {
    pub fn new(multiplier: f64) -> InternalResolution {
        InternalResolution {
            multiplier,
            minimum_reached: false,
            backup_multiplier: multiplier,
        }
    }
    pub fn increase(&mut self, animation: &AnimationData) {
        self.minimum_reached = false;
        let height = self.height(animation);
        if height == 720 {
            self.multiplier = self.backup_multiplier;
        } else if height == 486 {
            self.multiplier = 720.0 / animation.viewport_height as f64;
        } else if height == 480 {
            self.multiplier = 486.0 / animation.viewport_height as f64;
        } else if height == 243 {
            self.multiplier = 480.0 / animation.viewport_height as f64;
        } else if height == 240 {
            self.multiplier = 243.0 / animation.viewport_height as f64;
        } else if height == 224 {
            self.multiplier = 240.0 / animation.viewport_height as f64;
        } else if height == 160 {
            self.multiplier = 224.0 / animation.viewport_height as f64;
        } else if height == 152 {
            self.multiplier = 160.0 / animation.viewport_height as f64;
        } else if height == 144 {
            self.multiplier = 152.0 / animation.viewport_height as f64;
        } else if height == 102 {
            self.multiplier = 144.0 / animation.viewport_height as f64;
        } else if height == 51 {
            self.multiplier = 102.0 / animation.viewport_height as f64;
        } else {
            self.multiplier *= 2.0;
        }
    }
    pub fn decrease(&mut self, animation: &AnimationData) {
        let height = self.height(animation);
        if height <= 1440 && height > 720 {
            self.backup_multiplier = self.multiplier;
            self.multiplier = 720.0 / animation.viewport_height as f64;
        } else if height == 720 {
            self.multiplier = 486.0 / animation.viewport_height as f64;
        } else if height == 486 {
            self.multiplier = 480.0 / animation.viewport_height as f64;
        } else if height == 480 {
            self.multiplier = 243.0 / animation.viewport_height as f64;
        } else if height == 243 {
            self.multiplier = 240.0 / animation.viewport_height as f64;
        } else if height == 240 {
            self.multiplier = 224.0 / animation.viewport_height as f64;
        } else if height == 224 {
            self.multiplier = 160.0 / animation.viewport_height as f64;
        } else if height == 160 {
            self.multiplier = 152.0 / animation.viewport_height as f64;
        } else if height == 152 {
            self.multiplier = 144.0 / animation.viewport_height as f64;
        } else if height == 144 {
            self.multiplier = 102.0 / animation.viewport_height as f64;
        } else {
            self.multiplier /= 2.0;
            if self.height(animation) < 2 {
                self.increase(animation);
                self.minimum_reached = true;
            }
        }
    }
    pub fn width(&self, animation: &AnimationData) -> i32 {
        (animation.viewport_width as f64 * self.multiplier) as i32
    }
    pub fn height(&self, animation: &AnimationData) -> i32 {
        (animation.viewport_height as f64 * self.multiplier) as i32
    }
    pub fn to_label(&self, animation: &AnimationData) -> String {
        let height = self.height(animation);
        if height <= 1080 {
            format!("{}p", height)
        } else {
            format!("{}K", height / 540)
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, EnumLen, Copy, Clone)]
pub enum ScreenCurvatureKind {
    Flat,
    Curved1,
    Curved2,
    Curved3,
    Pulse,
}

impl std::fmt::Display for ScreenCurvatureKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ScreenCurvatureKind::Flat => write!(f, "Flat"),
            ScreenCurvatureKind::Curved1 => write!(f, "Curved 1"),
            ScreenCurvatureKind::Curved2 => write!(f, "Curved 2"),
            ScreenCurvatureKind::Curved3 => write!(f, "Curved 3"),
            ScreenCurvatureKind::Pulse => write!(f, "Weavy"),
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, EnumLen, Copy, Clone)]
pub enum TextureInterpolation {
    Nearest,
    Linear,
}

impl std::fmt::Display for TextureInterpolation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TextureInterpolation::Nearest => write!(f, "Nearest"),
            TextureInterpolation::Linear => write!(f, "Linear"),
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, EnumLen, Copy, Clone)]
pub enum ColorChannels {
    Combined,
    Overlapping,
    SplitHorizontal,
    SplitVertical,
}

impl std::fmt::Display for ColorChannels {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ColorChannels::Combined => write!(f, "Combined"),
            ColorChannels::Overlapping => write!(f, "Horizontal overlapping"),
            ColorChannels::SplitHorizontal => write!(f, "Horizontal split"),
            ColorChannels::SplitVertical => write!(f, "Vertical split"),
        }
    }
}

impl CrtFilters {
    pub fn new(change_speed: f32) -> CrtFilters {
        CrtFilters {
            internal_resolution: InternalResolution::new(1.0),
            texture_interpolation: TextureInterpolation::Linear,
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
            pixel_shadow_height_factor: 0.5,
            change_speed,
            pixels_pulse: 0.0,
            pixels_geometry_kind: PixelsGeometryKind::Squares,
            pixel_shadow_shape_kind: 1,
            color_channels: ColorChannels::Combined,
            screen_curvature_kind: ScreenCurvatureKind::Flat,
            showing_diffuse_foreground: true,
            showing_solid_background: true,
            solid_color_weight: 0.75,
            layering_kind: ScreenLayeringKind::ShadowWithSolidBackground50,
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

#[derive(Clone, Default, GettersMutByType)]
pub struct IncDec<T> {
    pub increase: T,
    pub decrease: T,
}

impl IncDec<BooleanButton> {
    pub fn any_just_pressed(&self) -> bool {
        self.increase.is_just_pressed() || self.decrease.is_just_pressed()
    }
    pub fn any_just_released(&self) -> bool {
        self.increase.is_just_released() || self.decrease.is_just_released()
    }
}

impl IncDec<bool> {
    pub fn any_active(&self) -> bool {
        self.increase || self.decrease
    }
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

#[derive(Default, GettersMutByType)]
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
    pub translation_speed: IncDec<BooleanButton>,
    pub turn_speed: IncDec<BooleanButton>,
    pub filter_speed: IncDec<BooleanButton>,
    pub mouse_click: BooleanButton,
    pub blur: IncDec<BooleanButton>,
    pub lpp: IncDec<BooleanButton>,
    pub next_pixels_shadow_shape_kind: IncDec<BooleanButton>,
    pub next_pixels_shadow_height_factor: IncDec<bool>,
    pub next_color_representation_kind: IncDec<BooleanButton>,
    pub next_pixel_geometry_kind: IncDec<BooleanButton>,
    pub next_layering_kind: IncDec<BooleanButton>,
    pub next_screen_curvature_type: IncDec<BooleanButton>,
    pub next_internal_resolution: IncDec<BooleanButton>,
    pub next_texture_interpolation: IncDec<BooleanButton>,
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
