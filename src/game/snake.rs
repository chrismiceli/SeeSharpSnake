mod framebuffer;

pub use framebuffer::FrameBuffer;

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Snake {
    _length: i8,
    _body: [Part; 30],
    _direction: Direction,
    _old_direction: Direction,
}

pub fn create_snake(x: u8, y: u8, direction: Direction) -> Snake {
    let mut snake = Snake {
        _length: 1,
        _old_direction: direction,
        _direction: direction,
        _body: [create_part(0, 0, ' '); 30],
    };
    snake._body[0] = Part {x: x, y: y, character: direction_to_char(direction, direction)};
    snake
}

impl Snake {
    fn set_direction(&mut self, direction: Direction) {
        if self._old_direction != self._direction {
            self._old_direction = self._direction;
        }

        if self._direction as i32 - direction as i32 != 2
            && direction as i32 - self._direction as i32 != 2
        {
            self._direction = direction;
        }
    }

    pub fn update(&mut self) -> bool {
        let old_head = &self._body[0];
        let new_head_x = {
            if self._direction == Direction::Left {
                if old_head.x == 0 {
                    40 - 1
                } else {
                    old_head.x - 1
                }
            } else if self._direction == Direction::Right {
                (old_head.x + 1) % 40
            } else {
                old_head.x
            }
        };
        let new_head_y = {
            if self._direction == Direction::Up {
                if old_head.y == 0 {
                    20 - 1
                } else {
                    old_head.y - 1
                }
            } else if self._direction == Direction::Down {
                (old_head.y + 1) % 20
            } else {
                old_head.y
            }
        };
        let new_head = Part {
            x: new_head_x,
            y: new_head_y,
            character: direction_to_char(self._direction, self._direction),
        };
        let mut result = true;
        for i in 0..self._length {
            let current = &self._body[i as usize];
            if current.x == new_head.x && current.y == new_head.y {
                result = false;
            }
        }

        self._body[0] = *old_head;
        let mut i = self._length - 2;
        while i >= 0 {
            self._body[(i + 1) as usize] = self._body[i as usize];
            i = i - 1;
        }
        self._body[0] = new_head;
        self._old_direction = self._direction;
        result
    }

    pub fn extend(&mut self) -> bool {
        if self._length < 30 {
            self._length = self._length + 1;
            return true;
        }

        false
    }

    pub fn draw(&self, fb: &mut framebuffer::FrameBuffer) {
        for i in 0..self._length {
            let p = &self._body[i as usize];
            fb.set_pixel(p.x, p.y, p.character);
        }
    }

    pub fn hit_test(&self, x: u8, y: u8) -> bool {
        for i in 0..self._length {
            let current = &self._body[i as usize];
            if current.x == x && current.y == y {
                return true;
            }
        }

        false
    }
}

fn direction_to_char(old_direction: Direction, new_direction: Direction) -> char {
    let direction_change_to_char = String::from("│┌?┐┘─┐??└│┘└?┌─");
    direction_change_to_char
        .chars()
        .nth((old_direction as usize) * 4 + (new_direction as usize))
        .unwrap()
}

fn create_part(x: u8, y: u8, character: char) -> Part {
    Part {
        x: x,
        y: y,
        character: character,
    }
}

#[derive(Clone, Copy)]
struct Part {
    pub x: u8,
    pub y: u8,
    pub character: char,
}
