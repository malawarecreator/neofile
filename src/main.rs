use clap::Parser;
use std::fs;
use std::io::{Write, self};
use std::path::Path;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: String,
}

fn main() {
    let args = Cli::parse();
    let stdout = io::stdout();
    let handle = stdout.lock();
    match args.pattern.as_str() {
        "identify" => {
            let extension = args.path.split('.').last().unwrap_or("");
            let fullextensionname = match extension {
                "rs" => "Rust source file",
                "txt" => "Text file",
                "cc" => "C++ source file",
                _ => "Unknown file type",
            };
            
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

