extern crate rand;

use rand::{Rng, ThreadRng};

use super::base::PasswordGenerator;
use super::base::{ASCII_LOWERCASE, ASCII_UPPERCASE, DIGITS};


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
    pub fn digits(length: usize) -> RandomString {
        Self::new(length).with_characters(DIGITS.chars().collect())
    }
    pub fn ascii_lowercase(length: usize) -> RandomString {
        Self::new(length).with_characters(ASCII_LOWERCASE.chars().collect())
    }
    pub fn ascii_uppercase(length: usize) -> RandomString {
        Self::new(length).with_characters(ASCII_UPPERCASE.chars().collect())
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
