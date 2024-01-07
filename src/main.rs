use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, Read},
    path,
};

use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    file: Option<std::path::PathBuf>,
    #[arg(short, help = "Print byte count", group = "count")]
    c_byte: bool,
    #[arg(short, help = "Print line count", group = "count")]
    l_line: bool,
    #[arg(short, help = "Print word count", group = "count")]
    w_word: bool,
    #[arg(short, help = "Print character count", group = "count")]
    m_char: bool,
}

#[derive(Debug)]
enum Path {
    Stdin,
    File(std::path::PathBuf),
}
impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Path::Stdin => write!(f, "stdin"),
            Path::File(path) => write!(f, "{}", path.display()),
        }
    }
}
#[derive(Debug)]
struct Ccwc {
    path: Path,
    buffer: String,
    target: Target,
}
impl Ccwc {
    fn from_args(args: Args) -> Result<Ccwc, io::Error> {
        let path = match &args.file {
            Some(path) => Path::File(path.clone()),
            None => Path::Stdin,
        };
        let buffer = Self::path_to_buffer(&path)?;
        let target = Target::from_args(buffer.clone(), args);

        Ok(Ccwc {
            path,
            buffer,
            target,
        })
    }

    fn path_to_buffer(path: &Path) -> Result<String, io::Error> {
        let mut buffer = String::new();
        match path {
            Path::Stdin => {
                let _ = io::stdin().read_to_string(&mut buffer)?;
                Ok(buffer)
            }
            Path::File(path) => {
                let mut file = File::open(path)?;
                let _ = file.read_to_string(&mut buffer)?;
                Ok(buffer)
            }
        }
    }
}
#[derive(Debug)]
enum Target {
    Byte(usize),
    Line(usize),
    Word(usize),
    Char(usize),
    None(usize, usize, usize),
}
impl Target {
    pub fn from_args(buffer: String, args: Args) -> Target {
        match args {
            Args { c_byte: true, .. } => Target::Byte(buffer.len()),
            Args { l_line: true, .. } => Target::Line(buffer.lines().count()),
            Args { w_word: true, .. } => Target::Word(buffer.split_whitespace().count()),
            Args { m_char: true, .. } => Target::Char(buffer.chars().count()),
            Args { .. } => {
                let lines = buffer.lines().count();
                let words = buffer.split_whitespace().count();
                let bytes = buffer.len();
                Target::None(lines, words, bytes)
            }
        }
    }
}
impl Display for Target {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Target::Byte(byte) => write!(f, "{}", byte),
            Target::Line(line) => write!(f, "{}", line),
            Target::Word(word) => write!(f, "{}", word),
            Target::Char(char) => write!(f, "{}", char),
            Target::None(line, word, byte) => write!(f, "{} {} {}", line, word, byte),
        }
    }
}

fn main() {
    let args = Args::parse();
    let ccwc = Ccwc::from_args(args).unwrap();

    println!("{} {}", ccwc.target, ccwc.path);
}
