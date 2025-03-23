use crate::window::Win;
use pancurses::*;

pub type Mod = fn(&mut Win, Input) -> bool;

macro_rules! to_control {
    ($e:expr) => {
        (0x1f & ($e as u8)) as char
    };
}

const ESC: char = to_control!('[');
pub static mut ESCAPE: bool = false;

pub fn ctrl_c() {
    unsafe { ESCAPE = true; }
}

pub fn insert(win: &mut Win, input: Input) -> bool {
    match input {
        Input::Character('\n') => {
            {
                let mut buf = win.buf.borrow_mut();
                let s_1 = buf.content[win.pos.1][..win.pos.0].to_string();
                let s_2 = buf.content[win.pos.1][win.pos.0..].to_string();
                buf.content[win.pos.1] = s_1 + "\n";
                buf.content.insert(win.pos.1 + 1, s_2);
            }
            win.move_forward();
        },
        Input::Character(ESC) => {
            unsafe { ESCAPE = true; }
        },
        Input::Character(x) => {
            if !x.is_ascii_control() {
                {
                    let mut buf = win.buf.borrow_mut();
                    buf.content[win.pos.1].insert(win.pos.0, x);
                }
                win.move_forward();
            }
        },
        Input::KeyBackspace => {
            if win.pos.0 == 0 {
                if win.pos.1 != 0 {
                    let len = win.buf.borrow().content[win.pos.1 - 1].len();
                    {
                        let mut buf = win.buf.borrow_mut();
                        buf.content[win.pos.1 - 1].pop();
                        buf.content[win.pos.1 - 1] =
                            buf.content[(win.pos.1 - 1)..=win.pos.1].join("");
                        buf.content.remove(win.pos.1);
                    }

                    win.move_up();
                    win.pos.0 = len - 1;
                    win.pos_x = win.pos.0;
                }
            } else {
                {
                    let mut buf = win.buf.borrow_mut();
                    buf.content[win.pos.1].remove(win.pos.0 - 1);
                }
                win.move_backward();
            }

        },
        _ => (),
    };

    return false;
}

pub fn select(win: &mut Win, input: Input) -> bool {
    match input {
        Input::Character('u') => {
            win.move_up();
        },
        Input::Character('d') => {
            win.move_down();
        },
        Input::Character('b') => {
            win.move_backward();
        },
        Input::Character('f') => {
            win.move_forward();
        },
        Input::Character('e') => {
            win.move_end();
        },
        Input::Character('s') => {
            win.move_start();
        },
        _ => (),
    };

    return false;
}
