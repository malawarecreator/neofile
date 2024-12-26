
use clap::Parser;
use tokio::time::sleep;
use std::fs::File;
use std::io::Write;
use std::fs;
use std::path::Path;
use std::io::prelude::*;
use std::time::Duration;
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: String,

    #[clap(long = "no-newline", required=false)]
    no_newline: bool,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    
    match args.pattern.as_str() {
        "identify" => {
            let extension = args.path.split('.').last().unwrap_or("");
            let extensioncaps = extension.to_uppercase();
            let fullextensionname = match extension {
                "rs" => String::from("Rust source file"),
                "txt" => String::from("Text file"),
                "cc" => String::from("C++ source file"),
                "c++" => String::from("C++ source file"),
                "java" => String::from("Java Source file"),
                _ => format!("{extensioncaps} file"),
            };

            println!("{fullextensionname}");
            
        }
        "cat" => {
            let message = fs::read_to_string(args.path).expect("Failed to read file");
            println!("read file contents: {message}");
        }
        "write" => {
            write_to_file().await;
            
        }
        "vf" => {
            if Path::new(&args.path).is_file() {
                println!("is a file");
            } else {
                println!("is not a file");
            }
        }

        "apnd" => {
            
            let mut in_data: String = String::new();
            println!("enter the data needed to be appended");
            std::io::stdin().read_line(&mut in_data).unwrap();
            println!("writing to file {}", args.path);
            append_file(in_data, args.path, args.no_newline).await;
            
        }
        _ => {
            println!("no such command");
        }

    }
}
async fn write_to_handle(mut handle: File, databuf: &[u8]) {
    handle.write_all(databuf).unwrap();
    
}
async fn write_to_file() {
    let mut br: i32 = 0;
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    let mut file = String::new();
    let mut input_data = String::new();

    writeln!(handle, "input the file name below:").unwrap();
    br += std::io::stdin().read_line(&mut file).unwrap() as i32;
    writeln!(handle, "read {} bytes from filename", br).unwrap();
    writeln!(handle, "data to input below:").unwrap();
    br += std::io::stdin().read_line(&mut input_data).unwrap() as i32;
    writeln!(handle, "read {} bytes total", br).unwrap();
    writeln!(handle, "writing to file {}", file).unwrap();
  
    let file_handle = match File::create(file.trim()) {
        Ok(file_handle) => file_handle,
        Err(err) => {
            eprintln!("Error creating file: {}", err);
            return; 
        }
    };
    write_to_handle(file_handle, input_data.as_bytes()).await;
    

}
async fn append_file(data: String, path: String, no_newline: bool) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .expect("Failed to open file");
    if no_newline {
        file.write_all(data.as_bytes()).expect("Failed to append to file");
    } else {
        file.write_all(format!("{}\n", data).as_bytes()).expect("Failed to append to file");
    }
    println!("Data appended successfully");
}