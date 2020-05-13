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

use crate::input_types::{Boolean2DAction, BooleanAction, Input, KeyCodeBooleanAction, Pressed};
use crate::simulation_core_state::{KeyEventKind, Resources};

pub(crate) fn trigger_hotkey_action(input: &mut Input, res: &mut Resources, keycode: &str, pressed: Pressed) -> ActionUsed {
    if let Some((kind, index)) = res.controller_events.get_mut(keycode) {
        let controller = &mut res.filters.get_ui_controllers_mut()[*index];
        let pressed = match pressed {
            Pressed::Yes => true,
            Pressed::No => false,
        };
        match kind {
            KeyEventKind::Inc => controller.read_key_inc(pressed),
            KeyEventKind::Dec => controller.read_key_dec(pressed),
            KeyEventKind::Set => unreachable!(),
        }
    }
    let (maybe_new_keycode, action) = get_contextualized_action(input, keycode);
    let action = match action {
        #[cfg(debug_assertions)]
        None => return ActionUsed::No(maybe_new_keycode.unwrap_or_else(|| keycode.into())),
        #[cfg(not(debug_assertions))]
        None => return ActionUsed::No,
        Some(action) => action,
    };
    process_modifiers(input, action, pressed);
    if pressed == Pressed::Yes && input.active_pressed_actions.iter().any(|(_, active_action)| *active_action == action) {
        return ActionUsed::Yes;
    }
    handle_action(input, action, pressed);
    match pressed {
        Pressed::Yes => input.active_pressed_actions.push((maybe_new_keycode.unwrap_or_else(|| keycode.into()), action)),
        Pressed::No => remove_action(input, action),
    }
    ActionUsed::Yes
}

#[derive(PartialEq, Debug)]
pub(crate) enum ActionUsed {
    Yes,
    #[cfg(debug_assertions)]
    No(String),
    #[cfg(not(debug_assertions))]
    No,
}

fn get_contextualized_action(input: &Input, keycode: &str) -> (Option<String>, Option<BooleanAction>) {
    let action = match to_boolean_action(keycode) {
        None => return (None, None),
        Some(action) => action,
    };
    let maybe_modification = if input.shift {
        try_modify_action(action, keycode, BooleanAction::Shift)
    } else if input.control {
        try_modify_action(action, keycode, BooleanAction::Control)
    } else if input.alt {
        try_modify_action(action, keycode, BooleanAction::Alt)
    } else {
        None
    };
    match maybe_modification {
        Some((modified_keycode, modified_action)) => (Some(modified_keycode), Some(modified_action)),
        None => (None, Some(action)),
    }
}

fn process_modifiers(input: &mut Input, action: BooleanAction, pressed: Pressed) {
    match action {
        BooleanAction::Shift => react_to_modifier(input, BooleanAction::Shift, pressed),
        BooleanAction::Control => react_to_modifier(input, BooleanAction::Control, pressed),
        BooleanAction::Alt => react_to_modifier(input, BooleanAction::Alt, pressed),
        _ => {}
    }
}

fn try_modify_action(action: BooleanAction, keycode: &str, modifier: BooleanAction) -> Option<(String, BooleanAction)> {
    if action == modifier {
        return None;
    }
    let modified_keycode = format!("{}{}", get_modifier_code(modifier), keycode);
    match to_boolean_action(&modified_keycode) {
        Some(modified_action) => Some((modified_keycode, modified_action)),
        None => None,
    }
}

fn get_modifier_code(modifier: BooleanAction) -> &'static str {
    match modifier {
        BooleanAction::Shift => "shift+",
        BooleanAction::Control => "ctrl+",
        BooleanAction::Alt => "alt+",
        _ => unreachable!(),
    }
}

fn react_to_modifier(input: &mut Input, modifier: BooleanAction, pressed: Pressed) {
    let modifier_code = get_modifier_code(modifier);
    let (to_add, to_delete) = match pressed {
        Pressed::Yes => modify_active_actions(&input.active_pressed_actions, modifier_code),
        Pressed::No => unmodify_active_actions(&input.active_pressed_actions, modifier_code),
    };
    resolve_modifications(input, to_add, to_delete);
}

