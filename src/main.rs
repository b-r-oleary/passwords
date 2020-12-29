use std::fs;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use structopt::StructOpt;

use passwords::{phrase_passwords, xkcd_passwords, ChainedGenerator, PasswordGenerator, Text};

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

    /// An optional source text filename.
    #[structopt(short, long)]
    filename: Option<String>,

    /// An optional specification of an existing text
    #[structopt(short, long)]
    text: Option<TextType>,
}

#[derive(Debug)]
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

#[derive(Debug)]
enum TextType {
    Nouns,
    AliceInWonderland,
    TheTimeMachine,
}

impl FromStr for TextType {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        match input {
            "nouns" => Ok(TextType::Nouns),
            "alice-in-wonderland" => Ok(TextType::AliceInWonderland),
            "the-time-machine" => Ok(TextType::TheTimeMachine),
            _ => Err(anyhow!("Did not recognize '{}' as a text type", input)),
        }
    }
}

impl TextType {
    fn to_text(&self) -> Text {
        match self {
            TextType::Nouns => Text::NOUNS,
            TextType::AliceInWonderland => Text::ALICE_IN_WONDERLAND,
            TextType::TheTimeMachine => Text::THE_TIME_MACHINE,
        }
    }
}

impl Opt {
    fn get_generator<'a>(&'a self) -> Result<ChainedGenerator<'a>> {
        let contents = self
            .filename
            .as_ref()
            .map(|f| fs::read_to_string(f))
            .map_or(Ok(None), |v| v.map(Some))?;

        let text = if let Some(ref contents) = contents {
            Some(Text::new(contents))
        } else if let Some(ref text) = self.text {
            Some(text.to_text())
        } else {
            None
        };

        match self.generator_type {
            GeneratorType::XKCD => Ok(xkcd_passwords(text.as_ref())),
            GeneratorType::Phrases => Ok(phrase_passwords(text.as_ref())),
        }
    }
}

fn main() -> Result<()> {
    let opts = Opt::from_args();
    let generator = opts.get_generator()?;
    for password in generator.iterator().into_iter().take(opts.n_samples) {
        println!("{}", password);
    }
    Ok(())
}
