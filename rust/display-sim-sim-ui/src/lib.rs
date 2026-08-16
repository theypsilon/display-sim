/* Copyright (c) 2019-2024 José manuel Barroso Galindo <theypsilon@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 */

use app_util::{AppError, AppResult};
use core::camera::{CameraChange, CameraLockMode};
use core::input_types::{Input, InputEventValue, Pressed};
use core::simulation_core_state::{KeyEventKind, Resources, ScalingMethod};
use core::ui_controller::filter_preset::FilterPresetOptions;
use core::ui_controller::EncodedValue;
use egui::{
    Align, Align2, Color32, Context, CornerRadius, CursorIcon, Event, FontId, Id, Popup, Pos2, Rect, Response, ScrollArea, Sense, Stroke, TextEdit, TextStyle,
    Ui, Vec2,
};
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

/// An [`EncodedValue`] implementation for in-process UI values.
///
/// The web frontend has to encode values through JavaScript. The native panel
/// can preserve their numeric type and only parses when a controller requests
/// a different representation.
#[derive(Clone, Debug, PartialEq)]
pub enum PanelEncodedValue {
    Number(f64),
    Text(String),
}

impl PanelEncodedValue {
    fn number(&self) -> AppResult<f64> {
        match self {
            Self::Number(value) => Ok(*value),
            Self::Text(value) => value
                .parse::<f64>()
                .map_err(|error| AppError::new(format!("invalid number {value:?}: {error}"))),
        }
    }
}

impl EncodedValue for PanelEncodedValue {
    fn to_f64(&self) -> AppResult<f64> {
        self.number()
    }

    fn to_f32(&self) -> AppResult<f32> {
        Ok(self.number()? as f32)
    }

    fn to_u32(&self) -> AppResult<u32> {
        Ok(self.number()? as u32)
    }

    fn to_i32(&self) -> AppResult<i32> {
        Ok(self.number()? as i32)
    }

    fn to_usize(&self) -> AppResult<usize> {
        Ok(self.number()? as usize)
    }

    fn to_string(&self) -> AppResult<String> {
        Ok(match self {
            Self::Number(value) => value.to_string(),
            Self::Text(value) => value.clone(),
        })
    }
}

/// Crosses the dispatcher/UI boundary without tying the panel to a platform.
#[derive(Default, Debug)]
pub struct PanelEventSink {
    fps: Option<f32>,
    messages: VecDeque<String>,
    toggle_requests: usize,
}

#[derive(Default, Debug, PartialEq)]
pub struct DrainedPanelEvents {
    pub fps: Option<f32>,
    pub messages: Vec<String>,
    pub toggle_requests: usize,
}

impl PanelEventSink {
    pub fn set_fps(&mut self, fps: f32) {
        self.fps = Some(fps);
    }

    pub fn push_message(&mut self, message: impl Into<String>) {
        self.messages.push_back(message.into());
    }

    pub fn request_toggle(&mut self) {
        self.toggle_requests += 1;
    }

    pub fn drain(&mut self) -> DrainedPanelEvents {
        DrainedPanelEvents {
            fps: self.fps.take(),
            messages: self.messages.drain(..).collect(),
            toggle_requests: std::mem::take(&mut self.toggle_requests),
        }
    }
}

pub type SharedPanelEvents = Rc<RefCell<PanelEventSink>>;

pub fn shared_panel_events() -> SharedPanelEvents {
    Rc::new(RefCell::new(PanelEventSink::default()))
}

#[derive(Default)]
struct SyntheticKeys {
    down: HashSet<String>,
    release_next_frame: HashSet<String>,
    continuous_keys: HashSet<String>,
    seen_continuous_keys: HashSet<String>,
}

impl SyntheticKeys {
    fn begin_frame(&mut self, output: &mut Vec<InputEventValue>) {
        self.seen_continuous_keys.clear();
        let releases = std::mem::take(&mut self.release_next_frame);
        for key in releases {
            self.set_held(&key, false, output);
        }
    }

    fn set_continuous(&mut self, key: &str, held: bool, output: &mut Vec<InputEventValue>) {
        self.continuous_keys.insert(key.to_owned());
        self.seen_continuous_keys.insert(key.to_owned());
        self.set_held(key, held, output);
    }

    fn drive_button(&mut self, ui: &Ui, key: &str, response: &Response, enabled: bool, output: &mut Vec<InputEventValue>) {
        let pointer_held = enabled && response.is_pointer_button_down_on();
        // Match the web controls' explicit key-down/key-up contract rather
        // than converting keyboard activation into a timed pulse.
        let keyboard_held = enabled && response.has_focus() && ui.input(|input| input.key_down(egui::Key::Enter) || input.key_down(egui::Key::Space));
        let held = pointer_held || keyboard_held;
        let was_down = self.down.contains(key);
        self.set_continuous(key, held, output);

        // egui can receive pointer/key down and up between two rendered
        // frames. In that case `clicked` is true, but the final held state is
        // already false. Preserve the activation for one simulation frame;
        // otherwise quick selector clicks and camera-pad taps disappear.
        if enabled && response.clicked() && !held && !was_down {
            self.pulse(key, output);
        }
    }

    fn end_frame(&mut self, output: &mut Vec<InputEventValue>) {
        let missing: Vec<_> = self.continuous_keys.difference(&self.seen_continuous_keys).cloned().collect();
        for key in missing {
            self.set_held(&key, false, output);
            self.continuous_keys.remove(&key);
        }
    }

    fn pulse(&mut self, key: &str, output: &mut Vec<InputEventValue>) {
        self.set_held(key, true, output);
        self.release_next_frame.insert(key.to_owned());
    }

    fn set_held(&mut self, key: &str, held: bool, output: &mut Vec<InputEventValue>) {
        if held == self.down.contains(key) {
            return;
        }
        if held {
            self.down.insert(key.to_owned());
        } else {
            self.down.remove(key);
            self.release_next_frame.remove(key);
        }
        output.push(InputEventValue::Keyboard {
            pressed: Pressed::from_bool(held),
            key: key.to_owned(),
        });
    }

    fn release_all(&mut self, output: &mut Vec<InputEventValue>) {
        self.release_next_frame.clear();
        self.continuous_keys.clear();
        self.seen_continuous_keys.clear();
        for key in std::mem::take(&mut self.down) {
            output.push(InputEventValue::Keyboard { pressed: Pressed::No, key });
        }
    }
}

#[derive(Clone)]
struct ControllerSet {
    event_tag: &'static str,
    value: PanelEncodedValue,
}

struct Toast {
    text: String,
    fade_at: f64,
    expires_at: f64,
}

const PANEL_X: f32 = 18.0;
const PANEL_WIDTH: f32 = 400.0;
const SECTION_HEIGHT: f32 = 28.0;
const ROW_HEIGHT: f32 = 28.0;
const TOGGLE_HEIGHT: f32 = 20.0;
// `.info-category` has 4 px of left padding and centers a 396 px
// `.menu-2` child, which places the child's left edge 6 px into the panel.
const CATEGORY_INSET: f32 = 6.0;
const CATEGORY_WIDTH: f32 = 396.0;
const CONTROL_X: f32 = 197.0;
const CONTROL_WIDTH: f32 = 200.0;
const INPUT_WIDTH: f32 = 150.0;
const SMALL_BUTTON_WIDTH: f32 = 25.0;
const INPUT_HEIGHT: f32 = 21.0;
const PANEL_ALPHA: u8 = 230;

#[derive(Clone, Copy)]
enum Accent {
    Red,
    Blue,
    Green,
    Yellow,
    Lilac,
    White,
    Grey,
}

impl Accent {
    fn color(self) -> Color32 {
        let (r, g, b) = match self {
            Self::Red => (230, 29, 95),
            Self::Blue => (47, 161, 214),
            Self::Green => (0, 255, 127),
            Self::Yellow => (240, 230, 140),
            Self::Lilac => (128, 103, 135),
            Self::White => (238, 238, 238),
            Self::Grey => (140, 140, 140),
        };
        web_color(r, g, b)
    }
}

