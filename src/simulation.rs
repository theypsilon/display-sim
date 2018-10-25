use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    console,
    WebGl2RenderingContext,
};
use js_sys::{Float32Array};
use std::rc::Rc;
use std::cell::RefCell;
use super::glm;

use wasm_error::{Result, WasmError};
use camera::{CameraDirection, Camera};
use dispatch_event::{dispatch_event, dispatch_event_with};
use web_utils::{now, window};
use pixels_render::PixelsRender;
use blur_render::BlurRender;
use event_listeners::set_event_listeners;
use state::{StateOwner, Resources, Input, AnimationData, Buttons, PixelsOrVoxels};

const PIXEL_MANIPULATION_BASE_SPEED: f32 = 20.0;
const TURNING_BASE_SPEED: f32 = 3.0;
const MOVEMENT_BASE_SPEED: f32 = 10.0;
const MOVEMENT_SPEED_FACTOR: f32 = 50.0;

pub fn program(gl: JsValue, animation: AnimationData) -> Result<()> {
    let gl = gl.dyn_into::<WebGl2RenderingContext>()?;
    gl.enable(WebGl2RenderingContext::DEPTH_TEST);
    let owned_state = Rc::new(RefCell::new(StateOwner::new(load_resources(&gl, animation)?)));
    let input = Rc::new(RefCell::new( Input::new().ok().expect("cannot create input")));
    let frame_closure: Closure<FnMut(JsValue)> = {
        let owned_state = owned_state.clone();
        let mut input = Rc::clone(&input);
        let window = window()?;
        Closure::wrap(Box::new(move |_| {
            let mut owned_state = owned_state.borrow_mut();
                let mut input = input.borrow_mut();
                input.now = now().unwrap_or(owned_state.resources.last_time);
                let update_status = update(&mut owned_state.resources, &input);

                input.mouse_scroll_y = 0.0;
                input.mouse_position_x = 0;
                input.mouse_position_y = 0;

                match update_status {
                    Ok(next_update_needed) => {
                        if next_update_needed == false {
                            return;
                        }
                        if let Err(e) = draw(&gl, &owned_state.resources) {
                            console::error_2(&"An unexpected error happened during draw.".into(), &e.to_js());
                            return;
                        }
                        let mut frame_id = None;
                        if let Some(ref frame_closure) = owned_state.closures[0] {
                            if let Ok(id) = window.request_animation_frame(frame_closure.as_ref().unchecked_ref()) {
                                frame_id = Some(id);
                            }
                        }
                        owned_state.animation_frame_id = frame_id;
                    },
                    Err(e) => console::error_2(&"An unexpected error happened during update.".into(), &e.to_js())
                }
        }))
    };
    let mut owned_state = owned_state.borrow_mut();
    owned_state.animation_frame_id = Some(window()?.request_animation_frame(frame_closure.as_ref().unchecked_ref())?);
    owned_state.closures.push(Some(frame_closure));

    let listeners = set_event_listeners(&input)?;
    owned_state.closures.extend(listeners);

    Ok(())
}

