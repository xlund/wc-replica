use ccwc::file_counter::Counter;

fn main() {
    let counter = Counter::new();
    match counter {
        Ok(counter) => println!("{}", counter),
        Err(e) => eprintln!("{}", e),
    }
}
