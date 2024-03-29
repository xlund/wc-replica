use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use clap::{command, Parser};
#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub file: Option<PathBuf>,
    #[arg(short, help = "Print byte count", group = "count")]
    pub c_byte: bool,
    #[arg(short, help = "Print line count", group = "count")]
    pub l_line: bool,
    #[arg(short, help = "Print word count", group = "count")]
    pub w_word: bool,
    #[arg(short, help = "Print character count", group = "count")]
    pub m_char: bool,
}
pub struct Counter {
    file_name: Option<String>,
    count: Count,
}
impl Counter {
    pub fn new() -> Result<Counter, io::Error> {
        let args = Args::parse();
        let count = Count::from_args(&args)?;
        let file_name = match &args.file {
            Some(path) => Some(path.to_string_lossy().into()),
            None => None,
        };
        Ok(Counter { file_name, count })
    }
}
impl Display for Counter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.count,
            self.file_name.as_ref().unwrap_or(&"".to_string())
        )
    }
}

#[derive(Debug)]
pub enum Count {
    Byte(usize),
    Line(usize),
    Word(usize),
    Char(usize),
    Unspecified(usize, usize, usize),
}

impl Count {
    pub fn from_args(args: &Args) -> Result<Count, io::Error> {
        let buffer = Self::buffer_from_args(args)?;
        match args {
            Args { c_byte: true, .. } => Ok(Count::Byte(buffer.len())),
            Args { l_line: true, .. } => Ok(Count::Line(buffer.lines().count())),
            Args { w_word: true, .. } => Ok(Count::Word(buffer.split_whitespace().count())),
            Args { m_char: true, .. } => Ok(Count::Char(buffer.chars().count())),
            Args { .. } => Ok({
                let lines = buffer.lines().count();
                let words = buffer.split_whitespace().count();
                let bytes = buffer.len();
                Count::Unspecified(lines, words, bytes)
            }),
        }
    }
    fn buffer_from_args(args: &Args) -> Result<String, io::Error> {
        let mut reader: Box<dyn BufRead> = match &args.file {
            Some(path) => Box::new(io::BufReader::new(File::open(path)?)),
            None => Box::new(io::BufReader::new(io::stdin())),
        };
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}
impl Display for Count {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Count::Byte(byte) => write!(f, "{}", byte),
            Count::Line(line) => write!(f, "{}", line),
            Count::Word(word) => write!(f, "{}", word),
            Count::Char(char) => write!(f, "{}", char),
            Count::Unspecified(line, word, byte) => {
                write!(f, "{} {} {}", line, word, byte)
            }
        }
    }
}
impl PartialEq for Count {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Count::Byte(a), Count::Byte(b)) => a == b,
            (Count::Line(a), Count::Line(b)) => a == b,
            (Count::Word(a), Count::Word(b)) => a == b,
            (Count::Char(a), Count::Char(b)) => a == b,
            (Count::Unspecified(a, b, c), Count::Unspecified(d, e, f)) => {
                a == d && b == e && c == f
            }
            _ => false,
        }
    }
}
