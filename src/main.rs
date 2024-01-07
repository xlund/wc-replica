use std::{
    fs::File,
    io::{self, Read},
};

use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    file: std::path::PathBuf,
    #[arg(short, help = "Print byte count", group = "count")]
    c_byte: bool,
    #[arg(short, help = "Print line count", group = "count")]
    l_line: bool,
    #[arg(short, help = "Print word count", group = "count")]
    w_word: bool,
    #[arg(short, help = "Print character count", group = "count")]
    m_char: bool,
}

enum Target {
    Byte,
    Line,
    Word,
    Char,
    None,
}
impl Target {
    pub fn new(args: &Args) -> Target {
        if args.c_byte {
            Target::Byte
        } else if args.l_line {
            Target::Line
        } else if args.w_word {
            Target::Word
        } else if args.m_char {
            Target::Char
        } else {
            Target::None
        }
    }
    pub fn from_buffer(&self, buffer: String) -> String {
        match self {
            Target::Byte => buffer.len().to_string(),
            Target::Line => buffer.lines().count().to_string(),
            Target::Word => buffer.split_whitespace().count().to_string(),
            Target::Char => buffer.chars().count().to_string(),
            Target::None => {
                let lines = buffer.lines().count();
                let words = buffer.split_whitespace().count();
                let bytes = buffer.len().to_string();
                format!("{} {} {}", lines, words, bytes)
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let path = &args.file;
    let file = File::open(path).expect("Unable to open file");
    let _ = run(args, file);
}

fn read_file(mut file: File) -> Result<String, io::Error> {
    let mut buffer = String::new();
    let _ = file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn run(args: Args, file: File) -> Result<(), io::Error> {
    let target = Target::new(&args);
    let buffer = read_file(file)?;
    let path = &args.file.display();
    let count = target.from_buffer(buffer);
    println!("{} {}", count, path);
    Ok(())
}
