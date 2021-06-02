use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read input");
    let number: i32 = input_line.trim().parse().expect("Input not an integer");

    println!("Tell me about {}", number);
    match number {
        // single value
        1 => println!("Uno"),

        // multiple
        2 | 3 | 5 | 7 | 11 => println!("Primo!"),

        // inclusive range
        13..=19 => println!("Teeny bopper"),

        _ => println!("Ain't special"),
    }

    let b: bool = true;

    let binary = match b {
        false => 0,
        true => 1
    };

    println!("{} -> {}", b, binary);
}

