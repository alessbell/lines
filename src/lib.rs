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
    width: u32,
    height: u32,
    glyph_width: u32,
    canvas: web_sys::HtmlCanvasElement,
    context: std::rc::Rc<web_sys::CanvasRenderingContext2d>,
}

#[wasm_bindgen]
impl Canvas {
    pub fn clear(&self) {
        self.context.clear_rect(0.0, 0.0, 2000.0, 2000.0);
    }

    pub fn initial_draw(&self) {
        Canvas::draw_grid_to_canvas(&self, 0, 0);
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

        {
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                context.clear_rect(0.0, 0.0, 2000.0, 2000.0);
                // Canvas::draw_grid_to_canvas(&mut self, event.offset_x() as u32, event.offset_y() as u32);
                for a in 0..(self.width * self.height) {
                    for i in 1..10 {
                        let y_offset = a / self.width * self.glyph_width;
                        let x_offset = (a % self.width) * self.glyph_width;
                        draw_to_canvas(
                            i,
                            x_offset,
                            y_offset,
                            event.offset_x() as u32,
                            event.offset_y() as u32,
                        );
                    }
                }
            }) as Box<dyn FnMut(_)>);
            self.canvas
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
                .expect("Error getting mousemove ref");
            closure.forget();
        }
    }

    pub fn draw_grid_to_canvas(&self, skew_x: u32, skew_y: u32) {
        for a in 0..(self.width * self.height) {
            for i in 1..10 {
                let y_offset = a / self.width * self.glyph_width;
                let x_offset = (a % self.width) * self.glyph_width;
                draw_to_canvas(i, x_offset, y_offset, skew_x, skew_y);
            }
        }
    }

    pub fn new() -> Canvas {
        utils::set_panic_hook();
        let width: u32 = 6;
        let height: u32 = 6;
        let glyph_width: u32 = 20;
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

        Canvas {
            width,
            height,
            canvas,
            context,
            glyph_width,
        }
    }
}

fn draw_to_canvas(
    // context: &Context2d,
    iter: u32,
    x_offset: u32,
    y_offset: u32,
    x_skew: u32,
    y_skew: u32,
) {
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

    let _iter = iter as f64;
    let _x_offset = x_offset as f64;
    let _y_offset = y_offset as f64;
    let _y_skew = y_skew as f64;
    let _x_skew = x_skew as f64;

    context.set_stroke_style(&JsValue::from_str("#f00"));

    context.begin_path();
    context.move_to(
        _x_offset + 75.0 + _iter + (_x_skew / 100.0 * _iter),
        _x_offset + 75.0 + _iter + (_y_skew / 100.0 * _iter),
    );
    context.line_to(
        _x_offset + 75.0 + _iter + (_x_skew / 100.0 * _iter),
        _y_offset + 75.0 + _iter + (_y_skew / 100.0 * _iter),
    );
    context.stroke();

    context.set_stroke_style(&JsValue::from_str("blue"));
    context.begin_path();
    context.move_to(
        _x_offset + 75.0 + _iter + (_x_skew / 100.0 * _iter),
        _x_offset + 75.0 + _iter + (_y_skew / 100.0 * _iter),
    );
    context.line_to(
        _x_offset + 75.0 + _iter + (_x_skew / 100.0 * _iter),
        _x_offset + 200.0 + _iter + (_y_skew / 100.0 * _iter),
    );
    context.stroke();
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
//     // Draw the outer circle.
//     context
//         .arc(
//             _x_offset + 75.0 + _iter + (_x_skew / 100.0 * _iter),
//             _y_offset + 75.0 + _iter + (_y_skew / 100.0 * _iter),
//             50.0 + _iter,
//             0.0,
//             f64::consts::PI * 2.0,
//         )
//         .unwrap();
//     context.stroke();

//     // Draw the mouth.
//     context.set_stroke_style(&JsValue::from_str("blue"));
//     context.begin_path();
//     context
//         .arc(
//             _x_offset + 75.0 + _iter + (_x_skew / 100.0 * _iter),
//             _y_offset + 75.0 + _iter + (_y_skew / 100.0 * _iter),
//             35.0 + _iter,
//             0.0,
//             f64::consts::PI,
//         )
//         .unwrap();
//     context.stroke();

//     // // Draw the left eye.
//     context.set_stroke_style(&JsValue::from_str("pink"));
//     context.begin_path();
//     context
//         .arc(
//             _x_offset + 60.0 + _iter + (_x_skew / 100.0 * _iter),
//             _y_offset + 65.0 + _iter + (_y_skew / 100.0 * _iter),
//             5.0 + _iter,
//             0.0,
//             f64::consts::PI * 2.0,
//         )
//         .unwrap();
//     context.stroke();

//     // Draw the right eye.
//     context.set_stroke_style(&JsValue::from_str("yellow"));
//     context.begin_path();
//     context
//         .arc(
//             _x_offset + 90.0 + _iter + (_x_skew / 100.0 * _iter),
//             _y_offset + 65.0 + _iter + (_y_skew / 100.0 * _iter),
//             5.0 + _iter,
//             0.0,
//             f64::consts::PI * 2.0,
//         )
//         .unwrap();

//     context.stroke();
// }
