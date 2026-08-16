use core::input_types::{InputEventValue, Pressed};
use glutin::event::{ElementState, KeyboardInput, ModifiersState, MouseScrollDelta, VirtualKeyCode};
use std::collections::{HashMap, HashSet, VecDeque};

const WHEEL_POINTS_PER_LINE: f32 = 100.0;

#[derive(Default)]
pub(crate) struct SimulationPointerInput {
    down: bool,
}

impl SimulationPointerInput {
    pub(crate) fn on_primary_button(&mut self, pressed: bool, blocked: bool) -> Option<InputEventValue> {
        if pressed {
            if blocked || self.down {
                return None;
            }
            self.down = true;
            Some(InputEventValue::MouseClick(Pressed::Yes))
        } else {
            self.release()
        }
    }

    pub(crate) fn release(&mut self) -> Option<InputEventValue> {
        if !self.down {
            return None;
        }
        self.down = false;
        Some(InputEventValue::MouseClick(Pressed::No))
    }

    pub(crate) fn clear(&mut self) {
        self.down = false;
    }

    pub(crate) fn is_down(&self) -> bool {
        self.down
    }
}

#[derive(Debug)]
struct PendingPrintableKey {
    scancode: u32,
    fallback: String,
    missing_text_value: String,
}

/// Translates winit's physical-key events into browser-style key values.
///
/// Printable keys wait for `ReceivedCharacter`, which reflects the active OS
/// keyboard layout. Command shortcuts without text fall back to their virtual
/// key, while unmodified no-text printables use the browser's `Dead` value.
/// Releases always reuse the value chosen on key-down.
#[derive(Default)]
pub(crate) struct SimulationKeyboardInput {
    pending_printable: VecDeque<PendingPrintableKey>,
    active: HashMap<u32, String>,
    ui_owned: HashSet<u32>,
    logical_counts: HashMap<String, usize>,
    modifiers: ModifiersState,
    dead_composition_pending: bool,
}

impl SimulationKeyboardInput {
    pub(crate) fn on_modifiers_changed(&mut self, modifiers: ModifiersState) {
        self.modifiers = modifiers;
    }

    #[cfg(test)]
    fn on_keyboard_input(&mut self, input: &KeyboardInput) -> Vec<InputEventValue> {
        self.on_keyboard_input_routed(input, true)
    }

    pub(crate) fn on_keyboard_input_routed(&mut self, input: &KeyboardInput, route_to_simulation: bool) -> Vec<InputEventValue> {
        match input.state {
            ElementState::Pressed => {
                if self.active.contains_key(&input.scancode)
                    || self.ui_owned.contains(&input.scancode)
                    || self.pending_printable.iter().any(|pending| pending.scancode == input.scancode)
                {
                    // Repeated physical presses do not create another owner
                    // of the logical key. The core already repeats held
                    // actions once per simulation tick.
                    return Vec::new();
                }
                let Some(key) = input.virtual_keycode else {
                    return Vec::new();
                };
                if !route_to_simulation {
                    self.ui_owned.insert(input.scancode);
                    return Vec::new();
                }
                let fallback = browser_key_name(key, self.modifiers.shift());
                if is_layout_sensitive_printable(key) {
                    let command_modifier =
                        self.modifiers.logo() || (self.modifiers.ctrl() && !self.modifiers.alt()) || (self.modifiers.alt() && !self.modifiers.ctrl());
                    let missing_text_value = if command_modifier { fallback.clone() } else { "Dead".into() };
                    self.pending_printable.push_back(PendingPrintableKey {
                        scancode: input.scancode,
                        fallback,
                        missing_text_value,
                    });
                    Vec::new()
                } else {
                    self.press(input.scancode, fallback).into_iter().collect()
                }
            }
            ElementState::Released => {
                if self.ui_owned.remove(&input.scancode) {
                    return Vec::new();
                }
                if let Some(index) = self.pending_printable.iter().position(|pending| pending.scancode == input.scancode) {
                    let pending = self.pending_printable.remove(index).expect("pending printable index disappeared");
                    let mut events = Vec::new();
                    if pending.missing_text_value == "Dead" {
                        self.dead_composition_pending = true;
                    }
                    events.extend(self.press(pending.scancode, pending.missing_text_value));
                    events.extend(self.release(pending.scancode));
                    return events;
                }
                self.release(input.scancode).into_iter().collect()
            }
        }
    }

