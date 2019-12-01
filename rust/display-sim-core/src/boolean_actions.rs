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

pub(crate) fn trigger_hotkey_action(input: &mut Input, keycode: &str, pressed: Pressed) -> Option<String> {
    let action = to_boolean_action(keycode);
    let (maybe_keycode, action) = if input.shift {
        get_modified_action(action, keycode, BooleanAction::Shift)
    } else if input.control {
        get_modified_action(action, keycode, BooleanAction::Control)
    } else if input.alt {
        get_modified_action(action, keycode, BooleanAction::Alt)
    } else {
        (None, action)
    };
    match action {
        BooleanAction::Shift => react_to_modifier(input, BooleanAction::Shift, pressed),
        BooleanAction::Control => react_to_modifier(input, BooleanAction::Control, pressed),
        BooleanAction::Alt => react_to_modifier(input, BooleanAction::Alt, pressed),
        _ => {}
    }
    if pressed == Pressed::Yes && input.active_pressed_actions.iter().any(|(_, active_action)| *active_action == action) {
        return None;
    }
    match handle_action(input, action, pressed) {
        ActionUsed::No => Some(keycode.into()),
        ActionUsed::Yes => {
            match pressed {
                Pressed::Yes => {
                    let keycode = if let Some(keycode) = maybe_keycode { keycode } else { keycode.into() };
                    input.active_pressed_actions.push((keycode, action));
                }
                Pressed::No => remove_action(input, action),
            }
            None
        }
    }
}

fn get_modified_action(action: BooleanAction, keycode: &str, modifier: BooleanAction) -> (Option<String>, BooleanAction) {
    if action == modifier {
        return (None, action);
    }
    let modified_keycode = format!("{}{}", get_modifier_code(modifier), keycode);
    let modified_action = to_boolean_action(&modified_keycode);
    if modified_action != BooleanAction::None {
        (Some(modified_keycode), modified_action)
    } else {
        (None, action)
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
        let modified_action = to_boolean_action(&modified_keycode);
        if let BooleanAction::None = modified_action {
            continue;
        }
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
        let unmodified_action = to_boolean_action(&unmodified_keycode);
        if let BooleanAction::None = unmodified_action {
            continue;
        }
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
            #[cfg(debug_assertions)]
            assert_eq!(index, None);
            index = Some(i);
            #[cfg(not(debug_assertions))]
            break;
        }
    }
    if let Some(i) = index {
        input.active_pressed_actions.remove(i);
    }
}

enum ActionUsed {
    Yes,
    No,
}

