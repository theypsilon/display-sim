use crate::simulation_state::{Input};
use crate::console;

pub fn on_button_action(input: &mut Input, button_action: &str, pressed: bool) {
    match button_action {
        "," => input.toggle_diffuse_foreground = pressed,
        "." => input.toggle_solid_background = pressed,
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
        "+" => input.rotate_left = pressed,
        "-" => input.rotate_right = pressed,
        "f" => input.speed_up = pressed,
        "r" => input.speed_down = pressed,
        "t" => input.reset_speeds = pressed,
        "camera-zoom-inc" => input.increase_camera_zoom = pressed,
        "camera-zoom-dec" => input.decrease_camera_zoom = pressed,
        "u" | "pixel-vertical-gap-inc" => input.increase_pixel_scale_x = pressed,
        "i" | "pixel-vertical-gap-dec" => input.decrease_pixel_scale_x = pressed,
        "j" | "pixel-horizontal-gap-inc" => input.increase_pixel_scale_y = pressed,
        "k" | "pixel-horizontal-gap-dec" => input.decrease_pixel_scale_y = pressed,
        "n" | "pixel-width-inc" => input.increase_pixel_gap = pressed,
        "m" | "pixel-width-dec" => input.decrease_pixel_gap = pressed,
        "b" | "blur-level-inc" => input.increase_blur = pressed,
        "v" | "bluer-level-dec" => input.decrease_blur = pressed,
        "<" | "&lt;" | "pixel-contrast-inc" => input.increase_contrast = pressed,
        "z" | "pixel-contrast-dec" => input.decrease_contrast = pressed,
        "c" | "pixel-brightness-inc" => input.increase_bright = pressed,
        "x" | "pixel-brightness-dec" => input.decrease_bright = pressed,
        "y" => input.toggle_split_colors = pressed,
        "o" => input.toggle_pixels_geometry_kind = pressed,
        "p" => input.showing_pixels_pulse = pressed,
        "g" => input.increase_lpp = pressed,
        "h" => input.decrease_lpp = pressed,
        "shift" => input.shift = pressed,
        "alt" => input.alt = pressed,
        " " | "space" => input.space = pressed,
        "escape" | "esc" => input.esc = pressed,
        "f4" => input.screenshot = pressed,
        "reset position" => input.reset_position = pressed,
        "reset filters" => input.reset_filters = pressed,
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