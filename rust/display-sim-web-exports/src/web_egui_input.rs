use app_util::{AppError, AppResult};
use egui::{Event, ImeEvent, Key, Modifiers, MouseWheelUnit, PointerButton, Pos2, RawInput, Rect, TouchPhase, Vec2, ViewportId};
use wasm_bindgen::JsValue;

/// Browser input translated into the same `egui::RawInput` contract used by
/// the native winit adapter. Coordinates are CSS pixels (egui points), while
/// the framebuffer dimensions remain physical pixels.
pub(crate) struct WebEguiInput {
    events: Vec<Event>,
    modifiers: Modifiers,
    pointer_pos: Option<Pos2>,
    pointer_buttons_down: Vec<PointerButton>,
    pointer_captured: bool,
    physical_size: [u32; 2],
    pixels_per_point: f32,
    focused: bool,
}

impl Default for WebEguiInput {
    fn default() -> Self {
        Self {
            events: Vec::new(),
            modifiers: Modifiers::default(),
            pointer_pos: None,
            pointer_buttons_down: Vec::new(),
            pointer_captured: false,
            physical_size: [1, 1],
            pixels_per_point: 1.0,
            focused: true,
        }
    }
}

impl WebEguiInput {
    pub(crate) fn set_metrics(&mut self, width: u32, height: u32, pixels_per_point: f32) {
        self.physical_size = [width.max(1), height.max(1)];
        self.pixels_per_point = pixels_per_point.max(0.1);
    }

    pub(crate) fn on_event(&mut self, kind: &str, value: &JsValue, panel_rect: Option<Rect>) -> AppResult<()> {
        match kind {
            "pointer-moved" => {
                let pos = point(value)?;
                self.pointer_pos = Some(pos);
                self.events.push(Event::PointerMoved(pos));
            }
            "pointer-button" => {
                let pos = point(value)?;
                self.pointer_pos = Some(pos);
                let button_number = number(value, "button")? as u16;
                let Some(button) = pointer_button(button_number) else {
                    return Ok(());
                };
                let pressed = boolean(value, "pressed")?;
                let was_pressed = self.pointer_buttons_down.contains(&button);
                if button == PointerButton::Primary {
                    if pressed && self.pointer_over(panel_rect) {
                        self.pointer_captured = true;
                    } else if !pressed {
                        self.pointer_captured = false;
                    }
                }
                if pressed == was_pressed {
                    return Ok(());
                }
                if pressed {
                    self.pointer_buttons_down.push(button);
                } else {
                    self.pointer_buttons_down.retain(|candidate| *candidate != button);
                }
                self.events.push(Event::PointerButton {
                    pos,
                    button,
                    pressed,
                    modifiers: self.modifiers,
                });
            }
            "pointer-gone" => {
                self.release_pointer_buttons();
                self.pointer_pos = None;
                self.pointer_captured = false;
                self.events.push(Event::PointerGone);
            }
            "wheel" => {
                self.update_modifiers(value)?;
                let mode = number(value, "deltaMode")? as u32;
                let (unit, scale) = match mode {
                    1 => (MouseWheelUnit::Line, 1.0),
                    2 => (MouseWheelUnit::Page, 1.0),
                    _ => (MouseWheelUnit::Point, 1.0),
                };
                // WheelEvent deltas describe wheel motion. egui expects the
                // direction in which the viewed content moves.
                let delta = Vec2::new(-number(value, "deltaX")? as f32 * scale, -number(value, "deltaY")? as f32 * scale);
                self.events.push(Event::MouseWheel {
                    unit,
                    delta,
                    phase: TouchPhase::Move,
                    modifiers: self.modifiers,
                });
            }
            "key" => {
                self.update_modifiers(value)?;
                let logical_name = string(value, "key")?;
                let physical_name = string(value, "code")?;
                let pressed = boolean(value, "pressed")?;
                let repeat = boolean(value, "repeat")?;

                if pressed && self.modifiers.command {
                    match logical_name.to_ascii_lowercase().as_str() {
                        "c" => self.events.push(Event::Copy),
                        "x" => self.events.push(Event::Cut),
                        // Paste text is supplied by the browser's `paste`
                        // event, which is the only standards-based source.
                        _ => {}
                    }
                }

                if let Some(key) = key_from_browser(&logical_name).or_else(|| key_from_code(&physical_name)) {
                    self.events.push(Event::Key {
                        key,
                        physical_key: key_from_code(&physical_name),
                        pressed,
                        repeat,
                        modifiers: self.modifiers,
                    });
                }
            }
            "text" => {
                let text = string(value, "text")?;
                if !text.is_empty() {
                    self.events.push(Event::Text(text));
                }
            }
            "ime-preedit" => {
                self.events.push(Event::Ime(ImeEvent::Preedit {
                    text: string(value, "text")?,
                    active_range_chars: None,
                }));
            }
            "ime-commit" => self.events.push(Event::Ime(ImeEvent::Commit(string(value, "text")?))),
            "copy" => self.events.push(Event::Copy),
            "cut" => self.events.push(Event::Cut),
            "paste" => self.events.push(Event::Paste(string(value, "text")?)),
            "focus" => {
                let focused = boolean(value, "focused")?;
                if focused != self.focused {
                    self.focused = focused;
                    if !focused {
                        self.release_pointer_buttons();
                        self.pointer_captured = false;
                    }
                    self.events.push(Event::WindowFocused(focused));
                }
            }
            _ => return Err(AppError::new(format!("Unknown web egui event: {kind}"))),
        }
        Ok(())
    }