pub fn load_resources(gl: &WebGl2RenderingContext, animation: AnimationData) -> Result<Resources> {
    let width = animation.width as usize;
    let height = animation.height as usize;
    let pixels_total = width * height;
    let offsets = Float32Array::new(&wasm_bindgen::JsValue::from(pixels_total as u32 * 2));
    {
        let half_width: f32 = width as f32 / 2.0;
        let half_height: f32 = height as f32 / 2.0;
        let center_dx = if width % 2 == 0 {0.5} else {0.0};
        let center_dy = if height % 2 == 0 {0.5} else {0.0};
        for i in 0..width {
            for j in 0..height {
                let index = (pixels_total - width - j * width + i) as u32;
                let x = i as f32 - half_width + center_dx;
                let y = j as f32 - half_height + center_dy;
                offsets.fill(x, index * 2 + 0, index * 2 + 1);
                offsets.fill(y, index * 2 + 1, index * 2 + 2);
            }
        }
    }

    let blur_render = BlurRender::new(&gl, animation.canvas_width as i32, animation.canvas_height as i32)?;
    
    let pixels_render = PixelsRender::new(&gl, &offsets)?;

    let now = now()?;

    let far_away_position = 0.5 + {
        let canvas_width_scaled = (animation.canvas_width as f32 / animation.scale_x) as u32;
        let width_ratio = canvas_width_scaled as f32 / width as f32;
        let height_ratio = animation.canvas_height as f32 / height as f32;
        let is_height_bounded = width_ratio > height_ratio;
        let mut bound_ratio = if is_height_bounded {height_ratio} else {width_ratio};
        let mut resolution = if is_height_bounded {animation.canvas_height} else {canvas_width_scaled} as i32;
        while bound_ratio < 1.0 {
            bound_ratio *= 2.0;
            resolution *= 2;
        }
        if animation.stretch == false {
            let mut divisor = bound_ratio as i32;
            while divisor > 1 {
                if resolution % divisor == 0 {
                    break;
                }
                divisor -= 1;
            };
            bound_ratio = divisor as f32;
        }
        (resolution as f32 / bound_ratio) * if is_height_bounded {1.2076} else {0.68 * animation.scale_x}
    };

    let mut camera = Camera::new(MOVEMENT_BASE_SPEED * far_away_position / MOVEMENT_SPEED_FACTOR, TURNING_BASE_SPEED);
    camera.set_position(glm::vec3(0.0, 0.0, far_away_position));

    check_error(&gl, line!())?;

    dispatch_event_with("app-event.change_pixel_scale_x", &1.0.into())?;
    dispatch_event_with("app-event.change_pixel_scale_y", &1.0.into())?;
    dispatch_event_with("app-event.change_pixel_gap", &1.0.into())?;
    dispatch_event_with("app-event.change_pixel_brightness", &0.0.into())?;

    Ok(Resources {
        pixels_render,
        blur_render,
        light_color: 0x00FF_FFFF,
        brightness_color: 0x00FF_FFFF,
        extra_bright: 0.0,
        frame_count: 0,
        translation_base_speed: camera.movement_speed,
        last_time: now,
        last_second: now,
        bloom_passes: 0,
        pixel_manipulation_speed: PIXEL_MANIPULATION_BASE_SPEED,
        cur_pixel_scale_x: 0.0,
        cur_pixel_scale_y: 0.0,
        cur_pixel_gap: 0.0,
        pixels_or_voxels: PixelsOrVoxels::Pixels,
        pixels_pulse: 0.0,
        showing_pixels_pulse: false,
        animation,
        camera,
        camera_zoom: 45.0,
        buttons: Buttons::new()
    })
}

