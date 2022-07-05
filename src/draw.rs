use crate::consts::*;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

pub struct Canvas {
    context: CanvasRenderingContext2d,
    cell_height: f64,
}

impl Canvas {
    pub fn new() -> Self {
        let document = window().unwrap().document().unwrap();
        let canvas = document.query_selector("canvas").unwrap().unwrap();
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        let height = canvas.offset_height();
        let cell_height = height as f64 / CELLS_PER_CANVAS as f64;
        Self {
            context,
            cell_height,
        }
    }

    pub fn draw_cell(&self, x: i32, y: i32, color: &str) {
        self.context.set_fill_style(&JsValue::from_str(color));
        self.context.fill_rect(
            x as f64 * self.cell_height,
            y as f64 * self.cell_height,
            self.cell_height as f64,
            self.cell_height as f64,
        );
    }
}
