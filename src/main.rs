use std::{fs::File, io::Read, path::Path};

use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: std::path::PathBuf,
    #[arg(short, long, help = "Print byte count")]
    count: bool,
    #[arg(short, long, help = "Print line count")]
    lines: bool,
    #[arg(short, long, help = "Print word count")]
    words: bool,
    #[arg(short = 'm', long, help = "Print character count")]
    chars: bool,
}
fn main() {
    let args = Args::parse();
    let file = File::open(&args.file);
    match file {
        Ok(file) => run(args, file),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_file(mut file: File) -> String {
    let mut buffer = String::new();
    let _ = file.read_to_string(&mut buffer);
    buffer
}

fn run(args: Args, file: File) -> () {
    let path = &args.file;
    let file = read_file(file);
    match file.is_empty() {
        true => println!("{} is empty", path.display()),
        false => print_counts(&args, path, file),
    }
}

fn print_counts(args: &Args, path: &Path, file: String) {
    let mut counts = String::new();
    if args.count {
        let count = format!("{}\t", file.bytes().count());
        counts.push_str(&count);
    }
    if args.lines {
        let count = format!("{}\t", file.lines().count());
        counts.push_str(&count);
    }
    if args.words {
        let count = format!("{}\t", file.split_whitespace().count());
        counts.push_str(&count)
    }
    if args.chars {
        let count = format!("{}\t", file.chars().count());
        counts.push_str(&count);
    }
    println!("{} {}", counts, path.display());
}