pub fn update(res: &mut Resources, input: &Input) -> Result<bool> {
    let dt: f32 = ((input.now - res.last_time) / 1000.0) as f32;
    let ellapsed = input.now - res.last_second;
    res.last_time = input.now;

    if ellapsed >= 1_000.0 {
        let fps = res.frame_count as f32;
        dispatch_event_with("app-event.fps", &fps.into())?;
        res.last_second = input.now;
        res.frame_count = 0;
    } else {
        res.frame_count += 1;
    }

    res.animation.needs_buffer_data_load = false;
    let next_frame_update = res.animation.last_frame_change + res.animation.frame_length as f64;
    if input.now >= next_frame_update {
        res.animation.last_frame_change = next_frame_update;
        let last_frame = res.animation.current_frame;
        res.animation.current_frame += 1;
        if res.animation.current_frame >= res.animation.steps.len() {
            res.animation.current_frame = 0;
        }
        if last_frame != res.animation.current_frame {
            res.animation.needs_buffer_data_load = true;
        }
    }

    let color_variable = match input.color_kind {
        0 => &mut res.light_color,
        1 => &mut res.brightness_color,
        other => return Err(("color kind invalid: ".to_string() + &other.to_string()).into()),
    };

    if input.color_value != *color_variable {
        *color_variable = input.color_value;
        dispatch_event_with("app-event.top_message", &"Color changed.".into())?;
    }

    let last_bloom_passes = res.bloom_passes;
    res.buttons.increase_bloom.track(input.increase_bloom);
    res.buttons.decrease_bloom.track(input.decrease_bloom);
    if res.buttons.increase_bloom.is_just_pressed() {
        res.bloom_passes += 1;
    }
    if res.buttons.decrease_bloom.is_just_pressed() && res.bloom_passes > 0 {
        res.bloom_passes -= 1;
    }

    if last_bloom_passes != res.bloom_passes {
        dispatch_event_with("app-event.top_message", &("Blur level: ".to_string() + &res.bloom_passes.to_string()).into())?;
    }

    res.buttons.esc.track(input.esc);
    if res.buttons.esc.is_just_pressed() {
        dispatch_event("app-event.exiting_session")?;
        return Ok(false);
    }

    res.buttons.space.track(input.space);
    if res.buttons.space.is_just_pressed() {
        dispatch_event("app-event.toggle_info_panel")?;
    }

    res.buttons.showing_pixels_pulse.track(input.showing_pixels_pulse);
    if res.buttons.showing_pixels_pulse.is_just_pressed() {
        res.showing_pixels_pulse = !res.showing_pixels_pulse;
        dispatch_event_with("app-event.top_message", &(if res.showing_pixels_pulse {"Screen wave ON."} else {"Screen wave OFF."}).into())?;
        dispatch_event_with("app-event.showing_pixels_pulse", &res.showing_pixels_pulse.into())?;
    }

    if res.showing_pixels_pulse {
        res.pixels_pulse += dt * 0.3;
    } else {
        res.pixels_pulse = 0.0;
    }

    res.buttons.toggle_pixels_or_voxels.track(input.toggle_pixels_or_voxels);
    if res.buttons.toggle_pixels_or_voxels.is_just_released() {
        res.pixels_or_voxels = match res.pixels_or_voxels {
            PixelsOrVoxels::Pixels => PixelsOrVoxels::Voxels,
            PixelsOrVoxels::Voxels => PixelsOrVoxels::Pixels
        };
        let message = match res.pixels_or_voxels {
            PixelsOrVoxels::Pixels => "squares",
            PixelsOrVoxels::Voxels => "cubes"
        };
        dispatch_event_with("app-event.top_message", &("Showing pixels as ".to_string() + &message+ &".").into())?;
        dispatch_event_with("app-event.showing_pixels_as", &message.into())?;
    }

    res.buttons.speed_up.track(input.speed_up);
    res.buttons.speed_down.track(input.speed_down);
    if input.alt {
        let last_turning_speed = res.camera.turning_speed;
        if res.buttons.speed_up.is_just_pressed() && res.camera.turning_speed < 10000.0 { res.camera.turning_speed *= 2.0; }
        if res.buttons.speed_down.is_just_pressed() && res.camera.turning_speed > 0.01 { res.camera.turning_speed /= 2.0; }
        if (last_turning_speed - res.camera.turning_speed).abs() < std::f32::EPSILON {
            let turning_speed = (res.camera.turning_speed / TURNING_BASE_SPEED * 1000.0).round() / 1000.0;
            let message = "Turning camera speed: ".to_string() + &turning_speed.to_string() + &"x".to_string();
            dispatch_event_with("app-event.top_message", &message.into())?;
            dispatch_event_with("app-event.turning_speed", &turning_speed.into())?;
        }
    } else if input.shift {
        let last_pixel_manipulation_speed = res.pixel_manipulation_speed;
        if res.buttons.speed_up.is_just_pressed() && res.pixel_manipulation_speed < 10000.0 { res.pixel_manipulation_speed *= 2.0; }
        if res.buttons.speed_down.is_just_pressed() && res.pixel_manipulation_speed > 0.01 { res.pixel_manipulation_speed /= 2.0; }
        if (last_pixel_manipulation_speed - res.pixel_manipulation_speed).abs() < std::f32::EPSILON {
            let pixel_manipulation_speed = (res.pixel_manipulation_speed / PIXEL_MANIPULATION_BASE_SPEED * 1000.0).round() / 1000.0;
            let message = "Pixel manipulation speed: ".to_string() + &pixel_manipulation_speed.to_string() + &"x".to_string();
            dispatch_event_with("app-event.top_message", &message.into())?;
            dispatch_event_with("app-event.pixel_manipulation_speed", &pixel_manipulation_speed.into())?;
        }
    } else {
        let last_movement_speed = res.camera.movement_speed;
        if res.buttons.speed_up.is_just_pressed() && res.camera.movement_speed < 10000.0 { res.camera.movement_speed *= 2.0; }
        if res.buttons.speed_down.is_just_pressed() && res.camera.movement_speed > 0.01 { res.camera.movement_speed /= 2.0; }
        if (last_movement_speed - res.camera.movement_speed).abs() < std::f32::EPSILON {
            let translation_speed = (res.camera.movement_speed / res.translation_base_speed * 1000.0).round() / 1000.0;
            let message = "Translation camera speed: ".to_string() + &translation_speed.to_string() + &"x".to_string();
            dispatch_event_with("app-event.top_message", &message.into())?;
            dispatch_event_with("app-event.translation_speed", &translation_speed.into())?;
        }
    }

    if input.reset_speeds {
        res.camera.turning_speed = TURNING_BASE_SPEED;
        res.camera.movement_speed = res.translation_base_speed;
        res.pixel_manipulation_speed = PIXEL_MANIPULATION_BASE_SPEED;
        dispatch_event_with("app-event.top_message", &"All speeds have been reset.".into())?;
        dispatch_event("app-event.speed_reset")?;
    }

    if input.walk_left { res.camera.advance(CameraDirection::Left, dt); }
    if input.walk_right { res.camera.advance(CameraDirection::Right, dt); }
    if input.walk_up { res.camera.advance(CameraDirection::Up, dt); }
    if input.walk_down { res.camera.advance(CameraDirection::Down, dt); }
    if input.walk_forward { res.camera.advance(CameraDirection::Forward, dt); }
    if input.walk_backward { res.camera.advance(CameraDirection::Backward, dt); }

    if input.turn_left { res.camera.turn(CameraDirection::Left, dt); }
    if input.turn_right { res.camera.turn(CameraDirection::Right, dt); }
    if input.turn_up { res.camera.turn(CameraDirection::Up, dt); }
    if input.turn_down { res.camera.turn(CameraDirection::Down, dt); }

    if input.rotate_left { res.camera.rotate(CameraDirection::Left, dt); }
    if input.rotate_right { res.camera.rotate(CameraDirection::Right, dt); }

    res.buttons.mouse_click.track(input.mouse_left_click);
    if res.buttons.mouse_click.is_just_pressed() {
        dispatch_event("app-event.request_pointer_lock")?;
    } else if res.buttons.mouse_click.is_activated() {
        res.camera.drag(input.mouse_position_x, input.mouse_position_y);
    } else if res.buttons.mouse_click.is_just_released() {
        dispatch_event("app-event.exit_pointer_lock")?;
    }

    if input.mouse_scroll_y != 0.0 {
        if res.camera_zoom >= 1.0 && res.camera_zoom <= 45.0 {
            res.camera_zoom -= input.mouse_scroll_y * 0.1;
        }
        if res.camera_zoom <= 1.0 {
            res.camera_zoom = 1.0;
        }
        if res.camera_zoom >= 45.0 {
            res.camera_zoom = 45.0;
        }
    }

    res.camera.update_view()?;

    let last_pixel_scale_x = res.cur_pixel_scale_x;
    if input.increase_pixel_scale_x {
        res.cur_pixel_scale_x += 0.005 * dt * res.pixel_manipulation_speed;
    }
    if input.decrease_pixel_scale_x {
        res.cur_pixel_scale_x -= 0.005 * dt * res.pixel_manipulation_speed;
    }

    if (last_pixel_scale_x - res.cur_pixel_scale_x).abs() < std::f32::EPSILON {
        if res.cur_pixel_scale_x <= 0.0 {
            res.cur_pixel_scale_x = 0.0;
        }
        let pixel_scale_x = res.cur_pixel_scale_x + 1.0;
        dispatch_event_with("app-event.change_pixel_scale_x", &pixel_scale_x.into())?;
    }

    let last_pixel_scale_y = res.cur_pixel_scale_y;
    if input.increase_pixel_scale_y {
        res.cur_pixel_scale_y += 0.005 * dt * res.pixel_manipulation_speed; 
    }
    if input.decrease_pixel_scale_y {
        res.cur_pixel_scale_y -= 0.005 * dt * res.pixel_manipulation_speed;
    }
    if (res.cur_pixel_scale_y - last_pixel_scale_y).abs() < std::f32::EPSILON {
        if res.cur_pixel_scale_y <= 0.0 {
            res.cur_pixel_scale_y = 0.0;
        }
        let pixel_scale_y = res.cur_pixel_scale_y + 1.0;
        dispatch_event_with("app-event.change_pixel_scale_y", &pixel_scale_y.into())?;
    }

    let last_pixel_gap = res.cur_pixel_gap;
    if input.increase_pixel_gap {
        res.cur_pixel_gap += 0.005 * dt * res.pixel_manipulation_speed;
    }
    if input.decrease_pixel_gap {
        res.cur_pixel_gap -= 0.005 * dt * res.pixel_manipulation_speed;
    }
    if (last_pixel_gap - res.cur_pixel_gap).abs() < std::f32::EPSILON {
        if res.cur_pixel_gap <= 0.0 {
            res.cur_pixel_gap = 0.0;
        }
        let pixel_gap = res.cur_pixel_gap + 1.0;
        dispatch_event_with("app-event.change_pixel_gap", &pixel_gap.into())?;
    }

    let last_bright = res.extra_bright;
    if input.increase_bright {
        res.extra_bright += 0.01 * dt * res.pixel_manipulation_speed;
    }
    if input.decrease_bright {
        res.extra_bright -= 0.01 * dt * res.pixel_manipulation_speed;
    }
    if input.reset_brightness {
        res.extra_bright = 0.0;
    }
    if (last_bright - res.extra_bright).abs() < std::f32::EPSILON {
        if res.extra_bright < -1.0 {
            res.extra_bright = -1.0;
        }
        if res.extra_bright > 1.0 {
            res.extra_bright = 1.0;
        }
        dispatch_event_with("app-event.change_pixel_brightness", &res.extra_bright.into())?;
    }

    Ok(true)
}

