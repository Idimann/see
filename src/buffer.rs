use bitflags::bitflags;
use pancurses::*;
use std::vec::Vec;

pub mod bindr;

bitflags! {
    pub struct Flags: u32 {
        const none = 0;
        const read_only = 1 << 0;
        const modified = 1 << 1;
    }
}

pub struct Attr {
    pub color: i16,
    pub flags: u32,
    pub size: usize,
}

pub struct Buf {
    pub content: String,
    pub attrs: Vec<Attr>,
    pub flags: Flags,
    pub bind: Box<dyn bindr::Bindr>,
}

impl Buf {
    pub fn new(content: &str, flag: Flags, bin: Box<dyn bindr::Bindr>) -> Self {
        return Buf {
            content: String::from(content),
            attrs: vec![],
            flags: flag,
            bind: bin,
        };
    }

    pub fn new_load(flag: Flags, bin: Box<dyn bindr::Bindr>) -> Self {
        let mut ret = Buf {
            content: String::from(""),
            attrs: vec![],
            flags: flag,
            bind: bin,
        };
        return match ret.bind.load(&mut ret.content) {
            Ok(_) => ret,
            Err(err) => {
                ret.content = String::from(format!("Can't read from source, Error: {}", err));
                ret
            }
        };
    }

    //Unicode baby!!!!! (27 lines only btw) (don't touch the iterator shit, that took forever)
    pub fn write(&self, win: &Window, pos: usize) {
        let mut iter = self.content.char_indices();
        let mut start = match iter.nth(pos) {
            Some((x, _)) => x,
            None => return,
        };

        for c in self.attrs.iter() {
            win.attrset(c.flags);
            win.color_set(c.color);
            match iter.nth(c.size - 1) {
                Some((end, _)) => {
                    win.addstr(&self.content[start..end]);
                    start = end;
                }
                None => {
                    win.addstr(&self.content[start..]);
                    start = self.content.len();
                    break;
                }
            };
        }

        if start < self.content.len() {
            win.color_set(-1);
            win.attrset(A_NORMAL);

            win.addstr(&self.content[start..]);
        }
    }
}
