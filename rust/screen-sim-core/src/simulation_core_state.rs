use enum_len_derive::EnumLen;
use getters_by_type::GettersMutByType;
use num_derive::{FromPrimitive, ToPrimitive};

use crate::boolean_button::BooleanButton;
use crate::camera::{CameraChange, CameraData};
use crate::general_types::Size2D;
use crate::internal_resolution::InternalResolution;

pub const PIXEL_MANIPULATION_BASE_SPEED: f32 = 20.0;
pub const TURNING_BASE_SPEED: f32 = 3.0;
pub const MOVEMENT_BASE_SPEED: f32 = 10.0;
pub const MOVEMENT_SPEED_FACTOR: f32 = 50.0;

#[derive(Default)]
pub struct VideoInputResources {
    pub steps: Vec<AnimationStep>,
    pub max_texture_size: i32,
    pub image_size: Size2D<u32>,
    pub background_size: Size2D<u32>,
    pub viewport_size: Size2D<u32>,
    pub pixel_width: f32,
    pub stretch: bool,
    pub current_frame: usize,
    pub last_frame_change: f64,
    pub needs_buffer_data_load: bool,
}

pub struct AnimationStep {
    pub delay: u32,
}

// Simulation Resources
pub struct Resources {
    pub video: VideoInputResources,
    pub camera: CameraData,
    pub filters: Filters,
    pub output: ViewModel,
    pub timers: SimulationTimers,
    pub initial_parameters: InitialParameters,
    pub screenshot_trigger: ScreenshotTrigger,
    pub drawable: bool,
    pub resetted: bool,
    pub quit: bool,
}

pub struct ScreenshotTrigger {
    pub is_triggered: bool,
    pub delay: i32,
}

impl Default for Resources {
    fn default() -> Self {
        Resources {
            initial_parameters: InitialParameters::default(),
            timers: SimulationTimers::default(),
            video: VideoInputResources::default(),
            camera: CameraData::new(MOVEMENT_BASE_SPEED / MOVEMENT_SPEED_FACTOR, TURNING_BASE_SPEED),
            output: ViewModel::default(),
            filters: Filters::new(PIXEL_MANIPULATION_BASE_SPEED),
            screenshot_trigger: ScreenshotTrigger { is_triggered: false, delay: 0 },
            drawable: false,
            resetted: true,
            quit: false,
        }
    }
}

impl Resources {
    pub fn initialize(&mut self, video_input: VideoInputResources, now: f64) {
        let initial_position_z = calculate_far_away_position(&video_input);
        let mut camera = CameraData::new(MOVEMENT_BASE_SPEED * initial_position_z / MOVEMENT_SPEED_FACTOR, TURNING_BASE_SPEED);
        let mut cur_pixel_width = video_input.pixel_width;
        {
            let res: &Resources = self; // let's avoid using '&mut res' when just reading values
            if res.resetted {
                cur_pixel_width = video_input.pixel_width;
                camera.set_position(glm::vec3(0.0, 0.0, initial_position_z));
            } else {
                let mut camera_position = res.camera.get_position();
                if res.initial_parameters.initial_position_z != camera_position.z {
                    camera_position.z = initial_position_z;
                }
                camera.set_position(camera_position);
                if res.filters.cur_pixel_width != res.video.pixel_width {
                    cur_pixel_width = res.filters.cur_pixel_width;
                }
            }
        }
        self.quit = false;
        self.resetted = true;
        self.filters.cur_pixel_width = cur_pixel_width;
        self.timers = SimulationTimers {
            frame_count: 0,
            last_time: now,
            last_second: now,
        };
        self.initial_parameters = InitialParameters {
            initial_position_z,
            initial_pixel_width: video_input.pixel_width,
            initial_movement_speed: camera.movement_speed,
        };
        self.filters
            .internal_resolution
            .initialize(video_input.viewport_size, video_input.max_texture_size);
        self.camera = camera;
        self.video = video_input;
    }
}

