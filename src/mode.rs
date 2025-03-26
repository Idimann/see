use crate::window::Win;
use pancurses::*;

pub trait Mod {
    fn proc(&mut self, win: &mut Win, input: Input) -> bool;
}

macro_rules! to_control {
    ($e:expr) => {
        (0x1f & ($e as u8)) as char
    };
}

const ESC: char = to_control!('[');
const C_SPACE: char = to_control!(' ');
pub static mut ESCAPE: bool = false;

pub fn ctrl_c() {
    unsafe {
        ESCAPE = true;
    }
}

pub struct Insert;
impl Mod for Insert {
    fn proc(&mut self, win: &mut Win, input: Input) -> bool {
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
            }
            Input::Character(ESC) | Input::Character(C_SPACE) => {
                ctrl_c();
            }
            Input::Character(x) => {
                if !x.is_ascii_control() {
                    {
                        let mut buf = win.buf.borrow_mut();
                        buf.content[win.pos.1].insert(win.pos.0, x);
                    }
                    win.move_forward();
                }
            }
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
            }
            _ => (),
        };

        win.redraw = true;
        return false;
    }
}

//u => Up
//d => Down
//f => Forward
//b => Backward
//s => Start of line
//e => End of line
//o => to Other bracket
//p => set Point (mark in vim)
//j => Jump to point
//w => select in Word
//l => select in Line
//k => enable Keep mode (visual mode in vim) (keeps selection)
//a => Around (switches pos with last_pos)
//m => Match (search using regex)
//n => next Match (while searching)
//; => command mode (like vims ':')
//v => Vertical keep mode (visual line mode in vim) (just like keep mode with lines)
//Still unmapped => q, r, t, y, i, g, z, x, c, all symbols and capital letters
pub struct Select;
impl Mod for Select {
    fn proc(&mut self, win: &mut Win, input: Input) -> bool {
        match input {
            Input::Character('u') => {
                win.last_pos = win.pos;
                if win.move_up().0 {
                    win.redraw = true;
                }
            }
            Input::Character('d') => {
                win.last_pos = win.pos;
                if win.move_down().0 {
                    win.redraw = true;
                }
            }
            Input::Character('b') => {
                win.last_pos = win.pos;
                if win.move_backward().0 {
                    win.redraw = true;
                }
            }
            Input::Character('f') => {
                win.last_pos = win.pos;
                if win.move_forward().0 {
                    win.redraw = true;
                }
            }
            Input::Character('e') => {
                win.last_pos = win.pos;
                win.move_end();
            }
            Input::Character('s') => {
                win.last_pos = win.pos;
                win.move_start();
            }
            _ => (),
        };

        return false;
    }
}
