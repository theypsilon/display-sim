use web_sys::WebGl2RenderingContext;

use crate::app_events;
use crate::pixels_render::PixelsUniform;
use crate::simulation_state::{ColorChannels, Materials, Resources, TextureInterpolation};
use crate::wasm_error::{WasmError, WasmResult};

pub fn draw(materials: &mut Materials, res: &Resources) -> WasmResult<()> {
    let gl = &materials.gl;

    if res.video.needs_buffer_data_load {
        materials.pixels_render.load_image(gl, &res.video);
    }

    materials.main_buffer_stack.set_depthbuffer(gl, res.output.pixel_have_depth);

    materials
        .main_buffer_stack
        .set_resolution(gl, res.filters.internal_resolution.width(), res.filters.internal_resolution.height());

    materials.main_buffer_stack.set_interpolation(
        gl,
        match res.filters.texture_interpolation {
            TextureInterpolation::Linear => WebGl2RenderingContext::LINEAR,
            TextureInterpolation::Nearest => WebGl2RenderingContext::NEAREST,
        },
    );

    materials.main_buffer_stack.push(gl)?;
    materials.main_buffer_stack.push(gl)?;
    materials.main_buffer_stack.bind_current(gl)?;

    gl.enable(WebGl2RenderingContext::DEPTH_TEST);
    gl.clear_color(0.0, 0.0, 0.0, 0.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    let view = res.camera.get_view();
    let projection = res
        .camera
        .get_projection(res.video.viewport_size.width as f32, res.video.viewport_size.height as f32);

    if res.output.showing_foreground {
        for j in 0..res.filters.lines_per_pixel {
            for i in 0..res.output.color_splits {
                if let ColorChannels::Overlapping = res.filters.color_channels {
                    materials.main_buffer_stack.push(gl)?;
                    materials.main_buffer_stack.bind_current(gl)?;
                    if j == 0 {
                        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
                    }
                }
                materials.pixels_render.render(
                    gl,
                    PixelsUniform {
                        shadow_kind: res.filters.pixel_shadow_shape_kind,
                        geometry_kind: res.filters.pixels_geometry_kind,
                        view: view.as_slice(),
                        projection: projection.as_slice(),
                        ambient_strength: res.output.ambient_strength,
                        contrast_factor: res.filters.extra_contrast,
                        light_color: &res.output.light_color[i],
                        extra_light: &res.output.extra_light,
                        light_pos: res.camera.get_position().as_slice(),
                        screen_curvature: res.output.screen_curvature_factor,
                        pixel_gap: &res.output.pixel_gap,
                        pixel_scale: &res.output.pixel_scale_foreground.get(j).expect("Bad pixel_scale_foreground")[i],
                        pixel_pulse: res.output.pixels_pulse,
                        pixel_offset: &res.output.pixel_offset_foreground.get(j).expect("Bad pixel_offset_foreground")[i],
                        height_modifier_factor: res.output.height_modifier_factor,
                    },
                );
            }
            if let ColorChannels::Overlapping = res.filters.color_channels {
                materials.main_buffer_stack.pop()?;
                materials.main_buffer_stack.pop()?;
                materials.main_buffer_stack.pop()?;
            }
        }

        if let ColorChannels::Overlapping = res.filters.color_channels {
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

    if res.output.showing_background {
        if res.output.is_background_diffuse {
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
                geometry_kind: res.filters.pixels_geometry_kind,
                view: view.as_slice(),
                projection: projection.as_slice(),
                ambient_strength: res.output.ambient_strength,
                contrast_factor: res.filters.extra_contrast,
                light_color: &[res.output.solid_color_weight, res.output.solid_color_weight, res.output.solid_color_weight],
                extra_light: &[0.0, 0.0, 0.0],
                light_pos: res.camera.get_position().as_slice(),
                pixel_gap: &res.output.pixel_gap,
                pixel_scale: &res.output.pixel_scale_base,
                screen_curvature: res.output.screen_curvature_factor,
                pixel_pulse: res.output.pixels_pulse,
                pixel_offset: &[0.0, 0.0, 0.0],
                height_modifier_factor: 0.0,
            },
        );
        if res.output.is_background_diffuse {
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

    if res.filters.blur_passes > 0 {
        let target = materials.main_buffer_stack.get_current()?.clone();
        materials
            .blur_render
            .render(&gl, &mut materials.main_buffer_stack, &target, &target, res.filters.blur_passes)?;
    }

    if res.launch_screenshot {
        let width = res.filters.internal_resolution.width();
        let height = res.filters.internal_resolution.height();
        let pixels = js_sys::Uint8Array::new(&(width * height * 4).into());
        gl.read_pixels_with_opt_array_buffer_view(
            0,
            0,
            width,
            height,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            Some(&pixels),
        )?;
        let array = js_sys::Array::new();
        array.push(&pixels);
        array.push(&res.filters.internal_resolution.multiplier.into());
        app_events::dispatch_screenshot(&array)?;
    }

    materials.main_buffer_stack.pop()?;
    materials.main_buffer_stack.assert_no_stack()?;

    gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
    gl.viewport(0, 0, res.video.viewport_size.width as i32, res.video.viewport_size.height as i32);

    materials
        .internal_resolution_render
        .render(gl, materials.main_buffer_stack.get_nth(1)?.texture());

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
