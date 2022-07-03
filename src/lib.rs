mod snake;
use js_sys::Function;
use snake::*;
use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast, UnwrapThrowExt};
use web_sys::{console, window, CanvasRenderingContext2d};

const UNIT_SIZE: f64 = 10.0;
const BOARD_SIZE: f64 = 200.0;

thread_local! {
    static GAME: Rc<RefCell<Game>> = Rc::new(RefCell::new(Game::new()));

    static TICK_CLOSURE: Closure<dyn FnMut()> = Closure::wrap(Box::new({
        let game = GAME.with(|game| game.clone());
        move || game.borrow_mut().tick()
    }));
}

#[wasm_bindgen(start)]
pub fn main() {
    render();
    TICK_CLOSURE.with(|tick_closure| {
        window()
            .unwrap_throw()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
                500,
            )
            .unwrap_throw();
    });
}

pub fn render() {
    let document = web_sys::window().unwrap_throw().document().unwrap_throw();
    let canvas = document.get_element_by_id("canvas").unwrap_throw();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap_throw();
    let ctx = canvas
        .get_context("2d")
        .unwrap_throw()
        .unwrap_throw()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap_throw();

    ctx.fill_rect(30.0, 30.0, 20.0, 20.0);

    draw_unit(ctx, 5, 5, "#000");
}

pub fn draw_unit(ctx: CanvasRenderingContext2d, x: i32, y: i32, color: &str) {
    // ctx.set_fill_style(&JsValue::from_str(color));
    ctx.fill_rect(
        x as f64 * UNIT_SIZE,
        (BOARD_SIZE - 1.0 - y as f64) * UNIT_SIZE,
        UNIT_SIZE,
        UNIT_SIZE,
    );
    console::log_1(&"Got here".into());
}
