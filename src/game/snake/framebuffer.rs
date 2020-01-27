pub struct FrameBuffer {
    pub width: u8,
    pub height: u8,
    pub chars: [char; 800],
}

impl FrameBuffer {
    pub fn set_pixel(&mut self, x: u8, y: u8, character: char) {
        let index = (y as u16) * (self.width as u16) + (x as u16); 
        self.chars[index as usize] = character;
    }

    pub fn clear(&mut self) {
        for number in 0..799 {
            self.chars[number] = ' ';
        }
    }

    pub fn clear_screen(&self) {
        print!("{}", termion::clear::All);
    }

    pub fn render(&self) {
        let snake_color = termion::color::Green;
        print!(
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::color::Fg(snake_color)
        );
        for _number in 0..self.width + 2 {
            print!("-");
        }
        println!();
        for number in 0..799 {
            let c = self.chars[number];
            if c == '*' || (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') {
                if c == '*' {
                    print!("{}{}", termion::color::Fg(termion::color::Red), c);
                } else {
                    print!("{}{}", termion::color::Fg(termion::color::White), c);
                }
                print!("{}", termion::color::Fg(snake_color));
            } else {
                print!("{}", c);
            }
            if number % (self.width as usize) == 1 {
                print!("|");
            } else if number % (self.width as usize) == 0 {
                print!(
                    "|{}",
                    termion::cursor::Goto(1, (((number as u16) / (self.width as u16)) + 2) as u16)
                );
            }
        }
        println!("|");
        for _number in 0..self.width + 2 {
            print!("-");
        }
        println!();
    }
}
