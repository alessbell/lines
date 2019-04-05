use std::cell::Cell;
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
pub struct Canvas {
    canvas: web_sys::HtmlCanvasElement,
    context: std::rc::Rc<web_sys::CanvasRenderingContext2d>,
}

#[wasm_bindgen]
impl Canvas {
    pub fn clear(&self) {
        self.context.clear_rect(0.0, 0.0, 2000.0, 2000.0);
    }

    pub fn initial_draw(&self) {
        draw_grid_to_canvas(&self.context, 0, 0);
    }

    pub fn enable_mouse_tracking(&self) {
        let context = self
            .canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<Context2d>()
            .unwrap();
        let context = Rc::new(context);
        let tracking_enabled = Rc::new(Cell::new(false));
        {
            let context = context.clone();
            let tracking_enabled = tracking_enabled.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                if !tracking_enabled.get() {
                    return;
                };
                context.clear_rect(0.0, 0.0, 2000.0, 2000.0);
                let x_offset = event.offset_x();
                let y_offset = event.offset_y();
                Canvas::draw(&context, x_offset, y_offset);
            }) as Box<dyn FnMut(_)>);
            self.canvas
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
                .expect("Error getting mousemove ref");
            closure.forget();
        }
        {
            let tracking_enabled = tracking_enabled.clone();

            let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                let old_tracking_enabled = tracking_enabled.get();
                tracking_enabled.set(!old_tracking_enabled);
            }) as Box<dyn FnMut(_)>);
            self.canvas
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
                .expect("Error getting mousemove ref");
            closure.forget();
        }
    }

    pub fn draw(context: &Context2d, x_offset: i32, y_offset: i32) {
        draw_grid_to_canvas(context, x_offset as u32, y_offset as u32);
    }

    pub fn new() -> Canvas {
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

        Canvas { canvas, context }
    }
}

fn draw_grid_to_canvas(context: &Context2d, skew_x: u32, skew_y: u32) {
    const WIDTH: u32 = 6;
    const HEIGHT: u32 = 6;
    const GLYPH_WIDTH: u32 = 130;

    for a in 0..(WIDTH * HEIGHT) {
        for i in 1..10 {
            let y_offset = a / WIDTH * GLYPH_WIDTH;
            let x_offset = (a % WIDTH) * GLYPH_WIDTH;
            draw_to_canvas(&context, i, x_offset, y_offset, skew_x, skew_y);
        }
    }
}

// fn draw_to_canvas(
//     context: &Context2d,
//     iter: u32,
//     x_offset: u32,
//     y_offset: u32,
//     x_skew: u32,
//     y_skew: u32,
// ) {
//     let _iter = iter as f64;
//     let _x_offset = x_offset as f64;
//     let _y_offset = y_offset as f64;
//     let _y_skew = y_skew as f64;
//     let _x_skew = x_skew as f64;

//     context.set_stroke_style(&JsValue::from_str("#f00"));

//     context.begin_path();
//     context.move_to(
//         _x_offset + 75.0 + _iter + (_x_skew / 100.0 * _iter),
//         _x_offset + 75.0 + _iter + (_y_skew / 100.0 * _iter),
//     );
//     context.line_to(
//         _x_offset + 75.0 + _iter + (_x_skew / 100.0 * _iter),
//         _y_offset + 75.0 + _iter + (_y_skew / 100.0 * _iter),
//     );
//     context.stroke();

//     context.set_stroke_style(&JsValue::from_str("blue"));
//     context.begin_path();
//     context.move_to(
//         _x_offset + 75.0 + _iter + (_x_skew / 100.0 * _iter),
//         _x_offset + 75.0 + _iter + (_y_skew / 100.0 * _iter),
//     );
//     context.line_to(
//         _x_offset + 75.0 + _iter + (_x_skew / 100.0 * _iter),
//         _x_offset + 200.0 + _iter + (_y_skew / 100.0 * _iter),
//     );
//     context.stroke();
// }

fn draw_to_canvas(
    context: &Context2d,
    iter: u32,
    x_offset: u32,
    y_offset: u32,
    x_skew: u32,
    y_skew: u32,
) {
    let _iter = iter as f64;
    let _x_offset = x_offset as f64;
    let _y_offset = y_offset as f64;
    let _y_skew = y_skew as f64;
    let _x_skew = x_skew as f64;

    context.set_stroke_style(&JsValue::from_str("#f00"));
    context.begin_path();
    let offset_x = _x_offset + (_iter % 4.0) + (_x_skew / 100.0 * _iter);
    let offset_y = _y_offset + (_iter % 4.0) + (_y_skew / 100.0 * _iter);
    // Draw the outer circle.
    context
        .arc(
            75.0 + offset_x,
            75.0 + offset_y,
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
            75.0 + offset_x,
            75.0 + offset_y,
            35.0 + _iter,
            0.0,
            f64::consts::PI,
        )
        .unwrap();
    context.stroke();

    // // Draw the left eye.
    context.set_stroke_style(&JsValue::from_str("purple"));
    context.begin_path();
    context
        .arc(
            60.0 + offset_x,
            65.0 + offset_y,
            5.0 + _iter,
            0.0,
            f64::consts::PI * 2.0,
        )
        .unwrap();
    context.stroke();

    // Draw the right eye.
    context.set_stroke_style(&JsValue::from_str("green"));
    context.begin_path();
    context
        .arc(
            90.0 + offset_x,
            65.0 + offset_y,
            5.0 + _iter,
            0.0,
            f64::consts::PI * 2.0,
        )
        .unwrap();

    context.stroke();
}
