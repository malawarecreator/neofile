use clap::Parser;
use std::fs;

use std::path::Path;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: String,
}

fn main() {
    let args = Cli::parse();
    
    match args.pattern.as_str() {
        "identify" => {
            let extension = args.path.split('.').last().unwrap_or("");
            let extensioncaps = extension.to_uppercase();
            let fullextensionname = match extension {
                "rs" => String::from("Rust source file"),
                "txt" => String::from("Text file"),
                "cc" => String::from("C++ source file"),

                _ => format!("{extensioncaps} file"),
            };

            println!("{fullextensionname}");
            
        }
        "cat" => {
            let message = fs::read_to_string(args.path).expect("Failed to read file");
            println!("read file contents: {message}");
        }
        "vf" => {
            if Path::new(&args.path).is_file() {
                println!("is a file");
            } else {
                println!("is not a file");
            }
        }
        _ => {
            println!("no such command");
        }
    }
}

