/* Copyright (c) 2019-2022 José manuel Barroso Galindo <theypsilon@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use crate::app_events::AppEventDispatcher;

pub(crate) enum CameraDirection {
    Down,
    Up,
    Left,
    Right,
    Forward,
    Backward,
}

#[derive(Copy, Clone, Debug)]
pub enum CameraChange {
    Zoom(f32),
    PosX(f32),
    PosY(f32),
    PosZ(f32),
    AxisUpX(f32),
    AxisUpY(f32),
    AxisUpZ(f32),
    DirectionX(f32),
    DirectionY(f32),
    DirectionZ(f32),
}

#[derive(Copy, Clone)]
pub enum CameraLockMode {
    TwoDimensional,
    ThreeDimensional,
}

impl std::fmt::Display for CameraLockMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CameraLockMode::TwoDimensional => "2D",
                CameraLockMode::ThreeDimensional => "3D",
            }
        )
    }
}

impl CameraChange {
    pub fn get_f32(self) -> f32 {
        match self {
            CameraChange::Zoom(n) => n,
            CameraChange::PosX(n) => n,
            CameraChange::PosY(n) => n,
            CameraChange::PosZ(n) => n,
            CameraChange::AxisUpX(n) => n,
            CameraChange::AxisUpY(n) => n,
            CameraChange::AxisUpZ(n) => n,
            CameraChange::DirectionX(n) => n,
            CameraChange::DirectionY(n) => n,
            CameraChange::DirectionZ(n) => n,
        }
    }
}

#[derive(Clone)]
pub struct CameraData {
    pub position_destiny: glm::Vec3,
    pub position_eye: glm::Vec3,
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
    pub locked_mode: CameraLockMode,
    pub position_changed: bool,
}

impl CameraData {
    pub(crate) fn new(movement_speed: f32, turning_speed: f32) -> CameraData {
        CameraData {
            position_destiny: glm::vec3(0.0, 0.0, 0.0),
            position_eye: glm::vec3(0.0, 0.0, 0.0),
            direction: glm::vec3(0.0, 0.0, -1.0),
            axis_up: glm::vec3(0.0, 1.0, 0.0),
            axis_right: glm::vec3(1.0, 0.0, 0.0),
            pitch: 0.0,
            heading: 0.0,
            rotate: 0.0,
            zoom: 45.0,
            movement_speed,
            turning_speed,
            position_changed: true,
            sending_camera_update_event: true,
            locked_mode: CameraLockMode::TwoDimensional,
        }
    }

    pub(crate) fn set_position(&mut self, new_position: glm::Vec3) {
        self.position_destiny = new_position;
        self.position_eye = new_position;
        self.position_changed = true;
    }

    pub fn get_position(&self) -> glm::Vec3 {
        self.position_eye
    }

    pub fn get_view(&self) -> glm::TMat4<f32> {
        glm::look_at(&self.position_eye, &(self.position_eye + self.direction), &self.axis_up)
    }

    pub fn get_projection(&self, width: f32, height: f32) -> glm::TMat4<f32> {
        glm::perspective::<f32>(width / height, crate::math::radians(self.zoom), 0.01, 10000.0)
    }
}

pub(crate) struct CameraSystem<'a> {
    data: &'a mut CameraData,
    dispatcher: &'a dyn AppEventDispatcher,
}

impl<'a> CameraSystem<'a> {
    pub(crate) fn new(data: &'a mut CameraData, dispatcher: &'a dyn AppEventDispatcher) -> CameraSystem<'a> {
        CameraSystem { data, dispatcher }
    }

    pub(crate) fn advance(&mut self, direction: CameraDirection, dt: f32) {
        let velocity = self.data.movement_speed
            * dt
            * match self.data.locked_mode {
                CameraLockMode::TwoDimensional => -1.0,
                CameraLockMode::ThreeDimensional => 1.0,
            };
        let position_delta = match self.data.locked_mode {
            CameraLockMode::TwoDimensional => match direction {
                CameraDirection::Forward => self.data.axis_up * velocity,
                CameraDirection::Backward => -self.data.axis_up * velocity,
                CameraDirection::Left => -self.data.axis_right * velocity,
                CameraDirection::Right => self.data.axis_right * velocity,
                _ => glm::vec3(0.0, 0.0, 0.0),
            },
            CameraLockMode::ThreeDimensional => match direction {
                CameraDirection::Up => self.data.axis_up * velocity,
                CameraDirection::Down => -self.data.axis_up * velocity,
                CameraDirection::Left => -self.data.axis_right * velocity,
                CameraDirection::Right => self.data.axis_right * velocity,
                CameraDirection::Forward => self.data.direction * velocity,
                CameraDirection::Backward => -self.data.direction * velocity,
            },
        };
        self.data.position_destiny += position_delta;
        self.data.position_changed = true;
    }

    pub(crate) fn turn(&mut self, direction: CameraDirection, dt: f32) {
        match self.data.locked_mode {
            CameraLockMode::TwoDimensional => return,
            CameraLockMode::ThreeDimensional => {}
        }
        let velocity = dt
            * self.data.turning_speed
            * match self.data.locked_mode {
                CameraLockMode::TwoDimensional => 0.03,
                CameraLockMode::ThreeDimensional => 0.06,
            };
        match direction {
            CameraDirection::Up => self.data.heading += velocity,
            CameraDirection::Down => self.data.heading -= velocity,
            CameraDirection::Left => self.data.pitch += velocity,
            CameraDirection::Right => self.data.pitch -= velocity,
            _ => unreachable!(),
        };
    }

    pub(crate) fn rotate(&mut self, direction: CameraDirection, dt: f32) {
        match self.data.locked_mode {
            CameraLockMode::TwoDimensional => return,
            CameraLockMode::ThreeDimensional => {}
        }
        let velocity = 60.0
            * dt
            * 0.001
            * self.data.turning_speed
            * match self.data.locked_mode {
                CameraLockMode::TwoDimensional => -1.0,
                CameraLockMode::ThreeDimensional => 1.0,
            };
        match direction {
            CameraDirection::Left => self.data.rotate += velocity,
            CameraDirection::Right => self.data.rotate -= velocity,
            _ => unreachable!(),
        };
    }

    pub(crate) fn drag(&mut self, xoffset: i32, yoffset: i32) {
        let xoffset = xoffset as f32;
        let yoffset = yoffset as f32;
        match self.data.locked_mode {
            CameraLockMode::TwoDimensional => {
                let position_delta = self.data.axis_up * yoffset * 0.1 - self.data.axis_right * xoffset * 0.1;
                self.data.position_destiny += position_delta;
                self.data.position_changed = true;
            }
            CameraLockMode::ThreeDimensional => {
                self.data.pitch -= xoffset * 0.0003;
                self.data.heading -= yoffset * 0.0003;
            }
        }
    }

    pub(crate) fn look_at(&mut self, target: glm::Vec3) {
        let mut new_direction = (target - self.data.position_eye).normalize();
        if glm::length(&new_direction) <= 0.1 {
            new_direction = self.data.direction;
        }
        self.data.direction = new_direction;
        self.data.axis_right = glm::quat_cross_vec(&glm::quat_look_at(&new_direction, &self.data.axis_up), &self.data.axis_right);
    }

    pub(crate) fn handle_camera_change(&mut self, change: CameraChange) {
        match change {
            CameraChange::PosX(x) => self.data.position_eye.x = x,
            CameraChange::PosY(y) => self.data.position_eye.y = y,
            CameraChange::PosZ(z) => self.data.position_eye.z = z,
            CameraChange::Zoom(zoom) => self.data.zoom = zoom,
            CameraChange::AxisUpX(x) => self.data.axis_up.x = x,
            CameraChange::AxisUpY(y) => self.data.axis_up.y = y,
            CameraChange::AxisUpZ(z) => self.data.axis_up.z = z,
            CameraChange::DirectionX(x) => self.data.direction.x = x,
            CameraChange::DirectionY(y) => self.data.direction.y = y,
            CameraChange::DirectionZ(z) => self.data.direction.z = z,
        }
        self.data.position_changed = true;
        self.data.position_destiny = self.data.position_eye;
    }

    pub(crate) fn change_zoom(&mut self, change: f32, dispatcher: &dyn AppEventDispatcher) {
        let last_zoom = self.data.zoom;
        if self.data.zoom >= 0.1 && self.data.zoom <= 90.0 {
            self.data.zoom -= change * 0.1;
        }
        if self.data.zoom <= 0.1 {
            self.data.zoom = 0.1;
            dispatcher.dispatch_top_message("Minimum value is 0.1");
        }
        if self.data.zoom >= 90.0 {
            self.data.zoom = 90.0;
            dispatcher.dispatch_top_message("Maximum value is 90.0");
        }
        if (self.data.zoom - last_zoom).abs() > std::f32::EPSILON {
            dispatcher.dispatch_change_camera_zoom(self.data.zoom);
        }
    }

    pub(crate) fn update_view(&mut self, dt: f32) {
        if self.data.pitch == 0.0 && self.data.heading == 0.0 && self.data.rotate == 0.0 && !self.data.position_changed {
            return;
        }
        self.data.position_changed = false;

        let pitch_quat = glm::quat_angle_axis(self.data.pitch, &self.data.axis_up);
        let heading_quat = glm::quat_angle_axis(self.data.heading, &self.data.axis_right);
        let rotate_quat = glm::quat_angle_axis(self.data.rotate, &self.data.direction);

        let temp = glm::quat_cross(&glm::quat_cross(&pitch_quat, &heading_quat), &rotate_quat);

        let new_direction = glm::quat_cross_vec(&temp, &self.data.direction);

        if matches!(self.data.locked_mode, CameraLockMode::ThreeDimensional) || new_direction.z <= -0.01 {
            self.data.direction = new_direction;
            self.data.axis_up = glm::quat_cross_vec(&temp, &self.data.axis_up);
            self.data.axis_right = glm::quat_cross_vec(&temp, &self.data.axis_right);
        }

        self.data.heading *= 0.5;
        self.data.pitch *= 0.5;
        self.data.rotate *= 0.5;

        if let CameraLockMode::TwoDimensional = self.data.locked_mode {
            if self.data.pitch.abs() > std::f32::EPSILON || self.data.heading.abs() > std::f32::EPSILON {
                let distance_to_origin = glm::length(&self.data.position_destiny);
                self.data.position_destiny = -self.data.direction * distance_to_origin;
            }
            self.data.position_destiny.z = self.data.position_destiny.z.clamp(0.8, 8000.0);
            self.data.position_destiny.x = self.data.position_destiny.x.clamp(-395.0, 395.0);
            self.data.position_destiny.y = self.data.position_destiny.y.clamp(-220.0, 220.0);
        }

        let position_movement = (self.data.position_destiny - self.data.position_eye) * dt * 10.0;
        if glm::length(&position_movement) < 5.0 * dt * self.data.turning_speed {
            self.data.position_eye = self.data.position_destiny;
        } else {
            self.data.position_eye += position_movement;
        }

        if !self.data.sending_camera_update_event {
            return;
        }

        self.dispatcher
            .dispatch_camera_update(&self.data.position_eye, &self.data.direction, &self.data.axis_up);
    }
}
