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

use std::collections::HashMap;

use arraygen::Arraygen;
use enum_len_derive::EnumLen;
use num_derive::{FromPrimitive, ToPrimitive};

use crate::camera::CameraData;
use crate::general_types::Size2D;
use crate::pixels_shadow::ShadowShape;
use crate::ui_controller::{
    backlight_percent::BacklightPercent, blur_passes::BlurPasses, brightness_color::BrightnessColor, color_gamma::ColorGamma, color_noise::ColorNoise,
    cur_pixel_horizontal_gap::CurPixelHorizontalGap, cur_pixel_spread::CurPixelSpread, cur_pixel_vertical_gap::CurPixelVerticalGap, extra_bright::ExtraBright,
    extra_contrast::ExtraContrast, horizontal_lpp::HorizontalLpp, internal_resolution::InternalResolution, light_color::LightColor,
    pixel_shadow_height::PixelShadowHeight, vertical_lpp::VerticalLpp, UiController,
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

pub enum KeyEventKind {
    Inc,
    Dec,
    Set,
}

#[derive(Default)]
pub struct MainState {
    pub dt: f32,
    pub filter_speed: f32,
    pub render: ViewModel,
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
    pub main: MainState,
    pub output: ViewModel,
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
        let mut filters = Filters::default();
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
            scaling: Scaling::default(),
            saved_filters: None,
            custom_is_changed: false,
            screenshot_trigger: ScreenshotTrigger { is_triggered: false, delay: 0 },
            drawable: false,
            resetted: true,
            quit: false,
            controller_events: {
                let mut map: HashMap<&'static str, (KeyEventKind, usize)> = HashMap::new();
                for (i, controller) in filters.get_ui_controllers_mut().iter().enumerate() {
                    for key in controller.keys_dec().clone() {
                        map.insert(*key, (KeyEventKind::Dec, i));
                    }
                    for key in controller.keys_inc().clone() {
                        map.insert(*key, (KeyEventKind::Inc, i));
                    }
                    map.insert(controller.event_tag(), (KeyEventKind::Set, i));
                }
                map
            },
            main: Default::default(),
            filters,
        }
    }
}

