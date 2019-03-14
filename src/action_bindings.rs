use crate::simulation_state::{Input};
use crate::console;

pub fn on_button_action(input: &mut Input, button_action: &str, pressed: bool) {
    match button_action {
        "," => if !input.input_focused { input.next_layering_kind.input = pressed},
        "." => if !input.input_focused { input.toggle_pixels_shadow_kind.input = pressed},
        "feature-change-screen-layering-type" => input.next_layering_kind.input = pressed,
        "feature-change-pixel-shadow" => input.toggle_pixels_shadow_kind.input = pressed,
        "+" => if !input.input_focused { input.rotate_left = pressed },
        "-" => if !input.input_focused { input.rotate_right = pressed },
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
        "f" | "feature-change-move-speed-inc" => input.speed_up.input = pressed,
        "r" | "feature-change-move-speed-dec" => input.speed_down.input = pressed,
        "feature-change-pixel-speed-inc" => {
            input.speed_up.input = pressed;
            input.shift = pressed;
        },
        "feature-change-pixel-speed-dec" => {
            input.speed_down.input = pressed;
            input.shift = pressed;
        },
        "t" | "reset-speeds" => input.reset_speeds = pressed,
        "camera-zoom-inc" => input.increase_camera_zoom = pressed,
        "camera-zoom-dec" => input.decrease_camera_zoom = pressed,
        "u" | "pixel-vertical-gap-inc" => input.increase_pixel_scale_x = pressed,
        "i" | "pixel-vertical-gap-dec" => input.decrease_pixel_scale_x = pressed,
        "j" | "pixel-horizontal-gap-inc" => input.increase_pixel_scale_y = pressed,
        "k" | "pixel-horizontal-gap-dec" => input.decrease_pixel_scale_y = pressed,
        "n" | "pixel-width-inc" => input.increase_pixel_gap = pressed,
        "m" | "pixel-width-dec" => input.decrease_pixel_gap = pressed,
        "b" | "blur-level-inc" => input.increase_blur.input = pressed,
        "v" | "bluer-level-dec" => input.decrease_blur.input = pressed,
        "<" | "&lt;" | "pixel-contrast-inc" => input.increase_contrast = pressed,
        "z" | "pixel-contrast-dec" => input.decrease_contrast = pressed,
        "c" | "pixel-brightness-inc" => input.increase_bright = pressed,
        "x" | "pixel-brightness-dec" => input.decrease_bright = pressed,
        "y" | "feature-change-color-representation" => input.next_color_representation_kind.input = pressed,
        "o" | "feature-change-pixel-geometry" => input.next_pixel_geometry_kind.input = pressed,
        "l" | "feature-change-screen-curvature" => input.showing_pixels_pulse.input = pressed,
        "g" | "lines-per-pixel-inc" => input.increase_lpp.input = pressed,
        "h" | "lines-per-pixel-dec" => input.decrease_lpp.input = pressed,
        "shift" => input.shift = pressed,
        "alt" => input.alt = pressed,
        " " | "space" => input.space.input = pressed,
        "escape" | "esc" | "feature-quit" => input.esc.input = pressed,
        "f4" => input.screenshot.input = pressed,
        "reset-camera" => input.reset_position = pressed,
        "reset-filters" => input.reset_filters = pressed,
        _ => {
            if button_action.contains("+") {
                for button_fraction in button_action.split("+") {
                    on_button_action(input, button_fraction, pressed);
                }
            } else if pressed {
                console!(log. "Ignored key: ", button_action);
            }
        }
    }
}