mod consts;
mod draw;
mod snake;
mod util;
extern crate console_error_panic_hook;
use draw::Canvas;
use wasm_bindgen::{prelude::*, JsCast, UnwrapThrowExt};
use web_sys::window;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    let mut x = 0;
    let canvas = Canvas::new();
    let tick_closure = Closure::wrap(Box::new(move || {
        canvas.draw_cell(x, x, "#fff");
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
