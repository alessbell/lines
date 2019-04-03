use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::CanvasRenderingContext2d as Context2d;

mod utils;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
//

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = match document.get_element_by_id("canvas") {
        Some(canvas) => canvas,
        None => panic!("No element with id canvas"),
    };
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|err| log!("{:?}", err))
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<Context2d>()
        .unwrap();

    for a in [0, 100, 200, 300, 400].iter() {
        log!("{}", a);
        for i in 0..10 {
            draw_to_canvas(&context, i, *a);
        }
    }
}

fn draw_to_canvas(context: &Context2d, iter: i32, offset: i32) {
    context.set_stroke_style(&JsValue::from_str("#f00"));
    context.begin_path();
    let _iter = iter as f64;
    let _offset = offset as f64;
    // context.move_to(25.0 + _offset, 25.0);

    // Draw the outer circle.
    context
        .arc(
            _offset + 75.0 + _iter,
            75.0 + _iter,
            50.0 + _iter,
            0.0,
            f64::consts::PI * 2.0,
        )
        .unwrap();

    // Draw the mouth.
    context.move_to(_offset + 110.0, 75.0);
    context
        .arc(
            _offset + 75.0 + _iter,
            75.0 + _iter,
            35.0 + _iter,
            0.0,
            f64::consts::PI,
        )
        .unwrap();

    // Draw the left eye.
    context.move_to(_offset + 65.0, 65.0);
    context
        .arc(
            _offset + 60.0 + _iter,
            65.0 + _iter,
            5.0 + _iter,
            0.0,
            f64::consts::PI * 2.0,
        )
        .unwrap();

    // Draw the right eye.
    context.move_to(_offset + 95.0, 65.0);
    context
        .arc(
            _offset + 90.0 + _iter,
            65.0 + _iter,
            5.0 + _iter,
            0.0,
            f64::consts::PI * 2.0,
        )
        .unwrap();

    context.stroke();
}
