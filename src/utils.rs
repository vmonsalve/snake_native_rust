use std::io::{self, Write};

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

pub fn enter_alternate_screen() {
    print!("\x1B[?1049h");
    io::stdout().flush().unwrap();
}

pub fn exit_alternate_screen() {
    print!("\x1B[?1049l");
    io::stdout().flush().unwrap();
}