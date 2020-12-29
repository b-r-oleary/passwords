use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use super::base::PasswordGenerator;

/// An object with convenience methods for loading words or phrases from a file.
pub struct Text<'a> {
    text: &'a str,
}

impl<'a> Text<'a> {
    pub const ALICE_IN_WONDERLAND: Text<'static> =
        Text::new(include_str!("../../texts/alice-in-wonderland.txt"));
    pub const THE_TIME_MACHINE: Text<'static> =
        Text::new(include_str!("../../texts/the-time-machine.txt"));
    pub const NOUNS: Text<'static> = Text::new(include_str!("../../texts/nouns.txt"));

    /// Create a `Text` object from a text string
    pub const fn new(text: &str) -> Text {
        Text { text }
    }
    /// Load a vector of lowercase words from file.
    fn load_words(&self) -> Vec<String> {
        self.text
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), " ")
            .split_whitespace()
            .map(|word| word.to_string())
            .collect()
    }
    /// Load a vector of vector of lowercase words corresponding to phrases from file.
    fn load_phrases(&self) -> Vec<Vec<String>> {
        self.text
            .to_lowercase()
            .replace(
                |c: char| !(c.is_alphanumeric() || c == '.' || c == ',' || c == '\n'),
                " ",
            )
            .split(|c: char| c == '.' || c == ',')
            .map(|p| p.split_whitespace().map(|word| word.to_string()).collect())
            .collect()
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
            .into_iter()
            .filter(|s| s.len() >= min_word_length)
            .collect();

        RandomWords { words, n_words }
    }
}

impl PasswordGenerator for RandomWords {
    fn generate_with_seed(&self, rng: &mut ThreadRng, seed: String) -> String {
        let mut string_array = Vec::new();
        if seed.len() > 0 {
            string_array.push(seed)
        }

        for _ in 0..self.n_words {
            let word = self.words.choose(rng).unwrap();
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
            .into_iter()
            .filter(|p| {
                let len = p.len();
                len >= min_length && len <= max_length
            })
            .collect();

        RandomPhrases { phrases }
    }
}

impl PasswordGenerator for RandomPhrases {
    fn generate_with_seed(&self, rng: &mut ThreadRng, seed: String) -> String {
        let phrase = self.phrases.choose(rng).unwrap();
        seed + &phrase.join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_text_load_words() {
        let words = Text::ALICE_IN_WONDERLAND.load_words();
        assert!(words.len() > 0);
    }

    #[test]
    fn test_text_load_phrases() {
        let phrases = Text::ALICE_IN_WONDERLAND.load_phrases();
        assert!(phrases.len() > 0);
    }

    #[test]
    fn test_texts() {
        assert!(Text::ALICE_IN_WONDERLAND.text.len() > 0);
        assert!(Text::THE_TIME_MACHINE.text.len() > 0);
        assert!(Text::NOUNS.text.len() > 0);
    }

    #[test]
    fn test_random_words_from_text() {
        let passwords = RandomWords::from_text(&Text::ALICE_IN_WONDERLAND, 4, 5);

        assert_eq!(passwords.n_words, 4);
        assert!(passwords.words.len() > 0);
        assert!(passwords.words.into_iter().all(|word| word.len() >= 5));
    }

    #[test]
    fn test_random_words_generate_with_seed() {
        let passwords = RandomWords::from_text(&Text::ALICE_IN_WONDERLAND, 4, 5);
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
    fn test_random_phrases_from_text() {
        let passwords = RandomPhrases::from_text(&Text::ALICE_IN_WONDERLAND, 3, 5);

        assert!(passwords.phrases.len() > 0);
        assert!(passwords
            .phrases
            .into_iter()
            .all(|phrase| phrase.len() >= 3 && phrase.len() <= 5));
    }

    #[test]
    fn test_random_phrases_generate_with_seed() {
        let passwords = RandomPhrases::from_text(&Text::ALICE_IN_WONDERLAND, 3, 5);
        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let password = passwords.generate_with_seed(&mut rng, String::new());
            let words: Vec<&str> = password.split(" ").collect();
            assert!(words.len() >= 3 && words.len() <= 5);
        }
    }
}
