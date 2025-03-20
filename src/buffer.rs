use bitflags::bitflags;
use pancurses::*;

pub mod bindr;

bitflags! {
    pub struct Flags: u32 {
        const NONE = 0;
        const SOME = 0;
    }
}

pub struct Attr {
    pub color: i16,
    pub flags: u32,
    pub size: usize,
}

pub struct Buf<T: bindr::Bindr> {
    pub content: String,
    pub attrs: std::vec::Vec<Attr>,
    pub flags: Flags,
    pub bind: T,
}

impl<T: bindr::Bindr> Buf<T> {
    pub fn new(content: &str, flag: Flags, bin: T) -> Self {
        return Buf {
            content: String::from(content),
            attrs: vec![],
            flags: flag,
            bind: bin,
        };
    }

    //Unicode baby!!!!! (27 lines only btw) (don't touch the iterator shit, that took forever)
    pub fn write(&self, win: &Window) {
        let mut iter = self.content.char_indices();
        let mut start = match iter.next() {
            Some((x, _)) => x,
            None => return,
        };

        for c in self.attrs.iter() {
            let end = match iter.nth(c.size - 1) {
                Some((x, _)) => x,
                None => break,
            };

            win.attrset(c.flags);
            win.color_set(c.color);
            win.addstr(&self.content[start..end]);

            start = end;
        }

        win.color_set(-1);
        win.attrset(A_NORMAL);

        win.addstr(&self.content[start..]);
    }
}