struct SectionStates {
    presets: bool,
    image_scaling: bool,
    performance: bool,
    colors: bool,
    geometry: bool,
    camera: bool,
    modifiers: bool,
    webgl: bool,
    extra: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SimPanelSection {
    Presets,
    ImageScaling,
    Performance,
    Colors,
    GeometryAndTextures,
    Camera,
    CommandModifiers,
    WebGlSettings,
    Extra,
}

impl Default for SectionStates {
    fn default() -> Self {
        Self {
            presets: true,
            image_scaling: false,
            performance: true,
            colors: false,
            geometry: false,
            camera: false,
            modifiers: false,
            webgl: false,
            extra: false,
        }
    }
}

/// The reusable simulation control panel. It owns no window or GL resources.
pub struct SimPanel {
    context: Context,
    visible: bool,
    fps: f32,
    time: f64,
    toasts: VecDeque<Toast>,
    panel_rect: Option<Rect>,
    synthetic: SyntheticKeys,
    sections: SectionStates,
}

impl Default for SimPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl SimPanel {
    pub fn new() -> Self {
        let context = Context::default();
        configure_web_style(&context);
        Self {
            context,
            visible: true,
            fps: 60.0,
            time: 0.0,
            toasts: VecDeque::new(),
            panel_rect: None,
            synthetic: SyntheticKeys::default(),
            sections: SectionStates::default(),
        }
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn panel_rect(&self) -> Option<Rect> {
        self.panel_rect
    }

    /// Sets a single expanded category, useful for restoring panel state and
    /// for deterministic visual-regression captures.
    pub fn open_only(&mut self, section: SimPanelSection) {
        self.sections = SectionStates {
            presets: section == SimPanelSection::Presets,
            image_scaling: section == SimPanelSection::ImageScaling,
            performance: section == SimPanelSection::Performance,
            colors: section == SimPanelSection::Colors,
            geometry: section == SimPanelSection::GeometryAndTextures,
            camera: section == SimPanelSection::Camera,
            modifiers: section == SimPanelSection::CommandModifiers,
            webgl: section == SimPanelSection::WebGlSettings,
            extra: section == SimPanelSection::Extra,
        };
    }

    pub fn release_all(&mut self, input: &mut Input) {
        let mut events = Vec::new();
        self.synthetic.release_all(&mut events);
        push_input_events(input, events);
    }

    pub fn run(&mut self, raw_input: egui::RawInput, res: &mut Resources, input: &mut Input, events: &SharedPanelEvents) -> AppResult<egui::FullOutput> {
        self.time = raw_input.time.unwrap_or(self.time + raw_input.predicted_dt as f64);
        let focused = raw_input.focused;
        let render_rect = raw_input.screen_rect.unwrap_or_else(|| {
            Rect::from_min_size(
                Pos2::ZERO,
                Vec2::new(res.video.viewport_size.width as f32, res.video.viewport_size.height as f32),
            )
        });
        let drained = events.borrow_mut().drain();
        if let Some(fps) = drained.fps {
            self.fps = fps;
        }
        for message in drained.messages {
            // The web frontend replaces the current top message rather than
            // stacking notifications.
            self.toasts.clear();
            let fade_at = self.time + message.chars().count() as f64 * 0.1;
            self.toasts.push_back(Toast {
                text: message,
                fade_at,
                expires_at: fade_at + 1.2,
            });
        }
        while self.toasts.front().is_some_and(|toast| toast.expires_at <= self.time) {
            self.toasts.pop_front();
        }

        let mut input_events = Vec::new();
        self.synthetic.begin_frame(&mut input_events);
        if drained.toggle_requests % 2 == 1 {
            self.visible = !self.visible;
            self.synthetic.release_all(&mut input_events);
        }
        if !focused {
            self.synthetic.release_all(&mut input_events);
        }

        let context = self.context.clone();
        let mut controller_sets = Vec::new();
        let mut selected_preset = None;
        let was_visible = self.visible;
        let output = context.run_ui(raw_input, |root_ui| {
            let ctx = root_ui.ctx().clone();
            self.show_chrome(&ctx);
            if res.quit {
                self.panel_rect = None;
                self.show_session_ended(&ctx);
                return;
            }
            // Match the web panel's `95vh` against the actual render target.
            // The native compositor can report a logical window height that
            // differs from the simulation framebuffer on fractional scaling.
            let max_content_height = (render_rect.height() * 0.95).max(TOGGLE_HEIGHT);
            let response = egui::Area::new(Id::new("display-sim-controls"))
                .fixed_pos(Pos2::new(PANEL_X, 0.0))
                .order(egui::Order::Foreground)
                .default_size(Vec2::new(PANEL_WIDTH, max_content_height + TOGGLE_HEIGHT))
                .constrain_to(render_rect)
                .show(&ctx, |ui| {
                    ui.set_min_width(PANEL_WIDTH);
                    ui.set_max_width(PANEL_WIDTH);
                    if self.visible {
                        ScrollArea::vertical().max_height(max_content_height).auto_shrink([false, true]).show(ui, |ui| {
                            ui.set_min_width(PANEL_WIDTH);
                            ui.set_max_width(PANEL_WIDTH);
                            self.show_panel(ui, res, &mut controller_sets, &mut selected_preset, &mut input_events);
                        });
                    }
                    if control_row(ui, if self.visible { "Close Controls" } else { "Open Controls" }).clicked() {
                        self.visible = !self.visible;
                    }
                });
            self.panel_rect = Some(response.response.rect);
        });

        self.synthetic.end_frame(&mut input_events);
        if was_visible && !self.visible {
            self.synthetic.release_all(&mut input_events);
        }
        for set in controller_sets {
            route_controller_value(res, set.event_tag, set.value)?;
        }
        if let Some(preset) = selected_preset {
            select_preset(res, preset)?;
        }
        push_input_events(input, input_events);
        Ok(output)
    }

    fn show_session_ended(&self, ctx: &Context) {
        egui::Area::new(Id::new("display-sim-session-ended"))
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let (rect, _) = ui.allocate_exact_size(Vec2::new(360.0, 76.0), Sense::hover());
                ui.painter()
                    .rect_filled(rect, CornerRadius::ZERO, Color32::from_rgba_unmultiplied(0, 0, 0, 210));
                ui.painter()
                    .rect_stroke(rect, CornerRadius::ZERO, Stroke::new(1.0, web_color(44, 44, 44)), egui::StrokeKind::Inside);
                ui.painter().text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    "Simulation ended",
                    FontId::proportional(24.0),
                    Color32::WHITE,
                );
            });
    }

    fn show_chrome(&self, ctx: &Context) {
        egui::Area::new(Id::new("display-sim-fps"))
            .anchor(Align2::RIGHT_TOP, [-20.0, 20.0])
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let (rect, _) = ui.allocate_exact_size(Vec2::new(20.0, 20.0), Sense::hover());
                ui.painter().text(
                    rect.left_top(),
                    Align2::LEFT_TOP,
                    format!("{:.0}", self.fps),
                    FontId::proportional(14.0),
                    Color32::WHITE,
                );
            });
        if let Some(toast) = self.toasts.back() {
            let opacity = if self.time <= toast.fade_at {
                0.75
            } else {
                (0.75 * ((toast.expires_at - self.time) / (toast.expires_at - toast.fade_at))).clamp(0.0, 0.75)
            };
            egui::Area::new(Id::new("display-sim-toasts"))
                .anchor(Align2::CENTER_BOTTOM, [0.0, -30.0])
                .order(egui::Order::Tooltip)
                .show(ctx, |ui| {
                    let font = FontId::proportional(28.0);
                    let galley = ui.painter().layout_no_wrap(toast.text.clone(), font.clone(), Color32::WHITE);
                    let (rect, _) = ui.allocate_exact_size(galley.size() + Vec2::new(40.0, 20.0), Sense::hover());
                    let alpha = (opacity * 255.0).round() as u8;
                    ui.painter()
                        .rect_filled(rect, CornerRadius::ZERO, Color32::from_rgba_unmultiplied(0, 0, 0, alpha));
                    ui.painter().galley(
                        rect.center() - galley.size() * 0.5,
                        galley,
                        Color32::from_rgba_unmultiplied(255, 255, 255, alpha),
                    );
                });
        }
    }

    fn show_panel(
        &mut self,
        ui: &mut Ui,
        res: &Resources,
        sets: &mut Vec<ControllerSet>,
        selected_preset: &mut Option<FilterPresetOptions>,
        input_events: &mut Vec<InputEventValue>,
    ) {
        if section_header(ui, "Presets", &mut self.sections.presets) {
            self.preset_grid(ui, res, selected_preset);
        }

        if section_header(ui, "Image Scaling", &mut self.sections.image_scaling) {
            self.selector(
                ui,
                "Scaling Method",
                None,
                Accent::Blue,
                &res.scaling.scaling_method.to_string(),
                "scaling-method-dec",
                "scaling-method-inc",
                input_events,
            );
            let custom = matches!(res.scaling.scaling_method, ScalingMethod::Custom);
            let mut width = res.scaling.custom_resolution.width;
            let mut height = res.scaling.custom_resolution.height;
            let (first, resolution_changed) = self.pair_f32(
                ui,
                "Image resolution",
                Accent::Lilac,
                &mut width,
                &mut height,
                1.0..=10_000.0,
                1.0..=10_000.0,
                1.0,
                "✕",
                ["custom-scaling-resolution-width-dec", "custom-scaling-resolution-width-inc"],
                ["custom-scaling-resolution-height-dec", "custom-scaling-resolution-height-inc"],
                custom,
                input_events,
            );
            if resolution_changed {
                input_events.push(InputEventValue::CustomScalingResolutionWidth(width));
                input_events.push(InputEventValue::CustomScalingResolutionHeight(height));
            }
            let mut aspect_x = res.scaling.custom_aspect_ratio.width;
            let mut aspect_y = res.scaling.custom_aspect_ratio.height;
            let (_, aspect_changed) = self.pair_f32(
                ui,
                "Aspect Ratio",
                Accent::Lilac,
                &mut aspect_x,
                &mut aspect_y,
                1.0..=7_680.0,
                1.0..=4_320.0,
                1.0,
                ":",
                ["custom-scaling-aspect-ratio-x-dec", "custom-scaling-aspect-ratio-x-inc"],
                ["custom-scaling-aspect-ratio-y-dec", "custom-scaling-aspect-ratio-y-inc"],
                custom,
                input_events,
            );
            if aspect_changed {
                input_events.push(InputEventValue::CustomScalingAspectRatioX(aspect_x));
                input_events.push(InputEventValue::CustomScalingAspectRatioY(aspect_y));
            }
            let mut stretch = res.scaling.custom_stretch;
            let (_, stretch_changed) = checkbox_row(ui, "Stretch to nearest border", Accent::Lilac, &mut stretch, custom);
            if stretch_changed {
                input_events.push(InputEventValue::CustomScalingStretchNearest(stretch));
            }
            let mut pixel_width = res.scaling.pixel_width;
            let (last, pixel_width_changed) = self.number_f32(
                ui,
                "Pixel width",
                ("O", "Shift + O"),
                Accent::Yellow,
                &mut pixel_width,
                0.0..=10.0,
                0.001,
                "pixel-width-dec",
                "pixel-width-inc",
                custom,
                input_events,
            );
            if pixel_width_changed {
                input_events.push(InputEventValue::PixelWidth(pixel_width));
            }
            if !custom {
                let overlay = Rect::from_min_max(Pos2::new(first.left() + CATEGORY_INSET, first.top()), Pos2::new(last.right(), last.bottom()));
                ui.painter()
                    .rect_filled(overlay, CornerRadius::ZERO, Color32::from_rgba_unmultiplied(0, 0, 0, 128));
            }
        }

        if section_header(ui, "Performance", &mut self.sections.performance) {
            self.selector(
                ui,
                "Internal Resolution",
                Some(("Y", "Shift + Y")),
                Accent::White,
                &res.controllers.internal_resolution.to_string(),
                "internal-resolution-dec",
                "internal-resolution-inc",
                input_events,
            );
            let mut blur = res.controllers.blur_passes.value;
            let (_, changed) = self.number_usize(
                ui,
                "Blur passes",
                ("J", "Shift + J"),
                Accent::Blue,
                &mut blur,
                0..=100,
                1.0,
                "blur-level-dec",
                "blur-level-inc",
                true,
                input_events,
            );
            if changed {
                set(sets, "front2back:blur-level", blur as f64);
            }
        }

        if section_header(ui, "Colors", &mut self.sections.colors) {
            self.rgb_matrix(ui, res, sets);
            let mut gamma = res.controllers.color_gamma.value;
            let (_, changed) = self.number_f32(
                ui,
                "Gamma correction",
                ("????", "Shift + ????"),
                Accent::Lilac,
                &mut gamma,
                0.0..=1.0,
                0.1,
                "color-gamma-dec",
                "color-gamma-inc",
                true,
                input_events,
            );
            if changed {
                set(sets, "front2back:color-gamma", gamma as f64);
            }
            let mut noise = res.controllers.color_noise.value;
            let (_, changed) = self.number_f32(
                ui,
                "Color noise",
                ("????", "Shift + ????"),
                Accent::Yellow,
                &mut noise,
                0.0..=1.0,
                0.1,
                "color-noise-dec",
                "color-noise-inc",
                true,
                input_events,
            );
            if changed {
                set(sets, "front2back:color-noise", noise as f64);
            }
            let packed = res.controllers.light_color.value as u32;
            let mut color = [((packed >> 16) & 0xff) as u8, ((packed >> 8) & 0xff) as u8, (packed & 0xff) as u8];
            if color_row(ui, "Source light color", Accent::Blue, &mut color) {
                let value = ((color[0] as u32) << 16) | ((color[1] as u32) << 8) | color[2] as u32;
                set(sets, "front2back:light-color", value as f64);
            }
            let mut bright = res.controllers.extra_bright.value;
            let (_, changed) = self.number_f32(
                ui,
                "Brightness",
                ("X", "Shift + X"),
                Accent::White,
                &mut bright,
                -1.0..=1.0,
                0.001,
                "pixel-brightness-dec",
                "pixel-brightness-inc",
                true,
                input_events,
            );
            if changed {
                set(sets, "front2back:pixel-brightness", bright as f64);
            }
            let mut contrast = res.controllers.extra_contrast.value;
            let (_, changed) = self.number_f32(
                ui,
                "Contrast",
                ("Z", "Shift + Z"),
                Accent::White,
                &mut contrast,
                0.0..=20.0,
                0.001,
                "pixel-contrast-dec",
                "pixel-contrast-inc",
                true,
                input_events,
            );
            if changed {
                set(sets, "front2back:pixel-contrast", contrast as f64);
            }
        }

        if section_header(ui, "Geometry & Textures", &mut self.sections.geometry) {
            self.selector(
                ui,
                "Screen curvature type",
                Some(("B", "Shift + B")),
                Accent::White,
                &res.controllers.screen_curvature_kind.value.to_string(),
                "screen-curvature-dec",
                "screen-curvature-inc",
                input_events,
            );
            let mut horizontal_gap = res.controllers.cur_pixel_horizontal_gap.value;
            let (_, changed) = self.number_f32(
                ui,
                "Horizontal gap",
                ("U", "Shift + U"),
                Accent::Red,
                &mut horizontal_gap,
                0.0..=10.0,
                0.001,
                "pixel-horizontal-gap-dec",
                "pixel-horizontal-gap-inc",
                true,
                input_events,
            );
            if changed {
                set(sets, "front2back:pixel-horizontal-gap", horizontal_gap as f64);
            }
            let mut vertical_gap = res.controllers.cur_pixel_vertical_gap.value;
            let (_, changed) = self.number_f32(
                ui,
                "Vertical gap",
                ("I", "Shift + I"),
                Accent::Red,
                &mut vertical_gap,
                0.0..=10.0,
                0.001,
                "pixel-vertical-gap-dec",
                "pixel-vertical-gap-inc",
                true,
                input_events,
            );
            if changed {
                set(sets, "front2back:pixel-vertical-gap", vertical_gap as f64);
            }
            let mut vertical_lpp = res.controllers.vertical_lpp.value;
            let (_, changed) = self.number_usize(
                ui,
                "V. lines per pixel",
                ("K", "Shift + K"),
                Accent::Lilac,
                &mut vertical_lpp,
                0..=100,
                1.0,
                "vertical-lpp-dec",
                "vertical-lpp-inc",
                true,
                input_events,
            );
            if changed {
                set(sets, "front2back:vertical-lpp", vertical_lpp as f64);
            }
            let mut horizontal_lpp = res.controllers.horizontal_lpp.value;
            let (_, changed) = self.number_usize(
                ui,
                "H. lines per pixel",
                ("L", "Shift + L"),
                Accent::Lilac,
                &mut horizontal_lpp,
                0..=100,
                1.0,
                "horizontal-lpp-dec",
                "horizontal-lpp-inc",
                true,
                input_events,
            );
            if changed {
                set(sets, "front2back:horizontal-lpp", horizontal_lpp as f64);
            }
            self.selector(
                ui,
                "Color channels type",
                Some(("C", "Shift + C")),
                Accent::Red,
                &res.controllers.color_channels.value.to_string(),
                "color-representation-dec",
                "color-representation-inc",
                input_events,
            );
            self.selector(
                ui,
                "Pixel geometry type",
                Some(("V", "Shift + V")),
                Accent::Yellow,
                &res.controllers.pixels_geometry_kind.value.to_string(),
                "pixel-geometry-dec",
                "pixel-geometry-inc",
                input_events,
            );
            self.selector(
                ui,
                "Pixel texture",
                Some(("N", "Shift + N")),
                Accent::Blue,
                &res.controllers.pixel_shadow_shape_kind.value.to_string(),
                "pixel-shadow-shape-dec",
                "pixel-shadow-shape-inc",
                input_events,
            );
            let mut height = res.controllers.pixel_shadow_height.value;
            let (_, changed) = self.number_f32(
                ui,
                "Pixel variable height",
                ("M", "Shift + M"),
                Accent::Lilac,
                &mut height,
                0.0..=1.0,
                0.001,
                "pixel-shadow-height-dec",
                "pixel-shadow-height-inc",
                true,
                input_events,
            );
            if changed {
                set(sets, "front2back:pixel-shadow-height", height as f64);
            }
            self.selector(
                ui,
                "Texture interpolation",
                Some(("H", "Shift + H")),
                Accent::Yellow,
                &res.controllers.texture_interpolation.value.to_string(),
                "texture-interpolation-dec",
                "texture-interpolation-inc",
                input_events,
            );
            let mut backlight = res.controllers.backlight_percent.value;
            let (_, changed) = self.number_f32(
                ui,
                "Backlight",
                (",", "."),
                Accent::Green,
                &mut backlight,
                0.0..=1.0,
                0.001,
                "backlight-percent-dec",
                "backlight-percent-inc",
                true,
                input_events,
            );
            if changed {
                set(sets, "front2back:backlight-percent", backlight as f64);
            }
            if action_row(ui, "Reset Filter Values", Accent::Grey).clicked() {
                self.synthetic.pulse("reset-filters", input_events);
            }
        }

        if section_header(ui, "Camera", &mut self.sections.camera) {
            self.selector(
                ui,
                "Movement Type",
                Some(("G", "Shift + G")),
                Accent::Lilac,
                &res.camera.locked_mode.to_string(),
                "camera-movement-mode-dec",
                "camera-movement-mode-inc",
                input_events,
            );
            self.camera_buttons(ui, res.camera.locked_mode, input_events);
            self.camera_matrix(ui, res, input_events);
            let mut zoom = res.camera.zoom;
            let (_, changed) = self.number_f32(
                ui,
                "Zoom",
                ("Mouse Wheel Up", "Mouse Wheel Down"),
                Accent::Blue,
                &mut zoom,
                1.0..=45.0,
                1.0,
                "camera-zoom-dec",
                "camera-zoom-inc",
                true,
                input_events,
            );
            if changed {
                input_events.push(InputEventValue::Camera(CameraChange::Zoom(zoom)));
            }
            if action_row(ui, "Reset Position", Accent::Grey).clicked() {
                self.synthetic.pulse("reset-camera", input_events);
            }
        }

        if section_header(ui, "Command Modifiers", &mut self.sections.modifiers) {
            self.selector(
                ui,
                "Camera speed",
                Some(("F", "Shift + F")),
                Accent::Red,
                &format!("x{}", format_number(res.camera.movement_speed)),
                "move-speed-dec",
                "move-speed-inc",
                input_events,
            );
            self.selector(
                ui,
                "Filter speed",
                Some(("R", "Shift + R")),
                Accent::Blue,
                &format!("x{}", format_number(res.main.filter_speed)),
                "pixel-speed-dec",
                "pixel-speed-inc",
                input_events,
            );
            if action_row(ui, "Reset Modifiers", Accent::Grey).clicked() {
                self.synthetic.pulse("reset-speeds", input_events);
            }
        }

        if section_header(ui, "WebGL Settings", &mut self.sections.webgl) {
            static_selector_row(ui, "Performance", Accent::Red, "host-managed");
            static_selector_row(ui, "Antialias", Accent::Red, "host-managed");
        }

        if section_header(ui, "Extra", &mut self.sections.extra) && action_row(ui, "Take Screenshot", Accent::Yellow).clicked() {
            self.synthetic.pulse("capture-framebuffer", input_events);
        }

        if exit_row(ui, "Exit Simulation").clicked() {
            self.synthetic.pulse("quit-simulation", input_events);
        }
    }

    fn preset_grid(&mut self, ui: &mut Ui, res: &Resources, selected_preset: &mut Option<FilterPresetOptions>) {
        let (full_rect, _) = ui.allocate_exact_size(Vec2::new(PANEL_WIDTH, 151.0), Sense::hover());
        let rect = Rect::from_min_size(full_rect.min + Vec2::new(CATEGORY_INSET, 0.0), Vec2::new(CATEGORY_WIDTH, 151.0));
        let painter = ui.painter().clone();
        painter.rect_filled(rect, CornerRadius::ZERO, web_color(26, 26, 26));
        painter.rect_filled(Rect::from_min_size(rect.min, Vec2::new(3.0, 150.0)), CornerRadius::ZERO, Accent::Grey.color());
        painter.line_segment(
            [Pos2::new(rect.left(), rect.bottom() - 0.5), Pos2::new(rect.right(), rect.bottom() - 0.5)],
            Stroke::new(1.0, web_color(44, 44, 44)),
        );

        for (index, preset) in FilterPresetOptions::ALL.into_iter().enumerate() {
            let column = index % 2;
            let row = index / 2;
            let selected = res.controllers.preset_kind.value == preset;
            let size = if selected { Vec2::new(166.0, 34.0) } else { Vec2::new(161.0, 30.0) };
            let cell_min = rect.min + Vec2::new(3.0 + column as f32 * 196.5, row as f32 * 50.0);
            let cell = Rect::from_min_size(cell_min, Vec2::new(196.5, 50.0));
            let button_rect = Rect::from_center_size(cell.center(), size);
            let response = ui
                .interact(button_rect, ui.make_persistent_id(("preset", preset.to_string())), Sense::click())
                .on_hover_cursor(CursorIcon::PointingHand);
            let fill = if response.hovered() || selected {
                web_color(70, 70, 70)
            } else {
                web_color(48, 48, 48)
            };
            painter.rect_filled(button_rect, CornerRadius::ZERO, fill);
            if selected {
                painter.rect_stroke(
                    button_rect,
                    CornerRadius::ZERO,
                    Stroke::new(2.0, Accent::Blue.color()),
                    egui::StrokeKind::Inside,
                );
            }
            painter.text(
                button_rect.center(),
                Align2::CENTER_CENTER,
                preset.get_description(),
                FontId::proportional(13.0),
                if selected { web_color(255, 255, 255) } else { Accent::Blue.color() },
            );
            // The web preset anchors dispatch even when the active preset is
            // clicked again; preserve that event behavior here as well.
            if response.clicked() {
                *selected_preset = Some(preset);
            }
        }
    }

    fn selector(
        &mut self,
        ui: &mut Ui,
        label: &str,
        hotkeys: Option<(&str, &str)>,
        accent: Accent,
        value: &str,
        dec: &str,
        inc: &str,
        events: &mut Vec<InputEventValue>,
    ) -> Rect {
        let rect = row_base(ui, label, hotkeys, accent, false);
        let input_rect = row_control_rect(rect);
        // The web `.selector-inc` hit target wraps both the displayed value
        // and the plus button, so either area starts the increment action.
        let increment_rect = Rect::from_min_size(input_rect.min, Vec2::new(INPUT_WIDTH + SMALL_BUTTON_WIDTH, INPUT_HEIGHT));
        let plus = ui
            .interact(increment_rect, ui.make_persistent_id((label, "inc")), Sense::click())
            .on_hover_cursor(CursorIcon::PointingHand);
        ui.painter().rect_filled(
            increment_rect,
            CornerRadius::ZERO,
            if plus.is_pointer_button_down_on() {
                web_color(70, 70, 70)
            } else if plus.hovered() {
                web_color(60, 60, 60)
            } else {
                web_color(48, 48, 48)
            },
        );
        ui.painter().text(
            Rect::from_min_size(input_rect.min, Vec2::new(INPUT_WIDTH, INPUT_HEIGHT)).center(),
            Align2::CENTER_CENTER,
            value,
            FontId::proportional(12.0),
            Accent::Blue.color(),
        );
        ui.painter().text(
            Rect::from_min_size(input_rect.min + Vec2::new(INPUT_WIDTH, 0.0), Vec2::new(SMALL_BUTTON_WIDTH, INPUT_HEIGHT)).center(),
            Align2::CENTER_CENTER,
            "+",
            FontId::proportional(12.0),
            Accent::Blue.color(),
        );
        let minus = small_button(
            ui,
            Rect::from_min_size(
                input_rect.min + Vec2::new(INPUT_WIDTH + SMALL_BUTTON_WIDTH, 0.0),
                Vec2::new(SMALL_BUTTON_WIDTH, INPUT_HEIGHT),
            ),
            "-",
            (label, "dec"),
        );
        self.synthetic.drive_button(ui, inc, &plus, true, events);
        self.synthetic.drive_button(ui, dec, &minus, true, events);
        rect
    }

    #[allow(clippy::too_many_arguments)]
    fn number_f32(
        &mut self,
        ui: &mut Ui,
        label: &str,
        hotkeys: (&str, &str),
        accent: Accent,
        value: &mut f32,
        range: std::ops::RangeInclusive<f32>,
        speed: f64,
        dec: &str,
        inc: &str,
        enabled: bool,
        events: &mut Vec<InputEventValue>,
    ) -> (Rect, bool) {
        let rect = row_base(ui, label, Some(hotkeys), accent, false);
        let input_rect = row_control_rect(rect);
        paint_input(ui, input_rect, false);
        let display = match label {
            "Backlight" => NumberDisplay::Fixed(3),
            "Zoom" => NumberDisplay::Fixed(2),
            _ => NumberDisplay::Trimmed(3),
        };
        let changed = if enabled {
            edit_f32_display(
                ui,
                Rect::from_min_size(input_rect.min, Vec2::new(INPUT_WIDTH, INPUT_HEIGHT)),
                ("f32", label),
                value,
                range,
                speed,
                display,
            )
        } else {
            paint_disabled_value(ui, input_rect, &display.format(*value as f64));
            false
        };
        let plus_rect = Rect::from_min_size(input_rect.min + Vec2::new(INPUT_WIDTH, 0.0), Vec2::new(SMALL_BUTTON_WIDTH, INPUT_HEIGHT));
        let minus_rect = Rect::from_min_size(
            input_rect.min + Vec2::new(INPUT_WIDTH + SMALL_BUTTON_WIDTH, 0.0),
            Vec2::new(SMALL_BUTTON_WIDTH, INPUT_HEIGHT),
        );
        let plus = small_button_enabled(ui, plus_rect, "+", (label, "inc"), enabled);
        let minus = small_button_enabled(ui, minus_rect, "-", (label, "dec"), enabled);
        self.synthetic.drive_button(ui, inc, &plus, enabled, events);
        self.synthetic.drive_button(ui, dec, &minus, enabled, events);
        (rect, changed)
    }

    #[allow(clippy::too_many_arguments)]
    fn number_usize(
        &mut self,
        ui: &mut Ui,
        label: &str,
        hotkeys: (&str, &str),
        accent: Accent,
        value: &mut usize,
        range: std::ops::RangeInclusive<usize>,
        speed: f64,
        dec: &str,
        inc: &str,
        enabled: bool,
        events: &mut Vec<InputEventValue>,
    ) -> (Rect, bool) {
        let rect = row_base(ui, label, Some(hotkeys), accent, false);
        let input_rect = row_control_rect(rect);
        paint_input(ui, input_rect, false);
        let changed = if enabled {
            edit_usize(
                ui,
                Rect::from_min_size(input_rect.min, Vec2::new(INPUT_WIDTH, INPUT_HEIGHT)),
                ("usize", label),
                value,
                range,
                speed,
            )
        } else {
            paint_disabled_value(ui, input_rect, &value.to_string());
            false
        };
        let plus_rect = Rect::from_min_size(input_rect.min + Vec2::new(INPUT_WIDTH, 0.0), Vec2::new(SMALL_BUTTON_WIDTH, INPUT_HEIGHT));
        let minus_rect = Rect::from_min_size(
            input_rect.min + Vec2::new(INPUT_WIDTH + SMALL_BUTTON_WIDTH, 0.0),
            Vec2::new(SMALL_BUTTON_WIDTH, INPUT_HEIGHT),
        );
        let plus = small_button_enabled(ui, plus_rect, "+", (label, "inc"), enabled);
        let minus = small_button_enabled(ui, minus_rect, "-", (label, "dec"), enabled);
        self.synthetic.drive_button(ui, inc, &plus, enabled, events);
        self.synthetic.drive_button(ui, dec, &minus, enabled, events);
        (rect, changed)
    }

    #[allow(clippy::too_many_arguments)]
    fn pair_f32(
        &mut self,
        ui: &mut Ui,
        label: &str,
        accent: Accent,
        left: &mut f32,
        right: &mut f32,
        left_range: std::ops::RangeInclusive<f32>,
        right_range: std::ops::RangeInclusive<f32>,
        speed: f64,
        separator: &str,
        left_keys: [&str; 2],
        right_keys: [&str; 2],
        enabled: bool,
        events: &mut Vec<InputEventValue>,
    ) -> (Rect, bool) {
        let rect = row_base(ui, label, None, accent, false);
        let control = row_control_rect(rect);
        let half_width = 92.0;
        let separator_width = CONTROL_WIDTH - half_width * 2.0;
        let mut changed = false;
        let left_input = Rect::from_min_size(control.min, Vec2::new(42.0, INPUT_HEIGHT));
        let right_origin = control.min + Vec2::new(half_width + separator_width, 0.0);
        let right_input = Rect::from_min_size(right_origin, Vec2::new(42.0, INPUT_HEIGHT));
        paint_input(ui, Rect::from_min_size(control.min, Vec2::new(half_width, INPUT_HEIGHT)), false);
        paint_input(ui, Rect::from_min_size(right_origin, Vec2::new(half_width, INPUT_HEIGHT)), false);
        if enabled {
            changed |= edit_f32(ui, left_input, ("pair-left", label), left, left_range, speed);
            changed |= edit_f32(ui, right_input, ("pair-right", label), right, right_range, speed);
        } else {
            paint_disabled_value(ui, left_input, &format_number(*left));
            paint_disabled_value(ui, right_input, &format_number(*right));
        }
        ui.painter().text(
            Pos2::new(control.left() + half_width + separator_width * 0.5, control.center().y),
            Align2::CENTER_CENTER,
            separator,
            FontId::proportional(11.0),
            web_color(238, 238, 238),
        );
        let left_plus = small_button_enabled(
            ui,
            Rect::from_min_size(control.min + Vec2::new(42.0, 0.0), Vec2::new(25.0, INPUT_HEIGHT)),
            "+",
            (label, "left-inc"),
            enabled,
        );
        let left_minus = small_button_enabled(
            ui,
            Rect::from_min_size(control.min + Vec2::new(67.0, 0.0), Vec2::new(25.0, INPUT_HEIGHT)),
            "-",
            (label, "left-dec"),
            enabled,
        );
        let right_plus = small_button_enabled(
            ui,
            Rect::from_min_size(right_origin + Vec2::new(42.0, 0.0), Vec2::new(25.0, INPUT_HEIGHT)),
            "+",
            (label, "right-inc"),
            enabled,
        );
        let right_minus = small_button_enabled(
            ui,
            Rect::from_min_size(right_origin + Vec2::new(67.0, 0.0), Vec2::new(25.0, INPUT_HEIGHT)),
            "-",
            (label, "right-dec"),
            enabled,
        );
        self.synthetic.drive_button(ui, left_keys[1], &left_plus, enabled, events);
        self.synthetic.drive_button(ui, left_keys[0], &left_minus, enabled, events);
        self.synthetic.drive_button(ui, right_keys[1], &right_plus, enabled, events);
        self.synthetic.drive_button(ui, right_keys[0], &right_minus, enabled, events);
        (rect, changed)
    }

    fn rgb_matrix(&mut self, ui: &mut Ui, res: &Resources, sets: &mut Vec<ControllerSet>) {
        let rows = [
            (
                "red",
                [
                    res.controllers.rgb_red_r.value,
                    res.controllers.rgb_red_g.value,
                    res.controllers.rgb_red_b.value,
                ],
                ["front2back:rgb-red-r", "front2back:rgb-red-g", "front2back:rgb-red-b"],
            ),
            (
                "green",
                [
                    res.controllers.rgb_green_r.value,
                    res.controllers.rgb_green_g.value,
                    res.controllers.rgb_green_b.value,
                ],
                ["front2back:rgb-green-r", "front2back:rgb-green-g", "front2back:rgb-green-b"],
            ),
            (
                "blue",
                [
                    res.controllers.rgb_blue_r.value,
                    res.controllers.rgb_blue_g.value,
                    res.controllers.rgb_blue_b.value,
                ],
                ["front2back:rgb-blue-r", "front2back:rgb-blue-g", "front2back:rgb-blue-b"],
            ),
        ];
        let full = matrix_background(ui, Accent::Red, ["", "R", "G", "B"]);
        for (row_index, (label, values, tags)) in rows.into_iter().enumerate() {
            paint_matrix_label(ui, full, row_index, label);
            for (column, (mut value, tag)) in values.into_iter().zip(tags).enumerate() {
                let input = matrix_input_rect(full, row_index, column);
                paint_input(ui, input, false);
                // The web RGB matrix declares a step but no min/max, so its
                // keyboard and wheel stepping remain unbounded.
                if edit_f32(ui, input, ("rgb", tag), &mut value, f32::MIN..=f32::MAX, 0.01) {
                    set(sets, tag, value as f64);
                }
            }
        }
    }

    fn camera_buttons(&mut self, ui: &mut Ui, mode: CameraLockMode, events: &mut Vec<InputEventValue>) {
        let (full, _) = ui.allocate_exact_size(Vec2::new(PANEL_WIDTH, 115.0), Sense::hover());
        let rect = Rect::from_min_size(full.min + Vec2::new(CATEGORY_INSET, 0.0), Vec2::new(CATEGORY_WIDTH, 115.0));
        let left = Rect::from_min_size(rect.min, Vec2::new(201.0, 115.0));
        let right = Rect::from_min_size(rect.min + Vec2::new(201.0, 0.0), Vec2::new(195.0, 115.0));
        ui.painter().rect_filled(left, CornerRadius::ZERO, web_color(26, 26, 26));
        ui.painter().rect_filled(right, CornerRadius::ZERO, web_color(26, 26, 26));
        ui.painter()
            .rect_filled(Rect::from_min_size(left.min, Vec2::new(3.0, 115.0)), CornerRadius::ZERO, Accent::Red.color());
        ui.painter()
            .line_segment([right.left_top(), right.left_bottom()], Stroke::new(1.0, web_color(44, 44, 44)));
        ui.painter().text(
            Pos2::new(left.center().x, left.top() + 18.5),
            Align2::CENTER_CENTER,
            "translation",
            FontId::proportional(16.0),
            web_color(238, 238, 238),
        );
        ui.painter().text(
            Pos2::new(right.center().x, right.top() + 18.5),
            Align2::CENTER_CENTER,
            "rotation",
            FontId::proportional(16.0),
            web_color(238, 238, 238),
        );

        let tx = left.center().x - 44.5;
        self.held_rect_button(
            ui,
            Rect::from_min_size(Pos2::new(tx + 31.0, left.top() + 47.0), Vec2::new(31.0, 26.0)),
            "W",
            "w",
            events,
        );
        self.held_rect_button(
            ui,
            Rect::from_min_size(Pos2::new(tx, left.top() + 73.0), Vec2::new(31.0, 26.0)),
            "A",
            "a",
            events,
        );
        self.held_rect_button(
            ui,
            Rect::from_min_size(Pos2::new(tx + 31.0, left.top() + 73.0), Vec2::new(31.0, 26.0)),
            "S",
            "s",
            events,
        );
        self.held_rect_button(
            ui,
            Rect::from_min_size(Pos2::new(tx + 62.0, left.top() + 73.0), Vec2::new(31.0, 26.0)),
            "D",
            "d",
            events,
        );
        if matches!(mode, CameraLockMode::ThreeDimensional) {
            self.held_rect_button(
                ui,
                Rect::from_min_size(Pos2::new(tx + 97.0, left.top() + 47.0), Vec2::new(31.0, 26.0)),
                "Q",
                "q",
                events,
            );
            self.held_rect_button(
                ui,
                Rect::from_min_size(Pos2::new(tx + 97.0, left.top() + 73.0), Vec2::new(31.0, 26.0)),
                "E",
                "e",
                events,
            );
        }

        let rx = right.left() + 24.0;
        self.held_rect_button(
            ui,
            Rect::from_min_size(Pos2::new(rx + 31.0, right.top() + 47.0), Vec2::new(31.0, 26.0)),
            "↑",
            "arrowup",
            events,
        );
        self.held_rect_button(
            ui,
            Rect::from_min_size(Pos2::new(rx, right.top() + 73.0), Vec2::new(31.0, 26.0)),
            "←",
            "arrowleft",
            events,
        );
        self.held_rect_button(
            ui,
            Rect::from_min_size(Pos2::new(rx + 31.0, right.top() + 73.0), Vec2::new(31.0, 26.0)),
            "↓",
            "arrowdown",
            events,
        );
        self.held_rect_button(
            ui,
            Rect::from_min_size(Pos2::new(rx + 62.0, right.top() + 73.0), Vec2::new(31.0, 26.0)),
            "→",
            "arrowright",
            events,
        );
        self.held_rect_button(
            ui,
            Rect::from_min_size(Pos2::new(rx + 97.0, right.top() + 47.0), Vec2::new(31.0, 26.0)),
            "+",
            "+",
            events,
        );
        self.held_rect_button(
            ui,
            Rect::from_min_size(Pos2::new(rx + 97.0, right.top() + 73.0), Vec2::new(31.0, 26.0)),
            "-",
            "-",
            events,
        );
        ui.painter().text(
            Pos2::new(rx + 139.0, right.top() + 60.0),
            Align2::CENTER_CENTER,
            "⟳",
            FontId::proportional(11.0),
            web_color(88, 88, 88),
        );
        ui.painter().text(
            Pos2::new(rx + 139.0, right.top() + 86.0),
            Align2::CENTER_CENTER,
            "⟲",
            FontId::proportional(11.0),
            web_color(88, 88, 88),
        );
    }

    fn camera_matrix(&mut self, ui: &mut Ui, res: &Resources, events: &mut Vec<InputEventValue>) {
        let rows = [
            (
                "positon",
                [res.camera.position_eye.x, res.camera.position_eye.y, res.camera.position_eye.z],
                [CameraAxis::PosX, CameraAxis::PosY, CameraAxis::PosZ],
            ),
            (
                "direction",
                [res.camera.direction.x, res.camera.direction.y, res.camera.direction.z],
                [CameraAxis::DirectionX, CameraAxis::DirectionY, CameraAxis::DirectionZ],
            ),
            (
                "axis up",
                [res.camera.axis_up.x, res.camera.axis_up.y, res.camera.axis_up.z],
                [CameraAxis::AxisUpX, CameraAxis::AxisUpY, CameraAxis::AxisUpZ],
            ),
        ];
        let full = matrix_background(ui, Accent::Red, ["", "X", "Y", "Z"]);
        for (row_index, (label, values, axes)) in rows.into_iter().enumerate() {
            paint_matrix_label(ui, full, row_index, label);
            for (column, (mut value, axis)) in values.into_iter().zip(axes).enumerate() {
                let input = matrix_input_rect(full, row_index, column);
                paint_input(ui, input, false);
                if edit_f32_display(
                    ui,
                    input,
                    ("camera", row_index, column),
                    &mut value,
                    f32::MIN..=f32::MAX,
                    0.01,
                    NumberDisplay::Trimmed(2),
                ) {
                    events.push(InputEventValue::Camera(axis.change(value)));
                }
            }
        }
    }

    fn held_rect_button(&mut self, ui: &mut Ui, rect: Rect, label: &str, key: &str, events: &mut Vec<InputEventValue>) {
        let response = flat_button(ui, rect, label, ("camera", key));
        self.synthetic.drive_button(ui, key, &response, true, events);
    }
}