    pub(crate) fn on_received_character(&mut self, character: char) -> Vec<InputEventValue> {
        if character.is_control() {
            return Vec::new();
        }
        let Some(pending) = self.pending_printable.pop_front() else {
            // Text produced by an IME composition has no one-to-one physical
            // key transition and belongs to the focused text editor only.
            return Vec::new();
        };
        let value = if std::mem::take(&mut self.dead_composition_pending) {
            pending.fallback
        } else {
            character.to_string()
        };
        self.press(pending.scancode, value).into_iter().collect()
    }

    pub(crate) fn flush_pending(&mut self) -> Vec<InputEventValue> {
        let pending = std::mem::take(&mut self.pending_printable);
        let mut events = Vec::new();
        for pending in pending {
            if pending.missing_text_value == "Dead" {
                self.dead_composition_pending = true;
            }
            events.extend(self.press(pending.scancode, pending.missing_text_value));
        }
        events
    }

    pub(crate) fn clear(&mut self) {
        self.pending_printable.clear();
        self.active.clear();
        self.ui_owned.clear();
        self.logical_counts.clear();
        self.modifiers = ModifiersState::empty();
        self.dead_composition_pending = false;
    }

    fn press(&mut self, scancode: u32, key: String) -> Option<InputEventValue> {
        self.active.insert(scancode, key.clone());
        let count = self.logical_counts.entry(key.clone()).or_default();
        *count += 1;
        (*count == 1).then(|| keyboard_event(true, key))
    }

    fn release(&mut self, scancode: u32) -> Option<InputEventValue> {
        let key = self.active.remove(&scancode)?;
        let count = self.logical_counts.get_mut(&key).expect("active physical key has no logical owner");
        *count -= 1;
        if *count == 0 {
            self.logical_counts.remove(&key);
            Some(keyboard_event(false, key))
        } else {
            None
        }
    }
}

fn keyboard_event(pressed: bool, key: String) -> InputEventValue {
    InputEventValue::Keyboard {
        pressed: Pressed::from_bool(pressed),
        key,
    }
}

fn is_layout_sensitive_printable(key: VirtualKeyCode) -> bool {
    use VirtualKeyCode as Key;
    matches!(
        key,
        Key::Key0
            | Key::Key1
            | Key::Key2
            | Key::Key3
            | Key::Key4
            | Key::Key5
            | Key::Key6
            | Key::Key7
            | Key::Key8
            | Key::Key9
            | Key::A
            | Key::B
            | Key::C
            | Key::D
            | Key::E
            | Key::F
            | Key::G
            | Key::H
            | Key::I
            | Key::J
            | Key::K
            | Key::L
            | Key::M
            | Key::N
            | Key::O
            | Key::P
            | Key::Q
            | Key::R
            | Key::S
            | Key::T
            | Key::U
            | Key::V
            | Key::W
            | Key::X
            | Key::Y
            | Key::Z
            | Key::Apostrophe
            | Key::Backslash
            | Key::Comma
            | Key::Equals
            | Key::Grave
            | Key::LBracket
            | Key::Minus
            | Key::Period
            | Key::RBracket
            | Key::Semicolon
            | Key::Slash
            | Key::Space
            | Key::Numpad0
            | Key::Numpad1
            | Key::Numpad2
            | Key::Numpad3
            | Key::Numpad4
            | Key::Numpad5
            | Key::Numpad6
            | Key::Numpad7
            | Key::Numpad8
            | Key::Numpad9
            | Key::NumpadAdd
            | Key::NumpadSubtract
            | Key::NumpadMultiply
            | Key::NumpadDivide
            | Key::NumpadDecimal
            | Key::NumpadComma
            | Key::NumpadEquals
            | Key::Plus
            | Key::Asterisk
            | Key::At
            | Key::Caret
            | Key::Colon
            | Key::Underline
    )
}

