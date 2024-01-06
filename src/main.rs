use std::{fs::File, io::Read, path::Path};

use clap::Parser;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "path", help = "Print the byte count")]
    count: Option<std::path::PathBuf>,
}
fn main() {
    let args = Args::parse();
    if let Some(c) = args.count {
        println!("Counting bytes in {:?}", c.as_path());
        let count = get_byte_count(&c).unwrap();
        println!("{} bytes", count);
    }
}

fn get_byte_count(path: &Path) -> std::io::Result<usize> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    let sum: usize = file.read_to_end(&mut buffer).iter().sum();

    Ok(sum)
}