fn web_color(r: u8, g: u8, b: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(r, g, b, PANEL_ALPHA)
}

fn configure_web_style(context: &Context) {
    // HTML buttons do not stop being clickable merely because a render frame
    // stalls between pointer-down and pointer-up. Both host adapters queue
    // edges until the next frame, so egui's 0.8 s processing-time limit could
    // otherwise discard ordinary clicks under load. Distance still cancels a
    // click when the pointer actually moves away from the control.
    context.options_mut(|options| options.input_options.max_click_duration = f64::INFINITY);
    context.all_styles_mut(|style| {
        style.spacing.item_spacing = Vec2::ZERO;
        style.spacing.button_padding = Vec2::ZERO;
        style.spacing.interact_size = Vec2::new(0.0, INPUT_HEIGHT);
        style.spacing.extra_text_line_spacing = 0.0;
        style.spacing.scroll.floating = true;
        style.spacing.scroll.bar_width = 5.0;
        style.spacing.scroll.floating_width = 5.0;
        style.spacing.scroll.floating_allocated_width = 0.0;
        style.spacing.scroll.bar_inner_margin = 0.0;
        style.spacing.scroll.bar_outer_margin = 0.0;
        style.text_styles.insert(TextStyle::Body, FontId::proportional(11.0));
        style.text_styles.insert(TextStyle::Button, FontId::proportional(11.0));
        style.text_styles.insert(TextStyle::Small, FontId::proportional(8.0));
        style.drag_value_text_style = TextStyle::Body;
        style.visuals.override_text_color = Some(web_color(238, 238, 238));
        style.visuals.panel_fill = Color32::TRANSPARENT;
        style.visuals.window_fill = web_color(26, 26, 26);
        style.visuals.window_corner_radius = CornerRadius::ZERO;
        style.visuals.menu_corner_radius = CornerRadius::ZERO;
        style.visuals.button_frame = false;
        style.visuals.collapsing_header_frame = false;
        style.visuals.interact_cursor = Some(CursorIcon::PointingHand);
        style.visuals.extreme_bg_color = web_color(48, 48, 48);
        style.visuals.text_edit_bg_color = Some(web_color(48, 48, 48));
        style.visuals.selection.bg_fill = web_color(47, 161, 214);
        style.visuals.selection.stroke = Stroke::new(1.0, web_color(238, 238, 238));

        for visuals in [
            &mut style.visuals.widgets.noninteractive,
            &mut style.visuals.widgets.inactive,
            &mut style.visuals.widgets.hovered,
            &mut style.visuals.widgets.active,
            &mut style.visuals.widgets.open,
        ] {
            visuals.corner_radius = CornerRadius::ZERO;
            visuals.bg_stroke = Stroke::NONE;
            visuals.fg_stroke = Stroke::new(1.0, Accent::Blue.color());
        }
        style.visuals.widgets.noninteractive.weak_bg_fill = Color32::TRANSPARENT;
        style.visuals.widgets.noninteractive.bg_fill = Color32::TRANSPARENT;
        style.visuals.widgets.inactive.weak_bg_fill = web_color(48, 48, 48);
        style.visuals.widgets.inactive.bg_fill = web_color(48, 48, 48);
        style.visuals.widgets.hovered.weak_bg_fill = web_color(60, 60, 60);
        style.visuals.widgets.hovered.bg_fill = web_color(60, 60, 60);
        style.visuals.widgets.active.weak_bg_fill = web_color(70, 70, 70);
        style.visuals.widgets.active.bg_fill = web_color(70, 70, 70);
        style.visuals.widgets.open.weak_bg_fill = web_color(70, 70, 70);
        style.visuals.widgets.open.bg_fill = web_color(70, 70, 70);
    });
}

