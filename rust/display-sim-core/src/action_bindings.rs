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

use crate::input_types::{Button2DAction, ButtonAction, Input, KeyCodeButtonAction};

pub fn on_button_action(input: &mut Input, key_code: &str, pressed: bool) -> Option<String> {
    let action = to_boolean_action(key_code);
    let (key_code, action) = if input.shift {
        get_modified_action(action, key_code, ButtonAction::Shift)
    } else if input.control {
        get_modified_action(action, key_code, ButtonAction::Control)
    } else if input.alt {
        get_modified_action(action, key_code, ButtonAction::Alt)
    } else {
        (key_code.into(), action)
    };
    match action {
        ButtonAction::Shift => react_to_modifier(input, ButtonAction::Shift, pressed),
        ButtonAction::Control => react_to_modifier(input, ButtonAction::Control, pressed),
        ButtonAction::Alt => react_to_modifier(input, ButtonAction::Alt, pressed),
        _ => {}
    }
    if pressed && input.active_button_actions.iter().any(|(_, active_action)| *active_action == action) {
        return None;
    }
    if !handle_action(input, action, pressed) {
        Some(key_code)
    } else {
        if pressed {
            input.active_button_actions.push((key_code, action));
        } else {
            remove_action(input, action);
        }
        None
    }
}

fn get_modified_action(action: ButtonAction, key_code: &str, modifier: ButtonAction) -> KeyCodeButtonAction {
    if action == modifier {
        return (key_code.into(), action);
    }
    let modified_key_code = format!("{}{}", get_modifier_code(modifier), key_code);
    let modified_action = to_boolean_action(&modified_key_code);
    if modified_action != ButtonAction::None {
        (modified_key_code, modified_action)
    } else {
        (key_code.into(), action)
    }
}

fn get_modifier_code(modifier: ButtonAction) -> &'static str {
    match modifier {
        ButtonAction::Shift => "shift+",
        ButtonAction::Control => "ctrl+",
        ButtonAction::Alt => "alt+",
        _ => unreachable!(),
    }
}

fn react_to_modifier(input: &mut Input, modifier: ButtonAction, pressed: bool) {
    let modifier_code = get_modifier_code(modifier);
    let (to_add, to_delete) = if pressed {
        modify_active_buttons(&input.active_button_actions, modifier_code)
    } else {
        unmodify_active_buttons(&input.active_button_actions, modifier_code)
    };
    resolve_modifications(input, to_add, to_delete);
}

type IndexButtonAction = (usize, ButtonAction);

fn modify_active_buttons(active_buttons: &Vec<KeyCodeButtonAction>, modifier_code: &str) -> (Vec<KeyCodeButtonAction>, Vec<IndexButtonAction>) {
    let mut to_delete = Vec::new();
    let mut to_add = Vec::new();
    for (i, (key_code, action)) in active_buttons.iter().enumerate() {
        let modified_key_code = format!("{}{}", modifier_code, key_code);
        let modified_action = to_boolean_action(&modified_key_code);
        if let ButtonAction::None = modified_action {
            continue;
        }
        to_delete.push((i, *action));
        to_add.push((modified_key_code, modified_action));
    }
    (to_add, to_delete)
}

fn unmodify_active_buttons(active_buttons: &Vec<KeyCodeButtonAction>, modifier_code: &str) -> (Vec<KeyCodeButtonAction>, Vec<IndexButtonAction>) {
    let mut to_delete = Vec::new();
    let mut to_add = Vec::new();
    for (i, (key_code, action)) in active_buttons.iter().enumerate() {
        if !key_code.starts_with(modifier_code) {
            continue;
        }
        to_delete.push((i, *action));
        let unmodified_key_code = key_code.replace(modifier_code, "");
        let unmodified_action = to_boolean_action(&unmodified_key_code);
        if let ButtonAction::None = unmodified_action {
            continue;
        }
        to_add.push((unmodified_key_code, unmodified_action));
    }
    (to_add, to_delete)
}