    pub(crate) fn pointer_is_captured(&self, panel_rect: Option<Rect>) -> bool {
        self.pointer_captured || self.pointer_over(panel_rect)
    }

    pub(crate) fn take_input(&mut self, time: f64) -> RawInput {
        let screen_size = Vec2::new(
            self.physical_size[0] as f32 / self.pixels_per_point,
            self.physical_size[1] as f32 / self.pixels_per_point,
        );
        let mut input = RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, screen_size)),
            time: Some(time),
            predicted_dt: 1.0 / 60.0,
            events: std::mem::take(&mut self.events),
            focused: self.focused,
            ..Default::default()
        };
        let viewport = input.viewports.get_mut(&ViewportId::ROOT).expect("root viewport");
        viewport.native_pixels_per_point = Some(self.pixels_per_point);
        viewport.inner_rect = input.screen_rect;
        viewport.focused = Some(self.focused);
        input
    }

    fn update_modifiers(&mut self, value: &JsValue) -> AppResult<()> {
        let next = Modifiers {
            alt: boolean(value, "altKey")?,
            ctrl: boolean(value, "ctrlKey")?,
            shift: boolean(value, "shiftKey")?,
            mac_cmd: boolean(value, "macCommand")?,
            command: boolean(value, "command")?,
        };
        if next != self.modifiers {
            self.modifiers = next;
            self.events.push(Event::ModifiersChanged(next));
        }
        Ok(())
    }

    fn pointer_over(&self, panel_rect: Option<Rect>) -> bool {
        matches!((self.pointer_pos, panel_rect), (Some(pos), Some(rect)) if rect.contains(pos))
    }

    fn release_pointer_buttons(&mut self) {
        let pos = self.pointer_pos.unwrap_or_default();
        let modifiers = self.modifiers;
        let buttons: Vec<_> = self.pointer_buttons_down.drain(..).collect();
        self.events.extend(buttons.into_iter().map(|button| Event::PointerButton {
            pos,
            button,
            pressed: false,
            modifiers,
        }));
    }
}

fn point(value: &JsValue) -> AppResult<Pos2> {
    Ok(Pos2::new(number(value, "x")? as f32, number(value, "y")? as f32))
}

fn field(value: &JsValue, name: &str) -> AppResult<JsValue> {
    js_sys::Reflect::get(value, &JsValue::from_str(name)).map_err(AppError::from)
}

fn number(value: &JsValue, name: &str) -> AppResult<f64> {
    field(value, name)?
        .as_f64()
        .ok_or_else(|| AppError::new(format!("egui event field {name:?} must be a number")))
}

fn boolean(value: &JsValue, name: &str) -> AppResult<bool> {
    field(value, name)?
        .as_bool()
        .ok_or_else(|| AppError::new(format!("egui event field {name:?} must be a boolean")))
}

fn string(value: &JsValue, name: &str) -> AppResult<String> {
    field(value, name)?
        .as_string()
        .ok_or_else(|| AppError::new(format!("egui event field {name:?} must be a string")))
}

fn pointer_button(button: u16) -> Option<PointerButton> {
    match button {
        0 => Some(PointerButton::Primary),
        1 => Some(PointerButton::Middle),
        2 => Some(PointerButton::Secondary),
        3 => Some(PointerButton::Extra1),
        4 => Some(PointerButton::Extra2),
        _ => None,
    }
}

