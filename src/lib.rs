mod error;
mod shader;

use wasm_bindgen::prelude::*;
use web_sys::js_sys;

use crate::error::ConsoleErrorUnwrap;

const WEBLGL2_ERROR: &'static str = "Webgl2 not supported!";

#[wasm_bindgen]
pub fn entry() {
    let window = web_sys::window().cunwrap("Window Required");
    let document = window.document().cunwrap("Document Required");
    let body = document.body().cunwrap("Body Required");

    // Create and push canvas to body element
    let canvas: web_sys::HtmlCanvasElement = document.create_element("canvas")
        .cunwrap("Failed to create canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .cunwrap("Failed to cast canvas to HtmlCanvasElement");
    canvas.set_width(900);
    canvas.set_height(600);
    let gl = canvas.get_context("webgl2")
        .cunwrap(WEBLGL2_ERROR).cunwrap(WEBLGL2_ERROR)
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .cunwrap(WEBLGL2_ERROR);
    body.append_child(&canvas.dyn_into::<web_sys::Node>()
        .cunwrap("Failed to cast canvas to node"))
        .cunwrap("Error Appending Canvas to Body");

    // Get a rendering context
    web_sys::console::log_1(&"Webgl clls".into());
    gl.clear_color(1.0, 0.0, 0.0, 1.0);
    gl.enable(web_sys::WebGl2RenderingContext::DEPTH_TEST);
    gl.clear(web_sys::WebGl2RenderingContext::COLOR_BUFFER_BIT);
    gl.clear(web_sys::WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    // Shaders
    const VERT_SRC: &'static str = include_str!("./shader/main_vert.glsl");
    const FRAG_SRC: &'static str = include_str!("./shader/main_frag.glsl");
    let program = shader::ShaderProgram::new(&gl, &[
        (VERT_SRC, web_sys::WebGl2RenderingContext::VERTEX_SHADER),
        (FRAG_SRC, web_sys::WebGl2RenderingContext::FRAGMENT_SHADER),
    ]);
    program.useme(&gl);

    // create a buffer
    let vertices = [
        0.5, 0.5, 0.0,
        0.5, -0.5, 0.0,
        -0.5, -0.5, 0.0,
    ];
    // Create a vertex array
    let vao = gl.create_vertex_array().cunwrap("Failed to create vertex array");
    gl.bind_vertex_array(Some(&vao));
    let vertex_buffer = gl.create_buffer().cunwrap("Failed to create vertex buffer");
    gl.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

    // SAFETY: This is probably safe
    let buffer_data_f32view = unsafe{ js_sys::Float32Array::view(&vertices) };
    gl.buffer_data_with_array_buffer_view(
        web_sys::WebGl2RenderingContext::ARRAY_BUFFER,
        &buffer_data_f32view,
        web_sys::WebGl2RenderingContext::STATIC_DRAW
    );

    // Define Vertex Attributes
    // layout(location = 0) in vec3 vertexPosition
    gl.vertex_attrib_pointer_with_i32(0, 3, web_sys::WebGl2RenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    // Actual Drawing
    gl.viewport(0, 0, 900, 600);
    gl.draw_arrays(web_sys::WebGl2RenderingContext::TRIANGLES, 0, 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