pub fn draw(gl: &WebGl2RenderingContext, res: &Resources) -> Result<()> {
    if res.animation.needs_buffer_data_load {
        res.pixels_render.apply_colors(gl, &res.animation.steps[res.animation.current_frame]);
    }

    let canvas_width = res.animation.canvas_width as f32;
    let canvas_height = res.animation.canvas_height as f32;

    let mut projection = glm::perspective::<f32>(canvas_width / canvas_height, radians(res.camera_zoom), 0.01, 10000.0);
    let mut view = res.camera.get_view();

    let mut pixel_scale : &mut [f32] = &mut [
        res.cur_pixel_scale_x + 1.0,
        res.cur_pixel_scale_y + 1.0,
        (res.cur_pixel_scale_x + res.cur_pixel_scale_x)/2.0 + 1.0
    ];

    let mut pixel_gap : &mut [f32] = &mut [1.0 + res.cur_pixel_gap, 1.0 + res.cur_pixel_gap];

    if (res.animation.scale_x - 1.0).abs() < std::f32::EPSILON {
        pixel_scale[0] /= res.animation.scale_x;
        pixel_gap[0] *= res.animation.scale_x;
    }

    let ambient_strength = match res.pixels_or_voxels { PixelsOrVoxels::Pixels => 1.0, PixelsOrVoxels::Voxels => 0.5};

    let mut light_color = get_3_f32color_from_int(res.light_color);
    let mut extra_light = get_3_f32color_from_int(res.brightness_color);

    for i in 0 .. 3 {
        extra_light[i] *= res.extra_bright;
    }

    gl.clear_color(0.05, 0.05, 0.05, 1.0);
    if res.bloom_passes > 0 {
        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, res.blur_render.framebuffers[1].as_ref());
    }
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT|WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    let camera_position = res.camera.get_position();

    gl.use_program(Some(&res.pixels_render.shader));
    gl.uniform_matrix4fv_with_f32_array(gl.get_uniform_location(&res.pixels_render.shader, "view").as_ref(), false, view.as_mut_slice());
    gl.uniform_matrix4fv_with_f32_array(gl.get_uniform_location(&res.pixels_render.shader, "projection").as_ref(), false, projection.as_mut_slice());
    gl.uniform3fv_with_f32_array(gl.get_uniform_location(&res.pixels_render.shader, "lightPos").as_ref(), &mut [camera_position.x, camera_position.y, camera_position.z]);
    gl.uniform3fv_with_f32_array(gl.get_uniform_location(&res.pixels_render.shader, "lightColor").as_ref(), &mut light_color);
    gl.uniform3fv_with_f32_array(gl.get_uniform_location(&res.pixels_render.shader, "extraLight").as_ref(), &mut extra_light);
    gl.uniform1f(gl.get_uniform_location(&res.pixels_render.shader, "ambientStrength").as_ref(), ambient_strength);
    gl.uniform2fv_with_f32_array(gl.get_uniform_location(&res.pixels_render.shader, "pixel_gap").as_ref(), &mut pixel_gap);
    gl.uniform3fv_with_f32_array(gl.get_uniform_location(&res.pixels_render.shader, "pixel_scale").as_ref(), &mut pixel_scale);
    gl.uniform1f(gl.get_uniform_location(&res.pixels_render.shader, "pixel_pulse").as_ref(), res.pixels_pulse);

    gl.bind_vertex_array(res.pixels_render.vao.as_ref());
    gl.draw_arrays_instanced(
        WebGl2RenderingContext::TRIANGLES,
        0,
        match res.pixels_or_voxels { PixelsOrVoxels::Pixels => 6, PixelsOrVoxels::Voxels => 36 },
        (res.animation.width * res.animation.height) as i32
    );

    if res.bloom_passes > 0 {
        gl.use_program(Some(&res.blur_render.shader));
        gl.bind_vertex_array(res.blur_render.vao.as_ref());
        for i in 0 ..= res.bloom_passes {
            let buffer_index = i % 2;
            let texture_index = (i + 1) % 2;

            let mut framebuffer = None;
            if i < res.bloom_passes {
                framebuffer = res.blur_render.framebuffers[buffer_index].as_ref();
            }
            gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, framebuffer);
            if i == 0 || i == res.bloom_passes {
                gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT|WebGl2RenderingContext::DEPTH_BUFFER_BIT);
            }
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, res.blur_render.textures[texture_index].as_ref());
            gl.uniform1i(gl.get_uniform_location(&res.blur_render.shader, "horizontal").as_ref(), buffer_index as i32);
            gl.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, 6, WebGl2RenderingContext::UNSIGNED_INT, 0);
        }
        gl.bind_vertex_array(None);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
    }

    check_error(&gl, line!())?;

    Ok(())
}

