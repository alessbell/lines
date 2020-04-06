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
    range: u32,
}

#[wasm_bindgen]
impl Canvas {
    pub fn clear(&self) {
        self.context.clear_rect(0.0, 0.0, 2000.0, 2000.0);
    }

    pub fn set_range(&mut self, range: u32) {
        self.range = range;
    }

    pub fn initial_draw(&self) {
        // draw_grid_to_canvas(&self.context, 0, 0, self.range);
        let faces = calculate_faces(&self.context, 0, 0, self.range);
        draw_faces(&self.context, faces);
    }

    pub fn enable_mouse_tracking(&self) {
        let tracking_enabled = Rc::new(Cell::new(false));
        {
            let range = self.range.clone();
            let context = self.context.clone();
            let tracking_enabled = tracking_enabled.clone();

            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                if !tracking_enabled.get() {
                    return;
                };
                context.clear_rect(0.0, 0.0, 2000.0, 2000.0);
                let x_offset = event.offset_x();
                let y_offset = event.offset_y();
                // let faces = calculate_faces(&context, x_offset as u32, y_offset as u32, range);
                // draw_faces(faces);
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

        Canvas {
            canvas,
            context,
            range: 0,
        }
    }
}

fn draw_faces(context: &Context2d, faces: Faces) {
    for face in faces.arcs.iter() {
        log!("{:?}", face);

        context.set_stroke_style(&JsValue::from_str("blue"));
        context.begin_path();
        context
            .arc(
                face.mouth
            )
            .unwrap();
        context.stroke();
    }
}

// pseudo code
// initial draw draws faces
// moving range slider might draw faces in diff locations
// when mouse tracking is turned on, mouse coords

fn calculate_faces(context: &Context2d, skew_x: u32, skew_y: u32, range: u32) -> Faces {
    const WIDTH: u32 = 6;
    const HEIGHT: u32 = 6;
    const GLYPH_WIDTH: u32 = 130;
    let mut arcs = Vec::new();

    for a in 0..(WIDTH * HEIGHT) {
        for i in 1..10 {
            let y_offset = a / WIDTH * GLYPH_WIDTH + range;
            let x_offset = (a % WIDTH) * GLYPH_WIDTH + range;
            arcs.push(calculate_face(
                // &context,
                i as f64,
                x_offset as f64,
                y_offset as f64,
                skew_x as f64,
                skew_y as f64,
            ));
        }
    }

    Faces { arcs }
}

#[derive(Debug)]
struct Arc(f64, f64, f64, f64, f64);

#[derive(Debug)]
struct Face {
    mouth: Arc,
    left_eye: Arc,
    right_eye: Arc,
    outer_circle: Arc,
}

#[derive(Debug)]
struct Faces {
    arcs: Vec<Face>,
}

fn calculate_face(
    // context: &Context2d,
    iter: f64,
    x_offset: f64,
    y_offset: f64,
    x_skew: f64,
    y_skew: f64,
) -> Face {
    let offset_x = x_offset + (iter % 4.0) + (x_skew / 100.0 * iter);
    let offset_y = y_offset + (iter % 4.0) + (y_skew / 100.0 * iter);

    let mouth = Arc(
        75.0 + offset_x,
        75.0 + offset_y,
        35.0 + iter,
        0.0,
        f64::consts::PI,
    );
    let left_eye = Arc(
        60.0 + offset_x,
        65.0 + offset_y,
        5.0 + iter,
        0.0,
        f64::consts::PI * 2.0,
    );
    let right_eye = Arc(
        90.0 + offset_x,
        65.0 + offset_y,
        5.0 + iter,
        0.0,
        f64::consts::PI * 2.0,
    );
    let outer_circle = Arc(
        75.0 + offset_x,
        75.0 + offset_y,
        50.0 + iter,
        0.0,
        f64::consts::PI * 2.0,
    );

    Face {
        mouth,
        left_eye,
        right_eye,
        outer_circle,
    }
}

// fn draw_to_canvas(
//     context: &Context2d,
//     iter: f64,
//     x_offset: f64,
//     y_offset: f64,
//     x_skew: f64,
//     y_skew: f64,
// ) {
//     context.set_stroke_style(&JsValue::from_str("#f00"));
//     context.begin_path();
//     let offset_x = x_offset + (iter % 4.0) + (x_skew / 100.0 * iter);
//     let offset_y = y_offset + (iter % 4.0) + (y_skew / 100.0 * iter);

//     // Draw the outer circle.
//     context
//         .arc(
//             75.0 + offset_x,
//             75.0 + offset_y,
//             50.0 + iter,
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
//             75.0 + offset_x,
//             75.0 + offset_y,
//             35.0 + iter,
//             0.0,
//             f64::consts::PI,
//         )
//         .unwrap();
//     context.stroke();

//     // // Draw the left eye.
//     context.set_stroke_style(&JsValue::from_str("purple"));
//     context.begin_path();
//     context
//         .arc(
//             60.0 + offset_x,
//             65.0 + offset_y,
//             5.0 + iter,
//             0.0,
//             f64::consts::PI * 2.0,
//         )
//         .unwrap();
//     context.stroke();

//     // Draw the right eye.
//     context.set_stroke_style(&JsValue::from_str("green"));
//     context.begin_path();
//     context
//         .arc(
//             90.0 + offset_x,
//             65.0 + offset_y,
//             5.0 + iter,
//             0.0,
//             f64::consts::PI * 2.0,
//         )
//         .unwrap();

//     context.stroke();
// }
