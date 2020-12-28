extern crate clap;
extern crate passwords;

use std::io;
use std::io::Write;

use clap::{App, Arg};

use passwords::{phrase_passwords, xkcd_passwords, ChainedGenerator, PasswordGenerator};

struct Config<'a> {
    generator: ChainedGenerator<'a>,
}

impl<'a> Config<'a> {
    fn get_generator(name: &str) -> Result<ChainedGenerator<'a>, String> {
        match name {
            "xkcd" => Ok(xkcd_passwords()),
            "phrases" => Ok(phrase_passwords()),
            _ => Err(format!("invalid generator name: {}", name)),
        }
    }

    fn parse() -> Result<Config<'a>, String> {
        let generator_names = ["phrases", "xkcd"];

        let matches = App::new("Password Generator")
            .arg(
                Arg::with_name("generator")
                    .short("g")
                    .long("generator")
                    .takes_value(true)
                    .help(&format!(
                        "specifies a password generator, one of {:?}",
                        generator_names
                    ))
                    .possible_values(&generator_names),
            )
            .get_matches();

        let generator_name = matches.value_of("generator").unwrap_or(generator_names[0]);

        let generator = Self::get_generator(generator_name)?;

        Ok(Config { generator })
    }
}

fn main() {
    let config = Config::parse().unwrap();

    for password in config.generator.iterator().into_iter() {
        println!("{}", password);

        let mut input = String::new();

        print!("> ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(_) if input.contains("exit") => {
                println!("Bye!");
                break;
            }
            Ok(_) => continue,
            Err(error) => println!("error: {}", error),
        }
    }
}