fn section_header(ui: &mut Ui, title: &str, open: &mut bool) -> bool {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(PANEL_WIDTH, SECTION_HEIGHT), Sense::click());
    let response = response.on_hover_cursor(CursorIcon::PointingHand);
    if response.clicked() {
        *open = !*open;
    }

    let fill = if response.hovered() { web_color(17, 17, 17) } else { web_color(9, 9, 9) };
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::ZERO, fill);
    painter.line_segment(
        [Pos2::new(rect.left(), rect.bottom() - 0.5), Pos2::new(rect.right(), rect.bottom() - 0.5)],
        Stroke::new(1.0, web_color(44, 44, 44)),
    );
    let triangle = if *open {
        vec![
            rect.min + Vec2::new(5.0, 11.0),
            rect.min + Vec2::new(11.0, 11.0),
            rect.min + Vec2::new(8.0, 16.0),
        ]
    } else {
        vec![
            rect.min + Vec2::new(6.0, 10.0),
            rect.min + Vec2::new(6.0, 16.0),
            rect.min + Vec2::new(11.0, 13.0),
        ]
    };
    painter.add(egui::Shape::convex_polygon(triangle, web_color(170, 170, 170), Stroke::NONE));
    painter.text(
        Pos2::new(rect.left() + 16.0, rect.center().y),
        Align2::LEFT_CENTER,
        title,
        FontId::proportional(11.0),
        web_color(238, 238, 238),
    );
    *open
}

