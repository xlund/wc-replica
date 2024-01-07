use std::path::PathBuf;

use ccwc::file_counter::{Args, Count};

#[test]
fn count_bytes() {
    let count = Count::from_args(&Args {
        file: Some(PathBuf::from("tests/test.txt")),
        c_byte: true,
        ..Args::default()
    })
    .unwrap();
    assert_eq!(count, Count::Byte(85));
}

#[test]
fn count_lines() {
    let count = Count::from_args(&Args {
        file: Some(PathBuf::from("tests/test.txt")),
        l_line: true,
        ..Args::default()
    })
    .unwrap();
    assert_eq!(count, Count::Line(4));
}

#[test]
fn count_words() {
    let count = Count::from_args(&Args {
        file: Some(PathBuf::from("tests/test.txt")),
        w_word: true,
        ..Args::default()
    })
    .unwrap();
    assert_eq!(count, Count::Word(15));
}

#[test]
fn count_chars() {
    let count = Count::from_args(&Args {
        file: Some(PathBuf::from("tests/test.txt")),
        m_char: true,
        ..Args::default()
    })
    .unwrap();
    assert_eq!(count, Count::Char(85));
}

#[test]
fn count_unspecified() {
    let count = Count::from_args(&Args {
        file: Some(PathBuf::from("tests/test.txt")),
        ..Args::default()
    })
    .unwrap();
    assert_eq!(count, Count::Unspecified(4, 15, 85));
}
