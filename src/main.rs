use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::process::exit;

#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
	#[arg(help = "File path/s, absolute or relative", required = true)]
	file_s: Vec<String>
}

fn read_hex(file_c: &File) {
	
}

fn main() {
	let parsed = CliArgs::parse();

	for i in parsed.file_s {
		let current = match File::open(&i) {
			Ok(c) => { c }
			Err(e) => {
				eprintln!("{}", e);
				exit(1);
			}
		};
		read_hex(&current);
	}
}
