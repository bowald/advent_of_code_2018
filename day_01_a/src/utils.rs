use std::env;
use std::fs::File;
use std::io::prelude::*;

fn help() {
    println!("usage: bin <string>");
}

pub fn read_input() -> std::io::Result<String> {
    let mut filename = String::new();
    parseargs(&mut filename);
    return read_file_content(filename);
}

fn parseargs(filename: &mut String) {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            filename.push_str(&args[1]);
        }
        _ => {
            help();
        }
    }
}

fn read_file_content(filename: String) -> std::io::Result<String> {
    //....read file content.....
    let mut content = String::new();
    let mut file = File::open(filename)?;
    file.read_to_string(&mut content)?;
    Ok(content)
}