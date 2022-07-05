use crate::consts::*;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

pub struct Canvas {
    context: CanvasRenderingContext2d,
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
        canvas.set_height(CANVAS_HEIGHT);
        canvas.set_width(CANVAS_HEIGHT);
        Self { context }
    }

    pub fn draw_cell(&self, x: i32, y: i32, color: &str) {
        self.context.set_fill_style(&JsValue::from_str(color));
        self.context.fill_rect(
            x as f64 * CELL_HEIGHT,
            y as f64 * CELL_HEIGHT,
            CELL_HEIGHT as f64,
            CELL_HEIGHT as f64,
        );
    }

    pub fn clear(&self) {
        self.context
            .clear_rect(0.0, 0.0, CANVAS_HEIGHT as f64, CANVAS_HEIGHT as f64);
    }
}
