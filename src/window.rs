use crate::buffer::Buf;
use pancurses::*;
use std::cell::RefCell;
use std::rc::Rc;

// This is stupid
pub struct Win {
    pub window: Window,
    pub buf: Rc<RefCell<Buf>>,
    pub drawing_pos: usize,
    pub cursor_pos: (i32, i32),
}

impl Win {
    pub fn new(win: Window, bu: Rc<RefCell<Buf>>) -> Self {
        return Win {
            window: win,
            buf: bu,
            drawing_pos: 0,
            cursor_pos: (0, 0),
        };
    }

    pub fn render(&self) {
        // self.window.mv(0, 0);
        self.window.clear();
        self.buf.borrow().write(&self.window, self.drawing_pos);
        self.window.mv(self.cursor_pos.1, self.cursor_pos.0);
        self.window.refresh();
    }
}
