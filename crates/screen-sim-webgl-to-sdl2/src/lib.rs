use std::cell::RefCell;
use std::ffi::CString;
use std::rc::Rc;

macro_rules! some {
    ( $x:expr ) => {{
        if let Some(x) = $x {
            x
        } else {
            panic!()
        }
    }};
}

#[derive(Default, Clone)]
struct GlData {
    last_shader_type: RefCell<u32>,
    line: RefCell<u32>,
}

#[derive(Clone, Default)]
pub struct WebGl2RenderingContext {
    data: Rc<GlData>,
}
impl WebGl2RenderingContext {
    pub const RGBA: u32 = gl::RGBA;
    pub const TRIANGLES: u32 = gl::TRIANGLES;
    pub const LINEAR: u32 = gl::LINEAR;
    pub const NEAREST: u32 = gl::NEAREST;
    pub const TEXTURE_WRAP_T: u32 = gl::TEXTURE_WRAP_T;
    pub const TEXTURE_WRAP_S: u32 = gl::TEXTURE_WRAP_S;
    pub const TEXTURE_MAG_FILTER: u32 = gl::TEXTURE_MAG_FILTER;
    pub const TEXTURE_MIN_FILTER: u32 = gl::TEXTURE_MIN_FILTER;
    pub const CLAMP_TO_EDGE: u32 = gl::CLAMP_TO_EDGE;
    pub const TEXTURE_2D: u32 = gl::TEXTURE_2D;
    pub const TEXTURE0: u32 = gl::TEXTURE0;
    pub const TEXTURE1: u32 = gl::TEXTURE1;
    pub const TEXTURE2: u32 = gl::TEXTURE2;
    pub const UNSIGNED_INT: u32 = gl::UNSIGNED_INT;
    pub const UNSIGNED_BYTE: u32 = gl::UNSIGNED_BYTE;
    pub const ELEMENT_ARRAY_BUFFER: u32 = gl::ELEMENT_ARRAY_BUFFER;
    pub const ARRAY_BUFFER: u32 = gl::ARRAY_BUFFER;
    pub const STATIC_DRAW: u32 = gl::STATIC_DRAW;
    pub const FLOAT: u32 = gl::FLOAT;
    pub const NO_ERROR: u32 = gl::NO_ERROR;
    pub const RENDERBUFFER: u32 = gl::RENDERBUFFER;
    pub const FRAMEBUFFER: u32 = gl::FRAMEBUFFER;
    pub const COLOR_BUFFER_BIT: u32 = gl::COLOR_BUFFER_BIT;
    pub const COLOR_ATTACHMENT0: u32 = gl::COLOR_ATTACHMENT0;
    pub const DEPTH_BUFFER_BIT: u32 = gl::DEPTH_BUFFER_BIT;
    pub const DEPTH_TEST: u32 = gl::DEPTH_TEST;
    pub const DEPTH_ATTACHMENT: u32 = gl::DEPTH_ATTACHMENT;
    pub const DEPTH_COMPONENT16: u32 = gl::DEPTH_COMPONENT16;
    pub const LINK_STATUS: u32 = gl::LINK_STATUS;
    pub const COMPILE_STATUS: u32 = gl::COMPILE_STATUS;
    pub const VERTEX_SHADER: u32 = gl::VERTEX_SHADER;
    pub const FRAGMENT_SHADER: u32 = gl::FRAGMENT_SHADER;
    pub fn draw_elements_with_i32(&self, mode: u32, count: i32, ty: u32, indices: u32) {
        let _scoped = self.guard_call("draw_elements_with_i32");
        unsafe {
            gl::DrawElements(mode, count, ty, [indices].as_ptr() as *const std::ffi::c_void);
        }
    }
    pub fn uniform1i(&self, location: i32, v0: i32) {
        let _scoped = self.guard_call("uniform1i");
        unsafe {
            gl::Uniform1i(location, v0);
        }
    }
    pub fn uniform1f(&self, location: i32, v0: f32) {
        let _scoped = self.guard_call("uniform1f");
        unsafe {
            gl::Uniform1f(location, v0);
        }
    }
    pub fn uniform2fv_with_f32_array(&self, location: i32, v: &[f32]) {
        let _scoped = self.guard_call("uniform2fv_with_f32_array");
        unsafe {
            gl::Uniform2f(location, v[0], v[1]);
        }
    }
    pub fn uniform3fv_with_f32_array(&self, location: i32, v: &[f32]) {
        let _scoped = self.guard_call("uniform3fv_with_f32_array");
        unsafe {
            gl::Uniform3f(location, v[0], v[1], v[2]);
        }
    }
    pub fn uniform_matrix4fv_with_f32_array(&self, location: i32, transpose: bool, v: &[f32]) {
        let _scoped = self.guard_call("uniform_matrix4fv_with_f32_array");
        unsafe {
            gl::UniformMatrix4fv(location, 1, if transpose { 1 } else { 0 }, v.as_ptr());
        }
    }
    pub fn draw_arrays_instanced(&self, mode: u32, first: i32, count: i32, instancecount: i32) {
        let _scoped = self.guard_call("draw_arrays_instanced");
        unsafe {
            gl::DrawArraysInstanced(mode, first, count, instancecount);
        }
    }
    pub fn get_uniform_location(&self, program: &WebGlProgram, name: &str) -> Location {
        let _scoped = self.guard_call("get_uniform_location");
        let value: i32;
        let name = CString::new(name).unwrap();
        unsafe {
            value = gl::GetUniformLocation(program.0, name.as_ptr());
        }
        Location(value)
    }
    pub fn create_shader(&self, ty: u32) -> Option<WebGlShader> {
        let _scoped = self.guard_call("create_shader");
        let value: u32;
        unsafe {
            value = gl::CreateShader(ty);
        }
        *(self.data.last_shader_type.borrow_mut()) = ty;
        Some(WebGlShader(value))
    }
    pub fn shader_source(&self, shader: &WebGlShader, code: &str) {
        let _scoped = self.guard_call("shader_source");
        let c_code = CString::new(code).unwrap();
        unsafe {
            gl::ShaderSource(shader.0, 1, [c_code.as_ptr()].as_ptr(), [code.len() as i32].as_ptr());
        }
    }
    pub fn compile_shader(&self, shader: &WebGlShader) {
        let _scoped = self.guard_call("compile_shader");
        unsafe {
            gl::CompileShader(shader.0);
        }
    }
    pub fn create_program(&self) -> Option<WebGlProgram> {
        let _scoped = self.guard_call("create_program");
        let value: u32;
        unsafe {
            value = gl::CreateProgram();
        }
        Some(WebGlProgram(value))
    }
    pub fn use_program(&self, program: Option<&WebGlProgram>) {
        let _scoped = self.guard_call("use_program");
        unsafe {
            gl::UseProgram(some!(program).0);
        }
    }
    pub fn link_program(&self, program: &WebGlProgram) {
        let _scoped = self.guard_call("link_program");
        unsafe {
            gl::LinkProgram(program.0);
        }
    }
    pub fn attach_shader(&self, program: &WebGlProgram, shader: &WebGlShader) {
        let _scoped = self.guard_call("attach_shader");
        unsafe {
            gl::AttachShader(program.0, shader.0);
        }
    }
    pub fn bind_vertex_array(&self, vao: Option<&WebGlVertexArrayObject>) {
        let _scoped = self.guard_call("bind_vertex_array");
        let vao = if let Some(vao) = vao { vao.0 } else { 0 };
        unsafe {
            gl::BindVertexArray(vao);
        }
    }
    pub fn clear(&self, bit: u32) {
        let _scoped = self.guard_call("clear");
        unsafe {
            gl::Clear(bit);
        }
    }
    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        let _scoped = self.guard_call("clear_color");
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }
    pub fn enable(&self, bit: u32) {
        let _scoped = self.guard_call("enable");
        unsafe {
            gl::Enable(bit);
        }
    }
    pub fn get_program_info_log(&self, program: &WebGlProgram) -> Option<String> {
        let _scoped = self.guard_call("get_program_info_log");
        let mut buff: [i8; 512] = [0; 512];
        let mut length = 512;
        unsafe {
            gl::GetProgramInfoLog(program.0, 512, &mut length, buff.as_mut_ptr());
        }
        Some(String::from_utf8(buff.iter().map(|i| *i as u8).collect::<Vec<u8>>()).unwrap())
    }
    pub fn get_shader_info_log(&self, shader: &WebGlShader) -> Option<String> {
        let _scoped = self.guard_call("get_shader_info_log");
        let mut buff: [i8; 512] = [0; 512];
        let mut length = 512;
        unsafe {
            gl::GetShaderInfoLog(shader.0, 512, &mut length, buff.as_mut_ptr());
        }
        Some(String::from_utf8(buff.iter().map(|i| *i as u8).collect::<Vec<u8>>()).unwrap())
    }
    pub fn get_program_parameter(&self, program: &WebGlProgram, param: u32) -> ProgramParameter {
        let _scoped = self.guard_call("get_program_parameter");
        let mut value: i32 = 0;
        unsafe {
            gl::GetProgramiv(program.0, param, &mut value);
        }
        ProgramParameter::new(value)
    }
    pub fn get_shader_parameter(&self, shader: &WebGlShader, param: u32) -> ProgramParameter {
        let _scoped = self.guard_call("get_shader_parameter");
        let mut value: i32 = 0;
        unsafe {
            gl::GetShaderiv(shader.0, param, &mut value);
        }
        ProgramParameter::new(value)
    }
    pub fn viewport(&self, x: i32, y: i32, w: i32, h: i32) {
        let _scoped = self.guard_call("viewport");
        unsafe {
            gl::Viewport(x, y, w, h);
        }
    }
    pub fn bind_framebuffer(&self, bit: u32, fb: Option<&WebGlFramebuffer>) {
        let _scoped = self.guard_call("bind_framebuffer");
        let fb = if let Some(fb) = fb { fb.0 } else { 0 };
        unsafe {
            println!("BindFramebufer {}", fb);
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, fb);
        }
    }
    pub fn bind_renderbuffer(&self, bit: u32, rb: Option<&WebGlRenderbuffer>) {
        let _scoped = self.guard_call("bind_renderbuffer");
        let rb = if let Some(rb) = rb { rb.0 } else { 0 };
        unsafe {
            gl::BindRenderbuffer(bit, rb);
        }
    }
    pub fn create_framebuffer(&self) -> Option<WebGlFramebuffer> {
        let _scoped = self.guard_call("create_framebuffer");
        let mut value: u32 = 324;
        unsafe {
            gl::GenFramebuffers(1, &mut value);
        }
        println!("CreateFramebuffer {}", value);
        Some(WebGlFramebuffer(value))
    }
    pub fn delete_framebuffer(&self, fb: Option<&WebGlFramebuffer>) {
        let _scoped = self.guard_call("delete_framebuffer");
        unsafe {
            gl::DeleteFramebuffers(1, &some!(fb).0);
        }
    }
    pub fn active_texture(&self, tex_number: u32) {
        let _scoped = self.guard_call("active_texture");
        unsafe {
            gl::ActiveTexture(tex_number);
        }
    }
    pub fn bind_texture(&self, bit: u32, tex: Option<&WebGlTexture>) {
        let _scoped = self.guard_call("bind_texture");
        let tex = if let Some(tex) = tex { tex.0 } else { 0 };
        unsafe {
            gl::BindTexture(bit, tex);
        }
    }
    pub fn bind_buffer(&self, bit: u32, buf: Option<&WebGlBuffer>) {
        let _scoped = self.guard_call("bind_buffer");
        let buf = if let Some(buf) = buf { buf.0 } else { 0 };
        unsafe {
            gl::BindBuffer(bit, buf);
        }
    }
    pub fn create_texture(&self) -> Option<WebGlTexture> {
        let _scoped = self.guard_call("create_texture");
        let mut value: u32 = 0;
        unsafe {
            gl::CreateTextures(Self::TEXTURE_2D, 1, &mut value);
        }
        Some(WebGlTexture(value))
    }
    pub fn delete_texture(&self, tex: Option<&WebGlTexture>) {
        let _scoped = self.guard_call("delete_texture");
        unsafe {
            gl::DeleteTextures(1, &some!(tex).0);
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn read_pixels_with_opt_u8_array(&self, _: u32, _: u32, _: i32, _: i32, _: u32, _: u32, _: Option<&mut [u8]>) -> WebResult<()> {
        let _scoped = self.guard_call("read_pixels_with_opt_u8_array");
        Ok(())
    }
    pub fn buffer_data_with_u8_array(&self, target: u32, data: &[u8], usage: u32) {
        let _scoped = self.guard_call("buffer_data_with_u8_array");
        unsafe {
            gl::BufferData(target, data.len() as isize, data.as_ptr() as *const std::ffi::c_void, usage);
        }
    }
    pub fn framebuffer_renderbuffer(&self, target: u32, attachment: u32, renderbuffertarget: u32, rb: Option<&WebGlRenderbuffer>) {
        let _scoped = self.guard_call("framebuffer_renderbuffer");
        unsafe {
            gl::FramebufferRenderbuffer(target, attachment, renderbuffertarget, some!(rb).0);
        }
    }
    pub fn renderbuffer_storage(&self, target: u32, internalformat: u32, width: i32, height: i32) {
        let _scoped = self.guard_call("renderbuffer_storage");
        unsafe {
            gl::RenderbufferStorage(target, internalformat, width, height);
        }
    }
    pub fn create_renderbuffer(&self) -> Option<WebGlRenderbuffer> {
        let _scoped = self.guard_call("create_renderbuffer");
        let mut value: u32 = 0;
        unsafe {
            gl::CreateRenderbuffers(1, &mut value);
        }
        Some(WebGlRenderbuffer(value))
    }
    pub fn create_buffer(&self) -> Option<WebGlBuffer> {
        let _scoped = self.guard_call("create_buffer");
        let mut value: u32 = 0;
        unsafe {
            gl::CreateBuffers(1, &mut value);
        }
        Some(WebGlBuffer(value))
    }
    pub fn create_vertex_array(&self) -> Option<WebGlVertexArrayObject> {
        let _scoped = self.guard_call("create_vertex_array");
        let mut value: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut value);
        }
        Some(WebGlVertexArrayObject(value))
    }
    pub fn vertex_attrib_pointer_with_i32(&self, index: u32, size: i32, ty: u32, normalized: bool, stride: i32, pointer: i32) {
        let _scoped = self.guard_call("vertex_attrib_pointer_with_i32");
        unsafe {
            gl::VertexAttribPointer(
                index,
                size,
                ty,
                if normalized { 1 } else { 0 },
                stride,
                [pointer].as_ptr() as *const std::ffi::c_void,
            );
        }
    }
    pub fn vertex_attrib_divisor(&self, index: u32, divisor: u32) {
        let _scoped = self.guard_call("vertex_attrib_divisor");
        unsafe {
            gl::VertexAttribDivisor(index, divisor);
        }
    }
    pub fn enable_vertex_attrib_array(&self, index: u32) {
        let _scoped = self.guard_call("enable_vertex_attrib_array");
        unsafe {
            gl::EnableVertexAttribArray(index);
        }
    }
    pub fn get_attrib_location(&self, program: &WebGlProgram, name: &str) -> i32 {
        let _scoped = self.guard_call("get_attrib_location");
        let value: i32;
        let name = CString::new(name).unwrap();
        unsafe {
            value = gl::GetAttribLocation(program.0, name.as_ptr());
        }
        value
    }
    pub fn framebuffer_texture_2d(&self, target: u32, attachment: u32, textarget: u32, tex: Option<&WebGlTexture>, level: i32) {
        let _scoped = self.guard_call("framebuffer_texture_2d");
        unsafe {
            gl::FramebufferTexture2D(target, attachment, textarget, some!(tex).0, level);
        }
    }
    pub fn tex_parameteri(&self, target: u32, pname: u32, param: i32) {
        let _scoped = self.guard_call("tex_parameteri");
        unsafe {
            gl::TexParameteri(target, pname, param);
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        &self,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    ) -> WebResult<()> {
        let _scoped = self.guard_call("tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array");
        let pixels = if let Some(pixels) = pixels { pixels } else { return Ok(()) };
        unsafe {
            gl::TexImage2D(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                format,
                ty,
                pixels.as_ptr() as *const std::ffi::c_void,
            );
        }
        Ok(())
    }
    fn guard_call<'a>(&'a self, id: &'static str) -> GuardCall<impl Fn() -> (&'static str, u32) + 'a> {
        let mut line = self.data.line.borrow_mut();
        *line += 1;
        GuardCall::new(*line, move || (id, self.get_error()))
    }
    pub fn get_error(&self) -> u32 {
        let err: u32;
        unsafe {
            err = gl::GetError();
        }
        err
    }
}