fn resolve_modifications(input: &mut Input, to_add: Vec<KeyCodeButtonAction>, to_delete: Vec<IndexButtonAction>) {
    for (i, removed_action) in to_delete.into_iter() {
        handle_action(input, removed_action, false);
        input.active_button_actions.remove(i);
    }
    for (modified_key_code, modified_action) in to_add.into_iter() {
        handle_action(input, modified_action, true);
        input.active_button_actions.push((modified_key_code, modified_action));
    }
}

fn remove_action(input: &mut Input, action: ButtonAction) {
    let mut index = None;
    for (i, value) in input.active_button_actions.iter().enumerate() {
        if value.1 == action {
            index = Some(i);
            break;
        }
    }
    if let Some(i) = index {
        input.active_button_actions.remove(i);
    }
}

fn handle_action(input: &mut Input, action: ButtonAction, pressed: bool) -> bool {
    match action {
        ButtonAction::Shift => input.shift = pressed,
        ButtonAction::Control => input.control = pressed,
        ButtonAction::Alt => input.alt = pressed,
        ButtonAction::Screenshot => input.screenshot.input = pressed,
        ButtonAction::ResetPosition => input.reset_position = pressed,
        ButtonAction::ResetFilters => input.reset_filters = pressed,
        ButtonAction::InputFocused => input.input_focused = pressed,
        ButtonAction::CanvasFocused => input.canvas_focused = pressed,
        ButtonAction::Esc => input.esc.input = pressed,
        ButtonAction::Space => input.space.input = pressed,
        ButtonAction::RotateLeft => input.rotate_left = pressed,
        ButtonAction::RotateRight => input.rotate_right = pressed,
        ButtonAction::TurnLeft => input.turn_left = pressed,
        ButtonAction::TurnRight => input.turn_right = pressed,
        ButtonAction::TurnUp => input.turn_up = pressed,
        ButtonAction::TurnDown => input.turn_down = pressed,
        ButtonAction::WalkLeft => input.walk_left = pressed,
        ButtonAction::WalkRight => input.walk_right = pressed,
        ButtonAction::WalkForward => input.walk_forward = pressed,
        ButtonAction::WalkBackward => input.walk_backward = pressed,
        ButtonAction::WalkUp => input.walk_up = pressed,
        ButtonAction::WalkDown => input.walk_down = pressed,
        ButtonAction::ScalingMethod(Button2DAction::Increase) => input.scaling_method.increase.input = pressed,
        ButtonAction::ScalingMethod(Button2DAction::Decrease) => input.scaling_method.decrease.input = pressed,
        ButtonAction::ScalingResolutionWidth(Button2DAction::Increase) => input.scaling_resolution_width.increase.input = pressed,
        ButtonAction::ScalingResolutionWidth(Button2DAction::Decrease) => input.scaling_resolution_width.decrease.input = pressed,
        ButtonAction::ScalingResolutionHeight(Button2DAction::Increase) => input.scaling_resolution_height.increase.input = pressed,
        ButtonAction::ScalingResolutionHeight(Button2DAction::Decrease) => input.scaling_resolution_height.decrease.input = pressed,
        ButtonAction::ScalingAspectRatioX(Button2DAction::Increase) => input.scaling_aspect_ratio_x.increase.input = pressed,
        ButtonAction::ScalingAspectRatioX(Button2DAction::Decrease) => input.scaling_aspect_ratio_x.decrease.input = pressed,
        ButtonAction::ScalingAspectRatioY(Button2DAction::Increase) => input.scaling_aspect_ratio_y.increase.input = pressed,
        ButtonAction::ScalingAspectRatioY(Button2DAction::Decrease) => input.scaling_aspect_ratio_y.decrease.input = pressed,
        ButtonAction::TranslationSpeed(Button2DAction::Increase) => input.translation_speed.increase.input = pressed,
        ButtonAction::TranslationSpeed(Button2DAction::Decrease) => input.translation_speed.decrease.input = pressed,
        ButtonAction::FilterSpeed(Button2DAction::Increase) => input.filter_speed.increase.input = pressed,
        ButtonAction::FilterSpeed(Button2DAction::Decrease) => input.filter_speed.decrease.input = pressed,
        ButtonAction::ResetSpeeds => input.reset_speeds = pressed,
        ButtonAction::CameraZoom(Button2DAction::Increase) => input.camera_zoom.increase = pressed,
        ButtonAction::CameraZoom(Button2DAction::Decrease) => input.camera_zoom.decrease = pressed,
        ButtonAction::PixelHorizontalGap(Button2DAction::Increase) => input.pixel_horizontal_gap.increase = pressed,
        ButtonAction::PixelHorizontalGap(Button2DAction::Decrease) => input.pixel_horizontal_gap.decrease = pressed,
        ButtonAction::PixelVerticalGap(Button2DAction::Increase) => input.pixel_vertical_gap.increase = pressed,
        ButtonAction::PixelVerticalGap(Button2DAction::Decrease) => input.pixel_vertical_gap.decrease = pressed,
        ButtonAction::PixelWidth(Button2DAction::Increase) => input.pixel_width.increase = pressed,
        ButtonAction::PixelWidth(Button2DAction::Decrease) => input.pixel_width.decrease = pressed,
        ButtonAction::PixelSpread(Button2DAction::Increase) => input.pixel_spread.increase = pressed,
        ButtonAction::PixelSpread(Button2DAction::Decrease) => input.pixel_spread.decrease = pressed,
        ButtonAction::Blur(Button2DAction::Increase) => input.blur.increase.input = pressed,
        ButtonAction::Blur(Button2DAction::Decrease) => input.blur.decrease.input = pressed,
        ButtonAction::VerticalLpp(Button2DAction::Increase) => input.vertical_lpp.increase.input = pressed,
        ButtonAction::VerticalLpp(Button2DAction::Decrease) => input.vertical_lpp.decrease.input = pressed,
        ButtonAction::HorizontalLpp(Button2DAction::Increase) => input.horizontal_lpp.increase.input = pressed,
        ButtonAction::HorizontalLpp(Button2DAction::Decrease) => input.horizontal_lpp.decrease.input = pressed,
        ButtonAction::Contrast(Button2DAction::Increase) => input.contrast.increase = pressed,
        ButtonAction::Contrast(Button2DAction::Decrease) => input.contrast.decrease = pressed,
        ButtonAction::Bright(Button2DAction::Increase) => input.bright.increase = pressed,
        ButtonAction::Bright(Button2DAction::Decrease) => input.bright.decrease = pressed,
        ButtonAction::NextColorRepresentationKind(Button2DAction::Increase) => input.next_color_representation_kind.increase.input = pressed,
        ButtonAction::NextColorRepresentationKind(Button2DAction::Decrease) => input.next_color_representation_kind.decrease.input = pressed,
        ButtonAction::NextPixelGeometryKind(Button2DAction::Increase) => input.next_pixel_geometry_kind.increase.input = pressed,
        ButtonAction::NextPixelGeometryKind(Button2DAction::Decrease) => input.next_pixel_geometry_kind.decrease.input = pressed,
        ButtonAction::NextScreenCurvatureType(Button2DAction::Increase) => input.next_screen_curvature_type.increase.input = pressed,
        ButtonAction::NextScreenCurvatureType(Button2DAction::Decrease) => input.next_screen_curvature_type.decrease.input = pressed,
        ButtonAction::NextPixelShadowShapeKind(Button2DAction::Increase) => input.next_pixel_shadow_shape_kind.increase.input = pressed,
        ButtonAction::NextPixelShadowShapeKind(Button2DAction::Decrease) => input.next_pixel_shadow_shape_kind.decrease.input = pressed,
        ButtonAction::NextPixelsShadowHeight(Button2DAction::Increase) => input.next_pixels_shadow_height.increase = pressed,
        ButtonAction::NextPixelsShadowHeight(Button2DAction::Decrease) => input.next_pixels_shadow_height.decrease = pressed,
        ButtonAction::NextInternalResolution(Button2DAction::Increase) => input.next_internal_resolution.increase.input = pressed,
        ButtonAction::NextInternalResolution(Button2DAction::Decrease) => input.next_internal_resolution.decrease.input = pressed,
        ButtonAction::NextTextureInterpolation(Button2DAction::Increase) => input.next_texture_interpolation.increase.input = pressed,
        ButtonAction::NextTextureInterpolation(Button2DAction::Decrease) => input.next_texture_interpolation.decrease.input = pressed,
        ButtonAction::BacklightPercent(Button2DAction::Increase) => input.backlight_percent.increase = pressed,
        ButtonAction::BacklightPercent(Button2DAction::Decrease) => input.backlight_percent.decrease = pressed,
        ButtonAction::NextCameraMovementMode(Button2DAction::Increase) => input.next_camera_movement_mode.increase.input = pressed,
        ButtonAction::NextCameraMovementMode(Button2DAction::Decrease) => input.next_camera_movement_mode.decrease.input = pressed,
        ButtonAction::TurnSpeed(Button2DAction::Increase) => input.turn_speed.increase.input = pressed,
        ButtonAction::TurnSpeed(Button2DAction::Decrease) => input.turn_speed.decrease.input = pressed,
        ButtonAction::MouseClick => input.mouse_click.input = pressed,

        ButtonAction::None => return false,
    }
    true
}

