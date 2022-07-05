mod snake;
mod util;
use std::f64;
// use util::console_log;
use wasm_bindgen::{prelude::*, JsCast, UnwrapThrowExt};
use web_sys::{window, CanvasRenderingContext2d};

const CELLS_PER_CANVAS: i32 = 50;

#[wasm_bindgen(start)]
pub fn main() {
    let mut x = 0;
    let tick_closure = Closure::wrap(Box::new(move || {
        render_cell(x, x);
        x += 1;
    }) as Box<dyn FnMut()>);

    window()
        .unwrap_throw()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            tick_closure.as_ref().unchecked_ref(),
            500,
        )
        .unwrap_throw();
    tick_closure.forget();
}

pub fn render_cell(x: i32, y: i32) {
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
    let cell_height = canvas_height as f64 / CELLS_PER_CANVAS as f64;

    draw_unit(ctx, x, y, cell_height, "#fff");
}

pub fn draw_unit(ctx: CanvasRenderingContext2d, x: i32, y: i32, cell_height: f64, color: &str) {
    ctx.set_fill_style(&JsValue::from_str(color));
    ctx.fill_rect(
        x as f64 * cell_height,
        y as f64 * cell_height,
        cell_height as f64,
        cell_height as f64,
    );
}
