use web_sys::{WebGlProgram, WebGlShader};

use crate::{error::{error_out, ConsoleErrorUnwrap}};

fn compile_shader(gl: &web_sys::WebGl2RenderingContext, src: &str, glshadertype: u32) -> web_sys::WebGlShader {
    let shader = gl.create_shader(glshadertype).cunwrap("Could not create additional shader");
    gl.shader_source(&shader, src);
    gl.compile_shader(&shader);

    if !gl.get_shader_parameter(&shader, web_sys::WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap() {
        let error = gl.get_shader_info_log(&shader).cunwrap("Failed to discover shader compilation error(s)");
        error_out(&error);
    }
    shader
}

fn link_program(gl: &web_sys::WebGl2RenderingContext, shaders: impl Iterator<Item=WebGlShader>) -> web_sys::WebGlProgram {
    let program = gl.create_program().cunwrap("Failed to create program");
    shaders.for_each(|shader| gl.attach_shader(&program, &shader));
    gl.link_program(&program);
    
    if !gl.get_program_parameter(&program, web_sys::WebGl2RenderingContext::LINK_STATUS).as_bool().unwrap() {
        let error = gl.get_program_info_log(&program).cunwrap("Failed to discover program compilation error(s)");
        error_out(&error)
    }
    program
}

pub struct ShaderProgram(WebGlProgram);

impl ShaderProgram {
    pub fn new(gl: &web_sys::WebGl2RenderingContext, details: &[(&str, u32)]) -> Self {
        let shaders = details.iter().map(|detail| compile_shader(gl, detail.0, detail.1));
        Self(link_program(gl, shaders))
    }
    pub fn useme(&self, gl: &web_sys::WebGl2RenderingContext) {
        gl.use_program(Some(&self.0));
    }
}