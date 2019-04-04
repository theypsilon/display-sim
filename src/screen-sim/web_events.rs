use crate::app_events::AppEventDispatcher;
use crate::dispatch_event::{dispatch_event, dispatch_event_with};
use crate::simulation_state::Resources;
use crate::wasm_error::{WasmError, WasmResult};
use js_sys::{Array, Float32Array};
use std::cell::RefCell;

#[derive(Default)]
pub struct WebEventDispatcher {
    error: RefCell<Option<WasmError>>,
}

impl AppEventDispatcher for WebEventDispatcher {
    fn dispatch_camera_update(&self, position: &glm::Vec3, direction: &glm::Vec3, axis_up: &glm::Vec3) {
        let values_array = Float32Array::new(&wasm_bindgen::JsValue::from(9));
        values_array.fill(position.x, 0, 1);
        values_array.fill(position.y, 1, 2);
        values_array.fill(position.z, 2, 3);
        values_array.fill(direction.x, 3, 4);
        values_array.fill(direction.y, 4, 5);
        values_array.fill(direction.z, 5, 6);
        values_array.fill(axis_up.x, 6, 7);
        values_array.fill(axis_up.y, 7, 8);
        values_array.fill(axis_up.z, 8, 9);
        self.catch_error(dispatch_event_with("app-event.camera_update", &values_array.into()));
    }

    fn dispatch_change_pixel_horizontal_gap(&self, size: f32) {
        self.catch_error(dispatch_event_with("app-event.change_pixel_horizontal_gap", &format!("{:.03}", size).into()));
    }

    fn dispatch_change_pixel_vertical_gap(&self, size: f32) {
        self.catch_error(dispatch_event_with("app-event.change_pixel_vertical_gap", &format!("{:.03}", size).into()));
    }

    fn dispatch_change_pixel_width(&self, size: f32) {
        self.catch_error(dispatch_event_with("app-event.change_pixel_width", &format!("{:.03}", size).into()));
    }

    fn dispatch_change_pixel_spread(&self, size: f32) {
        self.catch_error(dispatch_event_with("app-event.change_pixel_spread", &format!("{:.03}", size).into()));
    }

    fn dispatch_change_pixel_brightness(&self, res: &Resources) {
        self.catch_error(dispatch_event_with(
            "app-event.change_pixel_brightness",
            &format!("{:.02}", res.filters.extra_bright).into(),
        ));
    }

    fn dispatch_change_pixel_contrast(&self, res: &Resources) {
        self.catch_error(dispatch_event_with(
            "app-event.change_pixel_contrast",
            &format!("{:.03}", res.filters.extra_contrast).into(),
        ));
    }

    fn dispatch_change_light_color(&self, res: &Resources) {
        self.dispatch_change_color("app-event.change_light_color", res.filters.light_color);
    }

    fn dispatch_change_brightness_color(&self, res: &Resources) {
        self.dispatch_change_color("app-event.change_brightness_color", res.filters.brightness_color);
    }

    fn dispatch_change_camera_zoom(&self, zoom: f32) {
        self.catch_error(dispatch_event_with("app-event.change_camera_zoom", &format!("{:.02}", zoom).into()));
    }

    fn dispatch_change_blur_level(&self, res: &Resources) {
        self.catch_error(dispatch_event_with("app-event.change_blur_level", &(res.filters.blur_passes as i32).into()));
    }

    fn dispatch_change_lines_per_pixel(&self, res: &Resources) {
        self.catch_error(dispatch_event_with(
            "app-event.change_lines_per_pixel",
            &(res.filters.lines_per_pixel as i32).into(),
        ));
    }

    fn dispatch_color_representation(&self, res: &Resources) {
        self.catch_error(dispatch_event_with(
            "app-event.color_representation",
            &(res.filters.color_channels.to_string()).into(),
        ));
    }

    fn dispatch_pixel_geometry(&self, res: &Resources) {
        self.catch_error(dispatch_event_with(
            "app-event.pixel_geometry",
            &(res.filters.pixels_geometry_kind.to_string()).into(),
        ));
    }