type IndexBooleanAction = (usize, BooleanAction);

fn modify_active_actions(active_actions: &[KeyCodeBooleanAction], modifier_code: &str) -> (Vec<KeyCodeBooleanAction>, Vec<IndexBooleanAction>) {
    let mut to_delete = Vec::new();
    let mut to_add = Vec::new();
    for (i, (keycode, action)) in active_actions.iter().enumerate() {
        let modified_keycode = format!("{}{}", modifier_code, keycode);
        let modified_action = match to_boolean_action(&modified_keycode) {
            None => continue,
            Some(modified_action) => modified_action,
        };
        to_delete.push((i, *action));
        to_add.push((modified_keycode, modified_action));
    }
    (to_add, to_delete)
}

fn unmodify_active_actions(active_actions: &[KeyCodeBooleanAction], modifier_code: &str) -> (Vec<KeyCodeBooleanAction>, Vec<IndexBooleanAction>) {
    let mut to_delete = Vec::new();
    let mut to_add = Vec::new();
    for (i, (keycode, action)) in active_actions.iter().enumerate() {
        if !keycode.starts_with(modifier_code) {
            continue;
        }
        to_delete.push((i, *action));
        let unmodified_keycode = keycode.replace(modifier_code, "");
        let unmodified_action = match to_boolean_action(&unmodified_keycode) {
            None => continue,
            Some(unmodified_action) => unmodified_action,
        };
        to_add.push((unmodified_keycode, unmodified_action));
    }
    (to_add, to_delete)
}

fn resolve_modifications(input: &mut Input, to_add: Vec<KeyCodeBooleanAction>, to_delete: Vec<IndexBooleanAction>) {
    for (i, removed_action) in to_delete.into_iter() {
        handle_action(input, removed_action, Pressed::No);
        input.active_pressed_actions.remove(i);
    }
    for (modified_keycode, modified_action) in to_add.into_iter() {
        handle_action(input, modified_action, Pressed::Yes);
        input.active_pressed_actions.push((modified_keycode, modified_action));
    }
}

fn remove_action(input: &mut Input, action: BooleanAction) {
    let mut index = None;
    for (i, (_, active_action)) in input.active_pressed_actions.iter().enumerate() {
        if *active_action == action {
            debug_assert_eq!(index, None);
            index = Some(i);
            #[cfg(not(debug_assertions))]
            break;
        }
    }
    if let Some(i) = index {
        input.active_pressed_actions.remove(i);
    }
}

