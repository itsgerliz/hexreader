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

fn read_hex(c_file: &File) {
	
}

fn main() {
	let parsed = CliArgs::parse();

	for i in parsed.files {
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
