use crate::app_events::AppEventDispatcher;

pub enum CameraDirection {
    Down,
    Up,
    Left,
    Right,
    Forward,
    Backward,
}

pub struct CameraData {
    pub position: glm::Vec3,
    pub position_delta: glm::Vec3,
    pub position_temp: glm::Vec3,
    pub direction: glm::Vec3,
    pub axis_up: glm::Vec3,
    pub axis_right: glm::Vec3,
    pub pitch: f32,
    pub heading: f32,
    pub rotate: f32,
    pub zoom: f32,
    pub movement_speed: f32,
    pub turning_speed: f32,
    pub sending_camera_update_event: bool,
    pub locked_mode: bool,
}

impl CameraData {
    pub fn new(movement_speed: f32, turning_speed: f32) -> CameraData {
        CameraData {
            position: glm::vec3(0.0, 0.0, 0.0),
            position_delta: glm::vec3(0.0, 0.0, 0.0),
            position_temp: glm::vec3(0.0, 0.0, 0.0),
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
            locked_mode: true,
        }
    }

    pub fn set_position(&mut self, new_position: glm::Vec3) {
        self.position_delta = new_position - self.position;
        self.position_temp = new_position;
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

    pub fn get_view(&self) -> glm::TMat4<f32> {
        glm::look_at(&self.position_temp, &(self.position_temp + self.direction), &self.axis_up)
    }

    pub fn get_projection(&self, width: f32, height: f32) -> glm::TMat4<f32> {
        glm::perspective::<f32>(width / height, radians(self.zoom), 0.01, 10000.0)
    }
}

pub struct CameraSystem<'a, Dispatcher: AppEventDispatcher> {
    data: &'a mut CameraData,
    dispatcher: &'a Dispatcher,
}