fn handle_action(input: &mut Input, action: BooleanAction, pressed: Pressed) -> ActionUsed {
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
        BooleanAction::PixelHorizontalGap(Boolean2DAction::Increase) => input.pixel_horizontal_gap.increase = pressed,
        BooleanAction::PixelHorizontalGap(Boolean2DAction::Decrease) => input.pixel_horizontal_gap.decrease = pressed,
        BooleanAction::PixelVerticalGap(Boolean2DAction::Increase) => input.pixel_vertical_gap.increase = pressed,
        BooleanAction::PixelVerticalGap(Boolean2DAction::Decrease) => input.pixel_vertical_gap.decrease = pressed,
        BooleanAction::PixelWidth(Boolean2DAction::Increase) => input.pixel_width.increase = pressed,
        BooleanAction::PixelWidth(Boolean2DAction::Decrease) => input.pixel_width.decrease = pressed,
        BooleanAction::PixelSpread(Boolean2DAction::Increase) => input.pixel_spread.increase = pressed,
        BooleanAction::PixelSpread(Boolean2DAction::Decrease) => input.pixel_spread.decrease = pressed,
        BooleanAction::Blur(Boolean2DAction::Increase) => input.blur.increase.input = pressed,
        BooleanAction::Blur(Boolean2DAction::Decrease) => input.blur.decrease.input = pressed,
        BooleanAction::VerticalLpp(Boolean2DAction::Increase) => input.vertical_lpp.increase.input = pressed,
        BooleanAction::VerticalLpp(Boolean2DAction::Decrease) => input.vertical_lpp.decrease.input = pressed,
        BooleanAction::HorizontalLpp(Boolean2DAction::Increase) => input.horizontal_lpp.increase.input = pressed,
        BooleanAction::HorizontalLpp(Boolean2DAction::Decrease) => input.horizontal_lpp.decrease.input = pressed,
        BooleanAction::Contrast(Boolean2DAction::Increase) => input.contrast.increase = pressed,
        BooleanAction::Contrast(Boolean2DAction::Decrease) => input.contrast.decrease = pressed,
        BooleanAction::Bright(Boolean2DAction::Increase) => input.bright.increase = pressed,
        BooleanAction::Bright(Boolean2DAction::Decrease) => input.bright.decrease = pressed,
        BooleanAction::NextColorRepresentationKind(Boolean2DAction::Increase) => input.next_color_representation_kind.increase.input = pressed,
        BooleanAction::NextColorRepresentationKind(Boolean2DAction::Decrease) => input.next_color_representation_kind.decrease.input = pressed,
        BooleanAction::NextPixelGeometryKind(Boolean2DAction::Increase) => input.next_pixel_geometry_kind.increase.input = pressed,
        BooleanAction::NextPixelGeometryKind(Boolean2DAction::Decrease) => input.next_pixel_geometry_kind.decrease.input = pressed,
        BooleanAction::NextScreenCurvatureType(Boolean2DAction::Increase) => input.next_screen_curvature_type.increase.input = pressed,
        BooleanAction::NextScreenCurvatureType(Boolean2DAction::Decrease) => input.next_screen_curvature_type.decrease.input = pressed,
        BooleanAction::NextPixelShadowShapeKind(Boolean2DAction::Increase) => input.next_pixel_shadow_shape_kind.increase.input = pressed,
        BooleanAction::NextPixelShadowShapeKind(Boolean2DAction::Decrease) => input.next_pixel_shadow_shape_kind.decrease.input = pressed,
        BooleanAction::NextPixelsShadowHeight(Boolean2DAction::Increase) => input.next_pixels_shadow_height.increase = pressed,
        BooleanAction::NextPixelsShadowHeight(Boolean2DAction::Decrease) => input.next_pixels_shadow_height.decrease = pressed,
        BooleanAction::NextInternalResolution(Boolean2DAction::Increase) => input.next_internal_resolution.increase.input = pressed,
        BooleanAction::NextInternalResolution(Boolean2DAction::Decrease) => input.next_internal_resolution.decrease.input = pressed,
        BooleanAction::NextTextureInterpolation(Boolean2DAction::Increase) => input.next_texture_interpolation.increase.input = pressed,
        BooleanAction::NextTextureInterpolation(Boolean2DAction::Decrease) => input.next_texture_interpolation.decrease.input = pressed,
        BooleanAction::BacklightPercent(Boolean2DAction::Increase) => input.backlight_percent.increase = pressed,
        BooleanAction::BacklightPercent(Boolean2DAction::Decrease) => input.backlight_percent.decrease = pressed,
        BooleanAction::NextCameraMovementMode(Boolean2DAction::Increase) => input.next_camera_movement_mode.increase.input = pressed,
        BooleanAction::NextCameraMovementMode(Boolean2DAction::Decrease) => input.next_camera_movement_mode.decrease.input = pressed,
        BooleanAction::TurnSpeed(Boolean2DAction::Increase) => input.turn_speed.increase.input = pressed,
        BooleanAction::TurnSpeed(Boolean2DAction::Decrease) => input.turn_speed.decrease.input = pressed,
        BooleanAction::MouseClick => input.mouse_click.input = pressed,

        BooleanAction::None => return ActionUsed::No,
    }
    ActionUsed::Yes
}

