use crate::buffer::Buf;
use pancurses::*;
use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;

// This is stupid
pub struct Win {
    pub window: Window,
    pub buf: Rc<RefCell<Buf>>,
    drawing_pos: usize,
    pos: (usize, usize),
    pos_x: usize,
}

impl Win {
    pub fn new(win: Window, bu: Rc<RefCell<Buf>>) -> Self {
        return Win {
            window: win,
            buf: bu,
            drawing_pos: 0,
            pos: (0, 0),
            pos_x: 0,
        };
    }

    pub fn render(&self) {
        // self.window.mv(0, 0);
        self.window.clear();
        self.buf.borrow().write(&self.window, self.drawing_pos);
        self.window.mv(self.pos.1 as i32, self.pos.0 as i32);
        self.window.refresh();
    }

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
        return true;
    }

    pub fn move_down(&mut self) -> bool {
        if self.pos.1 == self.buf.borrow().content.len() - 1 {
            return false;
        }
        self.pos.1 += 1;
        self.pos.0 = min(self.pos_x, self.line_len() - 1);
        return true;
    }
}