impl<'a, Dispatcher: AppEventDispatcher> CameraSystem<'a, Dispatcher> {
    pub fn new(data: &'a mut CameraData, dispatcher: &'a Dispatcher) -> CameraSystem<'a, Dispatcher> {
        CameraSystem { data, dispatcher }
    }

    pub fn set_position(&mut self, new_position: glm::Vec3) {
        self.data.set_position(new_position)
    }

    pub fn get_position(&self) -> glm::Vec3 {
        self.data.get_position()
    }

    pub fn get_axis_up(&self) -> glm::Vec3 {
        self.data.get_axis_up()
    }

    pub fn set_axis_up(&mut self, new_axis_up: glm::Vec3) {
        self.data.set_axis_up(new_axis_up)
    }

    pub fn get_direction(&self) -> glm::Vec3 {
        self.data.get_direction()
    }

    pub fn set_direction(&mut self, new_direction: glm::Vec3) {
        self.data.set_direction(new_direction)
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.data.zoom = zoom;
    }

    pub fn get_zoom(&mut self) -> f32 {
        self.data.zoom
    }

    pub fn advance(&mut self, direction: CameraDirection, dt: f32) {
        let velocity = self.data.movement_speed * dt * if self.data.locked_mode { -1.0 } else { 1.0 };
        let position_delta = match direction {
            CameraDirection::Up => self.data.axis_up * velocity,
            CameraDirection::Down => -self.data.axis_up * velocity,
            CameraDirection::Left => -self.data.axis_right * velocity,
            CameraDirection::Right => self.data.axis_right * velocity,
            CameraDirection::Forward => self.data.direction * velocity,
            CameraDirection::Backward => -self.data.direction * velocity,
        };
        self.data.position_delta += position_delta;
    }

    pub fn turn(&mut self, direction: CameraDirection, dt: f32) {
        let velocity = dt * self.data.turning_speed * if self.data.locked_mode { 0.03 } else { 0.06 };
        match direction {
            CameraDirection::Up => self.data.heading += velocity,
            CameraDirection::Down => self.data.heading -= velocity,
            CameraDirection::Left => self.data.pitch += velocity,
            CameraDirection::Right => self.data.pitch -= velocity,
            _ => unreachable!(),
        };
    }

    pub fn rotate(&mut self, direction: CameraDirection, dt: f32) {
        let velocity = 60.0 * dt * 0.001 * self.data.turning_speed * if self.data.locked_mode { -1.0 } else { 1.0 };
        match direction {
            CameraDirection::Left => self.data.rotate += velocity,
            CameraDirection::Right => self.data.rotate -= velocity,
            _ => unreachable!(),
        };
    }

    pub fn drag(&mut self, xoffset: i32, yoffset: i32) {
        self.data.pitch -= xoffset as f32 * 0.0003;
        self.data.heading -= yoffset as f32 * 0.0003;
    }

    pub fn change_zoom(&mut self, change: f32, dispatcher: &impl AppEventDispatcher) {
        let last_zoom = self.data.zoom;
        if self.data.zoom >= 1.0 && self.data.zoom <= 45.0 {
            self.data.zoom -= change * 0.1;
        }
        if self.data.zoom <= 1.0 {
            self.data.zoom = 1.0;
            dispatcher.dispatch_top_message("Minimum value is 1.0");
        }
        if self.data.zoom >= 45.0 {
            self.data.zoom = 45.0;
            dispatcher.dispatch_top_message("Maximum value is 45.0");
        }
        if (self.data.zoom - last_zoom).abs() < std::f32::EPSILON {
            dispatcher.dispatch_change_camera_zoom(self.data.zoom);
        }
    }

    pub fn update_view(&mut self, dt: f32) {
        if self.data.pitch == 0.0 && self.data.heading == 0.0 && self.data.rotate == 0.0 && self.data.position_delta == glm::vec3(0.0, 0.0, 0.0) {
            return;
        }
        let old_direction = self.data.direction;

        let pitch_quat = glm::quat_angle_axis(self.data.pitch, &self.data.axis_up);
        let heading_quat = glm::quat_angle_axis(self.data.heading, &self.data.axis_right);
        let rotate_quat = glm::quat_angle_axis(self.data.rotate, &old_direction);

        let temp = glm::quat_cross(&glm::quat_cross(&pitch_quat, &heading_quat), &rotate_quat);

        self.data.direction = glm::quat_cross_vec(&temp, &old_direction);
        if self.data.locked_mode && self.data.direction.z > -0.01 {
            self.data.direction = old_direction;
        } else {
            self.data.axis_up = glm::quat_cross_vec(&temp, &self.data.axis_up);
            self.data.axis_right = glm::quat_cross_vec(&temp, &self.data.axis_right);
        }

        self.data.heading *= 0.5;
        self.data.pitch *= 0.5;
        self.data.rotate *= 0.5;

        let position_delta = self.data.position_delta;
        self.data.position += position_delta;
        self.data.position_delta = glm::vec3(0.0, 0.0, 0.0);

        if self.data.locked_mode {
            if self.data.pitch.abs() > std::f32::EPSILON || self.data.heading.abs() > std::f32::EPSILON {
                let distance_to_origin = glm::length(&self.data.position);
                self.data.position = -self.data.direction * distance_to_origin;
            }
            if self.data.position.z < 0.8 {
                self.data.position.z = 0.8;
            } else if self.data.position.z > 8000.0 {
                self.data.position.z = 8000.0;
            }
            if self.data.position.x < -395.0 {
                self.data.position.x = -395.0;
            } else if self.data.position.x > 395.0 {
                self.data.position.x = 395.0;
            }
            if self.data.position.y < -220.0 {
                self.data.position.y = -220.0;
            } else if self.data.position.y > 220.0 {
                self.data.position.y = 220.0;
            }
        }

        let position_movement = (self.data.position - self.data.position_temp) * dt * 10.0;
        if glm::length(&position_movement) < 5.0 * dt * self.data.turning_speed {
            self.data.position_temp = self.data.position;
        } else {
            self.data.position_temp += position_movement;
        }

        if !self.data.sending_camera_update_event {
            return;
        }

        self.dispatcher
            .dispatch_camera_update(&self.data.position_temp, &self.data.direction, &self.data.axis_up);
    }
}

pub fn radians(grad: f32) -> f32 {
    let pi: f32 = glm::pi();
    grad * pi / 180.0
}