pub(crate) fn browser_key_name(key: VirtualKeyCode, shift: bool) -> String {
    if let Some(letter) = ascii_letter(key) {
        return if shift { letter.to_ascii_uppercase().to_string() } else { letter.to_string() };
    }
    match key {
        VirtualKeyCode::LShift | VirtualKeyCode::RShift => "Shift".into(),
        VirtualKeyCode::LControl | VirtualKeyCode::RControl => "Control".into(),
        VirtualKeyCode::LAlt | VirtualKeyCode::RAlt => "Alt".into(),
        VirtualKeyCode::LWin | VirtualKeyCode::RWin => "Meta".into(),
        VirtualKeyCode::Space => " ".into(),
        VirtualKeyCode::Left => "ArrowLeft".into(),
        VirtualKeyCode::Right => "ArrowRight".into(),
        VirtualKeyCode::Up => "ArrowUp".into(),
        VirtualKeyCode::Down => "ArrowDown".into(),
        VirtualKeyCode::Return | VirtualKeyCode::NumpadEnter => "Enter".into(),
        VirtualKeyCode::Back => "Backspace".into(),
        VirtualKeyCode::Capital => "CapsLock".into(),
        VirtualKeyCode::Snapshot => "PrintScreen".into(),
        VirtualKeyCode::Sysrq => "PrintScreen".into(),
        VirtualKeyCode::Scroll => "ScrollLock".into(),
        VirtualKeyCode::Numlock => "NumLock".into(),
        VirtualKeyCode::Apps => "ContextMenu".into(),
        VirtualKeyCode::NoConvert => "NonConvert".into(),
        VirtualKeyCode::Kana => "KanaMode".into(),
        VirtualKeyCode::Kanji => "KanjiMode".into(),
        VirtualKeyCode::NextTrack => "MediaTrackNext".into(),
        VirtualKeyCode::PrevTrack => "MediaTrackPrevious".into(),
        VirtualKeyCode::PlayPause => "MediaPlayPause".into(),
        VirtualKeyCode::Mute => "AudioVolumeMute".into(),
        VirtualKeyCode::VolumeDown => "AudioVolumeDown".into(),
        VirtualKeyCode::VolumeUp => "AudioVolumeUp".into(),
        VirtualKeyCode::WebBack | VirtualKeyCode::NavigateBackward => "BrowserBack".into(),
        VirtualKeyCode::WebForward | VirtualKeyCode::NavigateForward => "BrowserForward".into(),
        VirtualKeyCode::WebFavorites => "BrowserFavorites".into(),
        VirtualKeyCode::WebHome => "BrowserHome".into(),
        VirtualKeyCode::WebRefresh => "BrowserRefresh".into(),
        VirtualKeyCode::WebSearch => "BrowserSearch".into(),
        VirtualKeyCode::WebStop => "BrowserStop".into(),
        VirtualKeyCode::Mail => "LaunchMail".into(),
        VirtualKeyCode::MediaSelect => "LaunchMediaPlayer".into(),
        VirtualKeyCode::Calculator => "LaunchCalculator".into(),
        VirtualKeyCode::Wake => "WakeUp".into(),
        VirtualKeyCode::Equals => if shift { "+" } else { "=" }.into(),
        VirtualKeyCode::Minus => if shift { "_" } else { "-" }.into(),
        VirtualKeyCode::Comma => if shift { "<" } else { "," }.into(),
        VirtualKeyCode::Period => if shift { ">" } else { "." }.into(),
        VirtualKeyCode::Slash => if shift { "?" } else { "/" }.into(),
        VirtualKeyCode::Backslash => if shift { "|" } else { "\\" }.into(),
        VirtualKeyCode::Semicolon => if shift { ":" } else { ";" }.into(),
        VirtualKeyCode::Apostrophe => if shift { "\"" } else { "'" }.into(),
        VirtualKeyCode::Grave => if shift { "~" } else { "`" }.into(),
        VirtualKeyCode::LBracket => if shift { "{" } else { "[" }.into(),
        VirtualKeyCode::RBracket => if shift { "}" } else { "]" }.into(),
        VirtualKeyCode::Key1 => if shift { "!" } else { "1" }.into(),
        VirtualKeyCode::Key2 => if shift { "@" } else { "2" }.into(),
        VirtualKeyCode::Key3 => if shift { "#" } else { "3" }.into(),
        VirtualKeyCode::Key4 => if shift { "$" } else { "4" }.into(),
        VirtualKeyCode::Key5 => if shift { "%" } else { "5" }.into(),
        VirtualKeyCode::Key6 => if shift { "^" } else { "6" }.into(),
        VirtualKeyCode::Key7 => if shift { "&" } else { "7" }.into(),
        VirtualKeyCode::Key8 => if shift { "*" } else { "8" }.into(),
        VirtualKeyCode::Key9 => if shift { "(" } else { "9" }.into(),
        VirtualKeyCode::Key0 => if shift { ")" } else { "0" }.into(),
        VirtualKeyCode::Numpad0 => "0".into(),
        VirtualKeyCode::Numpad1 => "1".into(),
        VirtualKeyCode::Numpad2 => "2".into(),
        VirtualKeyCode::Numpad3 => "3".into(),
        VirtualKeyCode::Numpad4 => "4".into(),
        VirtualKeyCode::Numpad5 => "5".into(),
        VirtualKeyCode::Numpad6 => "6".into(),
        VirtualKeyCode::Numpad7 => "7".into(),
        VirtualKeyCode::Numpad8 => "8".into(),
        VirtualKeyCode::Numpad9 => "9".into(),
        VirtualKeyCode::NumpadAdd => "+".into(),
        VirtualKeyCode::NumpadSubtract => "-".into(),
        VirtualKeyCode::NumpadMultiply => "*".into(),
        VirtualKeyCode::NumpadDivide => "/".into(),
        VirtualKeyCode::NumpadDecimal => ".".into(),
        VirtualKeyCode::NumpadComma => ",".into(),
        VirtualKeyCode::NumpadEquals => "=".into(),
        VirtualKeyCode::Plus => "+".into(),
        VirtualKeyCode::Asterisk => "*".into(),
        VirtualKeyCode::At => "@".into(),
        VirtualKeyCode::Caret => "^".into(),
        VirtualKeyCode::Colon => ":".into(),
        VirtualKeyCode::Underline => "_".into(),
        other => format!("{other:?}"),
    }
}

