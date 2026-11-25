use std::io::{self, Write};
use std::process::Command;

pub struct Random {
    state: u64,
}

impl Random {
    pub fn new(seed: u64) -> Random {
        Random {
            state: seed,
        }
    }

    pub fn next(&mut self) -> u64 {
        self.state = self.state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);

        self.state
    }

    pub fn range(&mut self, max: u64) -> u64 {
        self.next() % max
    }

    pub fn from_time() -> Random {
        use std::time::{SystemTime, UNIX_EPOCH};

        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Fecha anterior a Unix Epoch")
            .as_nanos() as u64;

        Random::new(seed)
    }
}

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

pub fn enable_raw_mode() {
    Command::new("stty")
        .args(["-icanon", "-echo"])
        .status()
        .unwrap();
}

pub fn disable_raw_mode() {
    Command::new("stty")
        .args(["icanon", "echo"])
        .status()
        .unwrap();
}