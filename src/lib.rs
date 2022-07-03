use std::collections::VecDeque;

const WIDTH: i32 = 10;
const HEIGHT: i32 = 10;

pub struct Position(i32, i32);

pub struct Game {
    snake: VecDeque<Position>,
    current_direction: Direction,
    next_direction: Direction,
    food: Vec<Position>,
    play_state: PlayState,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum PlayState {
    Paused,
    Playing,
    Lost,
}

impl Position {
    pub fn next(&self, direction: Direction) -> Option<Position> {
        let &Position(x, y) = self;
        let (x, y) = match direction {
            Up => (x, y + 1),
            Down => (x, y - 1),
            Left => (x - 1, y),
            Right => (x + 1, y),
        };
        if x < 0 || y < 0 || x >= WIDTH || y >= HEIGHT {
            None
        } else {
            Some(Position(x, y))
        }
    }
}

impl Direction {
    fn opposite(&self) -> Direction {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            snake: VecDeque::from([
                Position(WIDTH - 4, HEIGHT / 2),
                Position(WIDTH - 5, HEIGHT / 2),
                Position(WIDTH - 6, HEIGHT / 2),
            ]),
            current_direction: Direction::Right,
            next_direction: Direction::Right,
            food: vec![],
            play_state: PlayState::Playing,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if direction == self.current_direction.opposite() {
            return;
        }
        self.next_direction = direction;
    }

    pub fn tick(&mut self) {
        // Finalize the direction the snake will move this tick
        self.current_direction = self.next_direction;

        // Move snake forward
        let front = self.snake.front().expect("Snake body should not be empty");
        if let Some(next_front) = front.next(self.current_direction) {
            // TODO: check for body collisions
            self.snake.push_front(next_front);
        } else {
            // Snake ran off the screen
            self.play_state = PlayState::Lost;
        }
        // TODO: If snake ate, don't pop back
        self.snake.pop_back();
    }

    pub fn slither(&mut self, direction: Direction) {
        if let Some(front) = self.snake.front() {
            if let Some(next_front) = front.next(direction) {
                self.snake.push_front(next_front);
            }
        }
    }
}
