extern crate passwords;

use std::io;
use std::io::Write;

use passwords::{PasswordGenerator, xkcd_passwords};


fn main() {
    for password in xkcd_passwords().iterator().into_iter() {
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
