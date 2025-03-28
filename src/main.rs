extern crate pancurses;
use ctrlc;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;

use pancurses::*;

mod buffer;
mod colors;
mod mode;
mod movement;
mod run;
mod settings;
mod window;

fn init_colors() {
    if has_colors() {
        start_color();
        use_default_colors();
        colors::apply_theme(&colors::default_theme::th());
    }
}

fn init_bufs(
    buffers: &mut Vec<Rc<RefCell<buffer::Buf>>>,
    file: Option<String>,
    flags: buffer::Flags,
) {
    buffers.push(Rc::new(RefCell::new(match file {
        Some(n) => buffer::Buf::new_load(flags, Box::new(buffer::bindr::FileBind { file: n })),
        None => buffer::Buf::new(
            "No file provided",
            flags,
            Box::new(buffer::bindr::DefaultBind {}),
        ),
    })));
}

fn init_windows(
    buffers: &Vec<Rc<RefCell<buffer::Buf>>>,
    windows: &mut Vec<(Box<dyn mode::Mod>, window::Win)>,
    scr: &Window,
) {
    let (y_size, x_size) = scr.get_max_yx();
    let (y_pos, x_pos) = scr.get_beg_yx();

    windows.push((
        Box::new(window::DEFAULT_MODE),
        window::Win::new(
            match scr.subwin(y_size, x_size, y_pos, x_pos) {
                Ok(x) => x,
                Err(_) => return,
            },
            match buffers.first() {
                Some(x) => x.clone(),
                None => return,
            },
        ),
    ));
}

fn main() {
    let stdscr = initscr();
    noecho();

    if settings::COLOR {
        init_colors();
    }

    let mut buffers: Vec<Rc<RefCell<buffer::Buf>>> = Vec::new();
    let mut windows: Vec<(Box<dyn mode::Mod>, window::Win)> = Vec::new();

    match ctrlc::set_handler(|| {
        mode::ctrl_c();
    }) {
        Ok(_) => (),
        Err(x) => panic!("{:?}", x),
    };

    init_bufs(&mut buffers, env::args().nth(1), buffer::Flags::none);
    init_windows(&mut buffers, &mut windows, &stdscr);

    stdscr.keypad(true);
    let _ = run::run(&stdscr, &mut buffers, &mut windows);

    for (_, win) in windows {
        win.window.delwin();
    }

    endwin();
}
