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
    pub content: Vec<String>,
    attrs: Vec<Attr>,
    flags: Flags,
    bind: Box<dyn bindr::Bindr>,
}

impl Buf {
    pub fn new(content: &str, flag: Flags, bin: Box<dyn bindr::Bindr>) -> Self {
        return Buf {
            content: content
                .to_string()
                .split_inclusive('\n')
                .map(|x| x.to_string())
                .collect(),
            attrs: vec![],
            flags: flag,
            bind: bin,
        };
    }

    pub fn new_load(flag: Flags, bin: Box<dyn bindr::Bindr>) -> Self {
        let mut ret = Buf {
            content: vec![String::from("")],
            attrs: vec![],
            flags: flag,
            bind: bin,
        };
        return match ret.bind.load(&mut ret.content) {
            Ok(_) => ret,
            Err(err) => {
                ret.content = vec![String::from(format!(
                    "Can't read from source, Error: {}",
                    err
                ))];
                ret
            }
        };
    }

    //Unicode baby!!!!! (27 lines only btw) (don't touch the iterator shit, that took forever)
    pub fn write(&self, win: &Window, pos: usize) {
        let text = self.content[pos..].join("");

        let mut iter = text.char_indices();
        let mut start = match iter.next() {
            Some((x, _)) => x,
            None => return,
        };

        for c in self.attrs.iter() {
            win.attrset(c.flags);
            win.color_set(c.color);
            match iter.nth(c.size - 1) {
                Some((end, _)) => {
                    win.addstr(&text[start..end]);
                    start = end;
                }
                None => {
                    win.addstr(&text[start..]);
                    start = self.content.len();
                    break;
                }
            };
        }

        if start < self.content.len() {
            win.color_set(-1);
            win.attrset(A_NORMAL);

            win.addstr(&text[start..]);
        }
    }
}
