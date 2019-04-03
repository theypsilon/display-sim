use web_sys::WebGl2RenderingContext;

use crate::app_events;
use crate::pixels_render::{PixelsGeometryKind, PixelsUniform};
use crate::simulation_state::{ColorChannels, Materials, Resources, ScreenCurvatureKind, ScreenLayeringKind, TextureInterpolation};
use crate::wasm_error::{WasmError, WasmResult};

pub fn draw(materials: &mut Materials, res: &Resources) -> WasmResult<()> {
    let gl = &materials.gl;
    gl.enable(WebGl2RenderingContext::DEPTH_TEST);
    gl.clear_color(0.0, 0.0, 0.0, 0.0);

    if res.animation.needs_buffer_data_load {
        materials.pixels_render.load_image(gl, &res.animation);
    }

    materials.main_buffer_stack.set_depthbuffer(
        gl,
        match res.crt_filters.pixels_geometry_kind {
            PixelsGeometryKind::Cubes => true,
            PixelsGeometryKind::Squares => false,
        },
    );

    let internal_width = (res.animation.viewport_width as f32 * res.crt_filters.internal_resolution.multiplier) as i32;
    let internal_height = (res.animation.viewport_height as f32 * res.crt_filters.internal_resolution.multiplier) as i32;
    materials.main_buffer_stack.set_resolution(gl, internal_width, internal_height);

    materials.main_buffer_stack.set_interpolation(
        gl,
        match res.crt_filters.texture_interpolation {
            TextureInterpolation::Linear => WebGl2RenderingContext::LINEAR,
            TextureInterpolation::Nearest => WebGl2RenderingContext::NEAREST,
        },
    );

    materials.main_buffer_stack.push(gl)?;
    materials.main_buffer_stack.push(gl)?;
    materials.main_buffer_stack.bind_current(gl)?;
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    let screen_curvature = match res.crt_filters.screen_curvature_kind {
        ScreenCurvatureKind::Curved1 => 0.15,
        ScreenCurvatureKind::Curved2 => 0.3,
        ScreenCurvatureKind::Curved3 => 0.45,
        _ => 0.0,
    };

    if res.crt_filters.showing_diffuse_foreground {
        let mut extra_light = get_3_f32color_from_int(res.crt_filters.brightness_color);
        for light in extra_light.iter_mut() {
            *light *= res.crt_filters.extra_bright;
        }
        let vertical_lines_ratio = res.crt_filters.lines_per_pixel;
        for j in 0..vertical_lines_ratio {
            let color_splits = match res.crt_filters.color_channels {
                ColorChannels::Combined => 1,
                _ => 3,
            };
            for i in 0..color_splits {
                let mut light_color = get_3_f32color_from_int(res.crt_filters.light_color);
                let pixel_offset = &mut [0.0, 0.0, 0.0];
                let pixel_scale = &mut [
                    (res.crt_filters.cur_pixel_scale_x + 1.0) / res.crt_filters.cur_pixel_width,
                    res.crt_filters.cur_pixel_scale_y + 1.0,
                    (res.crt_filters.cur_pixel_scale_x + res.crt_filters.cur_pixel_scale_x) * 0.5 + 1.0,
                ];
                match res.crt_filters.color_channels {
                    ColorChannels::Combined => {}
                    _ => {
                        light_color[(i + 0) % 3] *= 1.0;
                        light_color[(i + 1) % 3] = 0.0;
                        light_color[(i + 2) % 3] = 0.0;
                        match res.crt_filters.color_channels {
                            ColorChannels::SplitHorizontal => {
                                pixel_offset[0] = (i as f32 - 1.0) * (1.0 / 3.0) * res.crt_filters.cur_pixel_width / (res.crt_filters.cur_pixel_scale_x + 1.0);
                                pixel_scale[0] *= color_splits as f32;
                            }
                            ColorChannels::Overlapping => {
                                pixel_offset[0] = (i as f32 - 1.0) * (1.0 / 3.0) * res.crt_filters.cur_pixel_width / (res.crt_filters.cur_pixel_scale_x + 1.0);
                                pixel_scale[0] *= 1.5;
                            }
                            ColorChannels::SplitVertical => {
                                pixel_offset[1] = (i as f32 - 1.0) * (1.0 / 3.0) * (1.0 - res.crt_filters.cur_pixel_scale_y);
                                pixel_scale[1] *= color_splits as f32;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                if vertical_lines_ratio > 1 {
                    pixel_offset[0] /= vertical_lines_ratio as f32;
                    pixel_offset[0] += (j as f32 / vertical_lines_ratio as f32 - calc_stupid_not_extrapoled_function(vertical_lines_ratio)) * res.crt_filters.cur_pixel_width
                        / (res.crt_filters.cur_pixel_scale_x + 1.0);
                    pixel_scale[0] *= vertical_lines_ratio as f32;
                }
                if let ColorChannels::Overlapping = res.crt_filters.color_channels {
                    materials.main_buffer_stack.push(gl)?;
                    materials.main_buffer_stack.bind_current(gl)?;
                    if j == 0 {
                        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
                    }
                }
                materials.pixels_render.render(
                    gl,
                    PixelsUniform {
                        shadow_kind: res.crt_filters.pixel_shadow_shape_kind,
                        geometry_kind: res.crt_filters.pixels_geometry_kind,
                        view: res.camera.get_view().as_mut_slice(),
                        projection: res.camera.get_projection(res.animation.viewport_width as f32, res.animation.viewport_height as f32).as_mut_slice(),
                        ambient_strength: match res.crt_filters.pixels_geometry_kind {
                            PixelsGeometryKind::Squares => 1.0,
                            PixelsGeometryKind::Cubes => 0.5,
                        },
                        contrast_factor: res.crt_filters.extra_contrast,
                        light_color: &mut light_color,
                        extra_light: &mut extra_light,
                        light_pos: res.camera.get_position().as_mut_slice(),
                        screen_curvature,
                        pixel_gap: &mut [(1.0 + res.crt_filters.cur_pixel_gap) * res.crt_filters.cur_pixel_width, 1.0 + res.crt_filters.cur_pixel_gap],
                        pixel_scale,
                        pixel_pulse: res.crt_filters.pixels_pulse,
                        pixel_offset,
                        height_modifier_factor: 1.0 - res.crt_filters.pixel_shadow_height_factor,
                    },
                );
            }
            if let ColorChannels::Overlapping = res.crt_filters.color_channels {
                materials.main_buffer_stack.pop()?;
                materials.main_buffer_stack.pop()?;
                materials.main_buffer_stack.pop()?;
            }
        }

        if let ColorChannels::Overlapping = res.crt_filters.color_channels {
            materials.main_buffer_stack.bind_current(gl)?;
            gl.active_texture(WebGl2RenderingContext::TEXTURE0 + 0);
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, materials.main_buffer_stack.get_nth(1)?.texture());
            gl.active_texture(WebGl2RenderingContext::TEXTURE0 + 1);
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, materials.main_buffer_stack.get_nth(2)?.texture());
            gl.active_texture(WebGl2RenderingContext::TEXTURE0 + 2);
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, materials.main_buffer_stack.get_nth(3)?.texture());

            materials.rgb_render.render(gl);

            gl.active_texture(WebGl2RenderingContext::TEXTURE0 + 0);
        }
    }

    materials.main_buffer_stack.push(gl)?;
    materials.main_buffer_stack.bind_current(gl)?;
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    if res.crt_filters.showing_solid_background {
        let diffuse_condition = res.crt_filters.showing_diffuse_foreground || if let ScreenLayeringKind::DiffuseOnly = res.crt_filters.layering_kind { true } else { false };
        if diffuse_condition {
            materials.bg_buffer_stack.set_resolution(gl, 1920 / 2, 1080 / 2);
            materials.bg_buffer_stack.set_depthbuffer(gl, false);
            materials.bg_buffer_stack.set_interpolation(gl, WebGl2RenderingContext::LINEAR);
            materials.bg_buffer_stack.push(gl)?;
            materials.bg_buffer_stack.bind_current(gl)?;
            gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        }
        materials.pixels_render.render(
            gl,
            PixelsUniform {
                shadow_kind: 0,
                geometry_kind: res.crt_filters.pixels_geometry_kind,
                view: res.camera.get_view().as_mut_slice(),
                projection: res.camera.get_projection(res.animation.viewport_width as f32, res.animation.viewport_height as f32).as_mut_slice(),
                ambient_strength: match res.crt_filters.pixels_geometry_kind {
                    PixelsGeometryKind::Squares => 1.0,
                    PixelsGeometryKind::Cubes => 0.5,
                },
                contrast_factor: res.crt_filters.extra_contrast,
                light_color: &mut [res.crt_filters.solid_color_weight, res.crt_filters.solid_color_weight, res.crt_filters.solid_color_weight],
                extra_light: &mut [0.0, 0.0, 0.0],
                light_pos: res.camera.get_position().as_mut_slice(),
                pixel_gap: &mut [(1.0 + res.crt_filters.cur_pixel_gap) * res.crt_filters.cur_pixel_width, 1.0 + res.crt_filters.cur_pixel_gap],
                pixel_scale: &mut [
                    (res.crt_filters.cur_pixel_scale_x + 1.0) / res.crt_filters.cur_pixel_width,
                    res.crt_filters.cur_pixel_scale_y + 1.0,
                    (res.crt_filters.cur_pixel_scale_x + res.crt_filters.cur_pixel_scale_x) * 0.5 + 1.0,
                ],
                screen_curvature,
                pixel_pulse: res.crt_filters.pixels_pulse,
                pixel_offset: &mut [0.0, 0.0, 0.0],
                height_modifier_factor: 0.0,
            },
        );
        if diffuse_condition {
            let source = materials.bg_buffer_stack.get_current()?.clone();
            let target = materials.main_buffer_stack.get_current()?;
            materials.blur_render.render(&gl, &mut materials.bg_buffer_stack, &source, &target, 6)?;
            materials.bg_buffer_stack.pop()?;
        }
    }
    materials.main_buffer_stack.pop()?;
    materials.main_buffer_stack.pop()?;
    materials.main_buffer_stack.bind_current(gl)?;
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    gl.active_texture(WebGl2RenderingContext::TEXTURE0 + 0);
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, materials.main_buffer_stack.get_nth(1)?.texture());
    gl.active_texture(WebGl2RenderingContext::TEXTURE0 + 1);
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, materials.main_buffer_stack.get_nth(2)?.texture());
    materials.background_render.render(gl);
    gl.active_texture(WebGl2RenderingContext::TEXTURE0 + 0);

    if res.crt_filters.blur_passes > 0 {
        let target = materials.main_buffer_stack.get_current()?.clone();
        materials.blur_render.render(&gl, &mut materials.main_buffer_stack, &target, &target, res.crt_filters.blur_passes)?;
    }

    if res.launch_screenshot {
        let multiplier: f32 = res.crt_filters.internal_resolution.multiplier;
        let width = (res.animation.viewport_width as f32 * multiplier) as i32;
        let height = (res.animation.viewport_height as f32 * multiplier) as i32;
        let pixels = js_sys::Uint8Array::new(&(width * height * 4).into());
        gl.read_pixels_with_opt_array_buffer_view(0, 0, width, height, WebGl2RenderingContext::RGBA, WebGl2RenderingContext::UNSIGNED_BYTE, Some(&pixels))?;
        let array = js_sys::Array::new();
        array.push(&pixels);
        array.push(&multiplier.into());
        app_events::dispatch_screenshot(&array)?;
    }

    materials.main_buffer_stack.pop()?;
    materials.main_buffer_stack.assert_no_stack()?;

    gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
    gl.viewport(0, 0, res.animation.viewport_width as i32, res.animation.viewport_height as i32);

    materials.internal_resolution_render.render(gl, materials.main_buffer_stack.get_nth(1)?.texture());

    check_error(&gl, line!())?;

    Ok(())
}

