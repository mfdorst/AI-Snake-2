const WIDTH: usize = 10;
const HEIGHT: usize = 10;

pub type Position = (usize, usize);

pub struct Game {
    width: usize,
    height: usize,
    snake: Vec<Position>,
    current_direction: Direction,
    next_direction: Direction,
    food: Vec<Position>,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Game {
    pub fn new() -> Self {
        Self {
            width: WIDTH,
            height: HEIGHT,
            snake: vec![
                (WIDTH - 4, HEIGHT / 2),
                (WIDTH - 5, HEIGHT / 2),
                (WIDTH - 6, HEIGHT / 2),
            ],
            current_direction: Direction::Right,
            next_direction: Direction::Right,
            food: vec![],
        }
    }
}
