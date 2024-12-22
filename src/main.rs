use clap::Parser;
use std::{fs, path::{self, Path}};

#[derive(clap::Parser)]
struct Cli {
    pattern: String,
    path: String,
}

fn main() {

    let args = Cli::parse();

    if args.pattern == String::from("identify") {
        let extension = args.path.split(".").last().unwrap_or("");
        let mut fullextensionname: &str = "nil";
        match extension {
            "rs" => fullextensionname = "Rust source file",
            "txt" => fullextensionname = "Text file",
            "cc" => fullextensionname = "C++ source file",
            &_ => todo!(),
        }
        println!("file extension identified as '{}'", fullextensionname);
    } else if args.pattern == String::from("cat") {
        let message: String = fs::read_to_string(args.path).expect("something messed up");
        println!("read file contents: {message}");
    } else if args.pattern == String::from("vf") {
       
        println!("is a file");
    } else {
        println!("no such command");
    }
}

