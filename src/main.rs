extern crate pancurses;
use buffer::bindr;
use pancurses::*;

mod buffer;
mod settings;

fn init_colors() {
    if has_colors() {
        start_color();
        use_default_colors();
    }
}

fn main() {
    let window = initscr();
    if settings::COLOR {
        init_colors();
    }

    let mut test: buffer::Buf<buffer::bindr::DefaultBind> =
    buffer::Buf::new("OMG", buffer::Flags::NONE, buffer::bindr::DefaultBind {});

    let test2: buffer::Buf<buffer::bindr::FileBind> =
    buffer::Buf::new("OMG", buffer::Flags::NONE, buffer::bindr::FileBind {});

    test.attrs.push(buffer::Attr {
        color: -1,
        flags: A_BOLD,
        size: 5,
    });

    test.write(&window);
    test2.write(&window);
    window.refresh();

    window.keypad(true);
    noecho();
    loop {
        match window.getch() {
            Some(Input::Character(c)) => {
                window.addch(c);
            }
            Some(Input::KeyDC) => break,
            Some(input) => {
                window.addstr(&format!("{:?}", input));
            }
            None => (),
        }
    }

    endwin();
}
