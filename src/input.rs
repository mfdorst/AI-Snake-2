use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, KeyboardEvent};

use crate::snake::{Direction, Game};

pub fn register_keyboard_listener(game: &Rc<RefCell<Game>>) -> Option<()> {
    let document = window()?.document()?;

    let listener = Closure::wrap(Box::new({
        let game = game.clone();
        move |event: KeyboardEvent| match event.key().as_str() {
            "ArrowUp" => game.borrow_mut().change_direction(Direction::Up),
            "ArrowDown" => game.borrow_mut().change_direction(Direction::Down),
            "ArrowLeft" => game.borrow_mut().change_direction(Direction::Left),
            "ArrowRight" => game.borrow_mut().change_direction(Direction::Right),
            " " => game.borrow_mut().toggle_pause_play(),
            _ => {}
        }
    }) as Box<dyn FnMut(_)>);

    document
        .add_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref())
        .unwrap();

    listener.forget();
    Some(())
}
