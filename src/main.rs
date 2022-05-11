pub mod compiler;

use std::fs;

use compiler::Compiler;

fn main() {
	let src: String = fs::read_to_string("test.bf").expect("Problem opening file");
	let rom: Vec<char> = src.chars().collect();

    let mut compiler = Compiler::new(rom);

	compiler.run();
}

