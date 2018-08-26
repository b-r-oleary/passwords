extern crate passwords;

use std::io;

use passwords::generators::random_string::{ASCII_LOWERCASE, DIGITS, RandomString};

fn main() {
    let digit_passwords = RandomString::new(16)
        .with_characters(DIGITS.chars().collect());

    let alpha_passwords = RandomString::new(16)
        .with_characters(ASCII_LOWERCASE.chars().collect());

    let passwords = digit_passwords.chain(alpha_passwords);

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