fn handle_action(input: &mut Input, action: BooleanAction, pressed: Pressed) {
    let pressed = match pressed {
        Pressed::Yes => true,
        Pressed::No => false,
    };
    match action {
        BooleanAction::Shift => input.shift = pressed,
        BooleanAction::Control => input.control = pressed,
        BooleanAction::Alt => input.alt = pressed,
        BooleanAction::Screenshot => input.screenshot.input = pressed,
        BooleanAction::ResetPosition => input.reset_position = pressed,
        BooleanAction::ResetFilters => input.reset_filters = pressed,
        BooleanAction::InputFocused => input.input_focused = pressed,
        BooleanAction::CanvasFocused => input.canvas_focused = pressed,
        BooleanAction::Esc => input.esc.input = pressed,
        BooleanAction::Space => input.space.input = pressed,
        BooleanAction::RotateLeft => input.rotate_left = pressed,
        BooleanAction::RotateRight => input.rotate_right = pressed,
        BooleanAction::TurnLeft => input.turn_left = pressed,
        BooleanAction::TurnRight => input.turn_right = pressed,
        BooleanAction::TurnUp => input.turn_up = pressed,
        BooleanAction::TurnDown => input.turn_down = pressed,
        BooleanAction::WalkLeft => input.walk_left = pressed,
        BooleanAction::WalkRight => input.walk_right = pressed,
        BooleanAction::WalkForward => input.walk_forward = pressed,
        BooleanAction::WalkBackward => input.walk_backward = pressed,
        BooleanAction::WalkUp => input.walk_up = pressed,
        BooleanAction::WalkDown => input.walk_down = pressed,
        BooleanAction::ScalingMethod(Boolean2DAction::Increase) => input.scaling_method.increase.input = pressed,
        BooleanAction::ScalingMethod(Boolean2DAction::Decrease) => input.scaling_method.decrease.input = pressed,
        BooleanAction::ScalingResolutionWidth(Boolean2DAction::Increase) => input.scaling_resolution_width.increase.input = pressed,
        BooleanAction::ScalingResolutionWidth(Boolean2DAction::Decrease) => input.scaling_resolution_width.decrease.input = pressed,
        BooleanAction::ScalingResolutionHeight(Boolean2DAction::Increase) => input.scaling_resolution_height.increase.input = pressed,
        BooleanAction::ScalingResolutionHeight(Boolean2DAction::Decrease) => input.scaling_resolution_height.decrease.input = pressed,
        BooleanAction::ScalingAspectRatioX(Boolean2DAction::Increase) => input.scaling_aspect_ratio_x.increase.input = pressed,
        BooleanAction::ScalingAspectRatioX(Boolean2DAction::Decrease) => input.scaling_aspect_ratio_x.decrease.input = pressed,
        BooleanAction::ScalingAspectRatioY(Boolean2DAction::Increase) => input.scaling_aspect_ratio_y.increase.input = pressed,
        BooleanAction::ScalingAspectRatioY(Boolean2DAction::Decrease) => input.scaling_aspect_ratio_y.decrease.input = pressed,
        BooleanAction::TranslationSpeed(Boolean2DAction::Increase) => input.translation_speed.increase.input = pressed,
        BooleanAction::TranslationSpeed(Boolean2DAction::Decrease) => input.translation_speed.decrease.input = pressed,
        BooleanAction::FilterSpeed(Boolean2DAction::Increase) => input.filter_speed.increase.input = pressed,
        BooleanAction::FilterSpeed(Boolean2DAction::Decrease) => input.filter_speed.decrease.input = pressed,
        BooleanAction::ResetSpeeds => input.reset_speeds = pressed,
        BooleanAction::CameraZoom(Boolean2DAction::Increase) => input.camera_zoom.increase = pressed,
        BooleanAction::CameraZoom(Boolean2DAction::Decrease) => input.camera_zoom.decrease = pressed,
        BooleanAction::PixelWidth(Boolean2DAction::Increase) => input.pixel_width.increase = pressed,
        BooleanAction::PixelWidth(Boolean2DAction::Decrease) => input.pixel_width.decrease = pressed,
        BooleanAction::Blur(Boolean2DAction::Increase) => input.blur.increase.input = pressed,
        BooleanAction::Blur(Boolean2DAction::Decrease) => input.blur.decrease.input = pressed,
        BooleanAction::VerticalLpp(Boolean2DAction::Increase) => input.vertical_lpp.increase.input = pressed,
        BooleanAction::VerticalLpp(Boolean2DAction::Decrease) => input.vertical_lpp.decrease.input = pressed,
        BooleanAction::HorizontalLpp(Boolean2DAction::Increase) => input.horizontal_lpp.increase.input = pressed,
        BooleanAction::HorizontalLpp(Boolean2DAction::Decrease) => input.horizontal_lpp.decrease.input = pressed,
        BooleanAction::NextColorRepresentationKind(Boolean2DAction::Increase) => input.next_color_representation_kind.increase.input = pressed,
        BooleanAction::NextColorRepresentationKind(Boolean2DAction::Decrease) => input.next_color_representation_kind.decrease.input = pressed,
        BooleanAction::NextPixelGeometryKind(Boolean2DAction::Increase) => input.next_pixel_geometry_kind.increase.input = pressed,
        BooleanAction::NextPixelGeometryKind(Boolean2DAction::Decrease) => input.next_pixel_geometry_kind.decrease.input = pressed,
        BooleanAction::NextScreenCurvatureType(Boolean2DAction::Increase) => input.next_screen_curvature_type.increase.input = pressed,
        BooleanAction::NextScreenCurvatureType(Boolean2DAction::Decrease) => input.next_screen_curvature_type.decrease.input = pressed,
        BooleanAction::NextPixelShadowShapeKind(Boolean2DAction::Increase) => input.next_pixel_shadow_shape_kind.increase.input = pressed,
        BooleanAction::NextPixelShadowShapeKind(Boolean2DAction::Decrease) => input.next_pixel_shadow_shape_kind.decrease.input = pressed,
        BooleanAction::NextInternalResolution(Boolean2DAction::Increase) => input.next_internal_resolution.increase.input = pressed,
        BooleanAction::NextInternalResolution(Boolean2DAction::Decrease) => input.next_internal_resolution.decrease.input = pressed,
        BooleanAction::NextTextureInterpolation(Boolean2DAction::Increase) => input.next_texture_interpolation.increase.input = pressed,
        BooleanAction::NextTextureInterpolation(Boolean2DAction::Decrease) => input.next_texture_interpolation.decrease.input = pressed,
        BooleanAction::NextCameraMovementMode(Boolean2DAction::Increase) => input.next_camera_movement_mode.increase.input = pressed,
        BooleanAction::NextCameraMovementMode(Boolean2DAction::Decrease) => input.next_camera_movement_mode.decrease.input = pressed,
        BooleanAction::TurnSpeed(Boolean2DAction::Increase) => input.turn_speed.increase.input = pressed,
        BooleanAction::TurnSpeed(Boolean2DAction::Decrease) => input.turn_speed.decrease.input = pressed,
        BooleanAction::MouseClick => input.mouse_click.input = pressed,
    }
}

