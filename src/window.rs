use crate::buffer;
use crate::buffer::Buf;
use crate::colors;
use crate::mode;
use crate::mode::Mod;
use pancurses::*;
use std::cell::RefCell;
use std::cmp::max;
use std::cmp::min;
use std::rc::Rc;

pub const DEFAULT_MODE: mode::Select = mode::Select;

pub struct Win {
    pub window: Window,
    pub buf: Rc<RefCell<Buf>>,
    pub drawing_pos: (usize, usize),
    pub last_pos: (usize, usize),
    pub pos: (usize, usize),
    pub pos_x: usize,
    pub redraw: bool,
}

impl Win {
    pub fn new(win: Window, bu: Rc<RefCell<Buf>>) -> Self {
        return Win {
            window: win,
            buf: bu,
            drawing_pos: (0, 0),
            last_pos: (0, 0),
            pos: (0, 0),
            pos_x: 0,
            redraw: true,
        };
    }

    pub fn render(&mut self) {
        if self.redraw || true {
            let mut buf = self.buf.borrow_mut();
            buf.attrs.clear();

            let mut top = self.pos;
            let mut bottom = self.last_pos;
            let mut changed = false; //All of this is just so it doesn't override the cursor
            if self.last_pos.1 < self.pos.1 {
                top = self.last_pos;
                bottom = self.pos;
                changed = true;
            }

            if top.1 != bottom.1 {
                let len = buf.content[top.1].len() - top.0;
                buf.attrs.insert((
                    top.0 + if changed { 0 } else { 1 },
                    top.1,
                    buffer::Attr {
                        size: len - if changed { 0 } else { 1 },
                        color: Some(colors::HIGHLIGHT),
                        flags: None,
                        link: None,
                    },
                ));

                buf.attrs.insert((
                    0,
                    bottom.1,
                    buffer::Attr {
                        size: bottom.0 + if changed { 0 } else { 1 },
                        color: Some(colors::HIGHLIGHT),
                        flags: None,
                        link: None,
                    },
                ));

                for x in (top.1 + 1)..bottom.1 {
                    let len = buf.content[x].len();
                    buf.attrs.insert((
                        0,
                        x,
                        buffer::Attr {
                            size: len,
                            color: Some(colors::HIGHLIGHT),
                            flags: None,
                            link: None,
                        },
                    ));
                }
            } else {
                buf.attrs.insert((
                    min(self.pos.0 + 1, self.last_pos.0),
                    top.1,
                    buffer::Attr {
                        size: max(top.0, bottom.0) - min(top.0, bottom.0),
                        color: Some(colors::HIGHLIGHT),
                        flags: None,
                        link: None,
                    },
                ));
            }

            self.window.clear();
            buf.write(
                &self.window,
                self.drawing_pos.1,
                self.drawing_pos.0,
                (
                    (self.window.get_max_x() - self.window.get_beg_x()) as usize,
                    (self.window.get_max_y() - self.window.get_beg_y()) as usize,
                ),
            );

            self.redraw = false;
        }
        self.window.mv(
            (self.pos.1 - self.drawing_pos.1) as i32,
            (self.pos.0 - self.drawing_pos.0) as i32,
        );
        self.window.refresh();
    }

    pub fn run(&mut self, mode: &mut Box<dyn Mod>, input: Input) -> bool {
        if unsafe { mode::ESCAPE } {
            *mode = Box::new(DEFAULT_MODE);
            unsafe {
                mode::ESCAPE = false;
            }
        }

        return mode.proc(self, input);
    }
}
