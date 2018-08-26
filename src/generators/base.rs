use std::str::FromStr;


pub static ASCII_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
pub static ASCII_UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub static DIGITS: &str = "0123456789";


pub trait PasswordGenerator {
    fn generate_with_seed(&mut self, seed: String) -> String {
        seed
    }
    fn generate(&mut self) -> String {
        self.generate_with_seed(String::new())
    }
    fn pipe<'a, T>(self, other: T) -> ChainedGenerator<'a>
        where
            Self: Sized + 'a,
            T: PasswordGenerator + Sized + 'a
    {
        ChainedGenerator { 
            first: Box::new(self),
            second: Box::new(other),
        }
    }
    fn iterator<'a>(self) -> PasswordIterator<'a>
        where Self: Sized + 'a
    {
        PasswordIterator { generator: Box::new(self) }
    }
}

pub struct ChainedGenerator<'a> {
    first: Box<dyn PasswordGenerator + 'a>,
    second: Box<dyn PasswordGenerator + 'a>,
}

impl<'a> PasswordGenerator for ChainedGenerator<'a> {
    fn generate_with_seed(&mut self, seed: String) -> String {
        let seed = self.first.generate_with_seed(seed);
        self.second.generate_with_seed(seed)
    }
}

pub struct Constant { value: String }

impl Constant {
    pub fn new(seed: &str) -> Constant {
        let value = String::from_str(&seed).unwrap();
        Constant { value }
    }
}

impl PasswordGenerator for Constant {
    fn generate_with_seed(&mut self, seed: String) -> String {
        seed + &self.value
    }
}

pub struct PasswordIterator<'a> { generator: Box<dyn PasswordGenerator + 'a> }

impl<'a> Iterator for PasswordIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let password = self.generator.generate();
        Some(password)
    }
}
