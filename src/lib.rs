mod snake;
use js_sys::Function;
use snake::*;
use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast, UnwrapThrowExt};
use web_sys::window;

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
    let context = canvas
        .get_context("2d")
        .unwrap_throw()
        .unwrap_throw()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap_throw();
    context.begin_path();
    // Draw the outer circle
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap_throw();

    // Draw the mouth
    context.move_to(110.0, 75.0);
    context
        .arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI)
        .unwrap_throw();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}
