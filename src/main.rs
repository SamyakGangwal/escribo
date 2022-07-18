use std::io::{self, stdout};

use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn die(e: std::io::Error) {
	panic!("{}", e);
}

fn main() {
	terminal::enable_raw_mode().expect("Could not turn on Raw mode");

	for key in io::stdin().keys() {
		match key {
			Ok(key) => match key {
				Key::Char(c) => {
					if c.is_control() {
						print!("{:?}\r", c as u8);
					} else {
						print!("{:?} ({})\r", c as u8, c);
					}
				}
				Key::Ctrl('q') => break,
				_ => println!("{:?}\r", key),
			},
			Err(err) => die(err),
		}
	}
}