fn control_row(ui: &mut Ui, label: &str) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(PANEL_WIDTH, TOGGLE_HEIGHT), Sense::click());
    let response = response.on_hover_cursor(CursorIcon::PointingHand);
    ui.painter().rect_filled(
        rect,
        CornerRadius::ZERO,
        if response.hovered() { web_color(17, 17, 17) } else { web_color(9, 9, 9) },
    );
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        label,
        FontId::proportional(11.0),
        web_color(238, 238, 238),
    );
    response
}

fn row_base(ui: &mut Ui, label: &str, hotkeys: Option<(&str, &str)>, accent: Accent, hovered: bool) -> Rect {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(PANEL_WIDTH, ROW_HEIGHT), Sense::hover());
    paint_row(ui, rect, label, hotkeys, accent, hovered);
    rect
}

fn paint_row(ui: &Ui, rect: Rect, label: &str, hotkeys: Option<(&str, &str)>, accent: Accent, hovered: bool) {
    let child = Rect::from_min_size(rect.min + Vec2::new(CATEGORY_INSET, 0.0), Vec2::new(CATEGORY_WIDTH, ROW_HEIGHT));
    let painter = ui.painter();
    painter.rect_filled(child, CornerRadius::ZERO, if hovered { web_color(60, 60, 60) } else { web_color(26, 26, 26) });
    painter.rect_filled(
        Rect::from_min_size(child.min, Vec2::new(3.0, ROW_HEIGHT - 1.0)),
        CornerRadius::ZERO,
        accent.color(),
    );
    painter.line_segment(
        [Pos2::new(child.left(), child.bottom() - 0.5), Pos2::new(child.right(), child.bottom() - 0.5)],
        Stroke::new(1.0, web_color(44, 44, 44)),
    );

    let label = label.to_lowercase();
    let font = FontId::proportional(11.0);
    let label_galley = painter.layout_no_wrap(label, font, web_color(238, 238, 238));
    let label_pos = Pos2::new(rect.left() + 13.0, rect.center().y - label_galley.size().y * 0.5);
    painter.galley(label_pos, label_galley.clone(), web_color(238, 238, 238));
    if let Some((increment, decrement)) = hotkeys {
        let hotkey_font = FontId::proportional(7.0);
        let increment_galley = painter.layout_no_wrap(format!("+: {increment}"), hotkey_font.clone(), web_color(88, 88, 88));
        let increment_pos = Pos2::new(label_pos.x + label_galley.size().x + 6.0, rect.center().y - 4.0);
        painter.galley(increment_pos, increment_galley.clone(), web_color(88, 88, 88));
        painter.text(
            Pos2::new(increment_pos.x + increment_galley.size().x + 6.0, rect.center().y - 4.0),
            Align2::LEFT_TOP,
            format!("-: {decrement}"),
            hotkey_font,
            web_color(88, 88, 88),
        );
    }
}