fn check_error(gl: &WebGl2RenderingContext, line: u32) -> WasmResult<()> {
    let error = gl.get_error();
    if error != WebGl2RenderingContext::NO_ERROR {
        return Err(WasmError::Str(error.to_string() + " on line: " + &line.to_string()));
    }
    Ok(())
}

fn get_3_f32color_from_int(color: i32) -> [f32; 3] {
    [(color >> 16) as f32 / 255.0, ((color >> 8) & 0xFF) as f32 / 255.0, (color & 0xFF) as f32 / 255.0]
}

fn calc_stupid_not_extrapoled_function(y: usize) -> f32 {
    match y {
        1 => (0.0),
        2 => (0.25),
        3 => (1.0 / 3.0),
        4 => (0.375),
        5 => (0.4),
        6 => (0.4 + 0.1 / 6.0),
        7 => (0.4 + 0.1 / 6.0 + 0.1 / 8.4),
        8 => (0.4 + 0.1 / 6.0 + 0.1 / 8.4 + 0.008_925_75),
        9 => (0.4 + 0.1 / 6.0 + 0.1 / 8.4 + 0.008_925_75 + 0.006_945),
        _ => (0.45), // originalmente: 0.4 + 0.1 / 6.0 + 0.1 / 8.4 + 0.00892575 + 0.006945 + 0.0055555555
    }
    /*
    Let's consider this was a function where we find the following points:
    f(1) = 0
    0.25
    f(2) = 0.25
    0.08333333333 | 0.33333
    f(3) = 0.33333333333
    0.0416666666 | 0.5
    f(4) = 0.375
    0.025 | 0.6
    f(5) = 0.4
    0.0166666666666 | 0.6666666666
    f(6) = 0.41666666666
    0.01190476190475190476190 | 0.71428571424028571
    f(7) = 0.42857142857
    0.00892575 | 0.749763
    f(8) = 0.43749717857142857142857142857143
    0.006945 | 0.77808587513
    f(9) = 0.444442178571428571428
    0.00555555555555555555555 | 0.79999
    f(10) = 0.45

    It looks like this function is growing less than a logarithmic one
    */
}

#[cfg(test)]
mod tests {
    mod get_3_f32color_from_int {
        mod gives_good {
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
        }
    }
}