struct GuardCall<T: Fn() -> (&'static str, u32)> {
    func: T,
    first_tuple: (&'static str, u32),
    line: u32,
}

impl<T: Fn() -> (&'static str, u32)> GuardCall<T> {
    pub fn new(line: u32, func: T) -> GuardCall<T> {
        let tuple = func();
        GuardCall {
            line,
            func,
            first_tuple: tuple,
        }
    }
    pub fn print(&self) {
        let func = &self.func;
        let second_tuple = func();
        println!("{}| {:?} => {:?} ; {}", self.line, self.first_tuple.1, second_tuple.1, second_tuple.0);
        if self.first_tuple.1 != gl::NO_ERROR || second_tuple.1 != gl::NO_ERROR {
            panic!("Errors! {}", line!());
        }
    }
}

impl<T: Fn() -> (&'static str, u32)> Drop for GuardCall<T> {
    fn drop(&mut self) {
        self.print();
    }
}

pub struct WebGlShader(u32);
pub struct WebGlProgram(u32);
pub struct WebGlVertexArrayObject(u32);
#[derive(Clone, Debug)]
pub struct WebGlTexture(u32);
#[derive(Clone, Debug)]
pub struct WebGlFramebuffer(u32);
pub struct WebGlRenderbuffer(u32);
#[derive(Debug, PartialEq)]
pub struct WebError {
    cause: String,
}
impl From<String> for WebError {
    fn from(cause: String) -> WebError {
        WebError { cause }
    }
}
impl<'a> From<&'a str> for WebError {
    fn from(msg: &'a str) -> Self {
        WebError { cause: msg.into() }
    }
}
pub type WebResult<T> = Result<T, WebError>;

pub struct Location(i32);

impl Location {
    pub fn as_ref(&self) -> i32 {
        self.0
    }
}
pub struct ProgramParameter {
    value: i32,
}
impl ProgramParameter {
    pub fn new(value: i32) -> ProgramParameter {
        ProgramParameter { value }
    }
    pub fn as_bool(&self) -> Option<bool> {
        Some(self.value == 1)
    }
}

pub struct WebGlBuffer(u32);