fn row_control_rect(row: Rect) -> Rect {
    Rect::from_min_size(row.min + Vec2::new(CONTROL_X, 3.0), Vec2::new(CONTROL_WIDTH, INPUT_HEIGHT))
}

fn paint_input(ui: &Ui, rect: Rect, hovered: bool) {
    ui.painter()
        .rect_filled(rect, CornerRadius::ZERO, if hovered { web_color(60, 60, 60) } else { web_color(48, 48, 48) });
}

fn small_button(ui: &mut Ui, rect: Rect, label: &str, id_salt: impl std::hash::Hash + std::fmt::Debug) -> Response {
    small_button_enabled(ui, rect, label, id_salt, true)
}

fn small_button_enabled(ui: &mut Ui, rect: Rect, label: &str, id_salt: impl std::hash::Hash + std::fmt::Debug, enabled: bool) -> Response {
    let response = ui.interact(
        rect,
        ui.make_persistent_id(Id::new(id_salt)),
        if enabled { Sense::click() } else { Sense::hover() },
    );
    let response = if enabled {
        response.on_hover_cursor(CursorIcon::PointingHand)
    } else {
        response
    };
    let fill = if enabled && response.is_pointer_button_down_on() {
        web_color(70, 70, 70)
    } else if enabled && response.hovered() {
        web_color(60, 60, 60)
    } else {
        web_color(48, 48, 48)
    };
    ui.painter().rect_filled(rect, CornerRadius::ZERO, fill);
    ui.painter()
        .text(rect.center(), Align2::CENTER_CENTER, label, FontId::proportional(12.0), Accent::Blue.color());
    response
}

fn edit_f32(
    ui: &mut Ui,
    rect: Rect,
    id_salt: impl std::hash::Hash + std::fmt::Debug,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
    speed: f64,
) -> bool {
    edit_f32_display(ui, rect, id_salt, value, range, speed, NumberDisplay::Trimmed(3))
}

#[derive(Clone, Copy)]
enum NumberDisplay {
    Trimmed(usize),
    Fixed(usize),
}

impl NumberDisplay {
    fn format(self, value: f64) -> String {
        match self {
            Self::Trimmed(decimals) => format_number_precision(value as f32, decimals),
            Self::Fixed(decimals) => format!("{value:.decimals$}"),
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn edit_f32_display(
    ui: &mut Ui,
    rect: Rect,
    id_salt: impl std::hash::Hash + std::fmt::Debug,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
    speed: f64,
    display: NumberDisplay,
) -> bool {
    let id = ui.make_persistent_id(Id::new(id_salt));
    let step_range = *range.start() as f64..=*range.end() as f64;
    let Some(text) = numeric_text_edit(ui, rect, id, display.format(*value as f64), speed, step_range) else {
        return false;
    };
    // JavaScript's unary `+` turns a cleared/invalid number input into zero,
    // then the wasm boundary casts the resulting f64 to f32. Do the same and
    // leave final range enforcement to the shared simulation controller.
    let parsed = browser_number_value(&text) as f32;
    if parsed == *value {
        return false;
    }
    *value = parsed;
    true
}

fn edit_usize(
    ui: &mut Ui,
    rect: Rect,
    id_salt: impl std::hash::Hash + std::fmt::Debug,
    value: &mut usize,
    range: std::ops::RangeInclusive<usize>,
    speed: f64,
) -> bool {
    let id = ui.make_persistent_id(Id::new(id_salt));
    let step_range = *range.start() as f64..=*range.end() as f64;
    let Some(text) = numeric_text_edit(ui, rect, id, value.to_string(), speed, step_range) else {
        return false;
    };
    // `JsEncodedValue::to_usize` uses Rust's saturating float-to-integer cast,
    // including truncation of fractions and mapping negative values to zero.
    let parsed = browser_number_value(&text) as usize;
    if parsed == *value {
        return false;
    }
    *value = parsed;
    true
}

/// Behaves like the web panel's `<input type="number">`: clicking edits text,
/// arrow keys apply the declared step, and the value is committed on Enter,
/// Tab, or blur. Pointer dragging never changes the value.
fn numeric_text_edit(ui: &mut Ui, rect: Rect, id: Id, current: String, step: f64, step_range: std::ops::RangeInclusive<f64>) -> Option<String> {
    let has_focus = ui.memory(|memory| memory.has_focus(id));
    let had_focus = ui.memory(|memory| memory.had_focus_last_frame(id));
    let mut text = if has_focus || had_focus {
        ui.data_mut(|data| data.get_temp::<String>(id)).unwrap_or(current)
    } else {
        current
    };

    if has_focus {
        let mut direction = ui.input_mut(|input| {
            if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
                1.0
            } else if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
                -1.0
            } else {
                0.0
            }
        });
        let wheel_steps = ui.input(|input| {
            if !input.pointer.hover_pos().is_some_and(|position| rect.contains(position)) {
                return 0.0;
            }
            input
                .raw
                .events
                .iter()
                .filter_map(|event| match event {
                    Event::MouseWheel { delta, .. } if delta.y != 0.0 => Some(delta.y.signum() as f64),
                    _ => None,
                })
                .sum::<f64>()
        });
        if wheel_steps != 0.0 {
            // A focused HTML number input owns wheel events under its pointer
            // instead of scrolling the containing panel.
            ui.input_mut(|input| input.smooth_scroll_delta = Vec2::ZERO);
            direction += wheel_steps;
        }
        if direction != 0.0 {
            let value = browser_number_value(&text);
            let stepped = (value + direction * step).clamp(*step_range.start(), *step_range.end());
            text = format_number_precision(stepped as f32, 6);
        }
    }

    let escape = ui.input(|input| input.key_pressed(egui::Key::Escape));
    let response = ui
        .scope(|ui| {
            ui.visuals_mut().override_text_color = Some(Accent::Blue.color());
            ui.spacing_mut().interact_size = rect.size();
            ui.put(
                rect,
                TextEdit::singleline(&mut text)
                    .id(id)
                    .frame(egui::Frame::NONE)
                    .clip_text(false)
                    .horizontal_align(Align::Center)
                    .vertical_align(Align::Center)
                    .margin(Vec2::ZERO)
                    .desired_width(rect.width())
                    .font(TextStyle::Body),
            )
        })
        .inner;

    // HTML number fields reject arbitrary text. Retain the complete syntax
    // needed for signed decimal and exponent input while editing.
    text.retain(|character| character.is_ascii_digit() || matches!(character, '.' | '+' | '-' | 'e' | 'E'));

    if response.has_focus() {
        ui.data_mut(|data| data.insert_temp(id, text));
        return None;
    }

    ui.data_mut(|data| data.remove::<String>(id));
    if response.lost_focus() && !escape {
        Some(text)
    } else {
        None
    }
}

fn browser_number_value(text: &str) -> f64 {
    text.parse::<f64>().unwrap_or(0.0)
}

fn paint_disabled_value(ui: &Ui, rect: Rect, value: &str) {
    ui.painter().text(
        Rect::from_min_size(rect.min, Vec2::new(INPUT_WIDTH.min(rect.width()), rect.height())).center(),
        Align2::CENTER_CENTER,
        value,
        FontId::proportional(11.0),
        Accent::Blue.color(),
    );
}

fn format_number(value: f32) -> String {
    format_number_precision(value, 3)
}

fn format_number_precision(value: f32, decimals: usize) -> String {
    if value.fract().abs() < f32::EPSILON {
        format!("{value:.0}")
    } else {
        let formatted = format!("{value:.decimals$}");
        formatted.trim_end_matches('0').trim_end_matches('.').to_owned()
    }
}

fn checkbox_row(ui: &mut Ui, label: &str, accent: Accent, checked: &mut bool, enabled: bool) -> (Rect, bool) {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(PANEL_WIDTH, ROW_HEIGHT), if enabled { Sense::click() } else { Sense::hover() });
    let response = if enabled {
        response.on_hover_cursor(CursorIcon::PointingHand)
    } else {
        response
    };
    let changed = enabled && response.clicked();
    if changed {
        *checked = !*checked;
    }
    paint_row(ui, rect, label, None, accent, enabled && response.hovered());
    let control = row_control_rect(rect);
    let box_rect = Rect::from_min_size(control.min + Vec2::new(4.0, 4.0), Vec2::splat(13.0));
    ui.painter().rect_filled(
        box_rect,
        CornerRadius::from(2),
        if *checked { web_color(0, 117, 255) } else { web_color(118, 118, 118) },
    );
    if *checked {
        ui.painter().line_segment(
            [box_rect.left_top() + Vec2::new(2.5, 6.5), box_rect.left_top() + Vec2::new(5.5, 9.5)],
            Stroke::new(1.5, web_color(255, 255, 255)),
        );
        ui.painter().line_segment(
            [box_rect.left_top() + Vec2::new(5.0, 9.5), box_rect.left_top() + Vec2::new(10.5, 3.0)],
            Stroke::new(1.5, web_color(255, 255, 255)),
        );
    }
    (rect, changed)
}

#[derive(Clone)]
struct PendingColor {
    value: [u8; 3],
    was_open: bool,
}

fn color_row(ui: &mut Ui, label: &str, accent: Accent, color: &mut [u8; 3]) -> bool {
    let rect = row_base(ui, label, None, accent, false);
    let control = row_control_rect(rect);
    let mut child = ui.new_child(egui::UiBuilder::new().id_salt(("color", label)).max_rect(control));
    child.spacing_mut().interact_size = control.size();
    let pending_id = child.make_persistent_id("pending-color");
    let popup_id = child.auto_id_with("popup");
    let pending = child.data_mut(|data| data.get_temp::<PendingColor>(pending_id));
    let mut editing = pending.as_ref().map_or(*color, |pending| pending.value);
    let escape_pressed = child.input(|input| input.key_pressed(egui::Key::Escape));
    let response = child.color_edit_button_srgb(&mut editing).on_hover_cursor(CursorIcon::PointingHand);
    let open = Popup::is_id_open(child.ctx(), popup_id);
    paint_input(ui, control, response.hovered());
    let swatch = Rect::from_min_max(control.min + Vec2::new(5.0, 6.0), control.max - Vec2::new(5.0, 6.0));
    ui.painter()
        .rect_filled(swatch, CornerRadius::ZERO, web_color(editing[0], editing[1], editing[2]));

    let commit = pending.is_some_and(|pending| pending.was_open) && !open;
    if open {
        child.data_mut(|data| {
            data.insert_temp(
                pending_id,
                PendingColor {
                    value: editing,
                    was_open: true,
                },
            );
        });
        false
    } else {
        child.data_mut(|data| data.remove::<PendingColor>(pending_id));
        // A browser/OS color dialog treats Escape as Cancel. egui's popup
        // closes on Escape too, but would otherwise leave the preview value
        // behind and accidentally commit it here.
        if commit && !escape_pressed && editing != *color {
            *color = editing;
            true
        } else {
            false
        }
    }
}

fn action_row(ui: &mut Ui, label: &str, accent: Accent) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(PANEL_WIDTH, ROW_HEIGHT), Sense::click());
    let response = response.on_hover_cursor(CursorIcon::PointingHand);
    paint_row(ui, rect, label, None, accent, response.hovered());
    response
}

