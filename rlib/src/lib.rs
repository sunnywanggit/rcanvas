extern crate wasm_bindgen;

use std::f64;
use image::GenericImageView;
use js_sys::Uint8Array;
use wasm_bindgen::{prelude::*, JsCast, Clamped};
use wasm_bindgen_futures::JsFuture;
use web_sys::ImageData;
use std::{ fs, time::Instant };

#[wasm_bindgen]
pub fn test() -> i32{
    return 22223;
}

// 这里使用的方法是拿到 DOM 中的 canvas 对象，然后在 rust 中执行 to_data_url
#[wasm_bindgen]
pub async fn unred(canvas: String) -> Result<String, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().expect("Could not get document");
    let canvas = document
        .get_element_by_id(&canvas)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    return canvas.to_data_url();
    // let png_8bpp_bytes = canvas.export_image(ImageFormat::PngGamma8Bpp).unwrap();
    // fs::write("./result.png", png_8bpp_bytes);
    // Ok(())
}