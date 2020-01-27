use std::{thread, time};
use std::io::{stdin};

mod random;
mod snake;

pub use snake::FrameBuffer;

#[derive(PartialEq)]
pub enum Result {
    Win,
    Loss,
}

pub struct Game {
    _random: random::Random,
}

pub fn create_game() -> Game {
    Game {_random: random::Random {seed: 3}} // make seed based on tick count
}

impl Game {
    pub fn run(&mut self, mut fb: snake::FrameBuffer) -> Result {
        let mut s = snake::create_snake(
            (self._random.next() % 40) as u8,
            (self._random.next() % 20) as u8,
            snake::Direction::Down, // todo make random
        );

        let mut food = self.make_food(&s);

        loop {
            let stdin = stdin();
            fb.clear();
            if !s.update() {
                s.draw(&mut fb);
                return Result::Loss;
            }

            s.draw(&mut fb);
            // key stuff

            if s.hit_test(food.0, food.1) {
                if s.extend() {
                    food = self.make_food(&s);
                } else {
                    return Result::Win;
                }
            }

            fb.set_pixel(food.0, food.1, '*');
            fb.render();

            thread::sleep(time::Duration::from_secs(1));
            // return Result::Loss;
        }
    }

    fn make_food(&mut self, snake: &snake::Snake) -> (u8, u8) {
        let mut food_x = (self._random.next() % 20) as u8;
        let mut food_y = (self._random.next() % 40) as u8;
        while snake.hit_test(food_x, food_y) { // todo do while
            food_x = (self._random.next() % 20) as u8;
            food_y = (self._random.next() % 40) as u8;
        }

        (food_x, food_y)
    }
}
