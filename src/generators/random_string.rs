extern crate rand;

use rand::{Rng, ThreadRng};


pub static ASCII_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
pub static ASCII_UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub static DIGITS: &str = "0123456789";


pub trait PasswordGenerator {
    fn generate_with_seed(&mut self, seed: String) -> String;

    fn generate(&mut self) -> String {
        self.generate_with_seed(String::new())
    }
}

#[derive(Debug)]
pub struct RandomString {
    characters: Vec<char>,
    length: usize,
    rng: ThreadRng,
}

impl RandomString {
    pub fn new(length: usize) -> RandomString {

        let characters: Vec<char> = format!(
                "{}{}{}",
                ASCII_LOWERCASE,
                ASCII_UPPERCASE,
                DIGITS
            )
            .chars()
            .collect();

        let rng = rand::thread_rng();

        RandomString { characters, length, rng }
    }

    pub fn with_characters(self, characters: Vec<char>) -> RandomString {
        RandomString {
            characters: characters,
            ..self
        }
    }
}

impl PasswordGenerator for RandomString {
    fn generate_with_seed(&mut self, mut seed: String) -> String {
        for _ in 0..self.length {
            let c = self.rng.choose(&self.characters).unwrap();
            seed.push(*c);
        }
        seed
    }
}

impl Iterator for RandomString {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let password = self.generate();

        Some(password)
    }
}