impl Resources {
    pub fn initialize(&mut self, video_input: VideoInputResources, now: f64) {
        self.quit = false;
        self.resetted = true;
        self.scaling.scaling_initialized = false;
        if let Some(preset) = video_input.preset {
            self.filters.preset_factory(preset, &None);
        }
        self.timers = SimulationTimers {
            frame_count: 0,
            last_time: now,
            last_second: now,
        };
        self.video = video_input;
        for controller in self.filters.get_ui_controllers_mut().iter_mut() {
            controller.reset_inputs();
        }
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

#[derive(Clone, Arraygen)]
#[gen_array(pub fn get_ui_controllers: &dyn UiController)]
#[gen_array(pub fn get_ui_controllers_mut: &mut dyn UiController)]
pub struct Filters {
    #[in_array(get_ui_controllers)]
    #[in_array(get_ui_controllers_mut)]
    pub internal_resolution: InternalResolution,
    pub texture_interpolation: TextureInterpolation,
    #[in_array(get_ui_controllers)]
    #[in_array(get_ui_controllers_mut)]
    pub blur_passes: BlurPasses,
    #[in_array(get_ui_controllers)]
    #[in_array(get_ui_controllers_mut)]
    pub vertical_lpp: VerticalLpp,
    #[in_array(get_ui_controllers)]
    #[in_array(get_ui_controllers_mut)]
    pub horizontal_lpp: HorizontalLpp,
    #[in_array(get_ui_controllers)]
    #[in_array(get_ui_controllers_mut)]
    pub light_color: LightColor,
    #[in_array(get_ui_controllers)]
    #[in_array(get_ui_controllers_mut)]
    pub brightness_color: BrightnessColor,
    #[in_array(get_ui_controllers)]
    #[in_array(get_ui_controllers_mut)]
    pub extra_bright: ExtraBright,
    #[in_array(get_ui_controllers)]
    #[in_array(get_ui_controllers_mut)]
    pub extra_contrast: ExtraContrast,
    #[in_array(get_ui_controllers)]
    #[in_array(get_ui_controllers_mut)]
    pub cur_pixel_vertical_gap: CurPixelVerticalGap,
    #[in_array(get_ui_controllers)]
    #[in_array(get_ui_controllers_mut)]
    pub cur_pixel_horizontal_gap: CurPixelHorizontalGap,
    #[in_array(get_ui_controllers)]
    #[in_array(get_ui_controllers_mut)]
    pub cur_pixel_spread: CurPixelSpread,
    #[in_array(get_ui_controllers)]
    #[in_array(get_ui_controllers_mut)]
    pub pixel_shadow_height: PixelShadowHeight,
    pub pixels_geometry_kind: PixelsGeometryKind,
    pub color_channels: ColorChannels,
    pub screen_curvature_kind: ScreenCurvatureKind,
    pub pixel_shadow_shape_kind: ShadowShape,
    #[in_array(get_ui_controllers)]
    #[in_array(get_ui_controllers_mut)]
    pub backlight_percent: BacklightPercent,
    pub rgb_red_r: f32,
    pub rgb_red_g: f32,
    pub rgb_red_b: f32,
    pub rgb_green_r: f32,
    pub rgb_green_g: f32,
    pub rgb_green_b: f32,
    pub rgb_blue_r: f32,
    pub rgb_blue_g: f32,
    pub rgb_blue_b: f32,
    #[in_array(get_ui_controllers)]
    #[in_array(get_ui_controllers_mut)]
    pub color_gamma: ColorGamma,
    #[in_array(get_ui_controllers)]
    #[in_array(get_ui_controllers_mut)]
    pub color_noise: ColorNoise,
    pub preset_kind: FiltersPreset,
}

impl Default for Filters {
    fn default() -> Self {
        let mut filters = Filters {
            internal_resolution: InternalResolution::default(),
            texture_interpolation: TextureInterpolation::Linear,
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
            pixels_geometry_kind: PixelsGeometryKind::Squares,
            pixel_shadow_shape_kind: ShadowShape { value: 0 },
            color_channels: ColorChannels::Combined,
            screen_curvature_kind: ScreenCurvatureKind::Flat,
            backlight_percent: 0.0.into(),
            rgb_red_r: 1.0,
            rgb_red_g: 0.0,
            rgb_red_b: 0.0,
            rgb_green_r: 0.0,
            rgb_green_g: 1.0,
            rgb_green_b: 0.0,
            rgb_blue_r: 0.0,
            rgb_blue_g: 0.0,
            rgb_blue_b: 1.0,
            color_gamma: 1.0.into(),
            color_noise: 0.0.into(),
            preset_kind: FiltersPreset::Sharp1,
        };
        filters.preset_crt_aperture_grille_1();
        filters
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
    pub fn preset_factory(&mut self, preset: FiltersPreset, previous_custom: &Option<Filters>) {
        match preset {
            FiltersPreset::Sharp1 => self.preset_sharp_1(),
            FiltersPreset::CrtApertureGrille1 => self.preset_crt_aperture_grille_1(),
            FiltersPreset::CrtShadowMask1 => self.preset_crt_shadow_mask_1(),
            FiltersPreset::CrtShadowMask2 => self.preset_crt_shadow_mask_2(),
            FiltersPreset::DemoFlight1 => self.preset_demo_1(),
            FiltersPreset::Custom => match previous_custom {
                Some(_) => {}
                None => self.preset_custom(),
            },
        }
    }
    pub fn preset_sharp_1(&mut self) {
        self.internal_resolution = InternalResolution::default();
        self.texture_interpolation = TextureInterpolation::Linear;
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
        self.pixels_geometry_kind = PixelsGeometryKind::Squares;
        self.pixel_shadow_shape_kind = ShadowShape { value: 0 };
        self.color_channels = ColorChannels::Combined;
        self.screen_curvature_kind = ScreenCurvatureKind::Flat;
        self.backlight_percent.value = 0.0;
        self.preset_kind = FiltersPreset::Sharp1;
    }

    pub fn preset_crt_aperture_grille_1(&mut self) {
        self.internal_resolution = InternalResolution::default();
        self.texture_interpolation = TextureInterpolation::Linear;
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
        self.pixels_geometry_kind = PixelsGeometryKind::Squares;
        self.pixel_shadow_shape_kind = ShadowShape { value: 3 };
        self.color_channels = ColorChannels::Combined;
        self.screen_curvature_kind = ScreenCurvatureKind::Flat;
        self.backlight_percent.value = 0.5;
        self.preset_kind = FiltersPreset::CrtApertureGrille1;
    }

    pub fn preset_crt_shadow_mask_1(&mut self) {
        self.internal_resolution = InternalResolution::default();
        self.texture_interpolation = TextureInterpolation::Linear;
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
        self.pixels_geometry_kind = PixelsGeometryKind::Squares;
        self.pixel_shadow_shape_kind = ShadowShape { value: 3 };
        self.color_channels = ColorChannels::Combined;
        self.screen_curvature_kind = ScreenCurvatureKind::Flat;
        self.backlight_percent.value = 0.25;
        self.preset_kind = FiltersPreset::CrtShadowMask1;
    }

    pub fn preset_crt_shadow_mask_2(&mut self) {
        self.internal_resolution = InternalResolution::default();
        self.texture_interpolation = TextureInterpolation::Linear;
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
        self.pixels_geometry_kind = PixelsGeometryKind::Squares;
        self.pixel_shadow_shape_kind = ShadowShape { value: 3 };
        self.color_channels = ColorChannels::Combined;
        self.screen_curvature_kind = ScreenCurvatureKind::Flat;
        self.backlight_percent.value = 0.4;
        self.preset_kind = FiltersPreset::CrtShadowMask2;
    }

    pub fn preset_demo_1(&mut self) {
        self.internal_resolution = InternalResolution::default();
        self.texture_interpolation = TextureInterpolation::Linear;
        self.blur_passes = 0.into();
        self.vertical_lpp = 1.into();
        self.horizontal_lpp = 1.into();
        self.light_color = self.light_color;
        self.brightness_color = 0x00FF_FFFF.into();
        self.extra_bright = 0.0.into();
        self.extra_contrast = 1.0.into();
        self.cur_pixel_vertical_gap = 0.0.into();
        self.cur_pixel_horizontal_gap = 0.0.into();
        self.cur_pixel_spread = 1.0.into();
        self.pixel_shadow_height = 1.0.into();
        self.pixels_geometry_kind = PixelsGeometryKind::Cubes;
        self.pixel_shadow_shape_kind = ShadowShape { value: 0 };
        self.color_channels = ColorChannels::Combined;
        self.screen_curvature_kind = ScreenCurvatureKind::Pulse;
        self.backlight_percent.value = 0.2;
        self.preset_kind = FiltersPreset::DemoFlight1;
    }

    pub fn preset_custom(&mut self) {
        self.preset_kind = FiltersPreset::Custom;
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
