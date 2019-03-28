use crate::console;
use crate::simulation_state::{DefaultReset, Input};

pub fn on_button_action(input: &mut Input, button_action: &str, pressed: bool) {
    match button_action {
        "," => {
            if !input.input_focused {
                input.next_layering_kind.increase.input = pressed
            }
        }
        "." => {
            if !input.input_focused {
                input.next_pixels_shadow_shape_kind.increase.input = pressed
            }
        }
        "+" => {
            if !input.input_focused {
                input.rotate_left = pressed
            }
        }
        "-" => {
            if !input.input_focused {
                input.rotate_right = pressed
            }
        }
        "input_focused" => input.input_focused = pressed,
        "a" => input.walk_left = pressed,
        "d" => input.walk_right = pressed,
        "w" => input.walk_forward = pressed,
        "s" => input.walk_backward = pressed,
        "q" => input.walk_up = pressed,
        "e" => input.walk_down = pressed,
        "arrowleft" | "←" | "◀" => input.turn_left = pressed,
        "arrowright" | "→" | "▶" => input.turn_right = pressed,
        "arrowup" | "↑" | "▲" => input.turn_up = pressed,
        "arrowdown" | "↓" | "▼" => input.turn_down = pressed,
        "f" => {
            if input.shift {
                input.filter_speed.increase.input = pressed
            } else {
                input.translation_speed.increase.input = pressed
            }
        }
        "r" => {
            if input.shift {
                input.filter_speed.decrease.input = pressed
            } else {
                input.translation_speed.decrease.input = pressed
            }
        }
        "feature-change-move-speed-inc" => input.translation_speed.increase.input = pressed,
        "feature-change-move-speed-dec" => input.translation_speed.decrease.input = pressed,
        "feature-change-pixel-speed-inc" => input.filter_speed.increase.input = pressed,
        "feature-change-pixel-speed-dec" => input.filter_speed.decrease.input = pressed,
        "t" | "reset-speeds" => input.reset_speeds = pressed,
        "camera-zoom-inc" => input.camera_zoom.increase = pressed,
        "camera-zoom-dec" => input.camera_zoom.decrease = pressed,
        "u" | "pixel-vertical-gap-inc" => input.pixel_scale_x.increase = pressed,
        "i" | "pixel-vertical-gap-dec" => input.pixel_scale_x.decrease = pressed,
        "j" | "pixel-horizontal-gap-inc" => input.pixel_scale_y.increase = pressed,
        "k" | "pixel-horizontal-gap-dec" => input.pixel_scale_y.decrease = pressed,
        "n" | "pixel-width-inc" => {
            if input.shift {
                input.pixel_gap.increase = pressed;
            } else {
                input.pixel_width.increase = pressed;
            }
        }
        "m" | "pixel-width-dec" => {
            if input.shift {
                input.pixel_gap.decrease = pressed;
            } else {
                input.pixel_width.decrease = pressed;
            }
        }
        "b" | "blur-level-inc" => input.blur.increase.input = pressed,
        "v" | "bluer-level-dec" => input.blur.decrease.input = pressed,
        "<" | "&lt;" | "pixel-contrast-inc" => input.contrast.increase = pressed,
        "z" | "pixel-contrast-dec" => input.contrast.decrease = pressed,
        "c" | "pixel-brightness-inc" => input.bright.increase = pressed,
        "x" | "pixel-brightness-dec" => input.bright.decrease = pressed,
        "y" | "feature-change-color-representation-inc" => input.next_color_representation_kind.increase.input = pressed,
        "feature-change-color-representation-dec" => input.next_color_representation_kind.decrease.input = pressed,
        "o" | "feature-change-pixel-geometry-inc" => input.next_pixel_geometry_kind.increase.input = pressed,
        "feature-change-pixel-geometry-dec" => input.next_pixel_geometry_kind.decrease.input = pressed,
        "l" | "feature-change-screen-curvature-inc" => input.next_screen_curvature_type.increase.input = pressed,
        "feature-change-screen-curvature-dec" => input.next_screen_curvature_type.decrease.input = pressed,
        "feature-change-screen-layering-type-inc" => input.next_layering_kind.increase.input = pressed,
        "feature-change-screen-layering-type-dec" => input.next_layering_kind.decrease.input = pressed,
        "feature-change-pixel-shadow-shape-inc" => input.next_pixels_shadow_shape_kind.increase.input = pressed,
        "feature-change-pixel-shadow-shape-dec" => input.next_pixels_shadow_shape_kind.decrease.input = pressed,
        "feature-change-pixel-shadow-height-inc" => input.next_pixels_shadow_height_factor.increase = pressed,
        "feature-change-pixel-shadow-height-dec" => input.next_pixels_shadow_height_factor.decrease = pressed,
        "feature-internal-resolution-inc" => input.next_internal_resolution.increase.input = pressed,
        "feature-internal-resolution-dec" => input.next_internal_resolution.decrease.input = pressed,
        "feature-texture-interpolation-inc" => input.next_texture_interpolation.increase.input = pressed,
        "feature-texture-interpolation-dec" => input.next_texture_interpolation.decrease.input = pressed,
        "g" | "lines-per-pixel-inc" => input.lpp.increase.input = pressed,
        "h" | "lines-per-pixel-dec" => input.lpp.decrease.input = pressed,
        "shift" => {
            input.shift = pressed;
            input.pixel_width.reset();
            input.pixel_gap.reset();
        }
        "alt" => input.alt = pressed,
        " " | "space" => input.space.input = pressed,
        "escape" | "esc" | "feature-quit" => input.esc.input = pressed,
        "f4" => input.screenshot.input = pressed,
        "reset-camera" => input.reset_position = pressed,
        "reset-filters" => input.reset_filters = pressed,
        _ => console!(log. "Ignored key: ", button_action),
    }
}
