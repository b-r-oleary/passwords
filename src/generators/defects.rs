extern crate rand;

use std::collections::HashMap;
use std::cmp::min;
use std::string::ToString;

use rand::{Rng, ThreadRng};

use super::base::PasswordGenerator;


pub struct Defects {
    defects: HashMap<char, Vec<char>>,
    min_defects: usize,
    max_defects: usize,
    rng: ThreadRng,
}

impl Defects {
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
                ("H", "#")
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

        let rng = rand::thread_rng();

        Defects { defects, min_defects, max_defects, rng }
    }

    pub fn with_vowels(min_defects: usize, max_defects: usize) -> Defects {

        let vowels = vec!['a', 'e', 'i', 'o', 'u'];

        let defects: HashMap<char, Vec<char>> = vowels.iter()
            .map(|v| {
                let vowels_less_v = vowels.clone().into_iter().filter(|c| c != v).collect();
                (*v, vowels_less_v)
            })
            .collect();

        let rng = rand::thread_rng();

        Defects { defects, min_defects, max_defects, rng }
    }
}

impl PasswordGenerator for Defects {
    fn generate_with_seed(&mut self, seed: String) -> String {

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

        let n_defects = self.rng.gen_range(n_min, n_max + 1);

        self.rng.shuffle(&mut possible_defect_locations);
    
        let defect_locations = &possible_defect_locations[0..n_defects];

        chars.into_iter()
            .enumerate()
            .map(|(i, c): (usize, char)| -> char {
                if defect_locations.contains(&i) {
                    let options = self.defects.get(&c).unwrap();
                    *self.rng.choose(options).unwrap()
                } else {
                    c
                }
            })
            .collect()
    }
}
