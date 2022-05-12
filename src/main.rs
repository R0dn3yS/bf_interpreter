pub mod interpreter;

use std::fs;

use interpreter::Interpreter;

fn main() {
	let src: String = fs::read_to_string("test.bf").expect("Problem opening file");
	let rom: Vec<char> = src.chars().collect();

    let mut interpreter = Interpreter::new(rom);

	interpreter.run();
}

