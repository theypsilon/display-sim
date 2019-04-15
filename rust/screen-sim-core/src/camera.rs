use crate::app_events::AppEventDispatcher;

pub enum CameraDirection {
    Down,
    Up,
    Left,
    Right,
    Forward,
    Backward,
}

pub struct Camera {
    position: glm::Vec3,
    position_delta: glm::Vec3,
    direction: glm::Vec3,
    axis_up: glm::Vec3,
    axis_right: glm::Vec3,
    pitch: f32,
    heading: f32,
    rotate: f32,
    pub zoom: f32,
    pub movement_speed: f32,
    pub turning_speed: f32,
    sending_camera_update_event: bool,
}

impl Camera {
    pub fn new(movement_speed: f32, turning_speed: f32) -> Camera {
        Camera {
            position: glm::vec3(0.0, 0.0, 0.0),
            position_delta: glm::vec3(0.0, 0.0, 0.0),
            direction: glm::vec3(0.0, 0.0, -1.0),
            axis_up: glm::vec3(0.0, 1.0, 0.0),
            axis_right: glm::vec3(1.0, 0.0, 0.0),
            pitch: 0.0,
            heading: 0.0,
            rotate: 0.0,
            zoom: 45.0,
            movement_speed,
            turning_speed,
            sending_camera_update_event: true,
        }
    }

    pub fn set_position(&mut self, new_position: glm::Vec3) {
        self.position_delta = new_position - self.position;
    }

    pub fn get_position(&self) -> glm::Vec3 {
        self.position_delta + self.position
    }

    pub fn get_axis_up(&self) -> glm::Vec3 {
        self.axis_up
    }

    pub fn set_axis_up(&mut self, new_axis_up: glm::Vec3) {
        self.axis_up = new_axis_up;
    }

    pub fn get_direction(&self) -> glm::Vec3 {
        self.direction
    }

    pub fn set_direction(&mut self, new_direction: glm::Vec3) {
        self.direction = new_direction;
    }

    pub fn advance(&mut self, direction: CameraDirection, dt: f32) {
        let velocity = self.movement_speed * dt;
        self.position_delta += match direction {
            CameraDirection::Up => self.axis_up * velocity,
            CameraDirection::Down => -self.axis_up * velocity,
            CameraDirection::Left => -self.axis_right * velocity,
            CameraDirection::Right => self.axis_right * velocity,
            CameraDirection::Forward => self.direction * velocity,
            CameraDirection::Backward => -self.direction * velocity,
        };
    }

    pub fn turn(&mut self, direction: CameraDirection, dt: f32) {
        let velocity = 20.0 * dt * 0.003 * self.turning_speed;
        match direction {
            CameraDirection::Up => self.heading += velocity,
            CameraDirection::Down => self.heading -= velocity,
            CameraDirection::Left => self.pitch += velocity,
            CameraDirection::Right => self.pitch -= velocity,
            _ => unreachable!(),
        };
    }

    pub fn rotate(&mut self, direction: CameraDirection, dt: f32) {
        let velocity = 60.0 * dt * 0.001 * self.turning_speed;
        match direction {
            CameraDirection::Left => self.rotate += velocity,
            CameraDirection::Right => self.rotate -= velocity,
            _ => unreachable!(),
        };
    }

    pub fn drag(&mut self, xoffset: i32, yoffset: i32) {
        self.pitch -= xoffset as f32 * 0.0003;
        self.heading -= yoffset as f32 * 0.0003;
    }

    pub fn change_zoom(&mut self, change: f32, dispatcher: &impl AppEventDispatcher) {
        let last_zoom = self.zoom;
        if self.zoom >= 1.0 && self.zoom <= 45.0 {
            self.zoom -= change * 0.1;
        }
        if self.zoom <= 1.0 {
            self.zoom = 1.0;
            dispatcher.dispatch_top_message("Minimum value is 1.0");
        }
        if self.zoom >= 45.0 {
            self.zoom = 45.0;
            dispatcher.dispatch_top_message("Maximum value is 45.0");
        }
        if (self.zoom - last_zoom).abs() < std::f32::EPSILON {
            dispatcher.dispatch_change_camera_zoom(self.zoom);
        }
    }

    pub fn update_view(&mut self, dispatcher: &impl AppEventDispatcher) {
        if self.pitch == 0.0 && self.heading == 0.0 && self.rotate == 0.0 && self.position_delta == glm::vec3(0.0, 0.0, 0.0) {
            return;
        }
        let pitch_quat = glm::quat_angle_axis(self.pitch, &self.axis_up);
        let heading_quat = glm::quat_angle_axis(self.heading, &self.axis_right);
        let rotate_quat = glm::quat_angle_axis(self.rotate, &self.direction);

        let temp = glm::quat_cross(&glm::quat_cross(&pitch_quat, &heading_quat), &rotate_quat);

        self.direction = glm::quat_cross_vec(&temp, &self.direction);
        self.axis_up = glm::quat_cross_vec(&temp, &self.axis_up);
        self.axis_right = glm::quat_cross_vec(&temp, &self.axis_right);

        self.heading *= 0.5;
        self.pitch *= 0.5;
        self.rotate *= 0.5;

        self.position += self.position_delta;
        self.position_delta = glm::vec3(0.0, 0.0, 0.0);

        if !self.sending_camera_update_event {
            return;
        }

        dispatcher.dispatch_camera_update(&self.position, &self.direction, &self.axis_up);
    }

    pub fn get_view(&self) -> glm::TMat4<f32> {
        glm::look_at(&self.position, &(self.position + self.direction), &self.axis_up)
    }

    pub fn get_projection(&self, width: f32, height: f32) -> glm::TMat4<f32> {
        glm::perspective::<f32>(width / height, radians(self.zoom), 0.01, 10000.0)
    }
}

pub fn radians(grad: f32) -> f32 {
    let pi: f32 = glm::pi();
    grad * pi / 180.0
}