    fn dispatch_pixel_shadow_shape(&self, res: &Resources) {
        self.catch_error(dispatch_event_with(
            "app-event.pixel_shadow_shape",
            &(res.filters.pixel_shadow_shape_kind.to_string()).into(),
        ));
    }

    fn dispatch_pixel_shadow_height(&self, res: &Resources) {
        self.catch_error(dispatch_event_with(
            "app-event.pixel_shadow_height",
            &(res.filters.pixel_shadow_height_factor).into(),
        ));
    }

    fn dispatch_screen_layering_type(&self, res: &Resources) {
        self.catch_error(dispatch_event_with(
            "app-event.screen_layering_type",
            &(res.filters.layering_kind.to_string()).into(),
        ));
    }

    fn dispatch_screen_curvature(&self, res: &Resources) {
        self.catch_error(dispatch_event_with(
            "app-event.screen_curvature",
            &(res.filters.screen_curvature_kind.to_string()).into(),
        ));
    }

    fn dispatch_internal_resolution(&self, res: &Resources) {
        self.catch_error(dispatch_event_with(
            "app-event.internal_resolution",
            &(res.filters.internal_resolution.to_label()).into(),
        ));
    }

    fn dispatch_texture_interpolation(&self, res: &Resources) {
        self.catch_error(dispatch_event_with(
            "app-event.texture_interpolation",
            &(res.filters.texture_interpolation.to_string()).into(),
        ));
    }

    fn dispatch_change_pixel_speed(&self, speed: f32) {
        self.dispatch_internal_speed("app-event.change_pixel_speed", speed);
    }

    fn dispatch_change_turning_speed(&self, speed: f32) {
        self.dispatch_internal_speed("app-event.change_turning_speed", speed);
    }

    fn dispatch_change_movement_speed(&self, speed: f32) {
        self.dispatch_internal_speed("app-event.change_movement_speed", speed);
    }

    fn dispatch_exiting_session(&self) {
        self.catch_error(dispatch_event("app-event.exiting_session"));
    }
    fn dispatch_toggle_info_panel(&self) {
        self.catch_error(dispatch_event("app-event.toggle_info_panel"));
    }
    fn dispatch_fps(&self, fps: f32) {
        self.catch_error(dispatch_event_with("app-event.fps", &fps.into()));
    }

    fn dispatch_request_pointer_lock(&self) {
        self.catch_error(dispatch_event("app-event.request_pointer_lock"));
    }

    fn dispatch_exit_pointer_lock(&self) {
        self.catch_error(dispatch_event("app-event.exit_pointer_lock"));
    }

    fn dispatch_screenshot(&self, pixels: &[u8], multiplier: f64) {
        let js_pixels = unsafe { js_sys::Uint8Array::view(pixels) };
        let array = Array::new();
        array.push(&js_pixels);
        array.push(&multiplier.into());
        self.catch_error(dispatch_event_with("app-event.screenshot", &array));
    }

    fn dispatch_top_message(&self, message: &str) {
        self.catch_error(dispatch_event_with("app-event.top_message", &message.into()));
    }
}

impl WebEventDispatcher {
    fn dispatch_internal_speed(&self, id: &str, speed: f32) {
        self.catch_error(dispatch_event_with(id, &format!("x{}", format!("{:.03}", speed)).into()));
    }

    fn dispatch_change_color(&self, id: &str, color: i32) {
        self.catch_error(dispatch_event_with(id, &format!("#{:X}", color).into()));
    }

    pub fn check_error(&self) -> WasmResult<()> {
        if let Some(e) = self.error.borrow_mut().take() {
            return Err(e);
        }
        Ok(())
    }

    fn catch_error(&self, result: WasmResult<()>) {
        if self.error.borrow().is_some() {
            return;
        }
        if let Err(e) = result {
            *self.error.borrow_mut() = Some(e);
        }
    }
}
