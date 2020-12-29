use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use super::base::PasswordGenerator;
use super::base::{ASCII_LOWERCASE, ASCII_UPPERCASE, DIGITS};

/// A `PasswordGenerator` that will generate a random string with size `length`
/// out of the candidate list of `characters`.
pub struct RandomString {
    characters: Vec<char>,
    length: usize,
}

impl RandomString {
    /// Create a `RandomString` object that will generate strings with size
    /// `length` with a default character set of ascii characters.
    pub fn new(length: usize) -> RandomString {
        let characters: Vec<char> = format!("{}{}{}", ASCII_LOWERCASE, ASCII_UPPERCASE, DIGITS)
            .chars()
            .collect();

        RandomString { characters, length }
    }
    /// Create a new `RandomString` object with the same `length`, but a
    /// different set of `characters`.
    pub fn with_characters(self, characters: Vec<char>) -> RandomString {
        RandomString {
            characters: characters,
            ..self
        }
    }
    /// Create a `RandomString` object that generates random strings of digits.
    pub fn digits(length: usize) -> RandomString {
        Self::new(length).with_characters(DIGITS.chars().collect())
    }
    /// Create a `RandomString` object that generates random strings of lowercase letters.
    pub fn ascii_lowercase(length: usize) -> RandomString {
        Self::new(length).with_characters(ASCII_LOWERCASE.chars().collect())
    }
    /// Create a `RandomString` object that generates random strings of uppercase letters.
    pub fn ascii_uppercase(length: usize) -> RandomString {
        Self::new(length).with_characters(ASCII_UPPERCASE.chars().collect())
    }
}

impl PasswordGenerator for RandomString {
    fn generate_with_seed(&self, rng: &mut ThreadRng, mut seed: String) -> String {
        for _ in 0..self.length {
            let c = self.characters.choose(rng).unwrap();
            seed.push(*c);
        }
        seed
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_random_string_new() {
        let characters = RandomString::new(5).characters.sort();
        let expected_characters = format!("{}{}{}", DIGITS, ASCII_LOWERCASE, ASCII_UPPERCASE)
            .chars()
            .into_iter()
            .collect::<Vec<char>>()
            .sort();

        assert_eq!(characters, expected_characters);
    }

    #[test]
    fn test_random_string_with_characters() {
        let characters = "lmnop";
        assert_eq!(
            RandomString::new(5)
                .with_characters(characters.chars().collect())
                .characters,
            characters.chars().collect::<Vec<char>>()
        );
    }

    #[test]
    fn test_random_string_digits() {
        assert_eq!(
            RandomString::digits(5).characters,
            "0123456789".chars().collect::<Vec<char>>()
        );
    }

    #[test]
    fn test_random_string_ascii_lowercase() {
        assert_eq!(
            RandomString::ascii_lowercase(5).characters,
            "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>()
        );
    }

    #[test]
    fn test_random_string_ascii_uppercase() {
        assert_eq!(
            RandomString::ascii_uppercase(5).characters,
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>()
        );
    }

    #[test]
    fn test_random_string_generate_with_trivial_seed() {
        let length = 5;
        let characters: Vec<char> = "lmnop".chars().collect();
        let passwords = RandomString::new(length).with_characters(characters.clone());

        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let password = passwords.generate_with_seed(&mut rng, String::new());
            assert_eq!(password.len(), length);
            assert!(password
                .chars()
                .into_iter()
                .all(|c| { characters.contains(&c) }));
        }
    }

    #[test]
    fn test_random_string_generate_with_nontrivial_seed() {
        let length = 5;
        let characters: Vec<char> = "01_.,!?".chars().collect();
        let passwords = RandomString::new(length).with_characters(characters.clone());

        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let password = passwords.generate_with_seed(&mut rng, "Ahh".to_string());
            assert_eq!(password.len(), length + 3);
            assert_eq!(&password[..3], "Ahh");
            assert!(password[3..]
                .chars()
                .into_iter()
                .all(|c| { characters.contains(&c) }));
        }
    }
}
