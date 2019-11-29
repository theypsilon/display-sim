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

use crate::simulation_core_state::Input;

pub fn on_button_action(input: &mut Input, button_action: &str, pressed: bool) -> Option<String> {
    let mut matched = true;
    match button_action {
        "shift" | "left shift" | "right shift" => input.shift = pressed,
        "control" => input.control = pressed,
        "alt" => input.alt = pressed,
        "f4" | "capture-framebuffer" => input.screenshot.input = pressed,
        "reset-camera" => input.reset_position = pressed,
        "reset-filters" => input.reset_filters = pressed,
        "input_focused" => input.input_focused = pressed,
        "canvas_focused" => input.canvas_focused = pressed,
        _ => matched = false,
    };
    if matched || input.input_focused {
        return None;
    }

    let action: String;
    if input.shift {
        action = format!("shift+{}", button_action);
    } else if input.control {
        action = format!("ctrl+{}", button_action);
    } else if input.alt {
        action = format!("alt+{}", button_action);
    } else {
        action = button_action.to_string();
    }
    match action.as_ref() {
        "escape" | "esc" | "quit-simulation" => input.esc.input = pressed,
        " " | "space" | "feature-close-panel" => input.space.input = pressed,
        "+" => input.rotate_left = pressed,
        "-" => input.rotate_right = pressed,
        "arrowleft" | "left" | "←" | "◀" => input.turn_left = pressed,
        "arrowright" | "right" | "→" | "▶" => input.turn_right = pressed,
        "arrowup" | "up" | "↑" | "▲" => input.turn_up = pressed,
        "arrowdown" | "down" | "↓" | "▼" => input.turn_down = pressed,
        "a" => input.walk_left = pressed,
        "d" => input.walk_right = pressed,
        "w" => input.walk_forward = pressed,
        "s" => input.walk_backward = pressed,
        "q" => input.walk_up = pressed,
        "e" => input.walk_down = pressed,
        "scaling-method-inc" => input.scaling_method.increase.input = pressed,
        "scaling-method-dec" => input.scaling_method.decrease.input = pressed,
        "custom-scaling-resolution-width-inc" => input.scaling_resolution_width.increase.input = pressed,
        "custom-scaling-resolution-width-dec" => input.scaling_resolution_width.decrease.input = pressed,
        "custom-scaling-resolution-height-inc" => input.scaling_resolution_height.increase.input = pressed,
        "custom-scaling-resolution-height-dec" => input.scaling_resolution_height.decrease.input = pressed,
        "custom-scaling-aspect-ratio-x-inc" => input.scaling_aspect_ratio_x.increase.input = pressed,
        "custom-scaling-aspect-ratio-x-dec" => input.scaling_aspect_ratio_x.decrease.input = pressed,
        "custom-scaling-aspect-ratio-y-inc" => input.scaling_aspect_ratio_y.increase.input = pressed,
        "custom-scaling-aspect-ratio-y-dec" => input.scaling_aspect_ratio_y.decrease.input = pressed,
        "f" | "move-speed-inc" => input.translation_speed.increase.input = pressed,
        "shift+f" | "move-speed-dec" => input.translation_speed.decrease.input = pressed,
        "r" | "pixel-speed-inc" => input.filter_speed.increase.input = pressed,
        "shift+r" | "pixel-speed-dec" => input.filter_speed.decrease.input = pressed,
        "t" | "reset-speeds" => input.reset_speeds = pressed,
        "camera-zoom-inc" => input.camera_zoom.increase = pressed,
        "camera-zoom-dec" => input.camera_zoom.decrease = pressed,
        "u" | "pixel-horizontal-gap-inc" => input.pixel_horizontal_gap.increase = pressed,
        "shift+u" | "pixel-horizontal-gap-dec" => input.pixel_horizontal_gap.decrease = pressed,
        "i" | "pixel-vertical-gap-inc" => input.pixel_vertical_gap.increase = pressed,
        "shift+i" | "pixel-vertical-gap-dec" => input.pixel_vertical_gap.decrease = pressed,
        "o" | "pixel-width-inc" => input.pixel_width.increase = pressed,
        "shift+o" | "pixel-width-dec" => input.pixel_width.decrease = pressed,
        "p" => input.pixel_spread.increase = pressed,
        "shift+p" => input.pixel_spread.decrease = pressed,
        "j" | "blur-level-inc" => input.blur.increase.input = pressed,
        "shift+j" | "blur-level-dec" => input.blur.decrease.input = pressed,
        "k" | "vertical-lpp-inc" => input.vertical_lpp.increase.input = pressed,
        "shift+k" | "vertical-lpp-dec" => input.vertical_lpp.decrease.input = pressed,
        "l" | "horizontal-lpp-inc" => input.horizontal_lpp.increase.input = pressed,
        "shift+l" | "horizontal-lpp-dec" => input.horizontal_lpp.decrease.input = pressed,
        "z" | "&lt;" | "pixel-contrast-inc" => input.contrast.increase = pressed,
        "shift+z" | "pixel-contrast-dec" => input.contrast.decrease = pressed,
        "x" | "pixel-brightness-inc" => input.bright.increase = pressed,
        "shift+x" | "pixel-brightness-dec" => input.bright.decrease = pressed,
        "c" | "color-representation-inc" => input.next_color_representation_kind.increase.input = pressed,
        "shift+c" | "color-representation-dec" => input.next_color_representation_kind.decrease.input = pressed,
        "v" | "pixel-geometry-inc" => input.next_pixel_geometry_kind.increase.input = pressed,
        "shift+v" | "pixel-geometry-dec" => input.next_pixel_geometry_kind.decrease.input = pressed,
        "b" | "screen-curvature-inc" => input.next_screen_curvature_type.increase.input = pressed,
        "shift+b" | "screen-curvature-dec" => input.next_screen_curvature_type.decrease.input = pressed,
        "n" | "pixel-shadow-shape-inc" => input.next_pixel_shadow_shape_kind.increase.input = pressed,
        "shift+n" | "pixel-shadow-shape-dec" => input.next_pixel_shadow_shape_kind.decrease.input = pressed,
        "m" | "pixel-shadow-height-inc" => input.next_pixels_shadow_height.increase = pressed,
        "shift+m" | "pixel-shadow-height-dec" => input.next_pixels_shadow_height.decrease = pressed,
        "y" | "internal-resolution-inc" => input.next_internal_resolution.increase.input = pressed,
        "shift+y" | "internal-resolution-dec" => input.next_internal_resolution.decrease.input = pressed,
        "h" | "texture-interpolation-inc" => input.next_texture_interpolation.increase.input = pressed,
        "shift+h" | "texture-interpolation-dec" => input.next_texture_interpolation.decrease.input = pressed,
        "," | "backlight-percent-inc" => input.backlight_percent.increase = pressed,
        "." | "backlight-percent-dec" => input.backlight_percent.decrease = pressed,
        "g" | "camera-movement-mode-inc" => input.next_camera_movement_mode.increase.input = pressed,
        "shift+g" | "camera-movement-mode-dec" => input.next_camera_movement_mode.decrease.input = pressed,
        _ => return Some(action),
    }
    None
}
