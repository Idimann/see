use bitflags::bitflags;
use pancurses::*;
use sorted_vec::partial::SortedVec;
use std::cmp::min;

pub mod bindr;

bitflags! {
    pub struct Flags: u32 {
        const none = 0;
        const read_only = 1 << 0;
        const modified = 1 << 1;
    }
}

#[derive(PartialEq, PartialOrd)]
pub struct Attr {
    pub size: usize,
    pub color: Option<i16>,
    pub flags: Option<u32>,
    pub link: Option<usize>,
}

pub struct Buf {
    pub content: Vec<String>,
    pub attrs: SortedVec<(usize, usize, Attr)>,
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
            attrs: SortedVec::new(),
            flags: flag,
            bind: bin,
        };
    }

    pub fn new_load(flag: Flags, bin: Box<dyn bindr::Bindr>) -> Self {
        let mut ret = Buf {
            content: vec![String::from("")],
            attrs: SortedVec::new(),
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

    //This used to be 27 lines (sadge)
    pub fn write(&self, win: &Window, pos: usize, xpos: usize, size: (usize, usize)) {
        let contents = &self.content[pos..min(pos + size.1, self.content.len())];

        for x in contents.iter() {
            win.addstr(&x[xpos..min(xpos + size.0, x.len())]);
        }

        for (x, y, a) in self.attrs.iter() {
            if *y < pos || *y > pos + size.1 {
                continue;
            }
            if *x < xpos || *x > xpos + size.0 {
                continue;
            }

            win.mvchgat(
                (y - pos) as i32,
                (x - xpos) as i32,
                a.size as i32,
                match a.flags {
                    Some(x) => x,
                    None => A_NORMAL,
                },
                match a.color {
                    Some(x) => x,
                    None => 0,
                },
            );
        }
    }
}