fn key_from_browser(key: &str) -> Option<Key> {
    use Key as K;
    Some(match key {
        "ArrowDown" => K::ArrowDown,
        "ArrowLeft" => K::ArrowLeft,
        "ArrowRight" => K::ArrowRight,
        "ArrowUp" => K::ArrowUp,
        "Escape" => K::Escape,
        "Tab" => K::Tab,
        "Backspace" => K::Backspace,
        "Enter" => K::Enter,
        " " => K::Space,
        "Insert" => K::Insert,
        "Delete" => K::Delete,
        "Home" => K::Home,
        "End" => K::End,
        "PageUp" => K::PageUp,
        "PageDown" => K::PageDown,
        "BrowserBack" => K::BrowserBack,
        ":" => K::Colon,
        "," => K::Comma,
        "\\" => K::Backslash,
        "/" => K::Slash,
        "|" => K::Pipe,
        "?" => K::Questionmark,
        "!" => K::Exclamationmark,
        "[" => K::OpenBracket,
        "]" => K::CloseBracket,
        "{" => K::OpenCurlyBracket,
        "}" => K::CloseCurlyBracket,
        "`" => K::Backtick,
        "-" => K::Minus,
        "." => K::Period,
        "+" => K::Plus,
        "=" => K::Equals,
        ";" => K::Semicolon,
        "'" => K::Quote,
        _ => return key_from_ascii_name(key).or_else(|| function_key(key)),
    })
}

fn key_from_code(code: &str) -> Option<Key> {
    use Key as K;
    Some(match code {
        "ArrowDown" => K::ArrowDown,
        "ArrowLeft" => K::ArrowLeft,
        "ArrowRight" => K::ArrowRight,
        "ArrowUp" => K::ArrowUp,
        "Escape" => K::Escape,
        "Tab" => K::Tab,
        "Backspace" => K::Backspace,
        "Enter" | "NumpadEnter" => K::Enter,
        "Space" => K::Space,
        "Insert" => K::Insert,
        "Delete" => K::Delete,
        "Home" => K::Home,
        "End" => K::End,
        "PageUp" => K::PageUp,
        "PageDown" => K::PageDown,
        "BrowserBack" => K::BrowserBack,
        "Comma" => K::Comma,
        "Period" | "NumpadDecimal" => K::Period,
        "Slash" | "NumpadDivide" => K::Slash,
        "Backslash" => K::Backslash,
        "IntlBackslash" => K::IntlBackslash,
        "Minus" | "NumpadSubtract" => K::Minus,
        "Equal" | "NumpadAdd" => K::Plus,
        "Semicolon" => K::Semicolon,
        "Quote" => K::Quote,
        "Backquote" => K::Backtick,
        "BracketLeft" => K::OpenBracket,
        "BracketRight" => K::CloseBracket,
        _ => {
            if let Some(name) = code.strip_prefix("Key") {
                return key_from_ascii_name(name);
            }
            if let Some(name) = code.strip_prefix("Digit").or_else(|| code.strip_prefix("Numpad")) {
                return key_from_ascii_name(name);
            }
            return function_key(code);
        }
    })
}

fn key_from_ascii_name(name: &str) -> Option<Key> {
    use Key as K;
    Some(match name.to_ascii_uppercase().as_str() {
        "0" => K::Num0,
        "1" => K::Num1,
        "2" => K::Num2,
        "3" => K::Num3,
        "4" => K::Num4,
        "5" => K::Num5,
        "6" => K::Num6,
        "7" => K::Num7,
        "8" => K::Num8,
        "9" => K::Num9,
        "A" => K::A,
        "B" => K::B,
        "C" => K::C,
        "D" => K::D,
        "E" => K::E,
        "F" => K::F,
        "G" => K::G,
        "H" => K::H,
        "I" => K::I,
        "J" => K::J,
        "K" => K::K,
        "L" => K::L,
        "M" => K::M,
        "N" => K::N,
        "O" => K::O,
        "P" => K::P,
        "Q" => K::Q,
        "R" => K::R,
        "S" => K::S,
        "T" => K::T,
        "U" => K::U,
        "V" => K::V,
        "W" => K::W,
        "X" => K::X,
        "Y" => K::Y,
        "Z" => K::Z,
        _ => return None,
    })
}

fn function_key(name: &str) -> Option<Key> {
    use Key as K;
    Some(match name {
        "F1" => K::F1,
        "F2" => K::F2,
        "F3" => K::F3,
        "F4" => K::F4,
        "F5" => K::F5,
        "F6" => K::F6,
        "F7" => K::F7,
        "F8" => K::F8,
        "F9" => K::F9,
        "F10" => K::F10,
        "F11" => K::F11,
        "F12" => K::F12,
        "F13" => K::F13,
        "F14" => K::F14,
        "F15" => K::F15,
        "F16" => K::F16,
        "F17" => K::F17,
        "F18" => K::F18,
        "F19" => K::F19,
        "F20" => K::F20,
        "F21" => K::F21,
        "F22" => K::F22,
        "F23" => K::F23,
        "F24" => K::F24,
        "F25" => K::F25,
        "F26" => K::F26,
        "F27" => K::F27,
        "F28" => K::F28,
        "F29" => K::F29,
        "F30" => K::F30,
        "F31" => K::F31,
        "F32" => K::F32,
        "F33" => K::F33,
        "F34" => K::F34,
        "F35" => K::F35,
        _ => return None,
    })
}
