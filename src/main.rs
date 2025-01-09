use std::fs::File;
use std::io::{Read};

/// Simple line counter that replicates the effect of wc -l \[fname\]
///
/// usage: LineCount \[file name(s)\]

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> std::io::Result<()>
{
	let args: Vec<String> = std::env::args().collect();

	if args.len() == 1
	{
		println!("Line Counter v: {}", VERSION);
		println!("Usage: lc [filename(s)]");

		return Ok(());
	}

	let mut total_lines = 0;
	let mut results: Vec<(usize, String)> = Vec::new();

	//iterate over args, but skip the first (the program itself)
	for argument in args.iter().skip(1)
	{
		// this is where the text of the file is kept
		let mut buf = vec![];

		match File::open(argument)
		{
			Ok(mut a) =>
				a.read_to_end(buf.as_mut())?,
			Err(error) => {
				eprintln!("Error reading file {}: {}", argument, error);
				// this program should exit gracefully if the arguments are bad. No need to pass the error back to the terminal.
				return Ok(());
			},
		};

		// because we do not know if the text file is in UTF-8 format, and a string in rust *must* be UTF-8, we have to read
		// the file as a Vec<u8> and convert it.
		let l = String::from_utf8_lossy(&buf).lines().count();

		results.push((l, argument.to_string()));

		total_lines += l;
	}

	// determine the amount of padding we will need to properly align the output.
	// log10(num).floor() would return the number of digits - 1. For aesthetics, we add two to add a blank space before the
	// numbers.
	let width = (f64::log10(total_lines as f64) + 2.0).floor() as usize;

	for result in results
	{
		// write out the lines, followed by the filename
		println!("{:w$} {}", result.0, result.1, w = width);
	}

	// if we pass more than one file to count, print a total
	if args.len() > 2
	{
		println!("{:w$} Total", total_lines, w = width);
	}

	Ok(())
}
