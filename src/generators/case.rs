extern crate inflector;
extern crate rand;

use inflector::Inflector;
use rand::ThreadRng;

use super::base::PasswordGenerator;


pub enum Case {
    Camel,
    Class,
    Kebab,
    Lower,
    Screaming,
    Sentence,
    Snake,
    Table,
    Title,
    Upper,
}

impl PasswordGenerator for Case {
    fn generate_with_seed(&self, _rng: &mut ThreadRng, seed: String) -> String {
        match self {
            Case::Camel => seed.to_camel_case(),
            Case::Class => seed.to_class_case(),
            Case::Kebab => seed.to_kebab_case(),
            Case::Lower => seed.to_lowercase(),
            Case::Screaming => seed.to_screaming_snake_case(),
            Case::Sentence => seed.to_sentence_case(),
            Case::Snake => seed.to_snake_case(),
            Case::Table => seed.to_table_case(),
            Case::Title => seed.to_title_case(),
            Case::Upper => seed.to_uppercase(),
        }.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_generate() {
        assert_eq!(Case::Camel.generate(), "");
    }

    #[test]
    fn test_case_generate_with_seed() {
        let mut rng = rand::thread_rng();
        let seed = "which case is this";
        let case_outputs = [
            (Case::Camel, "whichCaseIsThis"),
            // (Case::Class, "WhichCaseIsThis"),
            (Case::Kebab, "which-case-is-this"),
            (Case::Lower, "which case is this"),
            (Case::Screaming, "WHICH_CASE_IS_THIS"),
            (Case::Snake, "which_case_is_this"),
            (Case::Table, "which_case_is_this"),
            (Case::Title, "Which Case Is This"),
            (Case::Upper, "WHICH CASE IS THIS")
        ];

        for (case, output) in case_outputs.iter() {
            assert_eq!(case.generate_with_seed(&mut rng, seed.to_string()), *output);
        }
    }
}
