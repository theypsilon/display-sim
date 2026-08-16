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

use crate::error::AppResult;
use glow::GlowSafeAdapter;
use glow::HasContext;
use std::rc::Rc;

#[derive(Debug, Copy)]
pub struct TextureBuffer<GL: HasContext> {
    texture: Option<GL::Texture>,
    framebuffer: Option<GL::Framebuffer>,
    depthbuffer: Option<GL::Renderbuffer>,
    pub width: i32,
    pub height: i32,
}

impl<GL: HasContext> std::clone::Clone for TextureBuffer<GL> {
    fn clone(&self) -> Self {
        TextureBuffer {
            texture: self.texture,
            framebuffer: self.framebuffer,
            depthbuffer: self.depthbuffer,
            width: self.width,
            height: self.height,
        }
    }
}

impl<GL: HasContext> TextureBuffer<GL> {
    fn new(gl: &GlowSafeAdapter<GL>, width: i32, height: i32, interpolation: u32) -> AppResult<TextureBuffer<GL>> {
        let framebuffer = Some(gl.create_framebuffer()?);
        gl.bind_framebuffer(glow::FRAMEBUFFER, framebuffer);

        let texture = Some(gl.create_texture()?);
        gl.bind_texture(glow::TEXTURE_2D, texture);

        gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA as i32, width, height, 0, glow::RGBA, glow::UNSIGNED_BYTE, None);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, interpolation as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, interpolation as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
        gl.framebuffer_texture_2d(glow::FRAMEBUFFER, glow::COLOR_ATTACHMENT0, glow::TEXTURE_2D, texture, 0);

        Ok(TextureBuffer {
            texture,
            framebuffer,
            depthbuffer: None,
            width,
            height,
        })
    }

    fn new_with_depthbuffer(gl: &GlowSafeAdapter<GL>, width: i32, height: i32, interpolation: u32) -> AppResult<TextureBuffer<GL>> {
        let depthbuffer = Some(gl.create_renderbuffer()?);
        let mut texture_buffer = Self::new(gl, width, height, interpolation)?;
        gl.bind_renderbuffer(glow::RENDERBUFFER, depthbuffer);
        // The flight camera can be thousands of world units from one-unit
        // pixel cubes. Sixteen depth bits cannot distinguish their faces.
        gl.renderbuffer_storage(glow::RENDERBUFFER, glow::DEPTH_COMPONENT24, width, height);
        gl.framebuffer_renderbuffer(glow::FRAMEBUFFER, glow::DEPTH_ATTACHMENT, glow::RENDERBUFFER, depthbuffer);
        texture_buffer.depthbuffer = depthbuffer;
        Ok(texture_buffer)
    }

    pub fn texture(&self) -> Option<GL::Texture> {
        self.texture
    }

    pub fn framebuffer(&self) -> Option<GL::Framebuffer> {
        self.framebuffer
    }
}

pub struct TextureBufferStack<GL: HasContext> {
    pub stack: Vec<TextureBuffer<GL>>,
    width: i32,
    height: i32,
    interpolation: u32,
    cursor: usize,
    max_cursor: usize,
    cycle_depth_requirements: Vec<bool>,
    gl: Rc<GlowSafeAdapter<GL>>,
}

impl<GL: HasContext> TextureBufferStack<GL> {
    pub fn new(gl: Rc<GlowSafeAdapter<GL>>) -> TextureBufferStack<GL> {
        TextureBufferStack {
            stack: vec![],
            width: 800,
            height: 600,
            interpolation: glow::LINEAR,
            cursor: 0,
            max_cursor: 0,
            cycle_depth_requirements: vec![],
            gl,
        }
    }

