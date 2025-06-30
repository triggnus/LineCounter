//! # LineCounter
//!
//! Simple file line counter. This program was written from scratch. Though it replicates the effect of wc -l \[fname\], it
//! contains no code in common.
//!
//! usage: lc \[file name(s)\]
//!
//! Author: Rob Teeple <somethingobscure@gmail.com>
//!
//! This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public 
//! License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty
//! of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License along with this program. If not, see 
//! <https://www.gnu.org/licenses/>.

use std::fs::File;
use std::io::{Read};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> std::io::Result<()>
{
	let args: Vec<String> = std::env::args().collect();

	// if no arguments, print out usage and copyright
	if args.len() == 1
	{
		println!("Line Counter v: {VERSION}");
		println!("Usage: {} [filename(s)]\n", args[0].split("/").last().unwrap());
		println!("LineCounter Copyright (C) 2025 Rob Teeple");
		println!("Released under GPL-3.0-only or GPL-3.0-or-later <https://www.gnu.org/licenses/gpl-3.0.html>");
		println!("Source code: <https://github.com/triggnus/LineCounter>");

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
				eprintln!("Error reading file {argument}: {error}");
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
	let width = total_lines.to_string().chars().count() + 2;

	for result in results
	{
		// write out the lines, followed by the filename
		println!("{:w$} {}", result.0, result.1, w = width);
	}

	// if we pass more than one file to count, print a total
	if args.len() > 2
	{
		println!("{total_lines:width$} Total");
	}

	Ok(())
}