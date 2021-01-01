/* Copyright (c) 2019-2021 José manuel Barroso Galindo <theypsilon@gmail.com>
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
    pub width: i32,
    pub height: i32,
}

impl<GL: HasContext> std::clone::Clone for TextureBuffer<GL> {
    fn clone(&self) -> Self {
        TextureBuffer {
            texture: self.texture,
            framebuffer: self.framebuffer,
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
            width,
            height,
        })
    }

    fn new_with_depthbuffer(gl: &GlowSafeAdapter<GL>, width: i32, height: i32, interpolation: u32) -> AppResult<TextureBuffer<GL>> {
        let depthbuffer = Some(gl.create_renderbuffer()?);
        let texture_buffer = Self::new(gl, width, height, interpolation)?;
        gl.bind_renderbuffer(glow::RENDERBUFFER, depthbuffer);
        gl.renderbuffer_storage(glow::RENDERBUFFER, glow::DEPTH_COMPONENT16, width, height);
        gl.framebuffer_renderbuffer(glow::FRAMEBUFFER, glow::DEPTH_ATTACHMENT, glow::RENDERBUFFER, depthbuffer);
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
    depthbuffer_active: bool,
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
            depthbuffer_active: false,
            gl,
        }
    }

    pub fn set_depthbuffer(&mut self, new_value: bool) -> AppResult<()> {
        if self.depthbuffer_active != new_value {
            self.depthbuffer_active = new_value;
            self.reset_stack()?;
        }
        Ok(())
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
        for tb in self.stack.iter() {
            self.gl
                .delete_framebuffer(tb.framebuffer().ok_or_else(|| Into::<String>::into("can't access framebuffer"))?);
            self.gl
                .delete_texture(tb.texture().ok_or_else(|| Into::<String>::into("can't access texture"))?);
        }
        self.stack.clear();
        Ok(())
    }

    pub fn push(&mut self) -> AppResult<()> {
        if self.stack.len() == self.cursor {
            let tb = if self.depthbuffer_active {
                TextureBuffer::new_with_depthbuffer(&*self.gl, self.width, self.height, self.interpolation)?
            } else {
                TextureBuffer::new(&*self.gl, self.width, self.height, self.interpolation)?
            };
            self.stack.push(tb);
        }
        self.cursor += 1;
        if self.cursor > self.max_cursor {
            self.max_cursor = self.cursor;
        }
        Ok(())
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

    pub fn assert_no_stack(&self) -> AppResult<()> {
        if self.cursor != 0 {
            return Err(format!("Texture buffer stack cursor not zero, '{}' instead.", self.cursor).into());
        }
        Ok(())
    }
}
