mod consts;
mod draw;
mod snake;
mod util;
mod view;
extern crate console_error_panic_hook;
use std::{cell::RefCell, rc::Rc};

use draw::Canvas;
use snake::Game;
use view::draw_view;
use wasm_bindgen::{prelude::*, JsCast, UnwrapThrowExt};
use web_sys::window;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    let canvas = Rc::new(Canvas::new());
    let game = Rc::new(RefCell::new(Game::new()));
    let tick_closure = Closure::wrap(Box::new(move || {
        game.borrow_mut().tick();
        canvas.clear();
        draw_view(&canvas, &game.borrow());
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
