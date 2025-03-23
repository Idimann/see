extern crate pancurses;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use ctrlc;

use pancurses::*;

mod buffer;
mod run;
mod settings;
mod window;
mod mode;
mod movement;

fn init_colors() {
    if has_colors() {
        start_color();
        use_default_colors();
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
    windows: &mut Vec<window::Win>,
    scr: &Window,
) {
    let (y_size, x_size) = scr.get_max_yx();
    let (y_pos, x_pos) = scr.get_beg_yx();

    windows.push(window::Win::new(
        match scr.subwin(y_size, x_size, y_pos, x_pos) {
            Ok(x) => x,
            Err(_) => return,
        },
        match buffers.first() {
            Some(x) => x.clone(),
            None => return,
        },
    ));
}

fn main() {
    let stdscr = initscr();
    noecho();


    if settings::COLOR {
        init_colors();
    }

    let mut buffers: Vec<Rc<RefCell<buffer::Buf>>> = Vec::new();
    let mut windows: Vec<window::Win> = Vec::new();

    match ctrlc::set_handler(|| {
        mode::ctrl_c();
    }) {
        Ok(_) => (),
        Err(x) => panic!("{:?}", x)
    };

    init_bufs(&mut buffers, env::args().nth(1), buffer::Flags::none);
    init_windows(&mut buffers, &mut windows, &stdscr);

    stdscr.keypad(true);
    let _ = run::run(&stdscr, &mut buffers, &mut windows);

    for win in windows {
        win.window.delwin();
    }

    endwin();
}
