use web_sys::{
    WebGl2RenderingContext, WebGlTexture, WebGlFramebuffer
};
use crate::wasm_error::{WasmResult};

pub struct TextureBuffer {
    texture: Option<WebGlTexture>,
    framebuffer: Option<WebGlFramebuffer>,
}

impl TextureBuffer {
    pub fn new(gl: &WebGl2RenderingContext, width: i32, height: i32) -> WasmResult<TextureBuffer> {
        let framebuffer = gl.create_framebuffer();
        let texture = gl.create_texture();

        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, framebuffer.as_ref());
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
            None
        )?;
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        gl.framebuffer_texture_2d(WebGl2RenderingContext::FRAMEBUFFER, WebGl2RenderingContext::COLOR_ATTACHMENT0, WebGl2RenderingContext::TEXTURE_2D, texture.as_ref(), 0);

        Ok(TextureBuffer {texture, framebuffer})
    }

    pub fn new_with_depthbuffer(gl: &WebGl2RenderingContext, width: i32, height: i32) -> WasmResult<TextureBuffer> {
        let depthbuffer = gl.create_renderbuffer();
        let texture_buffer = Self::new(gl, width, height)?;
        gl.bind_renderbuffer(WebGl2RenderingContext::RENDERBUFFER, depthbuffer.as_ref());
        gl.renderbuffer_storage(WebGl2RenderingContext::RENDERBUFFER, WebGl2RenderingContext::DEPTH_COMPONENT16, width, height);
        gl.framebuffer_renderbuffer(WebGl2RenderingContext::FRAMEBUFFER, WebGl2RenderingContext::DEPTH_ATTACHMENT, WebGl2RenderingContext::RENDERBUFFER, depthbuffer.as_ref());
        Ok(texture_buffer)
    }

    pub fn texture(&self) -> Option<&WebGlTexture> {
        self.texture.as_ref()
    }

    pub fn framebuffer(&self) -> Option<&WebGlFramebuffer> {
        self.framebuffer.as_ref()
    }
}