fn calculate_far_away_position(video_input: &VideoInputResources) -> f32 {
    let width = video_input.background_size.width as f32;
    let height = video_input.background_size.height as f32;
    let viewport_width_scaled = (video_input.viewport_size.width as f32 / video_input.pixel_width) as u32;
    let width_ratio = viewport_width_scaled as f32 / width;
    let height_ratio = video_input.viewport_size.height as f32 / height;
    let is_height_bounded = width_ratio > height_ratio;
    let mut bound_ratio = if is_height_bounded { height_ratio } else { width_ratio };
    let mut resolution = if is_height_bounded {
        video_input.viewport_size.height
    } else {
        viewport_width_scaled
    } as i32;
    while bound_ratio < 1.0 {
        bound_ratio *= 2.0;
        resolution *= 2;
    }
    if !video_input.stretch {
        let mut divisor = bound_ratio as i32;
        while divisor > 1 {
            if resolution % divisor == 0 {
                break;
            }
            divisor -= 1;
        }
        bound_ratio = divisor as f32;
    }
    0.5 + (resolution as f32 / bound_ratio) * if is_height_bounded { 1.2076 } else { 0.68 * video_input.pixel_width }
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

pub struct Filters {
    pub internal_resolution: InternalResolution,
    pub texture_interpolation: TextureInterpolation,
    pub blur_passes: usize,
    pub lines_per_pixel: usize,
    pub light_color: i32,
    pub brightness_color: i32,
    pub extra_bright: f32,
    pub extra_contrast: f32,
    pub cur_pixel_width: f32,
    pub cur_pixel_vertical_gap: f32,
    pub cur_pixel_horizontal_gap: f32,
    pub cur_pixel_spread: f32,
    pub change_speed: f32,
    pub pixel_shadow_height_factor: f32,
    pub pixels_geometry_kind: PixelsGeometryKind,
    pub color_channels: ColorChannels,
    pub screen_curvature_kind: ScreenCurvatureKind,
    pub pixel_shadow_shape_kind: usize,
    pub layering_kind: ScreenLayeringKind,
}

impl Filters {
    pub fn new(change_speed: f32) -> Filters {
        Filters {
            internal_resolution: InternalResolution::new(1.0),
            texture_interpolation: TextureInterpolation::Linear,
            blur_passes: 1,
            lines_per_pixel: 2,
            light_color: 0x00FF_FFFF,
            brightness_color: 0x00FF_FFFF,
            extra_bright: 0.0,
            extra_contrast: 1.0,
            cur_pixel_width: 1.0,
            cur_pixel_vertical_gap: 0.0,
            cur_pixel_horizontal_gap: 0.0,
            cur_pixel_spread: 0.0,
            pixel_shadow_height_factor: 0.25,
            change_speed,
            pixels_geometry_kind: PixelsGeometryKind::Squares,
            pixel_shadow_shape_kind: 3,
            color_channels: ColorChannels::Combined,
            screen_curvature_kind: ScreenCurvatureKind::Flat,
            layering_kind: ScreenLayeringKind::ShadowWithSolidBackground50,
        }
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
    pub is_background_diffuse: bool,
    pub showing_background: bool,
    pub showing_foreground: bool,
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

#[derive(Clone, Copy)]
pub enum InputEventValue {
    None,
    PixelBrighttness(f32),
    PixelContrast(f32),
    LightColor(i32),
    BrightnessColor(i32),
    BlurLevel(usize),
    LinersPerPixel(usize),
    PixelShadowHeight(f32),
    PixelVerticalGap(f32),
    PixelHorizontalGap(f32),
    PixelWidth(f32),
    PixelSpread(f32),
    Camera(CameraChange),
}

impl InputEventValue {
    pub fn get_f32(self) -> f32 {
        match self {
            InputEventValue::PixelBrighttness(n) => n,
            InputEventValue::PixelContrast(n) => n,
            InputEventValue::PixelShadowHeight(n) => n,
            InputEventValue::PixelVerticalGap(n) => n,
            InputEventValue::PixelHorizontalGap(n) => n,
            InputEventValue::PixelWidth(n) => n,
            InputEventValue::PixelSpread(n) => n,
            InputEventValue::Camera(change) => change.get_f32(),
            _ => 0.0,
        }
    }
}

pub struct CustomInputEvent {
    values: Vec<InputEventValue>,
    kinds: Vec<String>,
}

// @Todo very bad impl, because makes it using it subpar.
// In the future, this should take a k-v approach probably.
// So, get_values should go away then.
// This was made like this, because it's an incremental from a single event struct.
impl CustomInputEvent {
    pub fn add_value(&mut self, kind: String, value: InputEventValue) {
        self.values.push(value);
        self.kinds.push(kind);
    }
    pub fn get_value(&self, kind: &str) -> InputEventValue {
        let mut i = 0;
        for k in &self.kinds {
            if k == kind {
                return self.values[i];
            }
            i += 1;
        }
        return InputEventValue::None;
    }
    pub fn get_values(&self) -> &Vec<InputEventValue> {
        &self.values
    }
    pub fn reset(&mut self) {
        self.values.resize(0, InputEventValue::None);
        self.kinds.resize_with(0, Default::default);
    }
}

impl Default for CustomInputEvent {
    fn default() -> Self {
        CustomInputEvent { values: vec![], kinds: vec![] }
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
    pub pixel_horizontal_gap: IncDec<bool>,
    pub pixel_vertical_gap: IncDec<bool>,
    pub pixel_width: IncDec<bool>,
    pub pixel_spread: IncDec<bool>,
    pub bright: IncDec<bool>,
    pub contrast: IncDec<bool>,
    pub next_camera_movement_mode: IncDec<BooleanButton>,
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
    pub fn new(now: f64) -> Input {
        let mut input: Input = Input::default();
        input.now = now;
        input
    }
}
