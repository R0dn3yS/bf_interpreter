use char;

use std::io::{self, Read};

pub struct Compiler {
	rom: Vec<char>,
	ram: [u8; 0x7530],
	pc: usize,
	pointer: usize,
	loop_count: u16,
}

impl Compiler {
	pub fn new(src: Vec<char>) -> Compiler {
		Compiler {
			rom: src,
			ram: [0; 0x7530],
			pc: 0x0,
			pointer: 0x00,
			loop_count: 0,
		}
	}

	pub fn run(&mut self) { 
		while self.pc <= (self.rom.len() - 1) {
			self.execute(self.rom[self.pc]);
			self.pc += 1;
		}
	}

	fn execute(&mut self, op: char) {
		match op {
			'>' => {
				self.pointer += 1;
			}

			'<' => {
				self.pointer -= 1;
			}

			'+' => {
				self.ram[self.pointer] = self.ram[self.pointer].wrapping_add(1);
			}

			'-' => {
				self.ram[self.pointer] = self.ram[self.pointer].wrapping_sub(1);
			}

			'.' => {
				let byte = self.ram[self.pointer];
				print!("{}", byte as char)
			}

			',' => {
				io::stdin().read(&mut self.ram[self.pointer..self.pointer + 1]).unwrap();
			}

			'[' => {
				self.loop_count = 1;

				if self.ram[self.pointer] == 0 {
					while {
						self.pc += 1;
						if self.rom[self.pc] == '[' {
							self.loop_count += 1;
						} else if self.rom[self.pc] == ']' {
							self.loop_count -= 1;
						}
						self.loop_count != 0
					} {}
				}
			}

			']' => {
				self.loop_count = 0;
				while {
					if self.rom[self.pc] == '[' {
						self.loop_count -= 1;
					} else if self.rom[self.pc] == ']' {
						self.loop_count += 1;
					}
					self.pc -= 1;
					self.loop_count != 0
				} {}
			}

			_ => {}
		}
	}
}