use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::process::exit;

#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
	#[arg(help = "File path/s, absolute or relative", required = true)]
	files: Vec<String>
}

fn read_hex(c_file: &mut File) {
	let mut buffer: [u8; 8] = [0; 8];
	loop {
		match c_file.read(&mut buffer) {
			Ok(0) => {
				println!();
				println!("Reached EOF");
				break;
			}
			Ok(n) => {
				let mut counter = 0;
				for i in &buffer[0..n] {
					print!("{:02X} ", i);
					counter += 1; // (0)->1->2->3->4->5->6->7->8
					counter = counter % 8; // 8 % 8 = 0
					if counter == 0 {
						println!();
					}
				}
			}
			Err(e) => {
				eprintln!("{}", e);
				break;
			}
		}
	}
}

fn main() {
	let parsed = CliArgs::parse();

	for i in parsed.files {
		let mut current = match File::open(&i) {
			Ok(c) => { c }
			Err(e) => {
				eprintln!("{}", e);
				exit(1);
			}
		};
		read_hex(&mut current);
	}
}
