extern crate inflector;

use inflector::Inflector;

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
    fn generate_with_seed(&mut self, seed: String) -> String {
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
