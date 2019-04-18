use crate::simulation_core_state::Resources;

pub trait AppEventDispatcher: Default {
    fn dispatch_camera_update(&self, position: &glm::Vec3, direction: &glm::Vec3, axis_up: &glm::Vec3);
    fn dispatch_change_pixel_horizontal_gap(&self, size: f32);
    fn dispatch_change_pixel_vertical_gap(&self, size: f32);
    fn dispatch_change_pixel_width(&self, size: f32);
    fn dispatch_change_pixel_spread(&self, size: f32);
    fn dispatch_change_pixel_brightness(&self, res: &Resources);
    fn dispatch_change_pixel_contrast(&self, res: &Resources);
    fn dispatch_change_light_color(&self, res: &Resources);
    fn dispatch_change_brightness_color(&self, res: &Resources);
    fn dispatch_change_camera_zoom(&self, zoom: f32);
    fn dispatch_change_blur_level(&self, res: &Resources);
    fn dispatch_change_lines_per_pixel(&self, res: &Resources);
    fn dispatch_color_representation(&self, res: &Resources);
    fn dispatch_pixel_geometry(&self, res: &Resources);
    fn dispatch_pixel_shadow_shape(&self, res: &Resources);
    fn dispatch_pixel_shadow_height(&self, res: &Resources);
    fn dispatch_screen_layering_type(&self, res: &Resources);
    fn dispatch_screen_curvature(&self, res: &Resources);
    fn dispatch_internal_resolution(&self, res: &Resources);
    fn dispatch_texture_interpolation(&self, res: &Resources);
    fn dispatch_change_pixel_speed(&self, speed: f32);
    fn dispatch_change_turning_speed(&self, speed: f32);
    fn dispatch_change_movement_speed(&self, speed: f32);
    fn dispatch_exiting_session(&self);
    fn dispatch_toggle_info_panel(&self);
    fn dispatch_fps(&self, fps: f32);
    fn dispatch_request_pointer_lock(&self);
    fn dispatch_exit_pointer_lock(&self);
    fn dispatch_screenshot(&self, pixels: &[u8], multiplier: f64);
    fn dispatch_change_camera_movement_mode(&self, locked_mode: bool);
    fn dispatch_top_message(&self, message: &str);
}