fn to_boolean_action(boolean_action: &str) -> Option<BooleanAction> {
    match boolean_action {
        "mouse_click" => Some(BooleanAction::MouseClick),
        "shift" | "left shift" | "right shift" => Some(BooleanAction::Shift),
        "control" => Some(BooleanAction::Control),
        "alt" => Some(BooleanAction::Alt),
        "f4" | "capture-framebuffer" => Some(BooleanAction::Screenshot),
        "reset-camera" => Some(BooleanAction::ResetPosition),
        "reset-filters" => Some(BooleanAction::ResetFilters),
        "input_focused" => Some(BooleanAction::InputFocused),
        "canvas_focused" => Some(BooleanAction::CanvasFocused),
        "escape" | "esc" | "quit-simulation" => Some(BooleanAction::Esc),
        " " | "space" | "feature-close-panel" => Some(BooleanAction::Space),
        "+" => Some(BooleanAction::RotateLeft),
        "-" => Some(BooleanAction::RotateRight),
        "arrowleft" | "left" | "←" | "◀" => Some(BooleanAction::TurnLeft),
        "arrowright" | "right" | "→" | "▶" => Some(BooleanAction::TurnRight),
        "arrowup" | "up" | "↑" | "▲" => Some(BooleanAction::TurnUp),
        "arrowdown" | "down" | "↓" | "▼" => Some(BooleanAction::TurnDown),
        "a" => Some(BooleanAction::WalkLeft),
        "d" => Some(BooleanAction::WalkRight),
        "w" => Some(BooleanAction::WalkForward),
        "s" => Some(BooleanAction::WalkBackward),
        "q" => Some(BooleanAction::WalkUp),
        "e" => Some(BooleanAction::WalkDown),
        "scaling-method-inc" => Some(BooleanAction::ScalingMethod(Boolean2DAction::Increase)),
        "scaling-method-dec" => Some(BooleanAction::ScalingMethod(Boolean2DAction::Decrease)),
        "custom-scaling-resolution-width-inc" => Some(BooleanAction::ScalingResolutionWidth(Boolean2DAction::Increase)),
        "custom-scaling-resolution-width-dec" => Some(BooleanAction::ScalingResolutionWidth(Boolean2DAction::Decrease)),
        "custom-scaling-resolution-height-inc" => Some(BooleanAction::ScalingResolutionHeight(Boolean2DAction::Increase)),
        "custom-scaling-resolution-height-dec" => Some(BooleanAction::ScalingResolutionHeight(Boolean2DAction::Decrease)),
        "custom-scaling-aspect-ratio-x-inc" => Some(BooleanAction::ScalingAspectRatioX(Boolean2DAction::Increase)),
        "custom-scaling-aspect-ratio-x-dec" => Some(BooleanAction::ScalingAspectRatioX(Boolean2DAction::Decrease)),
        "custom-scaling-aspect-ratio-y-inc" => Some(BooleanAction::ScalingAspectRatioY(Boolean2DAction::Increase)),
        "custom-scaling-aspect-ratio-y-dec" => Some(BooleanAction::ScalingAspectRatioY(Boolean2DAction::Decrease)),
        "f" | "move-speed-inc" => Some(BooleanAction::TranslationSpeed(Boolean2DAction::Increase)),
        "shift+f" | "move-speed-dec" => Some(BooleanAction::TranslationSpeed(Boolean2DAction::Decrease)),
        "r" | "pixel-speed-inc" => Some(BooleanAction::FilterSpeed(Boolean2DAction::Increase)),
        "shift+r" | "pixel-speed-dec" => Some(BooleanAction::FilterSpeed(Boolean2DAction::Decrease)),
        "turn-speed-inc" => Some(BooleanAction::TurnSpeed(Boolean2DAction::Increase)),
        "turn-speed-dec" => Some(BooleanAction::TurnSpeed(Boolean2DAction::Decrease)),
        "t" | "reset-speeds" => Some(BooleanAction::ResetSpeeds),
        "camera-zoom-inc" => Some(BooleanAction::CameraZoom(Boolean2DAction::Increase)),
        "camera-zoom-dec" => Some(BooleanAction::CameraZoom(Boolean2DAction::Decrease)),
        "o" | "pixel-width-inc" => Some(BooleanAction::PixelWidth(Boolean2DAction::Increase)),
        "shift+o" | "pixel-width-dec" => Some(BooleanAction::PixelWidth(Boolean2DAction::Decrease)),
        "j" | "blur-level-inc" => Some(BooleanAction::Blur(Boolean2DAction::Increase)),
        "shift+j" | "blur-level-dec" => Some(BooleanAction::Blur(Boolean2DAction::Decrease)),
        "k" | "vertical-lpp-inc" => Some(BooleanAction::VerticalLpp(Boolean2DAction::Increase)),
        "shift+k" | "vertical-lpp-dec" => Some(BooleanAction::VerticalLpp(Boolean2DAction::Decrease)),
        "l" | "horizontal-lpp-inc" => Some(BooleanAction::HorizontalLpp(Boolean2DAction::Increase)),
        "shift+l" | "horizontal-lpp-dec" => Some(BooleanAction::HorizontalLpp(Boolean2DAction::Decrease)),
        "c" | "color-representation-inc" => Some(BooleanAction::NextColorRepresentationKind(Boolean2DAction::Increase)),
        "shift+c" | "color-representation-dec" => Some(BooleanAction::NextColorRepresentationKind(Boolean2DAction::Decrease)),
        "v" | "pixel-geometry-inc" => Some(BooleanAction::NextPixelGeometryKind(Boolean2DAction::Increase)),
        "shift+v" | "pixel-geometry-dec" => Some(BooleanAction::NextPixelGeometryKind(Boolean2DAction::Decrease)),
        "b" | "screen-curvature-inc" => Some(BooleanAction::NextScreenCurvatureType(Boolean2DAction::Increase)),
        "shift+b" | "screen-curvature-dec" => Some(BooleanAction::NextScreenCurvatureType(Boolean2DAction::Decrease)),
        "n" | "pixel-shadow-shape-inc" => Some(BooleanAction::NextPixelShadowShapeKind(Boolean2DAction::Increase)),
        "shift+n" | "pixel-shadow-shape-dec" => Some(BooleanAction::NextPixelShadowShapeKind(Boolean2DAction::Decrease)),
        "y" | "internal-resolution-inc" => Some(BooleanAction::NextInternalResolution(Boolean2DAction::Increase)),
        "shift+y" | "internal-resolution-dec" => Some(BooleanAction::NextInternalResolution(Boolean2DAction::Decrease)),
        "h" | "texture-interpolation-inc" => Some(BooleanAction::NextTextureInterpolation(Boolean2DAction::Increase)),
        "shift+h" | "texture-interpolation-dec" => Some(BooleanAction::NextTextureInterpolation(Boolean2DAction::Decrease)),
        "g" | "camera-movement-mode-inc" => Some(BooleanAction::NextCameraMovementMode(Boolean2DAction::Increase)),
        "shift+g" | "camera-movement-mode-dec" => Some(BooleanAction::NextCameraMovementMode(Boolean2DAction::Decrease)),
        _ => None,
    }
}

