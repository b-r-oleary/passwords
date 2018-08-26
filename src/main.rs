extern crate passwords;

use std::io;

use passwords::generators::random_string::{DIGITS, RandomString};

fn main() {
    let passwords = RandomString::new(16)
        .with_characters(DIGITS.chars().collect());

    for password in passwords.into_iter() {
        println!("> {}", password);

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) if input.contains("exit") => break,
            Ok(_) => continue,
            Err(error) => println!("error: {}", error),
        }
    }
}
