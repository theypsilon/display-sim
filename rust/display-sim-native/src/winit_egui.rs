use egui::{
    CursorIcon, Event, Key, Modifiers, MouseWheelUnit, OutputCommand, PlatformOutput, PointerButton, Pos2, RawInput, Rect, TouchPhase, Vec2, ViewportId,
};
use glutin::dpi::PhysicalSize;
use glutin::event::{ElementState, KeyboardInput, ModifiersState, MouseButton, MouseScrollDelta, VirtualKeyCode, WindowEvent};
use glutin::window::{CursorIcon as WinitCursorIcon, Window};

pub struct WinitEguiInput {
    events: Vec<Event>,
    modifiers: Modifiers,
    pointer_pos: Option<Pos2>,
    pointer_captured: bool,
    pointer_buttons_down: Vec<PointerButton>,
    pixels_per_point: f32,
    physical_size: PhysicalSize<u32>,
    focused: bool,
    clipboard: Option<arboard::Clipboard>,
    clipboard_fallback: String,
}

impl WinitEguiInput {
    pub fn new(window: &Window) -> Self {
        Self {
            events: Vec::new(),
            modifiers: Modifiers::default(),
            pointer_pos: None,
            pointer_captured: false,
            pointer_buttons_down: Vec::new(),
            pixels_per_point: window.scale_factor() as f32,
            physical_size: window.inner_size(),
            focused: true,
            clipboard: match arboard::Clipboard::new() {
                Ok(clipboard) => Some(clipboard),
                Err(error) => {
                    eprintln!("System clipboard is unavailable: {error}");
                    None
                }
            },
            clipboard_fallback: String::new(),
        }
    }

    pub fn on_window_event(&mut self, event: &WindowEvent<'_>, panel_rect: Option<Rect>) {
        match event {
            WindowEvent::Resized(size) => self.physical_size = *size,
            WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => {
                self.pixels_per_point = *scale_factor as f32;
                self.physical_size = **new_inner_size;
            }
            WindowEvent::CursorMoved { position, .. } => {
                let pos = Pos2::new(position.x as f32 / self.pixels_per_point, position.y as f32 / self.pixels_per_point);
                self.pointer_pos = Some(pos);
                self.events.push(Event::PointerMoved(pos));
            }
            WindowEvent::CursorLeft { .. } => {
                self.release_pointer_buttons();
                self.pointer_pos = None;
                self.pointer_captured = false;
                self.events.push(Event::PointerGone);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = *state == ElementState::Pressed;
                if *button == MouseButton::Left {
                    if pressed && self.pointer_over(panel_rect) {
                        self.pointer_captured = true;
                    } else if !pressed {
                        self.pointer_captured = false;
                    }
                }
                if let Some(button) = pointer_button(*button) {
                    let was_pressed = self.pointer_buttons_down.contains(&button);
                    if pressed {
                        if !was_pressed {
                            self.pointer_buttons_down.push(button);
                        }
                    } else {
                        self.pointer_buttons_down.retain(|pressed_button| *pressed_button != button);
                    }
                    if pressed != was_pressed {
                        self.events.push(Event::PointerButton {
                            pos: self.pointer_pos.unwrap_or_default(),
                            button,
                            pressed,
                            modifiers: self.modifiers,
                        });
                    }
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let (unit, delta) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (MouseWheelUnit::Line, Vec2::new(*x, *y)),
                    MouseScrollDelta::PixelDelta(pos) => (
                        MouseWheelUnit::Point,
                        Vec2::new(pos.x as f32 / self.pixels_per_point, pos.y as f32 / self.pixels_per_point),
                    ),
                };
                self.events.push(Event::MouseWheel {
                    unit,
                    delta,
                    phase: TouchPhase::Move,
                    modifiers: self.modifiers,
                });
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers_from_winit(*modifiers);
                self.events.push(Event::ModifiersChanged(self.modifiers));
            }
            WindowEvent::KeyboardInput { input, .. } => self.keyboard_input(input),
            WindowEvent::ReceivedCharacter(character) => {
                if !character.is_control() && !self.modifiers.command {
                    self.events.push(Event::Text(character.to_string()));
                }
            }
            WindowEvent::Focused(focused) => {
                self.focused = *focused;
                if !focused {
                    self.release_pointer_buttons();
                    self.pointer_captured = false;
                }
                self.events.push(Event::WindowFocused(*focused));
            }
            _ => {}
        }
    }

