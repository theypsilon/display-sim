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

pub fn on_button_action(input: &mut Input, button_action: &str, pressed: bool) -> bool {
    let mut matched = true;
    match button_action {
        "shift" | "left shift" | "right shift" => input.shift = pressed,
        "control" => input.control = pressed,
        "alt" => input.alt = pressed,
        " " | "space" | "feature-close-panel" => input.space.input = pressed,
        "escape" | "esc" | "feature-quit" => input.esc.input = pressed,
        "f4" | "feature-capture-framebuffer" => input.screenshot.input = pressed,
        "reset-camera" => input.reset_position = pressed,
        "reset-filters" => input.reset_filters = pressed,
        "input_focused" => input.input_focused = pressed,
        "canvas_focused" => input.canvas_focused = pressed,
        _ => matched = false,
    };
    if matched || input.input_focused {
        return true;
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
        "f" | "feature-change-move-speed-inc" => input.translation_speed.increase.input = pressed,
        "shift+f" | "feature-change-move-speed-dec" => input.translation_speed.decrease.input = pressed,
        "r" | "feature-change-pixel-speed-inc" => input.filter_speed.increase.input = pressed,
        "shift+r" | "feature-change-pixel-speed-dec" => input.filter_speed.decrease.input = pressed,
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
        "c" | "feature-change-color-representation-inc" => input.next_color_representation_kind.increase.input = pressed,
        "shift+c" | "feature-change-color-representation-dec" => input.next_color_representation_kind.decrease.input = pressed,
        "v" | "feature-change-pixel-geometry-inc" => input.next_pixel_geometry_kind.increase.input = pressed,
        "shift+v" | "feature-change-pixel-geometry-dec" => input.next_pixel_geometry_kind.decrease.input = pressed,
        "b" | "feature-change-screen-curvature-inc" | "feature-change-screen-curvature-basic-inc" => input.next_screen_curvature_type.increase.input = pressed,
        "shift+b" | "feature-change-screen-curvature-dec" | "feature-change-screen-curvature-basic-dec" => {
            input.next_screen_curvature_type.decrease.input = pressed
        }
        "n" | "feature-change-pixel-shadow-shape-inc" => input.next_pixel_shadow_shape_kind.increase.input = pressed,
        "shift+n" | "feature-change-pixel-shadow-shape-dec" => input.next_pixel_shadow_shape_kind.decrease.input = pressed,
        "m" | "feature-change-pixel-shadow-height-inc" => input.next_pixels_shadow_height.increase = pressed,
        "shift+m" | "feature-change-pixel-shadow-height-dec" => input.next_pixels_shadow_height.decrease = pressed,
        "y" | "feature-internal-resolution-inc" | "feature-internal-resolution-basic-inc" => input.next_internal_resolution.increase.input = pressed,
        "shift+y" | "feature-internal-resolution-dec" | "feature-internal-resolution-basic-dec" => input.next_internal_resolution.decrease.input = pressed,
        "h" | "feature-texture-interpolation-inc" => input.next_texture_interpolation.increase.input = pressed,
        "shift+h" | "feature-texture-interpolation-dec" => input.next_texture_interpolation.decrease.input = pressed,
        "," | "feature-backlight-percent-inc" => input.backlight_percent.increase = pressed,
        "." | "feature-backlight-percent-dec" => input.backlight_percent.decrease = pressed,
        "g" | "camera-movement-mode-inc" => input.next_camera_movement_mode.increase.input = pressed,
        "shift+g" | "camera-movement-mode-dec" => input.next_camera_movement_mode.decrease.input = pressed,
        _ => return false,
    }
    true
}
