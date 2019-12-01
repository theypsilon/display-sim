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

use crate::boolean_button::BooleanButton;
use crate::camera::CameraChange;
use crate::general_types::{IncDec, Size2D};

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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Pressed {
    Yes,
    No,
}

impl Pressed {
    pub fn from_bool(pressed: bool) -> Self {
        if pressed {
            Pressed::Yes
        } else {
            Pressed::No
        }
    }
}

#[derive(Clone, Debug)]
pub enum InputEventValue {
    None,

    Keyboard { pressed: Pressed, key: String },
    MouseClick(Pressed),
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

pub(crate) struct CustomInputEvent {
    values: Vec<InputEventValue>,
}

impl CustomInputEvent {
    pub(crate) fn add_value(&mut self, value: InputEventValue) {
        self.values.push(value);
    }

    pub(crate) fn reset(&mut self) {
        self.values.resize(0, InputEventValue::None);
    }

    pub(crate) fn consume_values(&mut self) -> Vec<InputEventValue> {
        std::mem::replace(&mut self.values, vec![])
    }
}

impl Default for CustomInputEvent {
    fn default() -> Self {
        CustomInputEvent { values: vec![] }
    }
}

pub(crate) trait SetOptionNone {
    fn set_none(&mut self);
}
impl<T> SetOptionNone for Option<T> {
    fn set_none(&mut self) {
        *self = None;
    }
}
pub(crate) trait TrackedButton {
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
#[gen_array(pub(crate) fn get_options_to_be_noned: &mut dyn SetOptionNone)]
#[gen_array(pub(crate) fn get_tracked_buttons: &mut dyn TrackedButton)]
pub struct Input {
    pub(crate) custom_event: CustomInputEvent,
    pub(crate) now: f64,
    pub(crate) walk_left: bool,
    pub(crate) walk_right: bool,
    pub(crate) walk_up: bool,
    pub(crate) walk_down: bool,
    pub(crate) walk_forward: bool,
    pub(crate) walk_backward: bool,
    pub(crate) turn_left: bool,
    pub(crate) turn_right: bool,
    pub(crate) turn_up: bool,
    pub(crate) turn_down: bool,
    pub(crate) rotate_left: bool,
    pub(crate) rotate_right: bool,
    pub(crate) camera_zoom: IncDec<bool>,
    pub(crate) reset_speeds: bool,
    pub(crate) reset_position: bool,
    pub(crate) reset_filters: bool,
    pub(crate) shift: bool,
    pub(crate) control: bool,
    pub(crate) alt: bool,
    pub(crate) input_focused: bool,
    pub(crate) canvas_focused: bool,
    pub(crate) mouse_position_x: i32,
    pub(crate) mouse_position_y: i32,
    pub(crate) mouse_scroll_y: f32,
    pub(crate) pixel_horizontal_gap: IncDec<bool>,
    pub(crate) pixel_vertical_gap: IncDec<bool>,
    pub(crate) pixel_width: IncDec<bool>,
    pub(crate) pixel_spread: IncDec<bool>,
    pub(crate) bright: IncDec<bool>,
    pub(crate) contrast: IncDec<bool>,
    pub(crate) backlight_percent: IncDec<bool>,
    #[in_array(get_tracked_buttons)]
    pub(crate) next_camera_movement_mode: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub(crate) translation_speed: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub(crate) turn_speed: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub(crate) filter_speed: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub(crate) mouse_click: BooleanButton,
    #[in_array(get_tracked_buttons)]
    pub(crate) blur: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub(crate) vertical_lpp: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub(crate) horizontal_lpp: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub(crate) next_pixel_shadow_shape_kind: IncDec<BooleanButton>,
    pub(crate) next_pixels_shadow_height: IncDec<bool>,
    #[in_array(get_tracked_buttons)]
    pub(crate) next_color_representation_kind: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub(crate) next_pixel_geometry_kind: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub(crate) next_screen_curvature_type: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub(crate) next_internal_resolution: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub(crate) next_texture_interpolation: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub(crate) scaling_method: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub(crate) scaling_resolution_width: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub(crate) scaling_resolution_height: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub(crate) scaling_aspect_ratio_x: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub(crate) scaling_aspect_ratio_y: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub(crate) esc: BooleanButton,
    #[in_array(get_tracked_buttons)]
    pub(crate) space: BooleanButton,
    #[in_array(get_tracked_buttons)]
    pub(crate) screenshot: BooleanButton,

    #[in_array(get_options_to_be_noned)]
    pub(crate) event_filter_preset: Option<String>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_pixel_brighttness: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_pixel_contrast: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_light_color: Option<i32>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_brightness_color: Option<i32>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_blur_level: Option<usize>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_vertical_lpp: Option<usize>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_horizontal_lpp: Option<usize>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_scaling_resolution_width: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_scaling_resolution_height: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_scaling_aspect_ratio_x: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_scaling_aspect_ratio_y: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_custom_scaling_stretch_nearest: Option<bool>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_backlight_percent: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_pixel_shadow_height: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_pixel_vertical_gap: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_pixel_horizontal_gap: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_pixel_width: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_pixel_spread: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_viewport_resize: Option<Size2D<u32>>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_camera: Option<CameraChange>,

    pub(crate) active_pressed_actions: Vec<KeyCodeBooleanAction>,
}

impl Input {
    pub fn new(now: f64) -> Input {
        let mut input: Input = Input::default();
        input.now = now;
        input
    }

    pub fn push_event(&mut self, event: InputEventValue) {
        self.custom_event.add_value(event);
    }
}

pub(crate) type KeyCodeBooleanAction = (String, BooleanAction);

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum Boolean2DAction {
    Increase,
    Decrease,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum BooleanAction {
    WalkLeft,
    WalkRight,
    WalkUp,
    WalkDown,
    WalkForward,
    WalkBackward,
    TurnLeft,
    TurnRight,
    TurnUp,
    TurnDown,
    RotateLeft,
    RotateRight,
    ResetSpeeds,
    ResetPosition,
    ResetFilters,
    Shift,
    Control,
    Alt,
    Esc,
    Space,
    Screenshot,
    InputFocused,
    CanvasFocused,
    MouseClick,

    CameraZoom(Boolean2DAction),
    PixelHorizontalGap(Boolean2DAction),
    PixelVerticalGap(Boolean2DAction),
    PixelWidth(Boolean2DAction),
    PixelSpread(Boolean2DAction),
    Bright(Boolean2DAction),
    Contrast(Boolean2DAction),
    BacklightPercent(Boolean2DAction),
    NextCameraMovementMode(Boolean2DAction),
    TranslationSpeed(Boolean2DAction),
    TurnSpeed(Boolean2DAction),
    FilterSpeed(Boolean2DAction),
    Blur(Boolean2DAction),
    VerticalLpp(Boolean2DAction),
    HorizontalLpp(Boolean2DAction),
    NextPixelShadowShapeKind(Boolean2DAction),
    NextPixelsShadowHeight(Boolean2DAction),
    NextColorRepresentationKind(Boolean2DAction),
    NextPixelGeometryKind(Boolean2DAction),
    NextScreenCurvatureType(Boolean2DAction),
    NextInternalResolution(Boolean2DAction),
    NextTextureInterpolation(Boolean2DAction),
    ScalingMethod(Boolean2DAction),
    ScalingResolutionWidth(Boolean2DAction),
    ScalingResolutionHeight(Boolean2DAction),
    ScalingAspectRatioX(Boolean2DAction),
    ScalingAspectRatioY(Boolean2DAction),

    None,
}