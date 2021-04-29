use std::io;

pub fn run() {
    println!("# Problem 2");
    println!("Please insert a string");
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");
    println!("{}", string.trim_end().chars().rev().collect::<String>());
}
