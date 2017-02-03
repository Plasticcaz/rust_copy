//! rust_cp
//! A Rust program by Zac ("Plasticcaz")
//!
//! Simple program that copies the contents of one file into a new file.
//!
//! Example:
//! From command line, run "./rust_cp <source_file> <destination>"

fn main() {

    let args: Vec<String> = std::env::args().collect();

    if args.len() == 3 {
        match copy(&args[1], &args[2]) {
            Ok(()) => (),
            Err(msg) => println!("Error: {}", msg),
        }
    } else {
        println!("Please rerun in following format: './rust_cp <source_file> <destination>'.");
    }
}


fn copy(input: &str, output: &str) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::{Read, Write};
    let input = File::open(input)?;
    let mut input = std::io::BufReader::new(input);
    let mut buffer: Vec<u8> = Vec::new();
    input.read_to_end(&mut buffer)?;
    let output = File::create(output)?;
    let mut output = std::io::BufWriter::new(output);
    output.write_all(buffer.as_ref())?;
    Ok(())
}