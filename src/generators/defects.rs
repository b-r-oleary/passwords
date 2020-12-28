extern crate rand;

use std::cmp::min;
use std::collections::HashMap;
use std::string::ToString;

use rand::{Rng, ThreadRng};

use super::base::PasswordGenerator;

/// A `PasswordGenerator` object that will apply defects to an input `seed` string.
pub struct Defects {
    defects: HashMap<char, Vec<char>>,
    min_defects: usize,
    max_defects: usize,
}

impl Defects {
    /// Create a `Defects` object that will apply between `min_defects` and `max_defects`
    /// defects to an input seed string by replacing letters with numbers or symbols
    /// that look similar to those letters.
    pub fn with_symbols(min_defects: usize, max_defects: usize) -> Defects {
        let defects: HashMap<char, Vec<char>> = [
            ("A", "4"),
            ("OoQ", "0"),
            ("E", "3"),
            ("LlIJ", "1"),
            ("ij", "!:;"),
            ("Ss", "$5"),
            ("Zz", "2"),
            ("LVv", "7^"),
            ("a", "@"),
            ("N", r"\%"),
            ("B", r"8\%&"),
            ("Ppq", "9"),
            ("bd", "6&"),
            ("XxfF", "+"),
            ("H", "#"),
        ]
        .into_iter()
        .flat_map(|(s, t): &(&str, &str)| -> Vec<(char, Vec<char>)> {
            s.chars()
                .into_iter()
                .map(|c| {
                    let t_vec: Vec<char> = t.to_string().chars().collect();
                    (c, t_vec)
                })
                .collect()
        })
        .collect();

        Defects {
            defects,
            min_defects,
            max_defects,
        }
    }
    /// Create a `Defects` object that will apply between `min_defects` and `max_defects`
    /// defects to an input seed string by replacing vowels with different vowels.
    pub fn with_vowels(min_defects: usize, max_defects: usize) -> Defects {
        let vowels = vec!['a', 'e', 'i', 'o', 'u'];

        let defects: HashMap<char, Vec<char>> = vowels
            .iter()
            .map(|v| {
                let vowels_less_v = vowels.clone().into_iter().filter(|c| c != v).collect();
                (*v, vowels_less_v)
            })
            .collect();

        Defects {
            defects,
            min_defects,
            max_defects,
        }
    }
}

impl PasswordGenerator for Defects {
    fn generate_with_seed(&self, rng: &mut ThreadRng, seed: String) -> String {
        let chars: Vec<char> = seed.chars().into_iter().collect();

        let mut possible_defect_locations: Vec<usize> = chars
            .iter()
            .enumerate()
            .filter(|(_, c)| self.defects.contains_key(c))
            .map(|(i, _)| i)
            .collect();

        let n_possible: usize = possible_defect_locations.len();

        let n_min: usize = min(n_possible, self.min_defects);
        let n_max: usize = min(n_possible, self.max_defects);

        let n_defects = rng.gen_range(n_min, n_max + 1);

        rng.shuffle(&mut possible_defect_locations);

        let defect_locations = &possible_defect_locations[0..n_defects];

        chars
            .into_iter()
            .enumerate()
            .map(|(i, c): (usize, char)| -> char {
                if defect_locations.contains(&i) {
                    let options = self.defects.get(&c).unwrap();
                    *rng.choose(options).unwrap()
                } else {
                    c
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_defects_with_symbols() {
        let passwords = Defects::with_symbols(1, 1);
        let mut rng = rand::thread_rng();
        let input_outputs = [
            ("A", "4"),
            ("E", "3"),
            ("Q", "0"),
            ("J", "1"),
            ("Z", "2"),
            ("q", "9"),
            ("F", "+"),
            ("H", "#"),
        ];

        for (input, output) in input_outputs.iter() {
            assert_eq!(
                passwords.generate_with_seed(&mut rng, input.to_string()),
                output.to_string()
            );
        }
    }

    #[test]
    fn test_defects_with_vowels_with_replacement() {
        let passwords = Defects::with_vowels(1, 1);
        let mut rng = rand::thread_rng();
        let inputs = ["a", "e", "i", "o", "u"];

        for input in inputs.iter() {
            assert_ne!(
                passwords.generate_with_seed(&mut rng, input.to_string()),
                input.to_string()
            );
        }
    }

    #[test]
    fn test_defects_with_vowels_without_replacement() {
        let passwords = Defects::with_vowels(1, 1);
        let mut rng = rand::thread_rng();
        let inputs = ["A", "E", "I", "O", "U", "b", "c", "d", "f"];

        for input in inputs.iter() {
            assert_eq!(
                passwords.generate_with_seed(&mut rng, input.to_string()),
                input.to_string()
            );
        }
    }
}
