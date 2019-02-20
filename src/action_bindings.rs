use crate::simulation_state::{Input};
use crate::console;

pub fn on_button_action(input: &mut Input, button_action: &str, pressed: bool) {
    match button_action {
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
        "u" => input.increase_pixel_scale_x = pressed,
        "i" => input.decrease_pixel_scale_x = pressed,
        "j" => input.increase_pixel_scale_y = pressed,
        "k" => input.decrease_pixel_scale_y = pressed,
        "n" => input.increase_pixel_gap = pressed,
        "m" => input.decrease_pixel_gap = pressed,
        "b" => input.increase_blur = pressed,
        "v" => input.decrease_blur = pressed,
        "<" | "&lt;" => input.increase_contrast = pressed,
        "z" => input.decrease_contrast = pressed,
        "c" => input.increase_bright = pressed,
        "x" => input.decrease_bright = pressed,
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