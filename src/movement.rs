use crate::window::Win;
use std::cmp::min;
use crate::settings;

impl Win {
    pub fn line_len(&self) -> usize {
        return self.buf.borrow().content[self.pos.1].len();
    }

    pub fn line_end_dist(&self) -> usize {
        return self.line_len() - self.pos.0 - 1;
    }

    pub fn move_backward(&mut self) -> bool {
        if self.pos.0 != 0 {
            self.pos.0 -= 1;
            self.pos_x = self.pos.0;
            return true;
        }

        if self.move_up() {
            self.pos.0 = self.line_len() - 1;
            self.pos_x = self.pos.0;
            return true;
        }

        return false;
    }

    pub fn move_forward(&mut self) -> bool {
        if self.line_end_dist() != 0 {
            self.pos.0 += 1;
            self.pos_x = self.pos.0;
            return true;
        }

        if self.move_down() {
            self.pos.0 = 0;
            self.pos_x = self.pos.0;
            return true;
        }
        return false;
    }

    pub fn move_up(&mut self) -> bool {
        if self.pos.1 == 0 {
            return false;
        }
        self.pos.1 -= 1;
        self.pos.0 = min(self.pos_x, self.line_len() - 1);
        if self.pos.1 - self.drawing_pos < settings::PAD && self.drawing_pos > 0 {
            self.drawing_pos -= 1;
        }
        return true;
    }

    pub fn move_down(&mut self) -> bool {
        if self.pos.1 == self.buf.borrow().content.len() - 1 {
            return false;
        }
        self.pos.1 += 1;
        self.pos.0 = min(self.pos_x, self.line_len() - 1);
        if self.pos.1 - self.drawing_pos
            >= (self.window.get_max_y() - self.window.get_beg_y()) as usize - settings::PAD
            && self.drawing_pos
                < self.buf.borrow().content.len()
                    - (self.window.get_max_y() - self.window.get_beg_y()) as usize
        {
            self.drawing_pos += 1;
        }
        return true;
    }

    pub fn move_end(&mut self) {
        self.pos.0 = self.line_len() - 1;
        self.pos_x = self.pos.0;
    }

    pub fn move_start(&mut self) {
        self.pos.0 = 0;
        self.pos_x = self.pos.0;
        self.skip_whitespace(false);
    }

    pub fn is_whitespace(&self) -> bool {
        return match self.buf.borrow().content[self.pos.1]
            .chars()
            .nth(self.pos.0)
        {
            Some(x) => x.is_whitespace(),
            None => false,
        };
    }

    pub fn skip_whitespace(&mut self, reverse: bool) {
        while self.is_whitespace() {
            if reverse {
                self.move_backward();
            } else {
                self.move_forward();
            }
        }
    }
}