fn to_boolean_action(boolean_action: &str) -> BooleanAction {
    match boolean_action {
        "mouse_click" => BooleanAction::MouseClick,
        "shift" | "left shift" | "right shift" => BooleanAction::Shift,
        "control" => BooleanAction::Control,
        "alt" => BooleanAction::Alt,
        "f4" | "capture-framebuffer" => BooleanAction::Screenshot,
        "reset-camera" => BooleanAction::ResetPosition,
        "reset-filters" => BooleanAction::ResetFilters,
        "input_focused" => BooleanAction::InputFocused,
        "canvas_focused" => BooleanAction::CanvasFocused,
        "escape" | "esc" | "quit-simulation" => BooleanAction::Esc,
        " " | "space" | "feature-close-panel" => BooleanAction::Space,
        "+" => BooleanAction::RotateLeft,
        "-" => BooleanAction::RotateRight,
        "arrowleft" | "left" | "←" | "◀" => BooleanAction::TurnLeft,
        "arrowright" | "right" | "→" | "▶" => BooleanAction::TurnRight,
        "arrowup" | "up" | "↑" | "▲" => BooleanAction::TurnUp,
        "arrowdown" | "down" | "↓" | "▼" => BooleanAction::TurnDown,
        "a" => BooleanAction::WalkLeft,
        "d" => BooleanAction::WalkRight,
        "w" => BooleanAction::WalkForward,
        "s" => BooleanAction::WalkBackward,
        "q" => BooleanAction::WalkUp,
        "e" => BooleanAction::WalkDown,
        "scaling-method-inc" => BooleanAction::ScalingMethod(Boolean2DAction::Increase),
        "scaling-method-dec" => BooleanAction::ScalingMethod(Boolean2DAction::Decrease),
        "custom-scaling-resolution-width-inc" => BooleanAction::ScalingResolutionWidth(Boolean2DAction::Increase),
        "custom-scaling-resolution-width-dec" => BooleanAction::ScalingResolutionWidth(Boolean2DAction::Decrease),
        "custom-scaling-resolution-height-inc" => BooleanAction::ScalingResolutionHeight(Boolean2DAction::Increase),
        "custom-scaling-resolution-height-dec" => BooleanAction::ScalingResolutionHeight(Boolean2DAction::Decrease),
        "custom-scaling-aspect-ratio-x-inc" => BooleanAction::ScalingAspectRatioX(Boolean2DAction::Increase),
        "custom-scaling-aspect-ratio-x-dec" => BooleanAction::ScalingAspectRatioX(Boolean2DAction::Decrease),
        "custom-scaling-aspect-ratio-y-inc" => BooleanAction::ScalingAspectRatioY(Boolean2DAction::Increase),
        "custom-scaling-aspect-ratio-y-dec" => BooleanAction::ScalingAspectRatioY(Boolean2DAction::Decrease),
        "f" | "move-speed-inc" => BooleanAction::TranslationSpeed(Boolean2DAction::Increase),
        "shift+f" | "move-speed-dec" => BooleanAction::TranslationSpeed(Boolean2DAction::Decrease),
        "r" | "pixel-speed-inc" => BooleanAction::FilterSpeed(Boolean2DAction::Increase),
        "shift+r" | "pixel-speed-dec" => BooleanAction::FilterSpeed(Boolean2DAction::Decrease),
        "turn-speed-inc" => BooleanAction::TurnSpeed(Boolean2DAction::Increase),
        "turn-speed-dec" => BooleanAction::TurnSpeed(Boolean2DAction::Decrease),
        "t" | "reset-speeds" => BooleanAction::ResetSpeeds,
        "camera-zoom-inc" => BooleanAction::CameraZoom(Boolean2DAction::Increase),
        "camera-zoom-dec" => BooleanAction::CameraZoom(Boolean2DAction::Decrease),
        "u" | "pixel-horizontal-gap-inc" => BooleanAction::PixelHorizontalGap(Boolean2DAction::Increase),
        "shift+u" | "pixel-horizontal-gap-dec" => BooleanAction::PixelHorizontalGap(Boolean2DAction::Decrease),
        "i" | "pixel-vertical-gap-inc" => BooleanAction::PixelVerticalGap(Boolean2DAction::Increase),
        "shift+i" | "pixel-vertical-gap-dec" => BooleanAction::PixelVerticalGap(Boolean2DAction::Decrease),
        "o" | "pixel-width-inc" => BooleanAction::PixelWidth(Boolean2DAction::Increase),
        "shift+o" | "pixel-width-dec" => BooleanAction::PixelWidth(Boolean2DAction::Decrease),
        "p" => BooleanAction::PixelSpread(Boolean2DAction::Increase),
        "shift+p" => BooleanAction::PixelSpread(Boolean2DAction::Decrease),
        "j" | "blur-level-inc" => BooleanAction::Blur(Boolean2DAction::Increase),
        "shift+j" | "blur-level-dec" => BooleanAction::Blur(Boolean2DAction::Decrease),
        "k" | "vertical-lpp-inc" => BooleanAction::VerticalLpp(Boolean2DAction::Increase),
        "shift+k" | "vertical-lpp-dec" => BooleanAction::VerticalLpp(Boolean2DAction::Decrease),
        "l" | "horizontal-lpp-inc" => BooleanAction::HorizontalLpp(Boolean2DAction::Increase),
        "shift+l" | "horizontal-lpp-dec" => BooleanAction::HorizontalLpp(Boolean2DAction::Decrease),
        "z" | "&lt;" | "pixel-contrast-inc" => BooleanAction::Contrast(Boolean2DAction::Increase),
        "shift+z" | "pixel-contrast-dec" => BooleanAction::Contrast(Boolean2DAction::Decrease),
        "x" | "pixel-brightness-inc" => BooleanAction::Bright(Boolean2DAction::Increase),
        "shift+x" | "pixel-brightness-dec" => BooleanAction::Bright(Boolean2DAction::Decrease),
        "c" | "color-representation-inc" => BooleanAction::NextColorRepresentationKind(Boolean2DAction::Increase),
        "shift+c" | "color-representation-dec" => BooleanAction::NextColorRepresentationKind(Boolean2DAction::Decrease),
        "v" | "pixel-geometry-inc" => BooleanAction::NextPixelGeometryKind(Boolean2DAction::Increase),
        "shift+v" | "pixel-geometry-dec" => BooleanAction::NextPixelGeometryKind(Boolean2DAction::Decrease),
        "b" | "screen-curvature-inc" => BooleanAction::NextScreenCurvatureType(Boolean2DAction::Increase),
        "shift+b" | "screen-curvature-dec" => BooleanAction::NextScreenCurvatureType(Boolean2DAction::Decrease),
        "n" | "pixel-shadow-shape-inc" => BooleanAction::NextPixelShadowShapeKind(Boolean2DAction::Increase),
        "shift+n" | "pixel-shadow-shape-dec" => BooleanAction::NextPixelShadowShapeKind(Boolean2DAction::Decrease),
        "m" | "pixel-shadow-height-inc" => BooleanAction::NextPixelsShadowHeight(Boolean2DAction::Increase),
        "shift+m" | "pixel-shadow-height-dec" => BooleanAction::NextPixelsShadowHeight(Boolean2DAction::Decrease),
        "y" | "internal-resolution-inc" => BooleanAction::NextInternalResolution(Boolean2DAction::Increase),
        "shift+y" | "internal-resolution-dec" => BooleanAction::NextInternalResolution(Boolean2DAction::Decrease),
        "h" | "texture-interpolation-inc" => BooleanAction::NextTextureInterpolation(Boolean2DAction::Increase),
        "shift+h" | "texture-interpolation-dec" => BooleanAction::NextTextureInterpolation(Boolean2DAction::Decrease),
        "," | "backlight-percent-inc" => BooleanAction::BacklightPercent(Boolean2DAction::Increase),
        "." | "backlight-percent-dec" => BooleanAction::BacklightPercent(Boolean2DAction::Decrease),
        "g" | "camera-movement-mode-inc" => BooleanAction::NextCameraMovementMode(Boolean2DAction::Increase),
        "shift+g" | "camera-movement-mode-dec" => BooleanAction::NextCameraMovementMode(Boolean2DAction::Decrease),
        _ => BooleanAction::None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_action_i_then_i() {
        let mut input_owned = Input::default();
        let input = &mut input_owned;
        trigger_hotkey_action(input, "i", Pressed::Yes);
        assert_eq!(format!("{:?}", input.active_pressed_actions), "[(\"i\", PixelVerticalGap(Increase))]");
        trigger_hotkey_action(input, "i", Pressed::No);
        assert_eq!(format!("{:?}", input.active_pressed_actions), "[]");
    }

    #[test]
    fn test_action_i_shift_done() {
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
    fn test_action_shift_i_done() {
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
    fn test_action_i_shift_then_i() {
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
    fn test_action_shift_i_then_shift() {
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
