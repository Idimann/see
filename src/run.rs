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

pub fn run(
    std: &Window,
    bufs: &mut Vec<Rc<RefCell<Buf>>>,
    wins: &mut Vec<Win>,
) -> Result<(), RunErr> {
    'mainLoop: loop {
        for win in wins.iter_mut() {
            win.render();
        }

        let input = match std.getch() {
            Some(x) => x,
            None => continue,
        };

        for win in wins.iter_mut() {
            if win.run(input) {
                break 'mainLoop;
            }
        }
    }

    return Ok(());
}