fn static_selector_row(ui: &mut Ui, label: &str, accent: Accent, value: &str) {
    let rect = row_base(ui, label, None, accent, false);
    let control = row_control_rect(rect);
    paint_input(ui, control, false);
    ui.painter().text(
        Rect::from_min_size(control.min, Vec2::new(INPUT_WIDTH, INPUT_HEIGHT)).center(),
        Align2::CENTER_CENTER,
        value,
        FontId::proportional(12.0),
        Accent::Blue.color(),
    );
    small_button_enabled(
        ui,
        Rect::from_min_size(control.min + Vec2::new(INPUT_WIDTH, 0.0), Vec2::new(SMALL_BUTTON_WIDTH, INPUT_HEIGHT)),
        "+",
        ("static-selector", label, "inc"),
        false,
    );
    small_button_enabled(
        ui,
        Rect::from_min_size(
            control.min + Vec2::new(INPUT_WIDTH + SMALL_BUTTON_WIDTH, 0.0),
            Vec2::new(SMALL_BUTTON_WIDTH, INPUT_HEIGHT),
        ),
        "-",
        ("static-selector", label, "dec"),
        false,
    );
}

fn exit_row(ui: &mut Ui, label: &str) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(PANEL_WIDTH, ROW_HEIGHT), Sense::click());
    let response = response.on_hover_cursor(CursorIcon::PointingHand);
    ui.painter().rect_filled(
        rect,
        CornerRadius::ZERO,
        if response.hovered() { web_color(60, 60, 60) } else { web_color(26, 26, 26) },
    );
    ui.painter().rect_filled(
        Rect::from_min_size(rect.min, Vec2::new(3.0, ROW_HEIGHT - 1.0)),
        CornerRadius::ZERO,
        Accent::Grey.color(),
    );
    ui.painter().line_segment(
        [Pos2::new(rect.left(), rect.bottom() - 0.5), Pos2::new(rect.right(), rect.bottom() - 0.5)],
        Stroke::new(1.0, web_color(44, 44, 44)),
    );
    ui.painter().text(
        Pos2::new(rect.left() + 41.0, rect.center().y),
        Align2::LEFT_CENTER,
        label,
        FontId::proportional(16.5),
        web_color(126, 126, 126),
    );
    response
}

fn matrix_background(ui: &mut Ui, accent: Accent, headings: [&str; 4]) -> Rect {
    let (full, _) = ui.allocate_exact_size(Vec2::new(PANEL_WIDTH, ROW_HEIGHT * 4.0), Sense::hover());
    let grid = Rect::from_min_size(full.min + Vec2::new(CATEGORY_INSET, 0.0), Vec2::new(CATEGORY_WIDTH, ROW_HEIGHT * 4.0));
    let painter = ui.painter();
    painter.rect_filled(grid, CornerRadius::ZERO, web_color(26, 26, 26));
    painter.rect_filled(
        Rect::from_min_size(grid.min, Vec2::new(3.0, grid.height() - 1.0)),
        CornerRadius::ZERO,
        accent.color(),
    );
    for row in 1..=4 {
        let y = grid.top() + row as f32 * ROW_HEIGHT - 0.5;
        painter.line_segment([Pos2::new(grid.left(), y), Pos2::new(grid.right(), y)], Stroke::new(1.0, web_color(44, 44, 44)));
    }
    for (column, heading) in headings.into_iter().enumerate().skip(1) {
        painter.text(
            Pos2::new(grid.left() + 121.0 + (column - 1) as f32 * 110.0, grid.top() + ROW_HEIGHT * 0.5),
            Align2::CENTER_CENTER,
            heading,
            FontId::proportional(11.0),
            web_color(238, 238, 238),
        );
    }
    full
}

fn paint_matrix_label(ui: &Ui, full: Rect, row: usize, label: &str) {
    ui.painter().text(
        Pos2::new(full.left() + 13.0, full.top() + ROW_HEIGHT * (row as f32 + 1.5)),
        Align2::LEFT_CENTER,
        label,
        FontId::proportional(11.0),
        web_color(238, 238, 238),
    );
}

fn matrix_input_rect(full: Rect, row: usize, column: usize) -> Rect {
    Rect::from_min_size(
        Pos2::new(
            full.left() + CATEGORY_INSET + 71.0 + column as f32 * 110.0,
            full.top() + ROW_HEIGHT * (row as f32 + 1.0) + 3.0,
        ),
        Vec2::new(100.0, INPUT_HEIGHT),
    )
}

fn flat_button(ui: &mut Ui, rect: Rect, label: &str, id_salt: impl std::hash::Hash + std::fmt::Debug) -> Response {
    let response = ui.interact(rect, ui.make_persistent_id(Id::new(id_salt)), Sense::click());
    let response = response.on_hover_cursor(CursorIcon::PointingHand);
    let fill = if response.is_pointer_button_down_on() {
        web_color(70, 70, 70)
    } else if response.hovered() {
        web_color(60, 60, 60)
    } else {
        web_color(48, 48, 48)
    };
    ui.painter().rect_filled(rect, CornerRadius::ZERO, fill);
    match label {
        "↑" => paint_arrow(ui, rect.center(), Vec2::new(0.0, -1.0)),
        "↓" => paint_arrow(ui, rect.center(), Vec2::new(0.0, 1.0)),
        "←" => paint_arrow(ui, rect.center(), Vec2::new(-1.0, 0.0)),
        "→" => paint_arrow(ui, rect.center(), Vec2::new(1.0, 0.0)),
        _ => {
            ui.painter()
                .text(rect.center(), Align2::CENTER_CENTER, label, FontId::proportional(11.0), Accent::Blue.color());
        }
    }
    response
}

fn paint_arrow(ui: &Ui, center: Pos2, direction: Vec2) {
    let stroke = Stroke::new(1.0, Accent::Blue.color());
    let tip = center + direction * 4.0;
    let tail = center - direction * 3.0;
    let perpendicular = Vec2::new(-direction.y, direction.x);
    ui.painter().line_segment([tail, tip], stroke);
    ui.painter().line_segment([tip, tip - direction * 2.5 + perpendicular * 2.0], stroke);
    ui.painter().line_segment([tip, tip - direction * 2.5 - perpendicular * 2.0], stroke);
}

pub fn route_controller_value(res: &mut Resources, event_tag: &'static str, value: PanelEncodedValue) -> AppResult<()> {
    let index = match res.controller_events.get(event_tag) {
        Some((KeyEventKind::Set, index)) => *index,
        Some(_) => return Err(AppError::new(format!("{event_tag} is not a set event"))),
        None => return Err(AppError::new(format!("unknown controller event {event_tag}"))),
    };
    res.controllers.get_ui_controllers_mut()[index].read_event(Box::new(value))
}

pub fn select_preset(res: &mut Resources, preset: FilterPresetOptions) -> AppResult<()> {
    route_controller_value(res, "front2back:filter-presets-selected", PanelEncodedValue::Text(preset.to_string()))
}

fn push_input_events(input: &mut Input, events: Vec<InputEventValue>) {
    for event in events {
        input.push_event(event);
    }
}

fn set(sets: &mut Vec<ControllerSet>, event_tag: &'static str, value: f64) {
    sets.push(ControllerSet {
        event_tag,
        value: PanelEncodedValue::Number(value),
    });
}

#[derive(Clone, Copy)]
enum CameraAxis {
    PosX,
    PosY,
    PosZ,
    DirectionX,
    DirectionY,
    DirectionZ,
    AxisUpX,
    AxisUpY,
    AxisUpZ,
}