    pub fn set_resolution(&mut self, width: i32, height: i32) -> AppResult<()> {
        if width <= 0 || height <= 0 {
            return Ok(());
        }
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.reset_stack()?;
        }
        Ok(())
    }

    pub fn set_interpolation(&mut self, interpolation: u32) -> AppResult<()> {
        if self.interpolation != interpolation {
            self.interpolation = interpolation;
            self.reset_stack()?;
        }
        Ok(())
    }

    fn reset_stack(&mut self) -> AppResult<()> {
        self.cursor = 0;
        self.max_cursor = 0;
        self.cycle_depth_requirements.clear();
        for tb in std::mem::take(&mut self.stack) {
            self.delete_texture_buffer(tb)?;
        }
        Ok(())
    }

    pub fn push(&mut self) -> AppResult<()> {
        self.push_with_depth(false)
    }

    pub fn push_with_depth(&mut self, depth_required: bool) -> AppResult<()> {
        let index = self.cursor;
        let requirements_len = self.cycle_depth_requirements.len();
        let depth_required = if index < requirements_len {
            let previous_requirement = &mut self.cycle_depth_requirements[index];
            *previous_requirement |= depth_required;
            *previous_requirement
        } else if index == requirements_len {
            self.cycle_depth_requirements.push(depth_required);
            depth_required
        } else {
            return Err(format!("Bad texture buffer depth requirement index == {}.", index).into());
        };

        let buffer_has_depth = self.stack.get(index).map(|buffer| buffer.depthbuffer.is_some());
        if buffer_has_depth != Some(depth_required) {
            let replacement = self.create_texture_buffer(depth_required)?;
            if index == self.stack.len() {
                self.stack.push(replacement);
            } else if index < self.stack.len() {
                let previous = std::mem::replace(&mut self.stack[index], replacement);
                self.delete_texture_buffer(previous)?;
            } else {
                self.delete_texture_buffer(replacement)?;
                return Err(format!("Bad texture buffer stack allocation index == {}.", index).into());
            }
        }

        self.cursor += 1;
        if self.cursor > self.max_cursor {
            self.max_cursor = self.cursor;
        }
        Ok(())
    }

    pub fn clear(&mut self) -> AppResult<()> {
        if self.cursor != 0 {
            return Err(format!("Cannot clear a texture buffer stack with cursor at {}.", self.cursor).into());
        }
        self.reset_stack()
    }

    pub fn pop(&mut self) -> AppResult<()> {
        self.get_current()?;
        self.cursor -= 1;
        Ok(())
    }

    pub fn bind_current(&self) -> AppResult<()> {
        let current = self.get_current()?;
        self.gl.bind_framebuffer(glow::FRAMEBUFFER, current.framebuffer());
        self.gl.viewport(0, 0, self.width, self.height);
        Ok(())
    }

    pub fn get_current(&self) -> AppResult<&TextureBuffer<GL>> {
        if self.cursor == 0 {
            return Err("Bad texture buffer stack access on cursor == 0.".into());
        }
        Ok(&self.stack[self.cursor - 1])
    }

    pub fn get_nth(&self, n: i32) -> AppResult<&TextureBuffer<GL>> {
        let index = self.cursor as i32 + n - 1;
        if index < 0 || index >= self.stack.len() as i32 {
            return Err(format!("Bad texture buffer sttack access on index == {}", index).into());
        }
        Ok(&self.stack[index as usize])
    }

    pub fn assert_no_stack(&mut self) -> AppResult<()> {
        if self.cursor != 0 {
            return Err(format!("Texture buffer stack cursor not zero, '{}' instead.", self.cursor).into());
        }

        while self.stack.len() > self.max_cursor {
            let unused = self.stack.pop().ok_or_else(|| Into::<String>::into("can't access unused texture buffer"))?;
            self.delete_texture_buffer(unused)?;
        }
        self.max_cursor = 0;
        self.cycle_depth_requirements.clear();
        Ok(())
    }

    fn create_texture_buffer(&self, with_depth: bool) -> AppResult<TextureBuffer<GL>> {
        if with_depth {
            TextureBuffer::new_with_depthbuffer(&*self.gl, self.width, self.height, self.interpolation)
        } else {
            TextureBuffer::new(&*self.gl, self.width, self.height, self.interpolation)
        }
    }

    fn delete_texture_buffer(&self, texture_buffer: TextureBuffer<GL>) -> AppResult<()> {
        self.gl
            .delete_framebuffer(texture_buffer.framebuffer().ok_or_else(|| Into::<String>::into("can't access framebuffer"))?);
        self.gl
            .delete_texture(texture_buffer.texture().ok_or_else(|| Into::<String>::into("can't access texture"))?);
        if let Some(depthbuffer) = texture_buffer.depthbuffer {
            self.gl.delete_renderbuffer(depthbuffer);
        }
        Ok(())
    }
}
