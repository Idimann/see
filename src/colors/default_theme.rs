use crate::colors::Theme;
use crate::colors::Col;
use crate::colors;

pub fn th() -> Theme {
    return vec! [
        (colors::HIGHLIGHT, None, Some(Col::new_rgb(68, 65, 90))),
    ];
}
