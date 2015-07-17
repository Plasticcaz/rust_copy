//! rust_cp
//! A Rust program by Zac ("Plasticcaz")
//!
//! Simple program that copies the contents of one file into a new file.
//!
//! Example:
//! From command line, run "./rust_cp <source_file> <destination>"

fn main() {
    use std::io::prelude::*;
    use std::fs::File;

    let args: Vec<String> = std::env::args().collect();

    if args.len() == 3 {
        // Attempt to open the specified file.
        let fresult = File::open(&args[1]);
        match fresult {
            Ok(mut input) => {
                let mut buffer: Vec<u8> = Vec::new();
                match input.read_to_end(&mut buffer) {
                    Ok(_) => {
                        // Attempt to write to specified destination.
                        let fresult = File::create(&args[2]);
                        match fresult {
                            Ok(mut output) => {
                                match output.write_all(buffer.as_ref()) {
                                    Ok(_) => (), // We've accomplished our task.
                                    Err(msg) => println!("Error: {}", msg),
                                }
                            },
                            Err(msg) => println!("Error: {}", msg),
                        }
                    },
                    Err(msg) => println!("Error: {}", msg),
                }
            },
            Err(msg) => println!("Error: {}", msg),
        } // Holy Error Handling Batman!
    }
    else {
        println!("Run: \"{} {} {}\"", &args[0], "<original_file>", "<new_file>");
    }
}
