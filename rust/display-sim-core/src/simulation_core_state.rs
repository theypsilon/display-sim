/* Copyright (c) 2019-2022 José manuel Barroso Galindo <theypsilon@gmail.com>
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

use std::collections::HashMap;

use arraygen::Arraygen;
use enum_len_derive::EnumLen;
use num_derive::{FromPrimitive, ToPrimitive};

use crate::camera::CameraData;
use crate::general_types::Size2D;
use crate::ui_controller::{
    backlight_percent::BacklightPercent,
    blur_passes::BlurPasses,
    brightness_color::BrightnessColor,
    color_channels::{ColorChannels, ColorChannelsOptions},
    color_gamma::ColorGamma,
    color_noise::ColorNoise,
    cur_pixel_horizontal_gap::CurPixelHorizontalGap,
    cur_pixel_spread::CurPixelSpread,
    cur_pixel_vertical_gap::CurPixelVerticalGap,
    extra_bright::ExtraBright,
    extra_contrast::ExtraContrast,
    filter_preset::{FilterPreset, FilterPresetOptions},
    horizontal_lpp::HorizontalLpp,
    internal_resolution::InternalResolution,
    light_color::LightColor,
    pixel_geometry_kind::{PixelGeometryKind, PixelGeometryKindOptions},
    pixel_shadow_height::PixelShadowHeight,
    pixel_shadow_shape_kind::{PixelShadowShapeKind, ShadowShape},
    rgb_calibration::{RgbBlueB, RgbBlueG, RgbBlueR, RgbGreenB, RgbGreenG, RgbGreenR, RgbRedB, RgbRedG, RgbRedR},
    screen_curvature_kind::{ScreenCurvatureKind, ScreenCurvatureKindOptions},
    texture_interpolation::{TextureInterpolation, TextureInterpolationOptions},
    vertical_lpp::VerticalLpp,
    UiController,
};

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
    pub preset: Option<FilterPresetOptions>,
    pub current_frame: usize,
    pub last_frame_change: f64,
    pub needs_buffer_data_load: bool,
    pub drawing_activation: bool,
}

#[derive(Clone, Copy)]
pub struct AnimationStep {
    pub delay: u32,
}

pub enum KeyEventKind {
    Inc,
    Dec,
    Set,
}

#[derive(Default)]
pub struct MainState {
    pub dt: f32,
    pub filter_speed: f32,
    pub current_filter_preset: FilterPresetOptions,
    pub render: ViewModel,
}

// Simulation Resources
pub struct Resources {
    pub video: VideoInputResources,
    pub camera: CameraData,
    pub demo_1: FlightDemoData,
    pub controllers: Controllers,
    pub scaling: Scaling,
    pub speed: Speeds,
    pub saved_filters: Option<Controllers>,
    pub custom_is_changed: bool,
    pub main: MainState,
    pub timers: SimulationTimers,
    pub initial_parameters: InitialParameters,
    pub screenshot_trigger: ScreenshotTrigger,
    pub drawable: bool,
    pub resetted: bool,
    pub quit: bool,
    pub controller_events: HashMap<&'static str, (KeyEventKind, usize)>,
}

impl Default for Resources {
    fn default() -> Self {
        let mut controllers = Controllers::default();
        Resources {
            initial_parameters: InitialParameters::default(),
            timers: SimulationTimers::default(),
            video: VideoInputResources::default(),
            camera: CameraData::new(MOVEMENT_BASE_SPEED / MOVEMENT_SPEED_FACTOR, TURNING_BASE_SPEED),
            demo_1: FlightDemoData::default(),
            speed: Speeds {
                filter_speed: PIXEL_MANIPULATION_BASE_SPEED,
            },
            scaling: Scaling::default(),
            saved_filters: None,
            custom_is_changed: false,
            screenshot_trigger: ScreenshotTrigger { is_triggered: false, delay: 0 },
            drawable: false,
            resetted: true,
            quit: false,
            controller_events: {
                let mut map: HashMap<&'static str, (KeyEventKind, usize)> = HashMap::new();
                for (i, controller) in controllers.get_ui_controllers_mut().iter().enumerate() {
                    for key in controller.keys_dec() {
                        if map.contains_key(key) {
                            panic!("controller_events panic! key_dec already included '{}'.", key);
                        }
                        map.insert(*key, (KeyEventKind::Dec, i));
                    }
                    for key in controller.keys_inc() {
                        if map.contains_key(key) {
                            panic!("controller_events panic! keys_inc already included '{}'.", key);
                        }
                        map.insert(*key, (KeyEventKind::Inc, i));
                    }
                    let event_tag = controller.event_tag();
                    if event_tag.is_empty() {
                        continue;
                    }
                    if map.contains_key(event_tag) {
                        panic!("controller_events panic! event_tag already included '{}'.", event_tag);
                    }
                    map.insert(event_tag, (KeyEventKind::Set, i));
                }
                map
            },
            main: Default::default(),
            controllers,
        }
    }
}

impl Resources {
    pub fn initialize(&mut self, video_input: VideoInputResources, now: f64) {
        self.quit = false;
        self.resetted = true;
        self.scaling.scaling_initialized = false;
        if let Some(preset) = video_input.preset {
            self.controllers.preset_factory(preset, &None);
        }
        self.timers = SimulationTimers {
            frame_count: 0,
            last_time: now,
            last_second: now,
        };
        self.video = video_input;
        for controller in self.controllers.get_ui_controllers_mut().iter_mut() {
            controller.reset_inputs();
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

#[derive(Clone, Arraygen)]
#[gen_array(pub fn get_ui_controllers: &dyn UiController, implicit_select_all: _)]
#[gen_array(pub fn get_ui_controllers_mut: &mut dyn UiController, implicit_select_all: _)]
pub struct Controllers {
    pub internal_resolution: InternalResolution,
    pub texture_interpolation: TextureInterpolation,
    pub blur_passes: BlurPasses,
    pub vertical_lpp: VerticalLpp,
    pub horizontal_lpp: HorizontalLpp,
    pub light_color: LightColor,
    pub brightness_color: BrightnessColor,
    pub extra_bright: ExtraBright,
    pub extra_contrast: ExtraContrast,
    pub cur_pixel_vertical_gap: CurPixelVerticalGap,
    pub cur_pixel_horizontal_gap: CurPixelHorizontalGap,
    pub cur_pixel_spread: CurPixelSpread,
    pub pixel_shadow_height: PixelShadowHeight,
    pub pixels_geometry_kind: PixelGeometryKind,
    pub color_channels: ColorChannels,
    pub screen_curvature_kind: ScreenCurvatureKind,
    pub pixel_shadow_shape_kind: PixelShadowShapeKind,
    pub backlight_percent: BacklightPercent,
    pub rgb_red_r: RgbRedR,
    pub rgb_red_g: RgbRedG,
    pub rgb_red_b: RgbRedB,
    pub rgb_green_r: RgbGreenR,
    pub rgb_green_g: RgbGreenG,
    pub rgb_green_b: RgbGreenB,
    pub rgb_blue_r: RgbBlueR,
    pub rgb_blue_g: RgbBlueG,
    pub rgb_blue_b: RgbBlueB,
    pub color_gamma: ColorGamma,
    pub color_noise: ColorNoise,
    pub preset_kind: FilterPreset,
}

impl Default for Controllers {
    fn default() -> Self {
        let mut controllers = Controllers {
            internal_resolution: InternalResolution::default(),
            texture_interpolation: TextureInterpolationOptions::Linear.into(),
            blur_passes: 0.into(),
            vertical_lpp: 1.into(),
            horizontal_lpp: 1.into(),
            light_color: 0x00FF_FFFF.into(),
            brightness_color: 0x00FF_FFFF.into(),
            extra_bright: 0.0.into(),
            extra_contrast: 1.0.into(),
            cur_pixel_vertical_gap: 0.0.into(),
            cur_pixel_horizontal_gap: 0.0.into(),
            cur_pixel_spread: 0.0.into(),
            pixel_shadow_height: 1.0.into(),
            pixels_geometry_kind: PixelGeometryKindOptions::Squares.into(),
            pixel_shadow_shape_kind: ShadowShape { value: 0 }.into(),
            color_channels: ColorChannelsOptions::Combined.into(),
            screen_curvature_kind: ScreenCurvatureKindOptions::Flat.into(),
            backlight_percent: 0.0.into(),
            rgb_red_r: 1.0.into(),
            rgb_red_g: 0.0.into(),
            rgb_red_b: 0.0.into(),
            rgb_green_r: 0.0.into(),
            rgb_green_g: 1.0.into(),
            rgb_green_b: 0.0.into(),
            rgb_blue_r: 0.0.into(),
            rgb_blue_g: 0.0.into(),
            rgb_blue_b: 1.0.into(),
            color_gamma: 1.0.into(),
            color_noise: 0.0.into(),
            preset_kind: FilterPresetOptions::Sharp1.into(),
        };
        controllers.preset_crt_aperture_grille_1();
        controllers
    }
}

impl Controllers {
    pub fn preset_factory(&mut self, preset: FilterPresetOptions, previous_custom: &Option<Controllers>) {
        match preset {
            FilterPresetOptions::Sharp1 => self.preset_sharp_1(),
            FilterPresetOptions::CrtApertureGrille1 => self.preset_crt_aperture_grille_1(),
            FilterPresetOptions::CrtShadowMask1 => self.preset_crt_shadow_mask_1(),
            FilterPresetOptions::CrtShadowMask2 => self.preset_crt_shadow_mask_2(),
            FilterPresetOptions::DemoFlight1 => self.preset_demo_1(),
            FilterPresetOptions::Custom => match previous_custom {
                Some(_) => {}
                None => self.preset_custom(),
            },
        }
    }
    pub fn preset_sharp_1(&mut self) {
        self.internal_resolution = InternalResolution::default();
        self.texture_interpolation = TextureInterpolationOptions::Linear.into();
        self.blur_passes = 0.into();
        self.vertical_lpp = 1.into();
        self.horizontal_lpp = 1.into();
        self.light_color = 0x00FF_FFFF.into();
        self.brightness_color = 0x00FF_FFFF.into();
        self.extra_bright = 0.0.into();
        self.extra_contrast = 1.0.into();
        self.cur_pixel_vertical_gap = 0.0.into();
        self.cur_pixel_horizontal_gap = 0.0.into();
        self.cur_pixel_spread = 0.0.into();
        self.pixel_shadow_height = 1.0.into();
        self.pixels_geometry_kind = PixelGeometryKindOptions::Squares.into();
        self.pixel_shadow_shape_kind = ShadowShape { value: 0 }.into();
        self.color_channels = ColorChannelsOptions::Combined.into();
        self.screen_curvature_kind = ScreenCurvatureKindOptions::Flat.into();
        self.backlight_percent.value = 0.0;
        self.preset_kind = FilterPresetOptions::Sharp1.into();
    }

    pub fn preset_crt_aperture_grille_1(&mut self) {
        self.internal_resolution = InternalResolution::default();
        self.texture_interpolation = TextureInterpolationOptions::Linear.into();
        self.blur_passes = 1.into();
        self.vertical_lpp = 3.into();
        self.horizontal_lpp = 1.into();
        self.light_color = 0x00FF_FFFF.into();
        self.brightness_color = 0x00FF_FFFF.into();
        self.extra_bright = 0.0.into();
        self.extra_contrast = 1.0.into();
        self.cur_pixel_vertical_gap = 0.0.into();
        self.cur_pixel_horizontal_gap = 0.0.into();
        self.cur_pixel_spread = 0.0.into();
        self.pixel_shadow_height = 0.0.into();
        self.pixels_geometry_kind = PixelGeometryKindOptions::Squares.into();
        self.pixel_shadow_shape_kind = ShadowShape { value: 3 }.into();
        self.color_channels = ColorChannelsOptions::Combined.into();
        self.screen_curvature_kind = ScreenCurvatureKindOptions::Flat.into();
        self.backlight_percent.value = 0.5;
        self.preset_kind = FilterPresetOptions::CrtApertureGrille1.into();
    }

    pub fn preset_crt_shadow_mask_1(&mut self) {
        self.internal_resolution = InternalResolution::default();
        self.texture_interpolation = TextureInterpolationOptions::Linear.into();
        self.blur_passes = 2.into();
        self.vertical_lpp = 2.into();
        self.horizontal_lpp = 2.into();
        self.light_color = 0x00FF_FFFF.into();
        self.brightness_color = 0x00FF_FFFF.into();
        self.extra_bright = 0.05.into();
        self.extra_contrast = 1.2.into();
        self.cur_pixel_vertical_gap = 0.5.into();
        self.cur_pixel_horizontal_gap = 0.5.into();
        self.cur_pixel_spread = 0.0.into();
        self.pixel_shadow_height = 1.0.into();
        self.pixels_geometry_kind = PixelGeometryKindOptions::Squares.into();
        self.pixel_shadow_shape_kind = ShadowShape { value: 3 }.into();
        self.color_channels = ColorChannelsOptions::Combined.into();
        self.screen_curvature_kind = ScreenCurvatureKindOptions::Flat.into();
        self.backlight_percent.value = 0.25;
        self.preset_kind = FilterPresetOptions::CrtShadowMask1.into();
    }

    pub fn preset_crt_shadow_mask_2(&mut self) {
        self.internal_resolution = InternalResolution::default();
        self.texture_interpolation = TextureInterpolationOptions::Linear.into();
        self.blur_passes = 2.into();
        self.vertical_lpp = 1.into();
        self.horizontal_lpp = 2.into();
        self.light_color = 0x00FF_FFFF.into();
        self.brightness_color = 0x00FF_FFFF.into();
        self.extra_bright = 0.05.into();
        self.extra_contrast = 1.2.into();
        self.cur_pixel_vertical_gap = 1.0.into();
        self.cur_pixel_horizontal_gap = 0.5.into();
        self.cur_pixel_spread = 0.0.into();
        self.pixel_shadow_height = 1.0.into();
        self.pixels_geometry_kind = PixelGeometryKindOptions::Squares.into();
        self.pixel_shadow_shape_kind = ShadowShape { value: 3 }.into();
        self.color_channels = ColorChannelsOptions::Combined.into();
        self.screen_curvature_kind = ScreenCurvatureKindOptions::Flat.into();
        self.backlight_percent.value = 0.4;
        self.preset_kind = FilterPresetOptions::CrtShadowMask2.into();
    }

    pub fn preset_demo_1(&mut self) {
        self.internal_resolution = InternalResolution::default();
        self.texture_interpolation = TextureInterpolationOptions::Linear.into();
        self.blur_passes = 0.into();
        self.vertical_lpp = 1.into();
        self.horizontal_lpp = 1.into();
        self.brightness_color = 0x00FF_FFFF.into();
        self.extra_bright = 0.0.into();
        self.extra_contrast = 1.0.into();
        self.cur_pixel_vertical_gap = 0.0.into();
        self.cur_pixel_horizontal_gap = 0.0.into();
        self.cur_pixel_spread = 1.0.into();
        self.pixel_shadow_height = 1.0.into();
        self.pixels_geometry_kind = PixelGeometryKindOptions::Cubes.into();
        self.pixel_shadow_shape_kind = ShadowShape { value: 0 }.into();
        self.color_channels = ColorChannelsOptions::Combined.into();
        self.screen_curvature_kind = ScreenCurvatureKindOptions::Pulse.into();
        self.backlight_percent.value = 0.2;
        self.preset_kind = FilterPresetOptions::DemoFlight1.into();
    }

    pub fn preset_custom(&mut self) {
        self.preset_kind = FilterPresetOptions::Custom.into();
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
    pub rgb_red: [f32; 3],
    pub rgb_green: [f32; 3],
    pub rgb_blue: [f32; 3],
    pub color_gamma: f32,
    pub color_noise: f32,
    pub showing_background: bool,
    pub time: f64,
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