impl CameraAxis {
    fn change(self, value: f32) -> CameraChange {
        match self {
            Self::PosX => CameraChange::PosX(value),
            Self::PosY => CameraChange::PosY(value),
            Self::PosZ => CameraChange::PosZ(value),
            Self::DirectionX => CameraChange::DirectionX(value),
            Self::DirectionY => CameraChange::DirectionY(value),
            Self::DirectionZ => CameraChange::DirectionZ(value),
            Self::AxisUpX => CameraChange::AxisUpX(value),
            Self::AxisUpY => CameraChange::AxisUpY(value),
            Self::AxisUpZ => CameraChange::AxisUpZ(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encoded_values_convert() {
        let numeric = PanelEncodedValue::Number(42.75);
        assert_eq!(numeric.to_f64().unwrap(), 42.75);
        assert_eq!(numeric.to_i32().unwrap(), 42);
        assert_eq!(numeric.to_string().unwrap(), "42.75");
        let text = PanelEncodedValue::Text("17".into());
        assert_eq!(text.to_usize().unwrap(), 17);
        assert!(PanelEncodedValue::Text("nope".into()).to_f32().is_err());
    }

    #[test]
    fn controller_values_are_routed_by_tag() {
        let mut resources = Resources::default();
        assert!(route_controller_value(&mut resources, "front2back:blur-level", PanelEncodedValue::Number(3.0)).is_ok());
        assert!(route_controller_value(&mut resources, "front2back:missing", PanelEncodedValue::Number(3.0)).is_err());
    }

    #[test]
    fn every_settable_controller_accepts_the_shared_panel_route() {
        let mut resources = Resources::default();
        let set_tags: Vec<_> = resources
            .controller_events
            .iter()
            .filter_map(|(tag, kind)| matches!(kind, (KeyEventKind::Set, _)).then_some(*tag))
            .collect();

        assert_eq!(set_tags.len(), 24, "controller inventory changed; audit the shared panel");
        for tag in set_tags {
            let value = if tag == "front2back:filter-presets-selected" {
                PanelEncodedValue::Text(FilterPresetOptions::Custom.to_string())
            } else {
                PanelEncodedValue::Number(0.5)
            };
            route_controller_value(&mut resources, tag, value).unwrap_or_else(|error| panic!("shared panel cannot route {tag}: {error}"));
        }
    }

    #[test]
    fn routed_rgb_values_are_applied() {
        use core::simulation_context::make_fake_simulation_context;
        use core::ui_controller::UiController;

        let mut resources = Resources::default();
        route_controller_value(&mut resources, "front2back:rgb-red-g", PanelEncodedValue::Number(0.75)).unwrap();
        let changed = resources.controllers.rgb_red_g.update(&resources.main, &make_fake_simulation_context());
        assert!(changed);
        assert_eq!(resources.controllers.rgb_red_g.value, 0.75);
    }

    #[test]
    fn preset_events_use_shared_controller_route() {
        let mut resources = Resources::default();
        for preset in FilterPresetOptions::ALL {
            select_preset(&mut resources, preset).unwrap();
            assert_eq!(resources.controllers.preset_kind.value, preset);
        }
    }

    #[test]
    fn flight_preset_click_across_frames_is_applied_by_the_real_panel() {
        let mut panel = SimPanel::new();
        let mut resources = Resources::default();
        resources.video.viewport_size.width = 1_024;
        resources.video.viewport_size.height = 640;
        let mut input = Input::default();
        let sink = shared_panel_events();
        let screen_rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(1_024.0, 640.0));
        let raw_input = |time, events| egui::RawInput {
            screen_rect: Some(screen_rect),
            time: Some(time),
            events,
            ..Default::default()
        };

        for time in [0.0, 0.02] {
            panel
                .run(raw_input(time, Vec::new()), &mut resources, &mut input, &sink)
                .unwrap()
                .drop_without_applying_deltas();
        }
        let pointer = Pos2::new(125.0, 153.0);
        panel
            .run(
                raw_input(
                    1.0,
                    vec![
                        Event::PointerMoved(pointer),
                        Event::PointerButton {
                            pos: pointer,
                            button: egui::PointerButton::Primary,
                            pressed: true,
                            modifiers: egui::Modifiers::NONE,
                        },
                    ],
                ),
                &mut resources,
                &mut input,
                &sink,
            )
            .unwrap()
            .drop_without_applying_deltas();
        for time in [2.0, 3.0, 4.0] {
            panel
                .run(raw_input(time, Vec::new()), &mut resources, &mut input, &sink)
                .unwrap()
                .drop_without_applying_deltas();
        }
        panel
            .run(
                raw_input(
                    6.0,
                    vec![Event::PointerButton {
                        pos: pointer,
                        button: egui::PointerButton::Primary,
                        pressed: false,
                        modifiers: egui::Modifiers::NONE,
                    }],
                ),
                &mut resources,
                &mut input,
                &sink,
            )
            .unwrap()
            .drop_without_applying_deltas();

        assert_eq!(resources.controllers.preset_kind.value, FilterPresetOptions::DemoFlight1);
    }

    #[test]
    fn pulse_releases_on_the_next_frame() {
        let mut keys = SyntheticKeys::default();
        let mut events = Vec::new();
        keys.pulse("capture-framebuffer", &mut events);
        keys.pulse("capture-framebuffer", &mut events);
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], InputEventValue::Keyboard { pressed: Pressed::Yes, key } if key == "capture-framebuffer"));
        events.clear();
        keys.begin_frame(&mut events);
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], InputEventValue::Keyboard { pressed: Pressed::No, key } if key == "capture-framebuffer"));
    }

    #[test]
    fn focused_custom_button_holds_and_releases_with_enter() {
        let context = Context::default();
        let screen_rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(200.0, 100.0));
        let button_rect = Rect::from_min_size(Pos2::new(10.0, 10.0), Vec2::new(80.0, 24.0));
        let button_id = Id::new("keyboard-button");

        context
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(screen_rect),
                    ..Default::default()
                },
                |ui| {
                    ui.interact(button_rect, button_id, Sense::click()).request_focus();
                },
            )
            .drop_without_applying_deltas();

        let mut keys = SyntheticKeys::default();
        let mut events = Vec::new();
        context
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(screen_rect),
                    events: vec![egui::Event::Key {
                        key: egui::Key::Enter,
                        physical_key: Some(egui::Key::Enter),
                        pressed: true,
                        repeat: false,
                        modifiers: egui::Modifiers::NONE,
                    }],
                    ..Default::default()
                },
                |ui| {
                    let response = ui.interact(button_rect, button_id, Sense::click());
                    assert!(response.has_focus());
                    keys.drive_button(ui, "button-action", &response, true, &mut events);
                },
            )
            .drop_without_applying_deltas();

        assert!(matches!(&events[0], InputEventValue::Keyboard { pressed: Pressed::Yes, key } if key == "button-action"));

        events.clear();
        context
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(screen_rect),
                    events: vec![egui::Event::Key {
                        key: egui::Key::Enter,
                        physical_key: Some(egui::Key::Enter),
                        pressed: false,
                        repeat: false,
                        modifiers: egui::Modifiers::NONE,
                    }],
                    ..Default::default()
                },
                |ui| {
                    let response = ui.interact(button_rect, button_id, Sense::click());
                    assert!(response.has_focus());
                    keys.drive_button(ui, "button-action", &response, true, &mut events);
                },
            )
            .drop_without_applying_deltas();
        assert!(matches!(&events[0], InputEventValue::Keyboard { pressed: Pressed::No, key } if key == "button-action"));
    }

    #[test]
    fn fast_pointer_click_survives_when_both_edges_arrive_in_one_frame() {
        let context = Context::default();
        let screen_rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(200.0, 100.0));
        let button_rect = Rect::from_min_size(Pos2::new(10.0, 10.0), Vec2::new(80.0, 24.0));
        let pointer = button_rect.center();
        let mut keys = SyntheticKeys::default();
        let mut events = Vec::new();

        // egui hit-tests pointer events against the widget geometry retained
        // from the preceding frame, just as it does in the running panel.
        context
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(screen_rect),
                    ..Default::default()
                },
                |ui| {
                    ui.interact(button_rect, Id::new("fast-pointer-button"), Sense::click());
                },
            )
            .drop_without_applying_deltas();

        keys.begin_frame(&mut events);
        context
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(screen_rect),
                    events: vec![
                        egui::Event::PointerMoved(pointer),
                        egui::Event::PointerButton {
                            pos: pointer,
                            button: egui::PointerButton::Primary,
                            pressed: true,
                            modifiers: egui::Modifiers::NONE,
                        },
                        egui::Event::PointerButton {
                            pos: pointer,
                            button: egui::PointerButton::Primary,
                            pressed: false,
                            modifiers: egui::Modifiers::NONE,
                        },
                    ],
                    ..Default::default()
                },
                |ui| {
                    let response = ui.interact(button_rect, Id::new("fast-pointer-button"), Sense::click());
                    assert!(response.clicked());
                    assert!(!response.is_pointer_button_down_on());
                    keys.drive_button(ui, "button-action", &response, true, &mut events);
                },
            )
            .drop_without_applying_deltas();
        keys.end_frame(&mut events);

        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], InputEventValue::Keyboard { pressed: Pressed::Yes, key } if key == "button-action"));

        events.clear();
        keys.begin_frame(&mut events);
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], InputEventValue::Keyboard { pressed: Pressed::No, key } if key == "button-action"));
    }

    #[test]
    fn fast_keyboard_activation_survives_when_both_edges_arrive_in_one_frame() {
        let context = Context::default();
        let screen_rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(200.0, 100.0));
        let button_rect = Rect::from_min_size(Pos2::new(10.0, 10.0), Vec2::new(80.0, 24.0));
        let button_id = Id::new("fast-keyboard-button");
        let mut keys = SyntheticKeys::default();
        let mut events = Vec::new();

        context
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(screen_rect),
                    ..Default::default()
                },
                |ui| {
                    ui.interact(button_rect, button_id, Sense::click()).request_focus();
                },
            )
            .drop_without_applying_deltas();

        let key_event = |pressed| egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: Some(egui::Key::Enter),
            pressed,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        };
        context
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(screen_rect),
                    events: vec![key_event(true), key_event(false)],
                    ..Default::default()
                },
                |ui| {
                    let response = ui.interact(button_rect, button_id, Sense::click());
                    assert!(response.clicked());
                    assert!(!ui.input(|input| input.key_down(egui::Key::Enter)));
                    keys.drive_button(ui, "button-action", &response, true, &mut events);
                },
            )
            .drop_without_applying_deltas();

        assert!(matches!(&events[..], [InputEventValue::Keyboard { pressed: Pressed::Yes, key }] if key == "button-action"));
        events.clear();
        keys.begin_frame(&mut events);
        assert!(matches!(&events[..], [InputEventValue::Keyboard { pressed: Pressed::No, key }] if key == "button-action"));
    }

    #[test]
    fn releasing_an_observed_pointer_hold_does_not_retrigger_its_click() {
        let context = Context::default();
        let screen_rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(200.0, 100.0));
        let button_rect = Rect::from_min_size(Pos2::new(10.0, 10.0), Vec2::new(80.0, 24.0));
        let pointer = button_rect.center();
        let button_id = Id::new("held-pointer-button");
        let mut keys = SyntheticKeys::default();
        let mut events = Vec::new();

        context
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(screen_rect),
                    ..Default::default()
                },
                |ui| {
                    ui.interact(button_rect, button_id, Sense::click());
                },
            )
            .drop_without_applying_deltas();

        keys.begin_frame(&mut events);
        context
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(screen_rect),
                    events: vec![
                        egui::Event::PointerMoved(pointer),
                        egui::Event::PointerButton {
                            pos: pointer,
                            button: egui::PointerButton::Primary,
                            pressed: true,
                            modifiers: egui::Modifiers::NONE,
                        },
                    ],
                    ..Default::default()
                },
                |ui| {
                    let response = ui.interact(button_rect, button_id, Sense::click());
                    assert!(response.is_pointer_button_down_on());
                    keys.drive_button(ui, "button-action", &response, true, &mut events);
                },
            )
            .drop_without_applying_deltas();
        keys.end_frame(&mut events);
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], InputEventValue::Keyboard { pressed: Pressed::Yes, key } if key == "button-action"));

        events.clear();
        keys.begin_frame(&mut events);
        context
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(screen_rect),
                    events: vec![egui::Event::PointerButton {
                        pos: pointer,
                        button: egui::PointerButton::Primary,
                        pressed: false,
                        modifiers: egui::Modifiers::NONE,
                    }],
                    ..Default::default()
                },
                |ui| {
                    let response = ui.interact(button_rect, button_id, Sense::click());
                    assert!(response.clicked());
                    keys.drive_button(ui, "button-action", &response, true, &mut events);
                },
            )
            .drop_without_applying_deltas();
        keys.end_frame(&mut events);

        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], InputEventValue::Keyboard { pressed: Pressed::No, key } if key == "button-action"));
    }

    #[test]
    fn native_number_commit_uses_browser_conversion() {
        assert_eq!(browser_number_value("12.75"), 12.75);
        assert_eq!(browser_number_value(""), 0.0);
        assert_eq!(browser_number_value("not-a-number"), 0.0);
        assert_eq!(browser_number_value("-3.5") as usize, 0);
        assert_eq!(browser_number_value("3.9") as usize, 3);
    }

    #[test]
    fn release_all_cleans_up_held_buttons() {
        let mut keys = SyntheticKeys::default();
        let mut events = Vec::new();
        keys.set_held("w", true, &mut events);
        events.clear();
        keys.release_all(&mut events);
        assert!(matches!(&events[0], InputEventValue::Keyboard { pressed: Pressed::No, key } if key == "w"));
        assert!(keys.down.is_empty());
    }

    #[test]
    fn sink_drain_is_atomic() {
        let mut sink = PanelEventSink::default();
        sink.set_fps(59.5);
        sink.push_message("hello");
        sink.request_toggle();
        assert_eq!(
            sink.drain(),
            DrainedPanelEvents {
                fps: Some(59.5),
                messages: vec!["hello".into()],
                toggle_requests: 1
            }
        );
        assert_eq!(sink.drain(), DrainedPanelEvents::default());
    }

    #[test]
    fn default_panel_produces_paint_shapes_at_the_web_geometry() {
        let mut panel = SimPanel::new();
        let mut resources = Resources::default();
        resources.video.viewport_size.width = 1_024;
        resources.video.viewport_size.height = 640;
        let mut input = Input::default();
        let events = shared_panel_events();
        let raw_input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(1_024.0, 640.0))),
            ..Default::default()
        };

        let output = panel.run(raw_input, &mut resources, &mut input, &events).unwrap();

        let rect = panel.panel_rect().expect("panel rect");
        let has_shapes = !output.shapes.is_empty();
        output.drop_without_applying_deltas();

        assert!(has_shapes);
        assert_eq!(rect.left(), PANEL_X);
        assert_eq!(rect.width(), PANEL_WIDTH);
        assert_eq!(rect.height(), 507.0);
    }

    #[test]
    fn ended_session_uses_the_shared_terminal_surface() {
        let mut panel = SimPanel::new();
        let mut resources = Resources::default();
        resources.quit = true;
        let mut input = Input::default();
        let events = shared_panel_events();
        let output = panel
            .run(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(1_024.0, 640.0))),
                    ..Default::default()
                },
                &mut resources,
                &mut input,
                &events,
            )
            .unwrap();

        assert!(panel.panel_rect().is_none());
        assert!(!output.shapes.is_empty());
        output.drop_without_applying_deltas();
    }
}
