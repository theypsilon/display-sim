#[derive(Clone)]
pub struct WebGl2RenderingContext {}
impl WebGl2RenderingContext {
    pub const RGBA: u32 = 0;
    pub const TRIANGLES: u32 = 0;
    pub const LINEAR: u32 = 0;
    pub const NEAREST: u32 = 0;
    pub const TEXTURE_WRAP_T: u32 = 0;
    pub const TEXTURE_WRAP_S: u32 = 0;
    pub const TEXTURE_MAG_FILTER: u32 = 0;
    pub const TEXTURE_MIN_FILTER: u32 = 0;
    pub const CLAMP_TO_EDGE: u32 = 0;
    pub const TEXTURE_2D: u32 = 0;
    pub const TEXTURE0: u32 = 0;
    pub const TEXTURE1: u32 = 0;
    pub const TEXTURE2: u32 = 0;
    pub const UNSIGNED_INT: u32 = 0;
    pub const UNSIGNED_BYTE: u32 = 0;
    pub const ELEMENT_ARRAY_BUFFER: u32 = 0;
    pub const ARRAY_BUFFER: u32 = 0;
    pub const STATIC_DRAW: u32 = 0;
    pub const FLOAT: u32 = 0;
    pub const NO_ERROR: u32 = 0;
    pub const RENDERBUFFER: u32 = 0;
    pub const FRAMEBUFFER: u32 = 0;
    pub const COLOR_BUFFER_BIT: u32 = gl::COLOR_BUFFER_BIT;
    pub const COLOR_ATTACHMENT0: u32 = 0;
    pub const DEPTH_BUFFER_BIT: u32 = 0;
    pub const DEPTH_TEST: u32 = 0;
    pub const DEPTH_ATTACHMENT: u32 = 0;
    pub const DEPTH_COMPONENT16: u32 = 0;
    pub const LINK_STATUS: u32 = 0;
    pub const COMPILE_STATUS: u32 = 0;
    pub const VERTEX_SHADER: u32 = 0;
    pub const FRAGMENT_SHADER: u32 = 0;
    pub fn draw_elements_with_i32(&self, _: u32, _: u32, _: u32, _: u32) {}
    pub fn uniform1i(&self, _: u32, _: u32) {}
    pub fn uniform1f(&self, _: u32, _: f32) {}
    pub fn uniform2fv_with_f32_array(&self, _: u32, _: &[f32]) {}
    pub fn uniform3fv_with_f32_array(&self, _: u32, _: &[f32]) {}
    pub fn uniform_matrix4fv_with_f32_array(&self, _: u32, _: bool, _: &[f32]) {}
    pub fn draw_arrays_instanced(&self, _: u32, _: u32, _: u32, _: i32) {}
    pub fn get_uniform_location(&self, _: &WebGlProgram, _: &str) -> Location {
        Location {}
    }
    pub fn create_shader(&self, _: u32) -> Option<WebGlShader> {
        Some(WebGlShader {})
    }
    pub fn shader_source(&self, _: &WebGlShader, _: &str) {}
    pub fn compile_shader(&self, _: &WebGlShader) {}
    pub fn create_program(&self) -> Option<WebGlProgram> {
        Some(WebGlProgram {})
    }
    pub fn use_program(&self, _: Option<&WebGlProgram>) {}
    pub fn link_program(&self, _: &WebGlProgram) {}
    pub fn attach_shader(&self, _: &WebGlProgram, _: &WebGlShader) {}
    pub fn bind_vertex_array(&self, _: Option<&WebGlVertexArrayObject>) {}
    pub fn clear(&self, bit: u32) {
        unsafe { gl::Clear(bit); }
    }
    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe { gl::ClearColor(r, g, b, a); }
    }
    pub fn enable(&self, _: u32) {}
    pub fn get_error(&self) -> u32 {
        0
    }
    pub fn get_program_info_log(&self, _: &WebGlProgram) -> Option<&'static str> {
        Some("")
    }
    pub fn get_shader_info_log(&self, _: &WebGlShader) -> Option<&'static str> {
        Some("")
    }
    pub fn get_program_parameter(&self, _: &WebGlProgram, _: u32) -> ProgramParameter {
        ProgramParameter {}
    }
    pub fn get_shader_parameter(&self, _: &WebGlShader, _: u32) -> ProgramParameter {
        ProgramParameter {}
    }
    pub fn viewport(&self, _: u32, _: u32, _: i32, _: i32) {}
    pub fn bind_framebuffer(&self, _: u32, _: Option<&WebGlFramebuffer>) {}
    pub fn bind_renderbuffer(&self, _: u32, _: Option<&WebGlFramebuffer>) {}
    pub fn create_framebuffer(&self) -> Option<WebGlFramebuffer> {
        Some(WebGlFramebuffer {})
    }
    pub fn delete_framebuffer(&self, _: Option<&WebGlFramebuffer>) {}
    pub fn active_texture(&self, _: u32) {}
    pub fn bind_texture(&self, _: u32, _: Option<&WebGlTexture>) {}
    pub fn bind_buffer(&self, _: u32, _: Option<&WebGlBuffer>) {}
    pub fn create_texture(&self) -> Option<WebGlTexture> {
        Some(WebGlTexture {})
    }
    pub fn delete_texture(&self, _: Option<&WebGlTexture>) {}
    #[allow(clippy::too_many_arguments)]
    pub fn read_pixels_with_opt_u8_array(&self, _: u32, _: u32, _: i32, _: i32, _: u32, _: u32, _: Option<&mut [u8]>) -> WebResult<()> {
        Ok(())
    }
    pub fn buffer_data_with_u8_array(&self, _: u32, _: &[u8], _: u32) {}
    pub fn framebuffer_renderbuffer(&self, _: u32, _: u32, _: u32, _: Option<&WebGlFramebuffer>) {}
    pub fn renderbuffer_storage(&self, _: u32, _: u32, _: i32, _: i32) {}
    pub fn create_renderbuffer(&self) -> Option<WebGlFramebuffer> {
        Some(WebGlFramebuffer {})
    }
    pub fn create_buffer(&self) -> Option<WebGlBuffer> {
        Some(WebGlBuffer {})
    }
    pub fn create_vertex_array(&self) -> Option<WebGlVertexArrayObject> {
        Some(WebGlVertexArrayObject {})
    }
    pub fn vertex_attrib_pointer_with_i32(&self, _: u32, _: u32, _: u32, _: bool, _: i32, _: i32) {}
    pub fn vertex_attrib_divisor(&self, _: u32, _: u32) {}
    pub fn enable_vertex_attrib_array(&self, _: u32) {}
    pub fn get_attrib_location(&self, _: &WebGlProgram, _: &str) -> u32 {
        0
    }
    pub fn framebuffer_texture_2d(&self, _: u32, _: u32, _: u32, _: Option<&WebGlTexture>, _: u32) {}
    pub fn tex_parameteri(&self, _: u32, _: u32, _: i32) {}
    #[allow(clippy::too_many_arguments)]
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        &self,
        _: u32,
        _: u32,
        _: i32,
        _: i32,
        _: i32,
        _: u32,
        _: u32,
        _: u32,
        _: Option<&[u8]>,
    ) -> WebResult<()> {
        Ok(())
    }
}

pub struct WebGlShader {}
pub struct WebGlProgram {}
pub struct WebGlVertexArrayObject {}
#[derive(Clone, Debug)]
pub struct WebGlTexture {}
#[derive(Clone, Debug)]
pub struct WebGlFramebuffer {}
#[derive(Debug, PartialEq)]
pub struct WebError {}
impl From<String> for WebError {
    fn from(_: String) -> WebError {
        WebError {}
    }
}
impl<'a> From<&'a str> for WebError {
    fn from(_: &'a str) -> Self {
        WebError {}
    }
}
pub type WebResult<T> = Result<T, WebError>;

pub struct Location {}
impl Location {
    pub fn as_ref(&self) -> u32 {
        0
    }
}
pub struct ProgramParameter {}
impl ProgramParameter {
    pub fn as_bool(&self) -> Option<bool> {
        Some(true)
    }
}

pub struct ArrayBuffer {}
pub struct WebGlBuffer {}
