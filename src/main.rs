use anyhow::{anyhow, Error, Result};
use std::str::FromStr;
use structopt::StructOpt;

use passwords::{phrase_passwords, xkcd_passwords, ChainedGenerator, PasswordGenerator};

/// A tool for generating memorable, high entropy passwords
#[derive(StructOpt, Debug)]
#[structopt(name = "passwords")]
struct Opt {
    /// The number of password samples to generate.
    #[structopt(short, long, default_value = "1")]
    n_samples: usize,

    /// The type of generator to use to generate passwords.
    #[structopt(short, long, default_value = "phrases")]
    generator_type: GeneratorType,
}

#[derive(StructOpt, Debug)]
enum GeneratorType {
    XKCD,
    Phrases,
}

impl FromStr for GeneratorType {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        match input {
            "xkcd" => Ok(GeneratorType::XKCD),
            "phrases" => Ok(GeneratorType::Phrases),
            _ => Err(anyhow!("Did not recognize '{}' as a generator type", input)),
        }
    }
}

impl Opt {
    fn get_generator<'a>(&'a self) -> ChainedGenerator<'a> {
        match self.generator_type {
            GeneratorType::XKCD => xkcd_passwords(),
            GeneratorType::Phrases => phrase_passwords(),
        }
    }
}

fn main() {
    let opts = Opt::from_args();
    let generator = opts.get_generator();
    for password in generator.iterator().into_iter().take(opts.n_samples) {
        println!("{}", password);
    }
}