fn to_boolean_action(button_action: &str) -> ButtonAction {
    match button_action {
        "mouse_click" => ButtonAction::MouseClick,
        "shift" | "left shift" | "right shift" => ButtonAction::Shift,
        "control" => ButtonAction::Control,
        "alt" => ButtonAction::Alt,
        "f4" | "capture-framebuffer" => ButtonAction::Screenshot,
        "reset-camera" => ButtonAction::ResetPosition,
        "reset-filters" => ButtonAction::ResetFilters,
        "input_focused" => ButtonAction::InputFocused,
        "canvas_focused" => ButtonAction::CanvasFocused,
        "escape" | "esc" | "quit-simulation" => ButtonAction::Esc,
        " " | "space" | "feature-close-panel" => ButtonAction::Space,
        "+" => ButtonAction::RotateLeft,
        "-" => ButtonAction::RotateRight,
        "arrowleft" | "left" | "←" | "◀" => ButtonAction::TurnLeft,
        "arrowright" | "right" | "→" | "▶" => ButtonAction::TurnRight,
        "arrowup" | "up" | "↑" | "▲" => ButtonAction::TurnUp,
        "arrowdown" | "down" | "↓" | "▼" => ButtonAction::TurnDown,
        "a" => ButtonAction::WalkLeft,
        "d" => ButtonAction::WalkRight,
        "w" => ButtonAction::WalkForward,
        "s" => ButtonAction::WalkBackward,
        "q" => ButtonAction::WalkUp,
        "e" => ButtonAction::WalkDown,
        "scaling-method-inc" => ButtonAction::ScalingMethod(Button2DAction::Increase),
        "scaling-method-dec" => ButtonAction::ScalingMethod(Button2DAction::Decrease),
        "custom-scaling-resolution-width-inc" => ButtonAction::ScalingResolutionWidth(Button2DAction::Increase),
        "custom-scaling-resolution-width-dec" => ButtonAction::ScalingResolutionWidth(Button2DAction::Decrease),
        "custom-scaling-resolution-height-inc" => ButtonAction::ScalingResolutionHeight(Button2DAction::Increase),
        "custom-scaling-resolution-height-dec" => ButtonAction::ScalingResolutionHeight(Button2DAction::Decrease),
        "custom-scaling-aspect-ratio-x-inc" => ButtonAction::ScalingAspectRatioX(Button2DAction::Increase),
        "custom-scaling-aspect-ratio-x-dec" => ButtonAction::ScalingAspectRatioX(Button2DAction::Decrease),
        "custom-scaling-aspect-ratio-y-inc" => ButtonAction::ScalingAspectRatioY(Button2DAction::Increase),
        "custom-scaling-aspect-ratio-y-dec" => ButtonAction::ScalingAspectRatioY(Button2DAction::Decrease),
        "f" | "move-speed-inc" => ButtonAction::TranslationSpeed(Button2DAction::Increase),
        "shift+f" | "move-speed-dec" => ButtonAction::TranslationSpeed(Button2DAction::Decrease),
        "r" | "pixel-speed-inc" => ButtonAction::FilterSpeed(Button2DAction::Increase),
        "shift+r" | "pixel-speed-dec" => ButtonAction::FilterSpeed(Button2DAction::Decrease),
        "turn-speed-inc" => ButtonAction::TurnSpeed(Button2DAction::Increase),
        "turn-speed-dec" => ButtonAction::TurnSpeed(Button2DAction::Decrease),
        "t" | "reset-speeds" => ButtonAction::ResetSpeeds,
        "camera-zoom-inc" => ButtonAction::CameraZoom(Button2DAction::Increase),
        "camera-zoom-dec" => ButtonAction::CameraZoom(Button2DAction::Decrease),
        "u" | "pixel-horizontal-gap-inc" => ButtonAction::PixelHorizontalGap(Button2DAction::Increase),
        "shift+u" | "pixel-horizontal-gap-dec" => ButtonAction::PixelHorizontalGap(Button2DAction::Decrease),
        "i" | "pixel-vertical-gap-inc" => ButtonAction::PixelVerticalGap(Button2DAction::Increase),
        "shift+i" | "pixel-vertical-gap-dec" => ButtonAction::PixelVerticalGap(Button2DAction::Decrease),
        "o" | "pixel-width-inc" => ButtonAction::PixelWidth(Button2DAction::Increase),
        "shift+o" | "pixel-width-dec" => ButtonAction::PixelWidth(Button2DAction::Decrease),
        "p" => ButtonAction::PixelSpread(Button2DAction::Increase),
        "shift+p" => ButtonAction::PixelSpread(Button2DAction::Decrease),
        "j" | "blur-level-inc" => ButtonAction::Blur(Button2DAction::Increase),
        "shift+j" | "blur-level-dec" => ButtonAction::Blur(Button2DAction::Decrease),
        "k" | "vertical-lpp-inc" => ButtonAction::VerticalLpp(Button2DAction::Increase),
        "shift+k" | "vertical-lpp-dec" => ButtonAction::VerticalLpp(Button2DAction::Decrease),
        "l" | "horizontal-lpp-inc" => ButtonAction::HorizontalLpp(Button2DAction::Increase),
        "shift+l" | "horizontal-lpp-dec" => ButtonAction::HorizontalLpp(Button2DAction::Decrease),
        "z" | "&lt;" | "pixel-contrast-inc" => ButtonAction::Contrast(Button2DAction::Increase),
        "shift+z" | "pixel-contrast-dec" => ButtonAction::Contrast(Button2DAction::Decrease),
        "x" | "pixel-brightness-inc" => ButtonAction::Bright(Button2DAction::Increase),
        "shift+x" | "pixel-brightness-dec" => ButtonAction::Bright(Button2DAction::Decrease),
        "c" | "color-representation-inc" => ButtonAction::NextColorRepresentationKind(Button2DAction::Increase),
        "shift+c" | "color-representation-dec" => ButtonAction::NextColorRepresentationKind(Button2DAction::Decrease),
        "v" | "pixel-geometry-inc" => ButtonAction::NextPixelGeometryKind(Button2DAction::Increase),
        "shift+v" | "pixel-geometry-dec" => ButtonAction::NextPixelGeometryKind(Button2DAction::Decrease),
        "b" | "screen-curvature-inc" => ButtonAction::NextScreenCurvatureType(Button2DAction::Increase),
        "shift+b" | "screen-curvature-dec" => ButtonAction::NextScreenCurvatureType(Button2DAction::Decrease),
        "n" | "pixel-shadow-shape-inc" => ButtonAction::NextPixelShadowShapeKind(Button2DAction::Increase),
        "shift+n" | "pixel-shadow-shape-dec" => ButtonAction::NextPixelShadowShapeKind(Button2DAction::Decrease),
        "m" | "pixel-shadow-height-inc" => ButtonAction::NextPixelsShadowHeight(Button2DAction::Increase),
        "shift+m" | "pixel-shadow-height-dec" => ButtonAction::NextPixelsShadowHeight(Button2DAction::Decrease),
        "y" | "internal-resolution-inc" => ButtonAction::NextInternalResolution(Button2DAction::Increase),
        "shift+y" | "internal-resolution-dec" => ButtonAction::NextInternalResolution(Button2DAction::Decrease),
        "h" | "texture-interpolation-inc" => ButtonAction::NextTextureInterpolation(Button2DAction::Increase),
        "shift+h" | "texture-interpolation-dec" => ButtonAction::NextTextureInterpolation(Button2DAction::Decrease),
        "," | "backlight-percent-inc" => ButtonAction::BacklightPercent(Button2DAction::Increase),
        "." | "backlight-percent-dec" => ButtonAction::BacklightPercent(Button2DAction::Decrease),
        "g" | "camera-movement-mode-inc" => ButtonAction::NextCameraMovementMode(Button2DAction::Increase),
        "shift+g" | "camera-movement-mode-dec" => ButtonAction::NextCameraMovementMode(Button2DAction::Decrease),
        _ => ButtonAction::None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_action_i_then_i() {
        let mut input_owned = Input::default();
        let input = &mut input_owned;
        on_button_action(input, "i", true);
        assert_eq!(format!("{:?}", input.active_button_actions), "[(\"i\", PixelVerticalGap(Increase))]");
        on_button_action(input, "i", false);
        assert_eq!(format!("{:?}", input.active_button_actions), "[]");
    }

    #[test]
    fn test_action_i_shift_done() {
        let mut input_owned = Input::default();
        let input = &mut input_owned;
        on_button_action(input, "i", true);
        assert_eq!(format!("{:?}", input.active_button_actions), "[(\"i\", PixelVerticalGap(Increase))]");
        on_button_action(input, "shift", true);
        assert_eq!(
            format!("{:?}", input.active_button_actions),
            "[(\"shift+i\", PixelVerticalGap(Decrease)), (\"shift\", Shift)]"
        );
    }

    #[test]
    fn test_action_shift_i_done() {
        let mut input_owned = Input::default();
        let input = &mut input_owned;
        on_button_action(input, "shift", true);
        assert_eq!(format!("{:?}", input.active_button_actions), "[(\"shift\", Shift)]");
        on_button_action(input, "i", true);
        assert_eq!(
            format!("{:?}", input.active_button_actions),
            "[(\"shift\", Shift), (\"shift+i\", PixelVerticalGap(Decrease))]"
        );
    }

    #[test]
    fn test_action_i_shift_then_i() {
        let mut input_owned = Input::default();
        let input = &mut input_owned;
        on_button_action(input, "i", true);
        assert_eq!(format!("{:?}", input.active_button_actions), "[(\"i\", PixelVerticalGap(Increase))]");
        on_button_action(input, "shift", true);
        assert_eq!(
            format!("{:?}", input.active_button_actions),
            "[(\"shift+i\", PixelVerticalGap(Decrease)), (\"shift\", Shift)]"
        );
        on_button_action(input, "i", false);
        assert_eq!(format!("{:?}", input.active_button_actions), "[(\"shift\", Shift)]");
    }

    #[test]
    fn test_action_shift_i_then_shift() {
        let mut input_owned = Input::default();
        let input = &mut input_owned;
        on_button_action(input, "shift", true);
        assert_eq!(format!("{:?}", input.active_button_actions), "[(\"shift\", Shift)]");
        on_button_action(input, "i", true);
        assert_eq!(
            format!("{:?}", input.active_button_actions),
            "[(\"shift\", Shift), (\"shift+i\", PixelVerticalGap(Decrease))]"
        );
        on_button_action(input, "shift", false);
        assert_eq!(format!("{:?}", input.active_button_actions), "[(\"i\", PixelVerticalGap(Increase))]");
    }
}
