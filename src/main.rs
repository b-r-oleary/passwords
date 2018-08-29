extern crate passwords;

use std::io;
use std::io::Write;

use passwords::PasswordGenerator;
use passwords::{Constant, Case, RandomPhrases, RandomString, Defects};


fn main() {
    let passwords = Constant::new("")
        .pipe(RandomPhrases::new("./texts/alice-in-wonderland.txt", 3, 5))
        .pipe(Case::Class)
        .pipe(RandomString::digits(2))
        .pipe(Defects::with_symbols(1, 1))
        .pipe(Defects::with_vowels(1, 1));

    for password in passwords.iterator().into_iter() {
        println!("{}", password);

        let mut input = String::new();

        print!("> ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(_) if input.contains("exit") => break,
            Ok(_) => continue,
            Err(error) => println!("error: {}", error),
        }
    }
}