fn ascii_letter(key: VirtualKeyCode) -> Option<char> {
    match key {
        VirtualKeyCode::A => Some('a'),
        VirtualKeyCode::B => Some('b'),
        VirtualKeyCode::C => Some('c'),
        VirtualKeyCode::D => Some('d'),
        VirtualKeyCode::E => Some('e'),
        VirtualKeyCode::F => Some('f'),
        VirtualKeyCode::G => Some('g'),
        VirtualKeyCode::H => Some('h'),
        VirtualKeyCode::I => Some('i'),
        VirtualKeyCode::J => Some('j'),
        VirtualKeyCode::K => Some('k'),
        VirtualKeyCode::L => Some('l'),
        VirtualKeyCode::M => Some('m'),
        VirtualKeyCode::N => Some('n'),
        VirtualKeyCode::O => Some('o'),
        VirtualKeyCode::P => Some('p'),
        VirtualKeyCode::Q => Some('q'),
        VirtualKeyCode::R => Some('r'),
        VirtualKeyCode::S => Some('s'),
        VirtualKeyCode::T => Some('t'),
        VirtualKeyCode::U => Some('u'),
        VirtualKeyCode::V => Some('v'),
        VirtualKeyCode::W => Some('w'),
        VirtualKeyCode::X => Some('x'),
        VirtualKeyCode::Y => Some('y'),
        VirtualKeyCode::Z => Some('z'),
        _ => None,
    }
}

