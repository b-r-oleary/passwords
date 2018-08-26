extern crate rand;

use std::fs;

use rand::{Rng, ThreadRng};

use super::base::PasswordGenerator;


pub struct RandomWords {
    words: Vec<String>,
    length: usize,
    rng: ThreadRng,
}

impl RandomWords {
    pub fn new(path: &str, length: usize) -> RandomWords {

        let words = fs::read_to_string(path)
            .unwrap()
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), " ")
            .split_whitespace()
            .map(|s| { s.to_string() })
            .filter(|s| { s.len() > 3 })
            .collect();

        let rng = rand::thread_rng();

        RandomWords { words, length, rng }
    }
}

impl PasswordGenerator for RandomWords {
    fn generate_with_seed(&mut self, seed: String) -> String {
        let mut string_array = Vec::new();
        if seed.len() > 0 {
            string_array.push(seed)
        }

        for _ in 0..self.length {
            let word = self.rng.choose(&self.words).unwrap();
            string_array.push(word.to_string());
        }
        string_array.join(" ")
    }
}

pub struct RandomPhrases {
    phrases: Vec<Vec<String>>,
    min_length: usize,
    max_length: usize,
    rng: ThreadRng,
}

impl RandomPhrases {
    pub fn new(path: &str, min_length: usize, max_length: usize) -> RandomPhrases {

        let phrases: Vec<Vec<String>> = fs::read_to_string(path)
            .unwrap()
            .to_lowercase()
            .replace(|c: char| {
                !(c.is_alphanumeric() || c == '.' || c == ',')
            }, " ")
            .split(|c: char| c == '.' || c == ',')
            .map(|p| {
                p.split_whitespace()
                .map(|s| s.to_string())
                .collect()
            })
            .collect();

        // I'm not sure why i can't chain this `filter` after the `map`.
        let phrases = phrases
            .into_iter()
            .filter(|p| {
                let len = p.len();
                len >= min_length && len <= max_length
            })
            .collect();

        let rng = rand::thread_rng();

        RandomPhrases { phrases, min_length, max_length, rng }
    }
}

impl PasswordGenerator for RandomPhrases {
    fn generate_with_seed(&mut self, seed: String) -> String {
        let phrase = self.rng.choose(&self.phrases).unwrap();
        phrase.join(" ")
    }
}
