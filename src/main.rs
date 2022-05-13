pub mod interpreter;

use interpreter::Interpreter;
use std::fs::OpenOptions;
use std::io::Read;

fn main() {
	let mut rom = Vec::new();
	{
		let mut file_handle = OpenOptions::new()
			.read(true)
			.open("test.bf")
			.expect("Problem opening file");

		file_handle.read_to_end(&mut rom).unwrap();
		// `file_handle` gets dropped after here and closes it
	}

	let mut interpreter = Interpreter::new(rom);

	interpreter.run();
}
