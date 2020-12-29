pub mod generators;

pub use generators::base::{ChainedGenerator, Constant, PasswordGenerator};
pub use generators::case::Case;
pub use generators::defects::Defects;
pub use generators::phrase::{RandomPhrases, RandomWords, Text};
pub use generators::random_string::RandomString;

pub fn phrase_passwords() -> ChainedGenerator<'static> {
    Constant::new("")
        .pipe(RandomPhrases::from_text(&Text::THE_TIME_MACHINE, 3, 5))
        .pipe(Case::Class)
        .pipe(RandomString::digits(2))
        .pipe(Defects::with_symbols(1, 1))
        .pipe(Defects::with_vowels(1, 1))
}

pub fn xkcd_passwords<'a>() -> ChainedGenerator<'a> {
    Constant::new("").pipe(
        RandomWords::from_text(&Text::NOUNS, 4, 4).or(RandomWords::from_text(
            &Text::THE_TIME_MACHINE,
            4,
            5,
        )),
    )
}
