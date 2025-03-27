use pancurses::init_color;
use pancurses::init_pair;
use std::cmp::min;
use std::mem::MaybeUninit;

pub mod default_theme;

pub struct Col {
    r: i16,
    g: i16,
    b: i16,
}

impl Col {
    pub fn new(r: i16, g: i16, b: i16) -> Self {
        return Col { r, g, b };
    }

    pub fn new_rgb(r: i16, g: i16, b: i16) -> Self {
        return Col {
            r: min(r * 4, 1000),
            g: min(g * 4, 1000),
            b: min(b * 4, 1000),
        };
    }
}

type Theme = Vec<(i16, Option<Col>, Option<Col>)>;

fn belong(x: i16) -> (i16, i16) {
    return (x, MAX_COLORS + x);
}

fn ip(x: i16, b1: i16, b2: i16) {
    init_pair(x, b1, b2);
}

pub fn apply_theme(theme: &Theme) {
    for x in theme.iter() {
        let b = belong(x.0);
        ip(
            x.0,
            match x.1 {
                Some(_) => b.0,
                None => -1,
            },
            match x.2 {
                Some(_) => b.1,
                None => -1,
            },
        );

        match &x.1 {
            Some(x) => {
                init_color(b.0, x.r, x.g, x.b);
            }
            None => (),
        };
        match &x.2 {
            Some(x) => {
                init_color(b.1, x.r, x.g, x.b);
            }
            None => (),
        };
    }
}

const MAX_COLORS: i16 = 127; //255 is the max for colors (max 256 colors in term)

//Color groups (these shouldn't start at 0)
pub const HIGHLIGHT: i16 = 1;
