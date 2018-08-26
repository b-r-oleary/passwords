extern crate passwords;

use std::io;

use passwords::generators::base::{Constant, PasswordGenerator};
use passwords::generators::base::{ASCII_LOWERCASE, ASCII_UPPERCASE, DIGITS};
use passwords::generators::case::Case;
use passwords::generators::phrase::{RandomPhrases, RandomWords};
use passwords::generators::random_string::RandomString;


fn main() {
    let passwords = Constant::new("")
        .pipe(RandomPhrases::new("./texts/alice-in-wonderland.txt", 3, 5))
        .pipe(Case::Class)
        .pipe(RandomString::new(2).with_characters(DIGITS.chars().collect()));
        // .pipe(
        //     RandomString::new(2)
        //     .with_characters(DIGITS.chars().collect()))

    for password in passwords.iterator().into_iter() {
        println!("> {}", password);

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) if input.contains("exit") => break,
            Ok(_) => continue,
            Err(error) => println!("error: {}", error),
        }
    }
}
