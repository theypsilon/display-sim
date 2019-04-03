use crate::dispatch_event::{dispatch_event, dispatch_event_with};
use crate::simulation_state::Resources;
use crate::wasm_error::WasmResult;
use js_sys::Array;

pub fn dispatch_change_pixel_horizontal_gap(size: f32) -> WasmResult<()> {
    dispatch_event_with("app-event.change_pixel_horizontal_gap", &format!("{:.03}", size).into())
}

pub fn dispatch_change_pixel_vertical_gap(size: f32) -> WasmResult<()> {
    dispatch_event_with("app-event.change_pixel_vertical_gap", &format!("{:.03}", size).into())
}

pub fn dispatch_change_pixel_width(size: f32) -> WasmResult<()> {
    dispatch_event_with("app-event.change_pixel_width", &format!("{:.03}", size).into())
}

pub fn dispatch_change_pixel_spread(size: f32) -> WasmResult<()> {
    dispatch_event_with("app-event.change_pixel_spread", &format!("{:.03}", size).into())
}

pub fn dispatch_change_pixel_brightness(res: &Resources) -> WasmResult<()> {
    dispatch_event_with("app-event.change_pixel_brightness", &format!("{:.02}", res.crt_filters.extra_bright).into())
}

pub fn dispatch_change_pixel_contrast(res: &Resources) -> WasmResult<()> {
    dispatch_event_with("app-event.change_pixel_contrast", &format!("{:.03}", res.crt_filters.extra_contrast).into())
}

pub fn dispatch_change_light_color(res: &Resources) -> WasmResult<()> {
    dispatch_change_color("app-event.change_light_color", res.crt_filters.light_color)
}

pub fn dispatch_change_brightness_color(res: &Resources) -> WasmResult<()> {
    dispatch_change_color("app-event.change_brightness_color", res.crt_filters.brightness_color)
}

fn dispatch_change_color(id: &str, color: i32) -> WasmResult<()> {
    dispatch_event_with(id, &format!("#{:X}", color).into())
}

pub fn dispatch_change_camera_zoom(res: &Resources) -> WasmResult<()> {
    dispatch_event_with("app-event.change_camera_zoom", &format!("{:.02}", res.camera.zoom).into())
}

pub fn dispatch_change_blur_level(res: &Resources) -> WasmResult<()> {
    dispatch_event_with("app-event.change_blur_level", &(res.crt_filters.blur_passes as i32).into())
}

pub fn dispatch_change_lines_per_pixel(res: &Resources) -> WasmResult<()> {
    dispatch_event_with("app-event.change_lines_per_pixel", &(res.crt_filters.lines_per_pixel as i32).into())
}

pub fn dispatch_color_representation(res: &Resources) -> WasmResult<()> {
    dispatch_event_with("app-event.color_representation", &(res.crt_filters.color_channels.to_string()).into())
}

pub fn dispatch_pixel_geometry(res: &Resources) -> WasmResult<()> {
    dispatch_event_with("app-event.pixel_geometry", &(res.crt_filters.pixels_geometry_kind.to_string()).into())
}

pub fn dispatch_pixel_shadow_shape(res: &Resources) -> WasmResult<()> {
    dispatch_event_with("app-event.pixel_shadow_shape", &(res.crt_filters.pixel_shadow_shape_kind.to_string()).into())
}

pub fn dispatch_pixel_shadow_height(res: &Resources) -> WasmResult<()> {
    dispatch_event_with("app-event.pixel_shadow_height", &(res.crt_filters.pixel_shadow_height_factor).into())
}

pub fn dispatch_screen_layering_type(res: &Resources) -> WasmResult<()> {
    dispatch_event_with("app-event.screen_layering_type", &(res.crt_filters.layering_kind.to_string()).into())
}

pub fn dispatch_screen_curvature(res: &Resources) -> WasmResult<()> {
    dispatch_event_with("app-event.screen_curvature", &(res.crt_filters.screen_curvature_kind.to_string()).into())
}

pub fn dispatch_internal_resolution(res: &Resources) -> WasmResult<()> {
    dispatch_event_with("app-event.internal_resolution", &(res.crt_filters.internal_resolution.to_label(&res.animation)).into())
}

pub fn dispatch_texture_interpolation(res: &Resources) -> WasmResult<()> {
    dispatch_event_with("app-event.texture_interpolation", &(res.crt_filters.texture_interpolation.to_string()).into())
}

pub fn dispatch_change_pixel_speed(speed: f32) -> WasmResult<()> {
    dispatch_internal_speed("app-event.change_pixel_speed", speed)
}

pub fn dispatch_change_turning_speed(speed: f32) -> WasmResult<()> {
    dispatch_internal_speed("app-event.change_turning_speed", speed)
}

pub fn dispatch_change_movement_speed(speed: f32) -> WasmResult<()> {
    dispatch_internal_speed("app-event.change_movement_speed", speed)
}

fn dispatch_internal_speed(id: &str, speed: f32) -> WasmResult<()> {
    dispatch_event_with(id, &format!("x{}", format!("{:.03}", speed)).into())
}

pub fn dispatch_exiting_session() -> WasmResult<()> {
    dispatch_event("app-event.exiting_session")
}
pub fn dispatch_toggle_info_panel() -> WasmResult<()> {
    dispatch_event("app-event.toggle_info_panel")
}
pub fn dispatch_fps(fps: f32) -> WasmResult<()> {
    dispatch_event_with("app-event.fps", &fps.into())
}

pub fn dispatch_request_pointer_lock() -> WasmResult<()> {
    dispatch_event("app-event.request_pointer_lock")
}

pub fn dispatch_exit_pointer_lock() -> WasmResult<()> {
    dispatch_event("app-event.exit_pointer_lock")
}

pub fn dispatch_screenshot(array: &Array) -> WasmResult<()> {
    dispatch_event_with("app-event.screenshot", &array)
}

pub fn dispatch_top_message(message: String) -> WasmResult<()> {
    dispatch_event_with("app-event.top_message", &message.into())
}