#[cfg(test)]
mod test_trigger_hotkey_action {
    #![allow(non_snake_case)]

    use super::*;
    #[test]
    fn test_press__i___release__i() {
        let mut input_owned = Input::default();
        let input = &mut input_owned;
        trigger_hotkey_action(input, "i", Pressed::Yes);
        assert_eq!(format!("{:?}", input.active_pressed_actions), "[(\"i\", PixelVerticalGap(Increase))]");
        trigger_hotkey_action(input, "i", Pressed::No);
        assert_eq!(format!("{:?}", input.active_pressed_actions), "[]");
    }

    #[test]
    fn test_press__i_shift___done() {
        let mut input_owned = Input::default();
        let input = &mut input_owned;
        trigger_hotkey_action(input, "i", Pressed::Yes);
        assert_eq!(format!("{:?}", input.active_pressed_actions), "[(\"i\", PixelVerticalGap(Increase))]");
        trigger_hotkey_action(input, "shift", Pressed::Yes);
        assert_eq!(
            format!("{:?}", input.active_pressed_actions),
            "[(\"shift+i\", PixelVerticalGap(Decrease)), (\"shift\", Shift)]"
        );
    }

    #[test]
    fn test_press__shift_i___done() {
        let mut input_owned = Input::default();
        let input = &mut input_owned;
        trigger_hotkey_action(input, "shift", Pressed::Yes);
        assert_eq!(format!("{:?}", input.active_pressed_actions), "[(\"shift\", Shift)]");
        trigger_hotkey_action(input, "i", Pressed::Yes);
        assert_eq!(
            format!("{:?}", input.active_pressed_actions),
            "[(\"shift\", Shift), (\"shift+i\", PixelVerticalGap(Decrease))]"
        );
    }

