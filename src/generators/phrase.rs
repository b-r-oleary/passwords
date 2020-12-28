extern crate rand;

use std::fs;
use std::io;

use rand::{Rng, ThreadRng};

use super::base::PasswordGenerator;

/// An object with convenience methods for loading words or phrases from a file.
pub struct Text {
    filename: String,
}

impl Text {
    /// Create a `Text` object from a `filename`.
    fn new(filename: String) -> Text {
        Text { filename }
    }
    /// Load a vector of lowercase words from file.
    fn load_words(&self) -> Result<Vec<String>, io::Error> {
        let words: Vec<String> = fs::read_to_string(&self.filename)?
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), " ")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        Ok(words)
    }
    /// Load a vector of vector of lowercase words corresponding to phrases from file.
    fn load_phrases(&self) -> Result<Vec<Vec<String>>, io::Error> {
        let phrases: Vec<Vec<String>> = fs::read_to_string(&self.filename)?
            .to_lowercase()
            .replace(
                |c: char| !(c.is_alphanumeric() || c == '.' || c == ',' || c == '\n'),
                " ",
            )
            .split(|c: char| c == '.' || c == ',')
            .map(|p| p.split_whitespace().map(|s| s.to_string()).collect())
            .collect();

        Ok(phrases)
    }

    pub fn alice_in_wonderland() -> Text {
        let filename = "./texts/alice-in-wonderland.txt".to_string();
        Text::new(filename)
    }

    pub fn the_time_machine() -> Text {
        let filename = "./texts/the-time-machine.txt".to_string();
        Text::new(filename)
    }

    pub fn nouns() -> Text {
        let filename = "./texts/nouns.txt".to_string();
        Text::new(filename)
    }
}

/// A `PasswordGenerator` that will generate a random sequence of words of
/// length `n_words` selected from the `words` vector.
pub struct RandomWords {
    words: Vec<String>,
    n_words: usize,
}

impl RandomWords {
    /// Create a `RandomWords` object with `words` with lengths that are greater than
    /// or equal in length to `min_word_length` loaded from an input `text`.
    pub fn from_text(text: &Text, n_words: usize, min_word_length: usize) -> RandomWords {
        let words: Vec<String> = text
            .load_words()
            .unwrap()
            .into_iter()
            .filter(|s| s.len() >= min_word_length)
            .collect();

        RandomWords { words, n_words }
    }
    /// Create a `RandomWords` objet with `words` with lengths that are greater than
    /// or equal in length to `min_word_length` loaded from a `filename`.
    pub fn from_filename(filename: String, n_words: usize, min_word_length: usize) -> RandomWords {
        let text = Text::new(filename);
        RandomWords::from_text(&text, n_words, min_word_length)
    }
}

impl PasswordGenerator for RandomWords {
    fn generate_with_seed(&self, rng: &mut ThreadRng, seed: String) -> String {
        let mut string_array = Vec::new();
        if seed.len() > 0 {
            string_array.push(seed)
        }

        for _ in 0..self.n_words {
            let word = rng.choose(&self.words).unwrap();
            string_array.push(word.to_string());
        }
        string_array.join(" ")
    }
}

/// A `PasswordGenerator` object that will select a random phrase from a vector of `phrases`.
pub struct RandomPhrases {
    phrases: Vec<Vec<String>>,
}

impl RandomPhrases {
    /// Create a `RandomPhrases` object with `phrases` derived from a `text` with phrases
    /// that have lengths between `min_length` and `max_length`.
    pub fn from_text(text: &Text, min_length: usize, max_length: usize) -> RandomPhrases {
        let phrases: Vec<Vec<String>> = text
            .load_phrases()
            .unwrap()
            .into_iter()
            .filter(|p| {
                let len = p.len();
                len >= min_length && len <= max_length
            })
            .collect();

        RandomPhrases { phrases }
    }
    /// Create a `RandomPhrases` object with `phrases` loaded from a `filename` that have
    /// lengths between `min_length` and `max_length`.
    pub fn from_filename(filename: String, min_length: usize, max_length: usize) -> RandomPhrases {
        let text = Text::new(filename);
        RandomPhrases::from_text(&text, min_length, max_length)
    }
}

impl PasswordGenerator for RandomPhrases {
    fn generate_with_seed(&self, rng: &mut ThreadRng, seed: String) -> String {
        let phrase = rng.choose(&self.phrases).unwrap();
        seed + &phrase.join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_text_new() {
        let filename = "./a-filename";
        let text = Text::new(filename.to_string());
        assert_eq!(text.filename, filename)
    }

    #[test]
    fn test_text_load_words_with_err() {
        let error = Text::new("./an-invalid-filename".to_string()).load_words();
        assert!(error.is_err());
    }

    #[test]
    fn test_text_load_words_with_ok() {
        let result = Text::new("./texts/nouns.txt".to_string()).load_words();

        assert!(result.is_ok());
        assert!(result.unwrap().len() > 0);
    }

    #[test]
    fn test_text_load_phrases_with_err() {
        let error = Text::new("./an-invalid-filename".to_string()).load_phrases();
        assert!(error.is_err());
    }

    #[test]
    fn test_text_load_phrases_with_ok() {
        let result = Text::new("./texts/nouns.txt".to_string()).load_phrases();

        assert!(result.is_ok());
        assert!(result.unwrap().len() > 0);
    }

    #[test]
    fn test_texts() {
        assert_eq!(
            Text::alice_in_wonderland().filename,
            "./texts/alice-in-wonderland.txt"
        );
        assert_eq!(
            Text::the_time_machine().filename,
            "./texts/the-time-machine.txt"
        );
        assert_eq!(Text::nouns().filename, "./texts/nouns.txt");
    }

    #[test]
    fn test_random_words_from_filename() {
        let filename = "./texts/alice-in-wonderland.txt".to_string();
        let passwords = RandomWords::from_filename(filename, 4, 5);

        assert_eq!(passwords.n_words, 4);
        assert!(passwords.words.len() > 0);
        assert!(
            passwords
                .words
                .into_iter()
                .all(|word| word.len() >= 5)
        );
    }

    #[test]
    fn test_random_words_generate_with_seed() {
        let passwords = RandomWords::from_text(&Text::alice_in_wonderland(), 4, 5);
        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let password = passwords.generate_with_seed(&mut rng, String::new());
            let words: Vec<&str> = password.split(" ").collect();
            assert_eq!(words.len(), 4);
            assert!(words
                .into_iter()
                .all(|word| { word.len() >= 5 && passwords.words.contains(&word.to_string()) }));
        }
    }

    #[test]
    fn test_random_phrases_from_filename() {
        let filename = "./texts/alice-in-wonderland.txt".to_string();
        let passwords = RandomPhrases::from_filename(filename, 3, 5);

        assert!(passwords.phrases.len() > 0);
        assert!(passwords
            .phrases
            .into_iter()
            .all(|phrase| phrase.len() >= 3 && phrase.len() <= 5));
    }

    #[test]
    fn test_random_phrases_generate_with_seed() {
        let passwords = RandomPhrases::from_text(&Text::alice_in_wonderland(), 3, 5);
        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let password = passwords.generate_with_seed(&mut rng, String::new());
            let words: Vec<&str> = password.split(" ").collect();
            assert!(words.len() >= 3 && words.len() <= 5);
        }
    }
}
