

pub struct ChainedGenerator<'a> {
    first: Box<dyn PasswordGenerator + 'a>,
    second: Box<dyn PasswordGenerator + 'a>,
}

pub trait PasswordGenerator {
    fn generate_with_seed(&mut self, seed: String) -> String {
        seed
    }
    fn generate(&mut self) -> String {
        self.generate_with_seed(String::new())
    }
    fn chain<'a, T>(self, other: T) -> ChainedGenerator<'a>
        where
            Self: Sized + 'a,
            T: PasswordGenerator + Sized + 'a
    {
        ChainedGenerator { 
            first: Box::new(self),
            second: Box::new(other),
        }
    }
}

impl<'a> PasswordGenerator for ChainedGenerator<'a> {
    fn generate_with_seed(&mut self, seed: String) -> String {
        let seed = self.first.generate_with_seed(seed);
        self.second.generate_with_seed(seed)
    }
}
