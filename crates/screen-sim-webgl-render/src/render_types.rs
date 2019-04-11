use crate::error::WebResult;
use crate::web::{WebGl2RenderingContext, WebGlFramebuffer, WebGlTexture};

#[derive(Debug, Clone)]
pub struct TextureBuffer {
    texture: Option<WebGlTexture>,
    framebuffer: Option<WebGlFramebuffer>,
    pub width: i32,
    pub height: i32,
}

impl TextureBuffer {
    pub fn new(gl: &WebGl2RenderingContext, width: i32, height: i32, interpolation: u32) -> WebResult<TextureBuffer> {
        let framebuffer = gl.create_framebuffer();
        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, framebuffer.as_ref());

        let texture = gl.create_texture();
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());
        
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            width,
            height,
            0,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            None,
        )?;
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            interpolation as i32,
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            interpolation as i32,
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        gl.framebuffer_texture_2d(
            WebGl2RenderingContext::FRAMEBUFFER,
            WebGl2RenderingContext::COLOR_ATTACHMENT0,
            WebGl2RenderingContext::TEXTURE_2D,
            texture.as_ref(),
            0,
        );

        Ok(TextureBuffer {
            texture,
            framebuffer,
            width,
            height,
        })
    }

    pub fn new_with_depthbuffer(gl: &WebGl2RenderingContext, width: i32, height: i32, interpolation: u32) -> WebResult<TextureBuffer> {
        let depthbuffer = gl.create_renderbuffer();
        let texture_buffer = Self::new(gl, width, height, interpolation)?;
        gl.bind_renderbuffer(WebGl2RenderingContext::RENDERBUFFER, depthbuffer.as_ref());
        gl.renderbuffer_storage(WebGl2RenderingContext::RENDERBUFFER, WebGl2RenderingContext::DEPTH_COMPONENT16, width, height);
        gl.framebuffer_renderbuffer(
            WebGl2RenderingContext::FRAMEBUFFER,
            WebGl2RenderingContext::DEPTH_ATTACHMENT,
            WebGl2RenderingContext::RENDERBUFFER,
            depthbuffer.as_ref(),
        );
        Ok(texture_buffer)
    }

    pub fn texture(&self) -> Option<&WebGlTexture> {
        self.texture.as_ref()
    }

    pub fn framebuffer(&self) -> Option<&WebGlFramebuffer> {
        self.framebuffer.as_ref()
    }
}

pub struct TextureBufferStack {
    pub stack: Vec<TextureBuffer>,
    width: i32,
    height: i32,
    interpolation: u32,
    cursor: usize,
    max_cursor: usize,
    depthbuffer_active: bool,
    gl: WebGl2RenderingContext,
}

impl TextureBufferStack {
    pub fn new(gl: &WebGl2RenderingContext) -> TextureBufferStack {
        TextureBufferStack {
            stack: Vec::<TextureBuffer>::default(),
            width: i32::default(),
            height: i32::default(),
            interpolation: u32::default(),
            cursor: usize::default(),
            max_cursor: usize::default(),
            depthbuffer_active: bool::default(),
            gl: gl.clone(),
        }
    }

    pub fn set_depthbuffer(&mut self, new_value: bool) {
        if self.depthbuffer_active != new_value {
            self.depthbuffer_active = new_value;
            self.reset_stack();
        }
    }

    pub fn set_resolution(&mut self, width: i32, height: i32) {
        if width <= 0 || height <= 0 {
            return;
        }
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.reset_stack();
        }
    }

    pub fn set_interpolation(&mut self, interpolation: u32) {
        if self.interpolation != interpolation {
            self.interpolation = interpolation;
            self.reset_stack();
        }
    }

    fn reset_stack(&mut self) {
        self.cursor = 0;
        self.max_cursor = 0;
        for tb in self.stack.iter() {
            self.gl.delete_framebuffer(tb.framebuffer());
            self.gl.delete_texture(tb.texture());
        }
        self.stack.clear();
    }

    pub fn push(&mut self) -> WebResult<()> {
        if self.stack.len() == self.cursor {
            let tb = if self.depthbuffer_active {
                TextureBuffer::new_with_depthbuffer(&self.gl, self.width, self.height, self.interpolation)?
            } else {
                TextureBuffer::new(&self.gl, self.width, self.height, self.interpolation)?
            };
            self.stack.push(tb);
        }
        self.cursor += 1;
        if self.cursor > self.max_cursor {
            self.max_cursor = self.cursor;
        }
        Ok(())
    }

    pub fn pop(&mut self) -> WebResult<()> {
        self.get_current()?;
        self.cursor -= 1;
        Ok(())
    }

    pub fn bind_current(&self) -> WebResult<()> {
        let current = self.get_current()?;
        self.gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, current.framebuffer());
        self.gl.viewport(0, 0, self.width, self.height);
        Ok(())
    }

    pub fn get_current(&self) -> WebResult<&TextureBuffer> {
        if self.cursor == 0 {
            return Err("Bad texture buffer stack access on cursor == 0.".into());
        }
        Ok(&self.stack[self.cursor - 1])
    }

    pub fn get_nth(&self, n: i32) -> WebResult<&TextureBuffer> {
        let index = self.cursor as i32 + n - 1;
        if index < 0 || index >= self.stack.len() as i32 {
            return Err(format!("Bad texture buffer sttack access on index == {}", index).into());
        }
        Ok(&self.stack[index as usize])
    }

    pub fn assert_no_stack(&self) -> WebResult<()> {
        if self.cursor != 0 {
            return Err(format!("Texture buffer stack cursor not zero, '{}' instead.", self.cursor).into());
        }
        Ok(())
    }
}
