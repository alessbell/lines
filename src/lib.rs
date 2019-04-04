use std::f64;
use std::rc::Rc;
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

#[wasm_bindgen]
pub fn start() {
    utils::set_panic_hook();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
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

    let context = Rc::new(context);

    draw_grid_to_canvas(&context, 0, 0);
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            &context.clear_rect(0.0, 0.0, 2000.0, 2000.0);
            draw_grid_to_canvas(&context, event.offset_x(), event.offset_y());
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
            .expect("Error getting mousemove ref");
        closure.forget();
    }
}

#[wasm_bindgen]
pub fn save_to_stack() {}

fn draw_grid_to_canvas(context: &Context2d, skew_x: i32, skew_y: i32) {
    const WIDTH: i32 = 6;
    const HEIGHT: i32 = 6;
    const GLYPH_WIDTH: i32 = 130;

    for a in 0..(WIDTH * HEIGHT) {
        for i in 1..10 {
            let y_offset = a / WIDTH * GLYPH_WIDTH;
            let x_offset = { (a % WIDTH) * GLYPH_WIDTH };
            draw_to_canvas(&context, i, x_offset, y_offset, skew_x, skew_y);
        }
    }
}

fn draw_to_canvas(
    context: &Context2d,
    iter: i32,
    x_offset: i32,
    y_offset: i32,
    x_skew: i32,
    y_skew: i32,
) {
    let _iter = iter as f64;
    let _x_offset = x_offset as f64;
    let _y_offset = y_offset as f64;
    let _y_skew = y_skew as f64;
    let _x_skew = x_skew as f64;

    context.set_stroke_style(&JsValue::from_str("#f00"));

    context.begin_path();
    // Draw the outer circle.
    context
        .arc(
            _x_offset + 75.0 + _iter + (_x_skew / 100.0 * _iter),
            _y_offset + 75.0 + _iter + (_y_skew / 100.0 * _iter),
            50.0 + _iter,
            0.0,
            f64::consts::PI * 2.0,
        )
        .unwrap();
    context.stroke();

    // Draw the mouth.
    context.set_stroke_style(&JsValue::from_str("blue"));
    context.begin_path();
    context
        .arc(
            _x_offset + 75.0 + _iter + (_x_skew / 100.0 * _iter),
            _y_offset + 75.0 + _iter + (_y_skew / 100.0 * _iter),
            35.0 + _iter,
            0.0,
            f64::consts::PI,
        )
        .unwrap();
    context.stroke();

    // // Draw the left eye.
    context.set_stroke_style(&JsValue::from_str("pink"));
    context.begin_path();
    context
        .arc(
            _x_offset + 60.0 + _iter + (_x_skew / 100.0 * _iter),
            _y_offset + 65.0 + _iter + (_y_skew / 100.0 * _iter),
            5.0 + _iter,
            0.0,
            f64::consts::PI * 2.0,
        )
        .unwrap();
    context.stroke();

    // Draw the right eye.
    context.set_stroke_style(&JsValue::from_str("yellow"));
    context.begin_path();
    context
        .arc(
            _x_offset + 90.0 + _iter + (_x_skew / 100.0 * _iter),
            _y_offset + 65.0 + _iter + (_y_skew / 100.0 * _iter),
            5.0 + _iter,
            0.0,
            f64::consts::PI * 2.0,
        )
        .unwrap();

    context.stroke();
}