pub fn radians(grad: f32) -> f32 {
    let pi: f32 = glm::pi();
    grad * pi / 180.0
}

pub fn check_error(gl: &WebGl2RenderingContext, line: u32) -> Result<()> {
    let error = gl.get_error();
    if error != WebGl2RenderingContext::NO_ERROR {
        return Err(WasmError::Str(error.to_string() + " on line: " + &line.to_string()));
    }
    Ok(())
}

pub fn get_3_f32color_from_int(color: i32) -> [f32; 3] {[
    (color >> 16) as f32 / 255.0,
    ((color >> 8) & 0xFF) as f32 / 255.0,
    (color & 0xFF) as f32 / 255.0,
]}

#[cfg(test)]
mod tests { mod get_3_f32color_from_int { mod gives_good {
    use super::super::super::*;

    macro_rules! get_3_f32color_from_int_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, get_3_f32color_from_int(input));
            }
        )*
        }
    }

    get_3_f32color_from_int_tests! {
        white: (0x00FF_FFFF, [1.0, 1.0, 1.0]),
        black: (0x0000_0000, [0.0, 0.0, 0.0]),
        red: (0x00FF_0000, [1.0, 0.0, 0.0]),
        green: (0x0000_FF00, [0.0, 1.0, 0.0]),
        blue: (0x0000_00FF, [0.0, 0.0, 1.0]),
        yellow: (0x00eb_f114, [0.92156863, 0.94509804, 0.078431375]),
    }
} } }