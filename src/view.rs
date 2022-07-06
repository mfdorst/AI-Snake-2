use crate::draw::Canvas;
use crate::snake::{Game, Position};

pub fn draw_view(canvas: &Canvas, game: &Game) {
    for &Position(x, y) in game.snake.iter() {
        canvas.draw_cell(x, y, "#fff");
    }
    for &Position(x, y) in game.food.iter() {
        canvas.draw_cell(x, y, "#f00");
    }
}
