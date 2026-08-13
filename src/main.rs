//! # LineCounter
//!
//! Simple file line counter. This program was written from scratch. Though it replicates the effect of wc
//! -l \[fname\], it contains no code in common.
//!
//! usage: lc \[file name(s)\]
//!
//! Author: Rob Teeple <somethingobscure@gmail.com>
//!
//! This program is free software: you can redistribute it and/or modify it under the terms of the GNU
//! General Public License as published by the Free Software Foundation, either version 3 of the License,
//! or (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
//! the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General
//! Public License for more details.
//!
//! You should have received a copy of the GNU General Public License along with this program. If not, see
//! <https://www.gnu.org/licenses/>.

use std::fs::File;
use std::io::{self, BufRead, Read};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> std::io::Result<()> {
    // only print the total lines of all input
    let mut total_only = false;

    // if the -t option was passed, set program to total_only mode. Also, remove the flag from the files.
    let args: Vec<String> = std::env::args()
        .filter(|p| {
            if p == "-t" {
                total_only = true;
                false
            } else {
                true
            }
        })
        .collect();

    // if no files, print out usage and copyright
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        println!(
            "Usage: {} [filename(s)]",
            // This line returns the name of the executing program (even if it changes). By default, args[0]
            // includes a relative path. We don't need this so we split by the '/' char and get the last
            // element (the executable name).
            // SAFETY: last() returns an Option<T> because if it runs on an empty iterator, it can have no
            // value. args[0] always has a value, so we can safely unwrap.
            args[0].split("/").last().unwrap()
        );
        println!("  -t    Total count only.");
        println!("\nLineCounter v: {VERSION}. Copyright (C) 2025 Rob Teeple");
        println!(
            "Released under GPL-3.0-only or GPL-3.0-or-later <https://www.gnu.org/licenses/gpl-3.0.html>"
        );
        println!("Source code: <https://github.com/triggnus/LineCounter>");

        return Ok(());
    }

    // begin calculation of line count
    let mut total_lines = 0;
    let mut results: Vec<(usize, String)> = Vec::new();

    let files: Vec<&String> = args.iter().skip(1).collect();

    if files.len() == 0 {
        let reader = Box::new(io::stdin().lock());
        let lines = reader.lines().count();
        results.push((lines, "STDIN".to_string()));
        total_lines += lines;
    } else {
        //iterate over args, but skip the first (the program itself)
        for file in files {
            // this is where the text of the file is kept
            let mut buffer: Vec<u8> = Vec::new();

            match File::open(file) {
                Ok(mut a) => a.read_to_end(buffer.as_mut())?,
                Err(error) => {
                    eprintln!("Error reading file {file}: {error}");
                    // this program should exit gracefully if the files are bad. No need to pass the error
                    // back to the terminal. The stakes are quite low.
                    return Ok(());
                }
            };

            // because we do not know if the text file is in UTF-8 format, and a string in rust *must* be
            // UTF-8, we have to read the file as a Vec<u8> and convert it.
            let lines = String::from_utf8_lossy(&buffer).lines().count();

            results.push((lines, file.to_string()));

            total_lines += lines;
        }
    }

    if total_only {
        println!("{total_lines}");
    } else {
        // determine the amount of padding we will need to properly align the output.
        let width = total_lines.to_string().chars().count() + 2;

        for result in results {
            // write out the lines, followed by the filename
            println!("{:w$} {}", result.0, result.1, w = width);
        }

        // if we pass more than one file to count, print a total
        if args.len() > 2 {
            println!("{total_lines:width$} Total");
        }
    }

    Ok(())
}
