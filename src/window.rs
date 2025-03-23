use crate::buffer::Buf;
use crate::mode;
use crate::mode::Mod;
use pancurses::*;
use std::cell::RefCell;
use std::rc::Rc;

const DEFAULT_MODE: Mod = mode::select;

// This is stupid
pub struct Win {
    pub window: Window,
    pub buf: Rc<RefCell<Buf>>,
    pub drawing_pos: usize,
    pub pos: (usize, usize),
    pub pos_x: usize,
    pub mode: Mod,
}

impl Win {
    pub fn new(win: Window, bu: Rc<RefCell<Buf>>) -> Self {
        return Win {
            window: win,
            buf: bu,
            drawing_pos: 0,
            pos: (0, 0),
            pos_x: 0,
            mode: mode::insert,
        };
    }

    pub fn render(&self) {
        // self.window.mv(0, 0);
        self.window.clear();
        self.buf.borrow().write(&self.window, self.drawing_pos);
        self.window
            .mv((self.pos.1 - self.drawing_pos) as i32, self.pos.0 as i32);
        self.window.refresh();
    }

    pub fn run(&mut self, input: Input) -> bool {
        if unsafe { mode::ESCAPE } {
            self.mode = DEFAULT_MODE;
            unsafe { mode::ESCAPE = false; }
        }
        return (self.mode)(self, input);
    }
}