pub(crate) fn browser_wheel_delta(delta: &MouseScrollDelta) -> f32 {
    match delta {
        // Browser WheelEvent::deltaY is positive when scrolling down,
        // opposite to winit's positive-up convention.
        MouseScrollDelta::LineDelta(_, y) => -*y * WHEEL_POINTS_PER_LINE,
        MouseScrollDelta::PixelDelta(position) => -position.y as f32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(deprecated)]
    fn key(scancode: u32, state: ElementState, virtual_keycode: VirtualKeyCode, modifiers: ModifiersState) -> KeyboardInput {
        KeyboardInput {
            scancode,
            state,
            virtual_keycode: Some(virtual_keycode),
            modifiers,
        }
    }

    fn assert_keyboard(event: &InputEventValue, expected_pressed: Pressed, expected_key: &str) {
        assert!(matches!(event, InputEventValue::Keyboard { pressed, key } if *pressed == expected_pressed && key == expected_key));
    }

    #[test]
    fn printable_keys_follow_the_active_layout_character() {
        let mut input = SimulationKeyboardInput::default();
        assert!(input
            .on_keyboard_input(&key(16, ElementState::Pressed, VirtualKeyCode::Q, ModifiersState::empty()))
            .is_empty());
        let pressed = input.on_received_character('a');
        assert_keyboard(&pressed[0], Pressed::Yes, "a");
        let released = input.on_keyboard_input(&key(16, ElementState::Released, VirtualKeyCode::Q, ModifiersState::empty()));
        assert_keyboard(&released[0], Pressed::No, "a");
    }

    #[test]
    fn releases_reuse_the_key_value_chosen_on_press() {
        let mut input = SimulationKeyboardInput::default();
        input.on_modifiers_changed(ModifiersState::SHIFT);
        input.on_keyboard_input(&key(13, ElementState::Pressed, VirtualKeyCode::Equals, ModifiersState::SHIFT));
        input.on_received_character('+');
        input.on_modifiers_changed(ModifiersState::empty());
        let released = input.on_keyboard_input(&key(13, ElementState::Released, VirtualKeyCode::Equals, ModifiersState::empty()));
        assert_keyboard(&released[0], Pressed::No, "+");
    }

    #[test]
    fn auto_repeat_does_not_create_another_logical_key_owner() {
        let mut input = SimulationKeyboardInput::default();
        input.on_modifiers_changed(ModifiersState::SHIFT);
        input.on_keyboard_input(&key(13, ElementState::Pressed, VirtualKeyCode::Equals, ModifiersState::SHIFT));
        input.on_received_character('+');
        input.on_modifiers_changed(ModifiersState::empty());
        let repeated = input.on_keyboard_input(&key(13, ElementState::Pressed, VirtualKeyCode::Equals, ModifiersState::empty()));
        assert!(repeated.is_empty());
        let released = input.on_keyboard_input(&key(13, ElementState::Released, VirtualKeyCode::Equals, ModifiersState::empty()));
        assert_keyboard(&released[0], Pressed::No, "+");
    }

    #[test]
    fn logical_key_stays_down_until_every_physical_owner_releases() {
        let mut input = SimulationKeyboardInput::default();
        let left = input.on_keyboard_input(&key(42, ElementState::Pressed, VirtualKeyCode::LShift, ModifiersState::SHIFT));
        assert_keyboard(&left[0], Pressed::Yes, "Shift");
        let right = input.on_keyboard_input(&key(54, ElementState::Pressed, VirtualKeyCode::RShift, ModifiersState::SHIFT));
        assert!(right.is_empty());
        let left = input.on_keyboard_input(&key(42, ElementState::Released, VirtualKeyCode::LShift, ModifiersState::SHIFT));
        assert!(left.is_empty());
        let right = input.on_keyboard_input(&key(54, ElementState::Released, VirtualKeyCode::RShift, ModifiersState::empty()));
        assert_keyboard(&right[0], Pressed::No, "Shift");
    }

    #[test]
    fn printable_release_before_character_delivery_still_balances_edges() {
        let mut input = SimulationKeyboardInput::default();
        input.on_modifiers_changed(ModifiersState::CTRL);
        assert!(input
            .on_keyboard_input(&key(30, ElementState::Pressed, VirtualKeyCode::A, ModifiersState::CTRL))
            .is_empty());
        let events = input.on_keyboard_input(&key(30, ElementState::Released, VirtualKeyCode::A, ModifiersState::CTRL));
        assert_eq!(events.len(), 2);
        assert_keyboard(&events[0], Pressed::Yes, "a");
        assert_keyboard(&events[1], Pressed::No, "a");
    }

    #[test]
    fn non_text_keys_are_immediate_and_shortcuts_have_browser_style_fallbacks() {
        let mut input = SimulationKeyboardInput::default();
        let arrow = input.on_keyboard_input(&key(105, ElementState::Pressed, VirtualKeyCode::Left, ModifiersState::empty()));
        assert_keyboard(&arrow[0], Pressed::Yes, "ArrowLeft");
        input.on_modifiers_changed(ModifiersState::CTRL);
        let shortcut = input.on_keyboard_input(&key(46, ElementState::Pressed, VirtualKeyCode::C, ModifiersState::CTRL));
        assert!(shortcut.is_empty());
        let shortcut = input.flush_pending();
        assert_keyboard(&shortcut[0], Pressed::Yes, "c");
    }

    #[test]
    fn dead_keys_use_the_browser_value_before_the_tick() {
        let mut input = SimulationKeyboardInput::default();
        input.on_keyboard_input(&key(41, ElementState::Pressed, VirtualKeyCode::Grave, ModifiersState::empty()));
        let events = input.flush_pending();
        assert_keyboard(&events[0], Pressed::Yes, "Dead");
        let released = input.on_keyboard_input(&key(41, ElementState::Released, VirtualKeyCode::Grave, ModifiersState::empty()));
        assert_keyboard(&released[0], Pressed::No, "Dead");
    }

    #[test]
    fn alt_gr_no_text_keys_remain_dead_instead_of_becoming_shortcuts() {
        let mut input = SimulationKeyboardInput::default();
        input.on_modifiers_changed(ModifiersState::CTRL | ModifiersState::ALT);
        input.on_keyboard_input(&key(
            13,
            ElementState::Pressed,
            VirtualKeyCode::Equals,
            ModifiersState::CTRL | ModifiersState::ALT,
        ));
        let events = input.flush_pending();
        assert_keyboard(&events[0], Pressed::Yes, "Dead");
    }

    #[test]
    fn composed_text_does_not_replace_the_browser_keydown_value() {
        let mut input = SimulationKeyboardInput::default();
        input.on_keyboard_input(&key(41, ElementState::Pressed, VirtualKeyCode::Grave, ModifiersState::empty()));
        input.flush_pending();
        input.on_keyboard_input(&key(41, ElementState::Released, VirtualKeyCode::Grave, ModifiersState::empty()));

        input.on_keyboard_input(&key(18, ElementState::Pressed, VirtualKeyCode::E, ModifiersState::empty()));
        let events = input.on_received_character('é');
        assert_keyboard(&events[0], Pressed::Yes, "e");
        let released = input.on_keyboard_input(&key(18, ElementState::Released, VirtualKeyCode::E, ModifiersState::empty()));
        assert_keyboard(&released[0], Pressed::No, "e");
    }

    #[test]
    fn ui_owned_activation_keys_never_emit_simulation_edges() {
        let mut input = SimulationKeyboardInput::default();
        let space = key(57, ElementState::Pressed, VirtualKeyCode::Space, ModifiersState::empty());
        assert!(input.on_keyboard_input_routed(&space, false).is_empty());
        assert!(input.on_received_character(' ').is_empty());
        assert!(input.on_keyboard_input_routed(&space, true).is_empty());
        assert!(input
            .on_keyboard_input_routed(&key(57, ElementState::Released, VirtualKeyCode::Space, ModifiersState::empty()), true)
            .is_empty());
    }

    #[test]
    fn routed_key_release_survives_focus_moving_to_the_ui() {
        let mut input = SimulationKeyboardInput::default();
        input.on_keyboard_input_routed(&key(57, ElementState::Pressed, VirtualKeyCode::Space, ModifiersState::empty()), true);
        let pressed = input.on_received_character(' ');
        assert_keyboard(&pressed[0], Pressed::Yes, " ");
        let released = input.on_keyboard_input_routed(&key(57, ElementState::Released, VirtualKeyCode::Space, ModifiersState::empty()), false);
        assert_keyboard(&released[0], Pressed::No, " ");
    }

    #[test]
    fn maps_native_keys_to_browser_key_values() {
        assert_eq!(browser_key_name(VirtualKeyCode::Equals, false), "=");
        assert_eq!(browser_key_name(VirtualKeyCode::Equals, true), "+");
        assert_eq!(browser_key_name(VirtualKeyCode::Minus, false), "-");
        assert_eq!(browser_key_name(VirtualKeyCode::Comma, false), ",");
        assert_eq!(browser_key_name(VirtualKeyCode::Period, false), ".");
        assert_eq!(browser_key_name(VirtualKeyCode::NumpadDecimal, false), ".");
        assert_eq!(browser_key_name(VirtualKeyCode::Left, false), "ArrowLeft");
        assert_eq!(browser_key_name(VirtualKeyCode::Space, false), " ");
        assert_eq!(browser_key_name(VirtualKeyCode::Capital, false), "CapsLock");
        assert_eq!(browser_key_name(VirtualKeyCode::Apps, false), "ContextMenu");
        assert_eq!(browser_key_name(VirtualKeyCode::Numlock, false), "NumLock");
        assert_eq!(browser_key_name(VirtualKeyCode::NextTrack, false), "MediaTrackNext");
        assert_eq!(browser_key_name(VirtualKeyCode::C, false), "c");
        assert_eq!(browser_key_name(VirtualKeyCode::C, true), "C");
    }

    #[test]
    fn normalizes_wheel_direction_and_line_units_to_the_browser() {
        assert_eq!(browser_wheel_delta(&MouseScrollDelta::LineDelta(0.0, 1.0)), -100.0);
        assert_eq!(
            browser_wheel_delta(&MouseScrollDelta::PixelDelta(glutin::dpi::PhysicalPosition::new(0.0, -12.5))),
            12.5
        );
    }

    #[test]
    fn primary_pointer_edges_are_balanced_and_ui_presses_are_blocked() {
        let mut input = SimulationPointerInput::default();
        assert!(input.on_primary_button(true, true).is_none());
        assert!(matches!(input.on_primary_button(true, false), Some(InputEventValue::MouseClick(Pressed::Yes))));
        assert!(input.on_primary_button(true, false).is_none());
        assert!(matches!(input.on_primary_button(false, true), Some(InputEventValue::MouseClick(Pressed::No))));
        assert!(input.on_primary_button(false, false).is_none());
        assert!(!input.is_down());
        input.on_primary_button(true, false);
        input.clear();
        assert!(input.release().is_none());
    }
}
