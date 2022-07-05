mod snake;
use std::f64;
use wasm_bindgen::{prelude::*, JsCast, UnwrapThrowExt};
use web_sys::{window, CanvasRenderingContext2d};

const CELLS_PER_CANVAS: i32 = 50;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub fn main() {
    render();
    let tick_closure = Closure::wrap(Box::new(|| console_log!("Hello")) as Box<dyn FnMut()>);

    window()
        .unwrap_throw()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            tick_closure.as_ref().unchecked_ref(),
            500,
        )
        .unwrap_throw();
    tick_closure.forget();
}

pub fn render() {
    let document = web_sys::window().unwrap_throw().document().unwrap_throw();
    let canvas = document
        .query_selector("canvas")
        .unwrap_throw()
        .unwrap_throw();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap_throw();
    let ctx = canvas
        .get_context("2d")
        .unwrap_throw()
        .unwrap_throw()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap_throw();
    let canvas_height = canvas.offset_height();
    let cell_height = canvas_height / CELLS_PER_CANVAS;

    draw_unit(ctx, 5, 5, cell_height, "#fff");
}

pub fn draw_unit(ctx: CanvasRenderingContext2d, x: i32, y: i32, cell_height: i32, color: &str) {
    ctx.set_fill_style(&JsValue::from_str(color));
    ctx.fill_rect(
        (x * cell_height) as f64,
        (y * cell_height) as f64,
        cell_height as f64,
        cell_height as f64,
    );
}
