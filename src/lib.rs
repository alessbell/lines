use std::cell::RefCell;
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

    let context = Rc::new(context);

    // raf(); calls request animation frame
    // let mut x = 0;
    // let mut y = 0;
    recursively_draw_to_canvas(&context, 0, 0);
    {
        // let context = context.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            log!("{}", event.offset_x());
            &context.clear_rect(0.0, 0.0, 1000.0, 1000.0);
            recursively_draw_to_canvas(&context, event.offset_x(), event.offset_y());
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
            .expect("hello?");
        closure.forget();
    }
}

fn recursively_draw_to_canvas(context: &Context2d, skew_x: i32, skew_y: i32) {
    const WIDTH: i32 = 110;

    for a in 0..6 {
        for i in 0..10 {
            draw_to_canvas(&context, i, a * WIDTH, skew_x, skew_y);
        }
    }
}

fn draw_to_canvas(context: &Context2d, iter: i32, x_offset: i32, x_skew: i32, y_skew: i32) {
    let _iter = iter as f64;
    let _x_offset = x_offset as f64;
    let _y_skew = y_skew as f64;
    let _x_skew = x_skew as f64;

    context.set_stroke_style(&JsValue::from_str("#f00"));

    context.begin_path();
    // Draw the outer circle.
    context
        .arc(
            _x_offset + 75.0 + _iter + (_x_skew / 5.00),
            75.0 + _iter + _y_skew,
            50.0 + _iter,
            0.0,
            f64::consts::PI * 2.0,
        )
        .unwrap();
    context.stroke();

    // Draw the mouth.
    context.begin_path();
    // context.move_to(_x_offset + 110.0, 75.0);
    context
        .arc(
            _x_offset + 75.0 + _iter + (_x_skew / 5.00),
            75.0 + _iter + _y_skew,
            35.0 + _iter,
            0.0,
            f64::consts::PI,
        )
        .unwrap();
    context.stroke();

    // // Draw the left eye.
    context.begin_path();
    // context.move_to(_x_offset + 65.0, 65.0);
    context
        .arc(
            _x_offset + 60.0 + _iter + (_x_skew / 5.00),
            65.0 + _iter + _y_skew,
            5.0 + _iter,
            0.0,
            f64::consts::PI * 2.0,
        )
        .unwrap();
    context.stroke();

    // Draw the right eye.
    context.begin_path();
    // context.move_to(_x_offset + 95.0, 65.0);
    context
        .arc(
            _x_offset + 90.0 + _iter + (_x_skew / 5.00),
            65.0 + _iter + _y_skew,
            5.0 + _iter,
            0.0,
            f64::consts::PI * 2.0,
        )
        .unwrap();

    context.stroke();
}
