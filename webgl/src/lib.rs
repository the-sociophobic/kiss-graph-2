use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlCanvasElement, WebGlRenderingContext as GL, Window};

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().expect("Failed to get window");
    let document = window.document().expect("Failed to get document");
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;

    document.body().unwrap().append_child(&canvas)?;

    let gl = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<GL>()?;

    gl.clear_color(0.1, 0.2, 0.3, 1.0);
    gl.clear(GL::COLOR_BUFFER_BIT);

    // Add your WebGL code to render the cube here

    Ok(())
}
