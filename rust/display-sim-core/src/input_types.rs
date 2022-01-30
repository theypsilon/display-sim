/* Copyright (c) 2019-2021 José manuel Barroso Galindo <theypsilon@gmail.com>
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

    PixelWidth(f32),
    Camera(CameraChange),
    CustomScalingResolutionWidth(f32),
    CustomScalingResolutionHeight(f32),
    CustomScalingAspectRatioX(f32),
    CustomScalingAspectRatioY(f32),
    CustomScalingStretchNearest(bool),
    ViewportResize(u32, u32),
}

#[derive(Default)]
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
        std::mem::take(&mut self.values)
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
#[gen_array(pub(crate) fn get_options_to_be_noned: &mut dyn SetOptionNone, implicit_select_all: Option<_>)]
#[gen_array(pub(crate) fn get_tracked_buttons: &mut dyn TrackedButton, implicit_select_all: BooleanButton, IncDec<BooleanButton>)]
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

    pub(crate) active_pressed_actions: Vec<KeyCodeBooleanAction>,
    pub(crate) active_pressed_actions_2: Vec<String>,

    // get_tracked_buttons
    pub(crate) next_camera_movement_mode: IncDec<BooleanButton>,
    pub(crate) translation_speed: IncDec<BooleanButton>,
    pub(crate) turn_speed: IncDec<BooleanButton>,
    pub(crate) filter_speed: IncDec<BooleanButton>,
    pub(crate) mouse_click: BooleanButton,
    pub(crate) blur: IncDec<BooleanButton>,
    pub(crate) scaling_method: IncDec<BooleanButton>,
    pub(crate) scaling_resolution_width: IncDec<BooleanButton>,
    pub(crate) scaling_resolution_height: IncDec<BooleanButton>,
    pub(crate) scaling_aspect_ratio_x: IncDec<BooleanButton>,
    pub(crate) scaling_aspect_ratio_y: IncDec<BooleanButton>,
    pub(crate) esc: BooleanButton,
    pub(crate) space: BooleanButton,
    pub(crate) screenshot: BooleanButton,

    // get_options_to_be_noned
    pub(crate) event_scaling_resolution_width: Option<f32>,
    pub(crate) event_scaling_resolution_height: Option<f32>,
    pub(crate) event_scaling_aspect_ratio_x: Option<f32>,
    pub(crate) event_scaling_aspect_ratio_y: Option<f32>,
    pub(crate) event_custom_scaling_stretch_nearest: Option<bool>,
    pub(crate) event_pixel_width: Option<f32>,
    pub(crate) event_viewport_resize: Option<Size2D<u32>>,
    pub(crate) event_camera: Option<CameraChange>,
}

impl Input {
    pub fn new(now: f64) -> Input {
        Input {now, ..Default::default()}
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
    ScalingMethod(Boolean2DAction),
    ScalingResolutionWidth(Boolean2DAction),
    ScalingResolutionHeight(Boolean2DAction),
    ScalingAspectRatioX(Boolean2DAction),
    ScalingAspectRatioY(Boolean2DAction),
}
