use crate::buffer::Buf;
use crate::window::Win;
use pancurses::*;
use std::cell::RefCell;
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub enum RunErr {}

impl Error for RunErr {}
impl Display for RunErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:?}", self);
    }
}

pub fn run(bufs: &mut Vec<Rc<RefCell<Buf>>>, wins: &mut Vec<Win>) -> Result<(), RunErr> {
    'mainLoop: loop {
        for win in wins.iter_mut() {
            win.render();

            match win.window.getch() {
                Some(Input::Character('q')) => {
                    break 'mainLoop;
                },
                Some(Input::KeyLeft) => win.cursor_pos.0 -= 1,
                Some(Input::KeyRight) => win.cursor_pos.0 += 1,
                Some(Input::KeyDown) => win.cursor_pos.1 += 1,
                Some(Input::KeyUp) => win.cursor_pos.1 -= 1,
                Some(Input::Character(x)) => {
                    win.buf.borrow_mut().content.push(x);
                },
                Some(_) => (),
                None => (),
            };
        }
    }
    return Ok(());
}
