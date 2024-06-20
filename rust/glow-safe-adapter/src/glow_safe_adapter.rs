/* Copyright (c) 2019-2024 José manuel Barroso Galindo <theypsilon@gmail.com>
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

use glow::*;

pub struct GlowSafeAdapter<GL: HasContext> {
    gl: GL,
}

impl<GL: HasContext> GlowSafeAdapter<GL> {
    pub fn new(gl: GL) -> Self {
        GlowSafeAdapter { gl }
    }

    pub fn enable(&self, parameter: u32) {
        unsafe { self.gl.enable(parameter) }
    }

    pub fn enable_vertex_attrib_array(&self, index: Option<u32>) {
        unsafe { self.gl.enable_vertex_attrib_array(index.unwrap()) }
    }

    pub fn create_framebuffer(&self) -> Result<GL::Framebuffer, String> {
        unsafe { self.gl.create_framebuffer() }
    }

    pub fn create_renderbuffer(&self) -> Result<GL::Renderbuffer, String> {
        unsafe { self.gl.create_renderbuffer() }
    }

    pub fn create_sampler(&self) -> Result<GL::Sampler, String> {
        unsafe { self.gl.create_sampler() }
    }

    pub fn create_shader(&self, shader_type: u32) -> Result<GL::Shader, String> {
        unsafe { self.gl.create_shader(shader_type) }
    }

    pub fn create_texture(&self) -> Result<GL::Texture, String> {
        unsafe { self.gl.create_texture() }
    }

    pub fn delete_shader(&self, shader: GL::Shader) {
        unsafe { self.gl.delete_shader(shader) }
    }

    pub fn shader_source(&self, shader: GL::Shader, source: &str) {
        unsafe { self.gl.shader_source(shader, source) }
    }

    pub fn compile_shader(&self, shader: GL::Shader) {
        unsafe { self.gl.compile_shader(shader) }
    }

    pub fn get_shader_compile_status(&self, shader: GL::Shader) -> bool {
        unsafe { self.gl.get_shader_compile_status(shader) }
    }

    pub fn get_shader_info_log(&self, shader: GL::Shader) -> String {
        unsafe { self.gl.get_shader_info_log(shader) }
    }

    pub fn create_program(&self) -> Result<GL::Program, String> {
        unsafe { self.gl.create_program() }
    }

    pub fn delete_program(&self, program: GL::Program) {
        unsafe { self.gl.delete_program(program) }
    }

    pub fn attach_shader(&self, program: GL::Program, shader: GL::Shader) {
        unsafe { self.gl.attach_shader(program, shader) }
    }

    pub fn detach_shader(&self, program: GL::Program, shader: GL::Shader) {
        unsafe { self.gl.detach_shader(program, shader) }
    }

    pub fn link_program(&self, program: GL::Program) {
        unsafe { self.gl.link_program(program) }
    }

    pub fn get_program_link_status(&self, program: GL::Program) -> bool {
        unsafe { self.gl.get_program_link_status(program) }
    }

    pub fn get_program_info_log(&self, program: GL::Program) -> String {
        unsafe { self.gl.get_program_info_log(program) }
    }

    pub fn get_active_uniforms(&self, program: GL::Program) -> u32 {
        unsafe { self.gl.get_active_uniforms(program) }
    }

    pub fn use_program(&self, program: Option<GL::Program>) {
        unsafe { self.gl.use_program(program) }
    }

    pub fn create_buffer(&self) -> Result<GL::Buffer, String> {
        unsafe { self.gl.create_buffer() }
    }

    pub fn bind_buffer(&self, target: u32, buffer: Option<GL::Buffer>) {
        unsafe { self.gl.bind_buffer(target, buffer) }
    }

    pub fn bind_framebuffer(&self, target: u32, framebuffer: Option<GL::Framebuffer>) {
        unsafe { self.gl.bind_framebuffer(target, framebuffer) }
    }

    pub fn bind_renderbuffer(&self, target: u32, renderbuffer: Option<GL::Renderbuffer>) {
        unsafe { self.gl.bind_renderbuffer(target, renderbuffer) }
    }

    pub fn create_vertex_array(&self) -> Result<GL::VertexArray, String> {
        unsafe { self.gl.create_vertex_array() }
    }

    pub fn delete_vertex_array(&self, vertex_array: GL::VertexArray) {
        unsafe { self.gl.delete_vertex_array(vertex_array) }
    }

    pub fn bind_vertex_array(&self, vertex_array: Option<GL::VertexArray>) {
        unsafe { self.gl.bind_vertex_array(vertex_array) }
    }

    pub fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe { self.gl.clear_color(red, green, blue, alpha) }
    }

    pub fn clear_depth_f64(&self, depth: f64) {
        unsafe { self.gl.clear_depth_f64(depth) }
    }

    pub fn clear_depth_f32(&self, depth: f32) {
        unsafe { self.gl.clear_depth_f32(depth) }
    }

    pub fn clear_stencil(&self, stencil: i32) {
        unsafe { self.gl.clear_stencil(stencil) }
    }

    pub fn clear(&self, mask: u32) {
        unsafe { self.gl.clear(mask) }
    }

    pub fn patch_parameter_i32(&self, parameter: u32, value: i32) {
        unsafe { self.gl.patch_parameter_i32(parameter, value) }
    }

    pub fn buffer_data_u8_slice(&self, target: u32, data: &[u8], usage: u32) {
        unsafe { self.gl.buffer_data_u8_slice(target, data, usage) }
    }

    pub fn buffer_storage(&self, target: u32, size: i32, data: Option<&mut [u8]>, flags: u32) {
        unsafe { self.gl.buffer_storage(target, size, data.as_deref(), flags) }
    }

    pub fn delete_framebuffer(&self, framebuffer: GL::Framebuffer) {
        unsafe { self.gl.delete_framebuffer(framebuffer) }
    }

    pub fn delete_texture(&self, texture: GL::Texture) {
        unsafe { self.gl.delete_texture(texture) }
    }

    pub fn draw_arrays_instanced(&self, mode: u32, first: i32, count: i32, instance_count: i32) {
        unsafe { self.gl.draw_arrays_instanced(mode, first, count, instance_count) }
    }

    pub fn draw_elements(&self, mode: u32, count: i32, element_type: u32, offset: i32) {
        unsafe { self.gl.draw_elements(mode, count, element_type, offset) }
    }

    pub fn flush(&self) {
        unsafe { self.gl.flush() }
    }

    pub fn framebuffer_renderbuffer(&self, target: u32, attachment: u32, renderbuffer_target: u32, renderbuffer: Option<GL::Renderbuffer>) {
        unsafe { self.gl.framebuffer_renderbuffer(target, attachment, renderbuffer_target, renderbuffer) }
    }

    pub fn framebuffer_texture(&self, target: u32, attachment: u32, texture: Option<GL::Texture>, level: i32) {
        unsafe { self.gl.framebuffer_texture(target, attachment, texture, level) }
    }

    pub fn framebuffer_texture_2d(&self, target: u32, attachment: u32, texture_target: u32, texture: Option<GL::Texture>, level: i32) {
        unsafe { self.gl.framebuffer_texture_2d(target, attachment, texture_target, texture, level) }
    }

    pub fn get_error(&self) -> u32 {
        unsafe { self.gl.get_error() }
    }

    pub fn get_uniform_location(&self, program: GL::Program, name: &str) -> Option<GL::UniformLocation> {
        unsafe { self.gl.get_uniform_location(program, name) }
    }

    pub fn get_attrib_location(&self, program: GL::Program, name: &str) -> Option<u32> {
        unsafe { self.gl.get_attrib_location(program, name) }
    }

    pub fn get_active_attributes(&self, program: GL::Program) -> u32 {
        unsafe { self.gl.get_active_attributes(program) }
    }

    pub fn get_active_attribute(&self, program: GL::Program, index: u32) -> Option<ActiveAttribute> {
        unsafe { self.gl.get_active_attribute(program, index) }
    }

    pub fn renderbuffer_storage(&self, target: u32, internal_format: u32, width: i32, height: i32) {
        unsafe { self.gl.renderbuffer_storage(target, internal_format, width, height) }
    }

    pub fn tex_image_2d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    ) {
        unsafe { self.gl.tex_image_2d(target, level, internal_format, width, height, border, format, ty, pixels) }
    }

    pub fn uniform_1_i32(&self, location: Option<GL::UniformLocation>, x: i32) {
        unsafe { self.gl.uniform_1_i32(location.as_ref(), x) }
    }

    pub fn uniform_2_i32(&self, location: Option<GL::UniformLocation>, x: i32, y: i32) {
        unsafe { self.gl.uniform_2_i32(location.as_ref(), x, y) }
    }

    pub fn uniform_3_i32(&self, location: Option<GL::UniformLocation>, x: i32, y: i32, z: i32) {
        unsafe { self.gl.uniform_3_i32(location.as_ref(), x, y, z) }
    }

    pub fn uniform_1_f32(&self, location: Option<GL::UniformLocation>, x: f32) {
        unsafe { self.gl.uniform_1_f32(location.as_ref(), x) }
    }

    pub fn uniform_2_f32_slice(&self, location: Option<GL::UniformLocation>, v: &[f32; 2]) {
        unsafe { self.gl.uniform_2_f32_slice(location.as_ref(), v) }
    }

    pub fn uniform_3_f32_slice(&self, location: Option<GL::UniformLocation>, v: &[f32; 3]) {
        unsafe { self.gl.uniform_3_f32_slice(location.as_ref(), v) }
    }

    pub fn uniform_matrix_4_f32_slice(&self, location: Option<GL::UniformLocation>, transpose: bool, v: &[f32; 16]) {
        unsafe { self.gl.uniform_matrix_4_f32_slice(location.as_ref(), transpose, v) }
    }

    pub fn finish(&self) {
        unsafe { self.gl.finish() }
    }

    pub fn bind_texture(&self, target: u32, texture: Option<GL::Texture>) {
        unsafe { self.gl.bind_texture(target, texture) }
    }

    pub fn active_texture(&self, unit: u32) {
        unsafe { self.gl.active_texture(unit) }
    }

    pub fn tex_parameter_i32(&self, target: u32, parameter: u32, value: i32) {
        unsafe { self.gl.tex_parameter_i32(target, parameter, value) }
    }

    pub fn vertex_attrib_divisor(&self, index: Option<u32>, divisor: u32) {
        unsafe { self.gl.vertex_attrib_divisor(index.unwrap(), divisor) }
    }

    pub fn vertex_attrib_pointer_f32(&self, index: Option<u32>, size: i32, data_type: u32, normalized: bool, stride: i32, offset: i32) {
        unsafe { self.gl.vertex_attrib_pointer_f32(index.unwrap(), size, data_type, normalized, stride, offset) }
    }

    pub fn vertex_attrib_pointer_i32(&self, index: Option<u32>, size: i32, data_type: u32, stride: i32, offset: i32) {
        unsafe { self.gl.vertex_attrib_pointer_i32(index.unwrap(), size, data_type, stride, offset) }
    }

    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { self.gl.viewport(x, y, width, height) }
    }

    pub fn pop_debug_group(&self) {
        unsafe { self.gl.pop_debug_group() }
    }

    pub fn get_uniform_block_index(&self, program: GL::Program, name: &str) -> Option<u32> {
        unsafe { self.gl.get_uniform_block_index(program, name) }
    }
}