    #[test]
    fn test_press__i_shift___release__i() {
        let mut input_owned = Input::default();
        let input = &mut input_owned;
        trigger_hotkey_action(input, "i", Pressed::Yes);
        assert_eq!(format!("{:?}", input.active_pressed_actions), "[(\"i\", PixelVerticalGap(Increase))]");
        trigger_hotkey_action(input, "shift", Pressed::Yes);
        assert_eq!(
            format!("{:?}", input.active_pressed_actions),
            "[(\"shift+i\", PixelVerticalGap(Decrease)), (\"shift\", Shift)]"
        );
        trigger_hotkey_action(input, "i", Pressed::No);
        assert_eq!(format!("{:?}", input.active_pressed_actions), "[(\"shift\", Shift)]");
    }

    #[test]
    fn test_press__shift_i___release__shift() {
        let mut input_owned = Input::default();
        let input = &mut input_owned;
        trigger_hotkey_action(input, "shift", Pressed::Yes);
        assert_eq!(format!("{:?}", input.active_pressed_actions), "[(\"shift\", Shift)]");
        trigger_hotkey_action(input, "i", Pressed::Yes);
        assert_eq!(
            format!("{:?}", input.active_pressed_actions),
            "[(\"shift\", Shift), (\"shift+i\", PixelVerticalGap(Decrease))]"
        );
        trigger_hotkey_action(input, "shift", Pressed::No);
        assert_eq!(format!("{:?}", input.active_pressed_actions), "[(\"i\", PixelVerticalGap(Increase))]");
    }
}
