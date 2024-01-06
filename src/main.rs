use std::{fs::File, io::Read, path::Path};

use clap::Parser;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "path", help = "Print the byte count")]
    count: Option<std::path::PathBuf>,
    #[arg(short, long, value_name = "path", help = "Print the line count")]
    lines: Option<std::path::PathBuf>,
}
fn main() {
    let args = Args::parse();
    if let Some(c) = args.count {
        let count = read_bytes(&c).unwrap();
        println!("{} bytes", count);
    }
    if let Some(l) = args.lines {
        let lines = count_lines(&l).unwrap();
        println!("{} lines", lines);
    }
}

fn read_bytes(path: &Path) -> std::io::Result<usize> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    let sum: usize = file.read_to_end(&mut buffer).iter().sum();

    Ok(sum)
}

fn count_lines(path: &Path) -> std::io::Result<usize> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let lines: Vec<String> = buffer.lines().map(|s| s.to_string()).collect();

    Ok(lines.len())
}
