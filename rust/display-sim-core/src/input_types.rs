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
    PixelWidth(f32),
    Camera(CameraChange),
    Rgb(RgbChange),
    CustomScalingResolutionWidth(f32),
    CustomScalingResolutionHeight(f32),
    CustomScalingAspectRatioX(f32),
    CustomScalingAspectRatioY(f32),
    CustomScalingStretchNearest(bool),
    ViewportResize(u32, u32),
}

#[derive(Clone, Debug)]
pub enum RgbChange {
    RedR(f32),
    RedG(f32),
    RedB(f32),
    GreenR(f32),
    GreenG(f32),
    GreenB(f32),
    BlueR(f32),
    BlueG(f32),
    BlueB(f32),
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
    pub(crate) pixel_width: IncDec<bool>,
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
    #[in_array(get_tracked_buttons)]
    pub(crate) next_color_representation_kind: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub(crate) next_pixel_geometry_kind: IncDec<BooleanButton>,
    #[in_array(get_tracked_buttons)]
    pub(crate) next_screen_curvature_type: IncDec<BooleanButton>,
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
    pub(crate) event_pixel_width: Option<f32>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_viewport_resize: Option<Size2D<u32>>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_camera: Option<CameraChange>,
    #[in_array(get_options_to_be_noned)]
    pub(crate) event_rgb: Option<RgbChange>,

    pub(crate) active_pressed_actions: Vec<KeyCodeBooleanAction>,
    pub(crate) active_pressed_actions_2: Vec<String>,
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
    PixelWidth(Boolean2DAction),
    NextCameraMovementMode(Boolean2DAction),
    TranslationSpeed(Boolean2DAction),
    TurnSpeed(Boolean2DAction),
    FilterSpeed(Boolean2DAction),
    NextPixelShadowShapeKind(Boolean2DAction),
    NextColorRepresentationKind(Boolean2DAction),
    NextPixelGeometryKind(Boolean2DAction),
    NextScreenCurvatureType(Boolean2DAction),
    ScalingMethod(Boolean2DAction),
    ScalingResolutionWidth(Boolean2DAction),
    ScalingResolutionHeight(Boolean2DAction),
    ScalingAspectRatioX(Boolean2DAction),
    ScalingAspectRatioY(Boolean2DAction),
}
