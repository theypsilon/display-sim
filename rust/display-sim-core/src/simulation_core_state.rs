/* Copyright (c) 2019 José manuel Barroso Galindo <theypsilon@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use arraygen::Arraygen;
use enum_len_derive::EnumLen;
use num_derive::{FromPrimitive, ToPrimitive};

use crate::boolean_button::BooleanButton;
use crate::camera::{CameraChange, CameraData};
use crate::general_types::{IncDec, Size2D};
use crate::internal_resolution::InternalResolution;
use crate::pixels_shadow::ShadowShape;

pub const PIXEL_MANIPULATION_BASE_SPEED: f32 = 20.0;
pub const TURNING_BASE_SPEED: f32 = 3.0;
pub const MOVEMENT_BASE_SPEED: f32 = 10.0;
pub const MOVEMENT_SPEED_FACTOR: f32 = 50.0;

#[derive(Default, Clone)]
pub struct VideoInputResources {
    pub steps: Vec<AnimationStep>,
    pub max_texture_size: i32,
    pub image_size: Size2D<u32>,
    pub background_size: Size2D<u32>,
    pub viewport_size: Size2D<u32>,
    pub preset: Option<FiltersPreset>,
    pub current_frame: usize,
    pub last_frame_change: f64,
    pub needs_buffer_data_load: bool,
    pub drawing_activation: bool,
}

#[derive(Clone, Copy)]
pub struct AnimationStep {
    pub delay: u32,
}

// Simulation Resources
pub struct Resources {
    pub video: VideoInputResources,
    pub camera: CameraData,
    pub demo_1: FlightDemoData,
    pub filters: Filters,
    pub scaling: Scaling,
    pub speed: Speeds,
    pub saved_filters: Option<Filters>,
    pub custom_is_changed: bool,
    pub output: ViewModel,
    pub timers: SimulationTimers,
    pub initial_parameters: InitialParameters,
    pub screenshot_trigger: ScreenshotTrigger,
    pub drawable: bool,
    pub resetted: bool,
    pub quit: bool,
}

impl Default for Resources {
    fn default() -> Self {
        Resources {
            initial_parameters: InitialParameters::default(),
            timers: SimulationTimers::default(),
            video: VideoInputResources::default(),
            camera: CameraData::new(MOVEMENT_BASE_SPEED / MOVEMENT_SPEED_FACTOR, TURNING_BASE_SPEED),
            demo_1: FlightDemoData::default(),
            output: ViewModel::default(),
            speed: Speeds {
                filter_speed: PIXEL_MANIPULATION_BASE_SPEED,
            },
            filters: Filters::default(),
            scaling: Scaling::default(),
            saved_filters: None,
            custom_is_changed: false,
            screenshot_trigger: ScreenshotTrigger { is_triggered: false, delay: 0 },
            drawable: false,
            resetted: true,
            quit: false,
        }
    }
}


impl Resources {
    pub fn initialize(&mut self, video_input: VideoInputResources, now: f64) {
        self.quit = false;
        self.resetted = true;
        self.scaling.scaling_initialized = false;
        if let Some(preset) = video_input.preset {
            self.filters = self.filters.preset_factory(preset, &None);
        }
        self.timers = SimulationTimers {
            frame_count: 0,
            last_time: now,
            last_second: now,
        };
        self.video = video_input;
    }
}

#[derive(Clone)]
pub struct Scaling {
    pub pixel_width: f32,
    pub custom_resolution: Size2D<f32>,
    pub custom_aspect_ratio: Size2D<f32>,
    pub custom_stretch: bool,
    pub custom_change: LatestCustomScalingChange,
    pub scaling_initialized: bool,
    pub scaling_method: ScalingMethod,
}

impl Default for Scaling {
    fn default() -> Self {
        Scaling {
            scaling_initialized: false,
            scaling_method: ScalingMethod::AutoDetect,
            custom_resolution: Size2D { width: 256.0, height: 240.0 },
            custom_aspect_ratio: Size2D { width: 4.0, height: 3.0 },
            custom_stretch: false,
            pixel_width: 1.0,
            custom_change: LatestCustomScalingChange::AspectRatio,
        }
    }
}

#[derive(Clone, Copy)]
pub enum LatestCustomScalingChange {
    AspectRatio,
    PixelSize,
}

pub struct ScreenshotTrigger {
    pub is_triggered: bool,
    pub delay: i32,
}

pub struct FlightDemoData {
    pub camera_backup: CameraData,
    pub movement_target: glm::Vec3,
    pub movement_speed: glm::Vec3,
    pub movement_max_speed: f32,
    pub color_target: glm::Vec3,
    pub color_position: glm::Vec3,
    pub spreading: bool,
    pub needs_initialization: bool,
}

impl Default for FlightDemoData {
    fn default() -> FlightDemoData {
        FlightDemoData {
            camera_backup: CameraData::new(0.0, 0.0),
            movement_target: glm::vec3(0.0, 0.0, 0.0),
            movement_speed: glm::vec3(0.0, 0.0, 0.0),
            movement_max_speed: 0.3,
            color_target: glm::vec3(0.0, 0.0, 0.0),
            color_position: glm::vec3(0.0, 0.0, 0.0),
            spreading: true,
            needs_initialization: true,
        }
    }
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
}

pub struct Speeds {
    pub filter_speed: f32,
}

#[derive(Clone)]
pub struct Filters {
    pub internal_resolution: InternalResolution,
    pub texture_interpolation: TextureInterpolation,
    pub blur_passes: usize,
    pub vertical_lpp: usize,
    pub horizontal_lpp: usize,
    pub light_color: i32,
    pub brightness_color: i32,
    pub extra_bright: f32,
    pub extra_contrast: f32,
    pub cur_pixel_vertical_gap: f32,
    pub cur_pixel_horizontal_gap: f32,
    pub cur_pixel_spread: f32,
    pub pixel_shadow_height: f32,
    pub pixels_geometry_kind: PixelsGeometryKind,
    pub color_channels: ColorChannels,
    pub screen_curvature_kind: ScreenCurvatureKind,
    pub pixel_shadow_shape_kind: ShadowShape,
    pub backlight_presence: f32,
    pub preset_kind: FiltersPreset,
}

impl Default for Filters {
    fn default() -> Self {
        Filters {
            internal_resolution: InternalResolution::default(),
            texture_interpolation: TextureInterpolation::Linear,
            blur_passes: 0,
            vertical_lpp: 1,
            horizontal_lpp: 1,
            light_color: 0x00FF_FFFF,
            brightness_color: 0x00FF_FFFF,
            extra_bright: 0.0,
            extra_contrast: 1.0,
            cur_pixel_vertical_gap: 0.0,
            cur_pixel_horizontal_gap: 0.0,
            cur_pixel_spread: 0.0,
            pixel_shadow_height: 1.0,
            pixels_geometry_kind: PixelsGeometryKind::Squares,
            pixel_shadow_shape_kind: ShadowShape { value: 0 },
            color_channels: ColorChannels::Combined,
            screen_curvature_kind: ScreenCurvatureKind::Flat,
            backlight_presence: 0.0,
            preset_kind: FiltersPreset::Sharp1,
        }
        .preset_crt_aperture_grille_1()
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FiltersPreset {
    Sharp1,
    CrtApertureGrille1,
    CrtShadowMask1,
    CrtShadowMask2,
    DemoFlight1,
    Custom,
}

impl std::fmt::Display for FiltersPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FiltersPreset::Sharp1 => write!(f, "sharp-1"),
            FiltersPreset::CrtApertureGrille1 => write!(f, "crt-aperture-grille-1"),
            FiltersPreset::CrtShadowMask1 => write!(f, "crt-shadow-mask-1"),
            FiltersPreset::CrtShadowMask2 => write!(f, "crt-shadow-mask-2"),
            FiltersPreset::DemoFlight1 => write!(f, "demo-1"),
            FiltersPreset::Custom => write!(f, "custom"),
        }
    }
}

impl std::str::FromStr for FiltersPreset {
    type Err = String;
    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "sharp-1" => Ok(Self::Sharp1),
            "crt-aperture-grille-1" => Ok(Self::CrtApertureGrille1),
            "crt-shadow-mask-1" => Ok(Self::CrtShadowMask1),
            "crt-shadow-mask-2" => Ok(Self::CrtShadowMask2),
            "demo-1" => Ok(Self::DemoFlight1),
            "custom" => Ok(Self::Custom),
            _ => Err("Unknown name for a preset".into()),
        }
    }
}

impl FiltersPreset {
    pub fn get_description(&self) -> &str {
        match self {
            FiltersPreset::Sharp1 => "Sharp 1",
            FiltersPreset::CrtApertureGrille1 => "CRT Aperture Grille 1",
            FiltersPreset::CrtShadowMask1 => "CRT Shadow Mask 1",
            FiltersPreset::CrtShadowMask2 => "CRT Shadow Mask 2",
            FiltersPreset::DemoFlight1 => "Flight Demo",
            FiltersPreset::Custom => "Custom",
        }
    }
}

#[cfg(test)]
mod filter_presets_tests {
    use super::FiltersPreset;
    use app_error::AppResult;
    use std::str::FromStr;
    #[test]
    fn test_from_str_to_str() -> AppResult<()> {
        // @TODO ensure a way to have this array correctly updated automatically
        let presets: [FiltersPreset; 6] = [
            FiltersPreset::Sharp1,
            FiltersPreset::CrtApertureGrille1,
            FiltersPreset::CrtShadowMask1,
            FiltersPreset::CrtShadowMask2,
            FiltersPreset::DemoFlight1,
            FiltersPreset::Custom,
        ];
        for preset in presets.iter() {
            assert_eq!(FiltersPreset::from_str(preset.to_string().as_ref())?, *preset);
        }
        Ok(())
    }
}

impl Default for FiltersPreset {
    fn default() -> Self {
        Self::CrtApertureGrille1
    }
}

impl Filters {
    pub fn preset_factory(&self, preset: FiltersPreset, previous_custom: &Option<Filters>) -> Filters {
        match preset {
            FiltersPreset::Sharp1 => self.preset_sharp_1(),
            FiltersPreset::CrtApertureGrille1 => self.preset_crt_aperture_grille_1(),
            FiltersPreset::CrtShadowMask1 => self.preset_crt_shadow_mask_1(),
            FiltersPreset::CrtShadowMask2 => self.preset_crt_shadow_mask_2(),
            FiltersPreset::DemoFlight1 => self.preset_demo_1(),
            FiltersPreset::Custom => match previous_custom {
                Some(ref filter) => filter.clone(),
                None => self.preset_custom(),
            },
        }
    }
    pub fn preset_sharp_1(&self) -> Filters {
        Filters {
            internal_resolution: InternalResolution::default(),
            texture_interpolation: TextureInterpolation::Linear,
            blur_passes: 0,
            vertical_lpp: 1,
            horizontal_lpp: 1,
            light_color: 0x00FF_FFFF,
            brightness_color: 0x00FF_FFFF,
            extra_bright: 0.0,
            extra_contrast: 1.0,
            cur_pixel_vertical_gap: 0.0,
            cur_pixel_horizontal_gap: 0.0,
            cur_pixel_spread: 0.0,
            pixel_shadow_height: 1.0,
            pixels_geometry_kind: PixelsGeometryKind::Squares,
            pixel_shadow_shape_kind: ShadowShape { value: 0 },
            color_channels: ColorChannels::Combined,
            screen_curvature_kind: ScreenCurvatureKind::Flat,
            backlight_presence: 0.0,
            preset_kind: FiltersPreset::Sharp1,
        }
    }

    pub fn preset_crt_aperture_grille_1(&self) -> Filters {
        Filters {
            internal_resolution: InternalResolution::default(),
            texture_interpolation: TextureInterpolation::Linear,
            blur_passes: 1,
            vertical_lpp: 3,
            horizontal_lpp: 1,
            light_color: 0x00FF_FFFF,
            brightness_color: 0x00FF_FFFF,
            extra_bright: 0.0,
            extra_contrast: 1.0,
            cur_pixel_vertical_gap: 0.0,
            cur_pixel_horizontal_gap: 0.0,
            cur_pixel_spread: 0.0,
            pixel_shadow_height: 0.0,
            pixels_geometry_kind: PixelsGeometryKind::Squares,
            pixel_shadow_shape_kind: ShadowShape { value: 3 },
            color_channels: ColorChannels::Combined,
            screen_curvature_kind: ScreenCurvatureKind::Flat,
            backlight_presence: 0.5,
            preset_kind: FiltersPreset::CrtApertureGrille1,
        }
    }

    pub fn preset_crt_shadow_mask_1(&self) -> Filters {
        Filters {
            internal_resolution: InternalResolution::default(),
            texture_interpolation: TextureInterpolation::Linear,
            blur_passes: 2,
            vertical_lpp: 2,
            horizontal_lpp: 2,
            light_color: 0x00FF_FFFF,
            brightness_color: 0x00FF_FFFF,
            extra_bright: 0.05,
            extra_contrast: 1.2,
            cur_pixel_vertical_gap: 0.5,
            cur_pixel_horizontal_gap: 0.5,
            cur_pixel_spread: 0.0,
            pixel_shadow_height: 1.0,
            pixels_geometry_kind: PixelsGeometryKind::Squares,
            pixel_shadow_shape_kind: ShadowShape { value: 3 },
            color_channels: ColorChannels::Combined,
            screen_curvature_kind: ScreenCurvatureKind::Flat,
            backlight_presence: 0.25,
            preset_kind: FiltersPreset::CrtShadowMask1,
        }
    }

    pub fn preset_crt_shadow_mask_2(&self) -> Filters {
        Filters {
            internal_resolution: InternalResolution::default(),
            texture_interpolation: TextureInterpolation::Linear,
            blur_passes: 2,
            vertical_lpp: 1,
            horizontal_lpp: 2,
            light_color: 0x00FF_FFFF,
            brightness_color: 0x00FF_FFFF,
            extra_bright: 0.05,
            extra_contrast: 1.2,
            cur_pixel_vertical_gap: 1.0,
            cur_pixel_horizontal_gap: 0.5,
            cur_pixel_spread: 0.0,
            pixel_shadow_height: 1.0,
            pixels_geometry_kind: PixelsGeometryKind::Squares,
            pixel_shadow_shape_kind: ShadowShape { value: 3 },
            color_channels: ColorChannels::Combined,
            screen_curvature_kind: ScreenCurvatureKind::Flat,
            backlight_presence: 0.4,
            preset_kind: FiltersPreset::CrtShadowMask2,
        }
    }

    pub fn preset_demo_1(&self) -> Self {
        Filters {
            internal_resolution: InternalResolution::default(),
            texture_interpolation: TextureInterpolation::Linear,
            blur_passes: 0,
            vertical_lpp: 1,
            horizontal_lpp: 1,
            light_color: self.light_color,
            brightness_color: 0x00FF_FFFF,
            extra_bright: 0.0,
            extra_contrast: 1.0,
            cur_pixel_vertical_gap: 0.0,
            cur_pixel_horizontal_gap: 0.0,
            cur_pixel_spread: 1.0,
            pixel_shadow_height: 1.0,
            pixels_geometry_kind: PixelsGeometryKind::Cubes,
            pixel_shadow_shape_kind: ShadowShape { value: 0 },
            color_channels: ColorChannels::Combined,
            screen_curvature_kind: ScreenCurvatureKind::Pulse,
            backlight_presence: 0.2,
            preset_kind: FiltersPreset::DemoFlight1,
        }
    }

    pub fn preset_custom(&self) -> Self {
        let mut clone = self.clone();
        clone.preset_kind = FiltersPreset::Custom;
        clone
    }
}

#[derive(Default)]
pub struct ViewModel {
    pub screen_curvature_factor: f32,
    pub pixels_pulse: f32,
    pub color_splits: usize,
    pub light_color: [[f32; 3]; 3],
    pub light_color_background: [f32; 3],
    pub extra_light: [f32; 3],
    pub ambient_strength: f32,
    pub pixel_have_depth: bool,
    pub pixel_spread: [f32; 2],
    pub pixel_scale_base: [f32; 3],
    pub height_modifier_factor: f32,
    pub pixel_scale_foreground: Vec<[[f32; 3]; 3]>,
    pub pixel_offset_foreground: Vec<[[f32; 3]; 3]>,
    pub pixel_scale_background: Vec<[f32; 3]>,
    pub pixel_offset_background: Vec<[f32; 3]>,
    pub showing_background: bool,
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

#[derive(FromPrimitive, ToPrimitive, EnumLen, Clone, Copy)]
pub enum PixelsGeometryKind {
    Squares,
    Cubes,
}

impl std::fmt::Display for PixelsGeometryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            PixelsGeometryKind::Squares => write!(f, "Squares"),
            PixelsGeometryKind::Cubes => write!(f, "Cubes"),
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

#[derive(FromPrimitive, ToPrimitive, EnumLen, Copy, Clone)]
pub enum ScalingMethod {
    AutoDetect,
    SquaredPixels,
    FullImage4By3,
    StretchToBothEdges,
    StretchToNearestEdge,
    Custom,
}

impl std::fmt::Display for ScalingMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ScalingMethod::AutoDetect => write!(f, "Automatic"),
            ScalingMethod::SquaredPixels => write!(f, "Squared pixels"),
            ScalingMethod::FullImage4By3 => write!(f, "4:3 on full image"),
            ScalingMethod::StretchToBothEdges => write!(f, "Stretch to both edges"),
            ScalingMethod::StretchToNearestEdge => write!(f, "Stretch to nearest edge"),
            ScalingMethod::Custom => write!(f, "Custom"),
        }
    }
}

pub mod frontend_event {
    pub const KEYBOARD: &str = "front2back:keyboard";
    pub const MOUSE_CLICK: &str = "front2back:mouse-click";
    pub const MOUSE_MOVE: &str = "front2back:mouse-move";
    pub const MOUSE_WHEEL: &str = "front2back:mouse-wheel";
    pub const BLURRED_WINDOW: &str = "front2back:blurred-window";

    pub const FILTER_PRESET: &str = "front2back:filter-presets-selected";
    pub const PIXEL_BRIGHTNESS: &str = "front2back:pixel-brightness";
    pub const PIXEL_CONTRAST: &str = "front2back:pixel-contrast";
    pub const LIGHT_COLOR: &str = "front2back:light-color";
    pub const BRIGHTNESS_COLOR: &str = "front2back:brightness-color";
    pub const BLUR_LEVEL: &str = "front2back:blur-level";
    pub const VERTICAL_LPP: &str = "front2back:vertical-lpp";
    pub const HORIZONTAL_LPP: &str = "front2back:horizontal-lpp";
    pub const PIXEL_SHADOW_HEIGHT: &str = "front2back:pixel-shadow-height";
    pub const PIXEL_VERTICAL_GAP: &str = "front2back:pixel-vertical-gap";
    pub const PIXEL_HORIZONTAL_GAP: &str = "front2back:pixel-horizontal-gap";
    pub const PIXEL_WIDTH: &str = "front2back:pixel-width";
    pub const PIXEL_SPREAD: &str = "front2back:pixel-spread";
    pub const BACKLIGHT_PERCENT: &str = "front2back:backlight-percent";
    pub const CAMERA_ZOOM: &str = "front2back:camera_zoom";
    pub const CAMERA_POS_X: &str = "front2back:camera-pos-x";
    pub const CAMERA_POS_Y: &str = "front2back:camera-pos-y";
    pub const CAMERA_POS_Z: &str = "front2back:camera-pos-z";
    pub const CAMERA_AXIS_UP_X: &str = "front2back:camera-axis-up-x";
    pub const CAMERA_AXIS_UP_Y: &str = "front2back:camera-axis-up-y";
    pub const CAMERA_AXIS_UP_Z: &str = "front2back:camera-axis-up-z";
    pub const CAMERA_DIRECTION_X: &str = "front2back:camera-dir-x";
    pub const CAMERA_DIRECTION_Y: &str = "front2back:camera-dir-y";
    pub const CAMERA_DIRECTION_Z: &str = "front2back:camera-dir-z";

    pub const CUSTOM_SCALING_RESOLUTION_WIDTH: &str = "front2back:custom-scaling-resolution-width";
    pub const CUSTOM_SCALING_RESOLUTION_HEIGHT: &str = "front2back:custom-scaling-resolution-height";
    pub const CUSTOM_SCALING_ASPECT_RATIO_X: &str = "front2back:custom-scaling-aspect-ratio-x";
    pub const CUSTOM_SCALING_ASPECT_RATIO_Y: &str = "front2back:custom-scaling-aspect-ratio-y";
    pub const CUSTOM_SCALING_STRETCH_NEAREST: &str = "front2back:custom-scaling-stretch-nearest";
    pub const VIEWPORT_RESIZE: &str = "front2back:viewport-resize";
}

#[derive(Clone, Debug)]
pub enum InputEventValue {
    None,

    Keyboard { pressed: bool, key: String },
    MouseClick(bool),
    MouseMove { x: i32, y: i32 },
    MouseWheel(f32),
    BlurredWindow,

    FilterPreset(String),
    PixelBrighttness(f32),
    PixelContrast(f32),
    LightColor(i32),
    BrightnessColor(i32),
    BlurLevel(usize),
    VerticalLpp(usize),
    HorizontalLpp(usize),
    BacklightPercent(f32),
    PixelShadowHeight(f32),
    PixelVerticalGap(f32),
    PixelHorizontalGap(f32),
    PixelWidth(f32),
    PixelSpread(f32),
    Camera(CameraChange),
    CustomScalingResolutionWidth(f32),
    CustomScalingResolutionHeight(f32),
    CustomScalingAspectRatioX(f32),
    CustomScalingAspectRatioY(f32),
    CustomScalingStretchNearest(bool),
    ViewportResize(u32, u32),
}

pub struct CustomInputEvent {
    values: Vec<InputEventValue>,
}

impl CustomInputEvent {
    pub fn add_value(&mut self, value: InputEventValue) {
        self.values.push(value);
    }

    pub fn reset(&mut self) {
        self.values.resize(0, InputEventValue::None);
    }

    pub fn consume_values(&mut self) -> Vec<InputEventValue> {
        std::mem::replace(&mut self.values, vec![])
    }
}

impl Default for CustomInputEvent {
    fn default() -> Self {
        CustomInputEvent { values: vec![] }
    }
}

pub trait SetOptionNone {
    fn set_none(&mut self);
}
impl<T> SetOptionNone for Option<T> {
    fn set_none(&mut self) {
        *self = None;
    }
}
pub trait TrackedButton {
    fn track(&mut self);
}
impl TrackedButton for BooleanButton {
    fn track(&mut self) {
        self.track_input();
    }
}
impl TrackedButton for IncDec<BooleanButton> {
    fn track(&mut self) {
        self.get_buttons().iter_mut().for_each(|button| button.track_input());
    }
}

#[derive(Default, Arraygen)]
#[gen_array(pub fn get_options_to_be_noned: &mut dyn SetOptionNone)]
#[gen_array(pub fn get_tracked_buttons: &mut dyn TrackedButton)]
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
    pub control: bool,
    pub alt: bool,
    pub input_focused: bool,
    pub canvas_focused: bool,
    pub mouse_position_x: i32,
    pub mouse_position_y: i32,
    pub mouse_scroll_y: f32,
    pub pixel_horizontal_gap: IncDec<bool>,
    pub pixel_vertical_gap: IncDec<bool>,
    pub pixel_width: IncDec<bool>,
    pub pixel_spread: IncDec<bool>,
    pub bright: IncDec<bool>,
    pub contrast: IncDec<bool>,
    pub backlight_percent: IncDec<bool>,
    #[in_array(get_tracked_buttons)]
    pub next_camera_movement_mode: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub translation_speed: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub turn_speed: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub filter_speed: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub mouse_click: BooleanButton,
    #[in_array(get_tracked_buttons)]
    pub blur: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub vertical_lpp: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub horizontal_lpp: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub next_pixel_shadow_shape_kind: IncDec<BooleanButton>,
    pub next_pixels_shadow_height: IncDec<bool>,
    #[in_array(get_tracked_buttons)]
    pub next_color_representation_kind: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub next_pixel_geometry_kind: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub next_screen_curvature_type: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub next_internal_resolution: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub next_texture_interpolation: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub scaling_method: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub scaling_resolution_width: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub scaling_resolution_height: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub scaling_aspect_ratio_x: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub scaling_aspect_ratio_y: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub esc: BooleanButton,
    #[in_array(get_tracked_buttons)]
    pub space: BooleanButton,
    #[in_array(get_tracked_buttons)]
    pub screenshot: BooleanButton,

    #[in_array(get_options_to_be_noned)]
    pub event_filter_preset: Option<String>,
    #[in_array(get_options_to_be_noned)]
    pub event_pixel_brighttness: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub event_pixel_contrast: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub event_light_color: Option<i32>,
    #[in_array(get_options_to_be_noned)]
    pub event_brightness_color: Option<i32>,
    #[in_array(get_options_to_be_noned)]
    pub event_blur_level: Option<usize>,
    #[in_array(get_options_to_be_noned)]
    pub event_vertical_lpp: Option<usize>,
    #[in_array(get_options_to_be_noned)]
    pub event_horizontal_lpp: Option<usize>,
    #[in_array(get_options_to_be_noned)]
    pub event_scaling_resolution_width: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub event_scaling_resolution_height: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub event_scaling_aspect_ratio_x: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub event_scaling_aspect_ratio_y: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub event_custom_scaling_stretch_nearest: Option<bool>,
    #[in_array(get_options_to_be_noned)]
    pub event_backlight_percent: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub event_pixel_shadow_height: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub event_pixel_vertical_gap: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub event_pixel_horizontal_gap: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub event_pixel_width: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub event_pixel_spread: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub event_viewport_resize: Option<Size2D<u32>>,
    #[in_array(get_options_to_be_noned)]
    pub event_camera: Option<CameraChange>,
}

impl Input {
    pub fn new(now: f64) -> Input {
        let mut input: Input = Input::default();
        input.now = now;
        input
    }
}