    pub fn pointer_is_captured(&self, panel_rect: Option<Rect>) -> bool {
        self.pointer_captured || self.pointer_over(panel_rect)
    }

    pub fn on_suspended(&mut self) {
        self.focused = false;
        self.release_pointer_buttons();
        self.pointer_captured = false;
        self.events.push(Event::WindowFocused(false));
    }

    pub fn take_input(&mut self, time: f64) -> RawInput {
        let screen_size = Vec2::new(
            self.physical_size.width as f32 / self.pixels_per_point,
            self.physical_size.height as f32 / self.pixels_per_point,
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

    pub fn handle_platform_output(&mut self, window: &Window, output: PlatformOutput, simulation_cursor_hidden: bool) {
        let icon = output.cursor_icon;
        if simulation_cursor_hidden {
            window.set_cursor_visible(false);
        } else {
            window.set_cursor_visible(icon != CursorIcon::None);
            if icon != CursorIcon::None {
                window.set_cursor_icon(cursor_icon(icon));
            }
        }
        for command in output.commands {
            if let OutputCommand::CopyText(text) = command {
                self.clipboard_fallback.clone_from(&text);
                if let Some(clipboard) = &mut self.clipboard {
                    if let Err(error) = clipboard.set_text(text) {
                        eprintln!("Could not write to the system clipboard: {error}");
                    }
                }
            }
        }
    }

    fn pointer_over(&self, panel_rect: Option<Rect>) -> bool {
        match (self.pointer_pos, panel_rect) {
            (Some(pos), Some(rect)) => rect.contains(pos),
            _ => false,
        }
    }

    fn release_pointer_buttons(&mut self) {
        self.events.extend(pointer_release_events(
            &mut self.pointer_buttons_down,
            self.pointer_pos.unwrap_or_default(),
            self.modifiers,
        ));
    }

    fn keyboard_input(&mut self, input: &KeyboardInput) {
        let Some(code) = input.virtual_keycode else { return };
        let pressed = input.state == ElementState::Pressed;
        if pressed && self.modifiers.command {
            match code {
                VirtualKeyCode::C => self.events.push(Event::Copy),
                VirtualKeyCode::X => self.events.push(Event::Cut),
                VirtualKeyCode::V => {
                    let text = self
                        .clipboard
                        .as_mut()
                        .and_then(|clipboard| clipboard.get_text().ok())
                        .unwrap_or_else(|| self.clipboard_fallback.clone());
                    self.events.push(Event::Paste(text));
                }
                _ => {}
            }
        }
        if let Some(key) = key_from_virtual(code) {
            self.events.push(Event::Key {
                key,
                physical_key: Some(key),
                pressed,
                repeat: false,
                modifiers: self.modifiers,
            });
        }
    }
}

fn pointer_release_events(buttons: &mut Vec<PointerButton>, pos: Pos2, modifiers: Modifiers) -> Vec<Event> {
    buttons
        .drain(..)
        .map(|button| Event::PointerButton {
            pos,
            button,
            pressed: false,
            modifiers,
        })
        .collect()
}

pub(crate) fn modifiers_from_winit(value: ModifiersState) -> Modifiers {
    let ctrl = value.ctrl();
    Modifiers {
        alt: value.alt(),
        ctrl,
        shift: value.shift(),
        mac_cmd: cfg!(target_os = "macos") && value.logo(),
        command: if cfg!(target_os = "macos") { value.logo() } else { ctrl },
    }
}

pub(crate) fn key_from_virtual(key: VirtualKeyCode) -> Option<Key> {
    use VirtualKeyCode as V;
    Some(match key {
        V::Down => Key::ArrowDown,
        V::Left => Key::ArrowLeft,
        V::Right => Key::ArrowRight,
        V::Up => Key::ArrowUp,
        V::Escape => Key::Escape,
        V::Tab => Key::Tab,
        V::Back => Key::Backspace,
        V::Return | V::NumpadEnter => Key::Enter,
        V::Space => Key::Space,
        V::Insert => Key::Insert,
        V::Delete => Key::Delete,
        V::Home => Key::Home,
        V::End => Key::End,
        V::PageUp => Key::PageUp,
        V::PageDown => Key::PageDown,
        V::Key0 | V::Numpad0 => Key::Num0,
        V::Key1 | V::Numpad1 => Key::Num1,
        V::Key2 | V::Numpad2 => Key::Num2,
        V::Key3 | V::Numpad3 => Key::Num3,
        V::Key4 | V::Numpad4 => Key::Num4,
        V::Key5 | V::Numpad5 => Key::Num5,
        V::Key6 | V::Numpad6 => Key::Num6,
        V::Key7 | V::Numpad7 => Key::Num7,
        V::Key8 | V::Numpad8 => Key::Num8,
        V::Key9 | V::Numpad9 => Key::Num9,
        V::A => Key::A,
        V::B => Key::B,
        V::C => Key::C,
        V::D => Key::D,
        V::E => Key::E,
        V::F => Key::F,
        V::G => Key::G,
        V::H => Key::H,
        V::I => Key::I,
        V::J => Key::J,
        V::K => Key::K,
        V::L => Key::L,
        V::M => Key::M,
        V::N => Key::N,
        V::O => Key::O,
        V::P => Key::P,
        V::Q => Key::Q,
        V::R => Key::R,
        V::S => Key::S,
        V::T => Key::T,
        V::U => Key::U,
        V::V => Key::V,
        V::W => Key::W,
        V::X => Key::X,
        V::Y => Key::Y,
        V::Z => Key::Z,
        V::F1 => Key::F1,
        V::F2 => Key::F2,
        V::F3 => Key::F3,
        V::F4 => Key::F4,
        V::F5 => Key::F5,
        V::F6 => Key::F6,
        V::F7 => Key::F7,
        V::F8 => Key::F8,
        V::F9 => Key::F9,
        V::F10 => Key::F10,
        V::F11 => Key::F11,
        V::F12 => Key::F12,
        V::F13 => Key::F13,
        V::F14 => Key::F14,
        V::F15 => Key::F15,
        V::F16 => Key::F16,
        V::F17 => Key::F17,
        V::F18 => Key::F18,
        V::F19 => Key::F19,
        V::F20 => Key::F20,
        V::F21 => Key::F21,
        V::F22 => Key::F22,
        V::F23 => Key::F23,
        V::F24 => Key::F24,
        V::WebBack | V::NavigateBackward => Key::BrowserBack,
        V::Comma => Key::Comma,
        V::Period => Key::Period,
        V::Slash => Key::Slash,
        V::Backslash => Key::Backslash,
        V::Minus | V::NumpadSubtract => Key::Minus,
        V::Equals | V::NumpadAdd => Key::Plus,
        V::Semicolon => Key::Semicolon,
        V::Apostrophe => Key::Quote,
        V::Grave => Key::Backtick,
        V::LBracket => Key::OpenBracket,
        V::RBracket => Key::CloseBracket,
        _ => return None,
    })
}

fn pointer_button(button: MouseButton) -> Option<PointerButton> {
    match button {
        MouseButton::Left => Some(PointerButton::Primary),
        MouseButton::Right => Some(PointerButton::Secondary),
        MouseButton::Middle => Some(PointerButton::Middle),
        MouseButton::Other(1) => Some(PointerButton::Extra1),
        MouseButton::Other(2) => Some(PointerButton::Extra2),
        _ => None,
    }
}

fn cursor_icon(icon: CursorIcon) -> WinitCursorIcon {
    match icon {
        CursorIcon::Default => WinitCursorIcon::Default,
        CursorIcon::ContextMenu => WinitCursorIcon::ContextMenu,
        CursorIcon::Help => WinitCursorIcon::Help,
        CursorIcon::PointingHand => WinitCursorIcon::Hand,
        CursorIcon::Progress => WinitCursorIcon::Progress,
        CursorIcon::Wait => WinitCursorIcon::Wait,
        CursorIcon::Cell => WinitCursorIcon::Cell,
        CursorIcon::Crosshair => WinitCursorIcon::Crosshair,
        CursorIcon::Text | CursorIcon::VerticalText => WinitCursorIcon::Text,
        CursorIcon::Alias => WinitCursorIcon::Alias,
        CursorIcon::Copy => WinitCursorIcon::Copy,
        CursorIcon::Move => WinitCursorIcon::Move,
        CursorIcon::NoDrop => WinitCursorIcon::NoDrop,
        CursorIcon::NotAllowed => WinitCursorIcon::NotAllowed,
        CursorIcon::Grab => WinitCursorIcon::Grab,
        CursorIcon::Grabbing => WinitCursorIcon::Grabbing,
        CursorIcon::AllScroll => WinitCursorIcon::AllScroll,
        CursorIcon::ResizeHorizontal | CursorIcon::ResizeColumn => WinitCursorIcon::EwResize,
        CursorIcon::ResizeVertical | CursorIcon::ResizeRow => WinitCursorIcon::NsResize,
        CursorIcon::ResizeNeSw => WinitCursorIcon::NeswResize,
        CursorIcon::ResizeNwSe => WinitCursorIcon::NwseResize,
        CursorIcon::ResizeEast => WinitCursorIcon::EResize,
        CursorIcon::ResizeSouthEast => WinitCursorIcon::SeResize,
        CursorIcon::ResizeSouth => WinitCursorIcon::SResize,
        CursorIcon::ResizeSouthWest => WinitCursorIcon::SwResize,
        CursorIcon::ResizeWest => WinitCursorIcon::WResize,
        CursorIcon::ResizeNorthWest => WinitCursorIcon::NwResize,
        CursorIcon::ResizeNorth => WinitCursorIcon::NResize,
        CursorIcon::ResizeNorthEast => WinitCursorIcon::NeResize,
        CursorIcon::ZoomIn => WinitCursorIcon::ZoomIn,
        CursorIcon::ZoomOut => WinitCursorIcon::ZoomOut,
        CursorIcon::None => WinitCursorIcon::Default,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_navigation_and_text_keys() {
        assert_eq!(key_from_virtual(VirtualKeyCode::Left), Some(Key::ArrowLeft));
        assert_eq!(key_from_virtual(VirtualKeyCode::A), Some(Key::A));
        assert_eq!(key_from_virtual(VirtualKeyCode::NumpadEnter), Some(Key::Enter));
        assert_eq!(key_from_virtual(VirtualKeyCode::F24), Some(Key::F24));
        assert_eq!(key_from_virtual(VirtualKeyCode::WebBack), Some(Key::BrowserBack));
        assert_eq!(key_from_virtual(VirtualKeyCode::LShift), None);
    }

    #[test]
    fn maps_modifiers() {
        let value = ModifiersState::CTRL | ModifiersState::SHIFT;
        let mapped = modifiers_from_winit(value);
        assert!(mapped.ctrl && mapped.shift && mapped.command);
        assert!(!mapped.alt);
    }

    #[test]
    fn pointer_leave_synthesizes_one_release_for_every_pressed_button() {
        let mut buttons = vec![PointerButton::Primary, PointerButton::Secondary];
        let events = pointer_release_events(&mut buttons, Pos2::new(12.0, 34.0), Modifiers::SHIFT);
        assert_eq!(events.len(), 2);
        assert!(buttons.is_empty());
        assert!(events
            .iter()
            .all(|event| matches!(event, Event::PointerButton { pos, pressed: false, modifiers, .. }
            if *pos == Pos2::new(12.0, 34.0) && *modifiers == Modifiers::SHIFT)));
    }
}
