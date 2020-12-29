pub mod generators;

pub use generators::base::{ChainedGenerator, Constant, PasswordGenerator};
pub use generators::case::Case;
pub use generators::defects::Defects;
pub use generators::phrase::{RandomPhrases, RandomWords, Text};
pub use generators::random_string::RandomString;

pub fn phrase_passwords<'a>(text: Option<&Text>) -> ChainedGenerator<'a> {
    let text = text.unwrap_or_else(|| &Text::THE_TIME_MACHINE);
    Constant::empty()
        .pipe(RandomPhrases::from_text(text, 3, 5))
        .pipe(Case::Class)
        .pipe(RandomString::digits(2))
        .pipe(Defects::with_symbols(1, 1))
        .pipe(Defects::with_vowels(1, 1))
}

pub fn xkcd_passwords<'a>(text: Option<&Text>) -> ChainedGenerator<'a> {
    if let Some(text) = text {
        Constant::empty().pipe(RandomWords::from_text(text, 4, 4))
    } else {
        Constant::empty().pipe(
            RandomWords::from_text(&Text::NOUNS, 4, 4).or(RandomWords::from_text(
                &Text::THE_TIME_MACHINE,
                4,
                5,
            )),
        )
    }
}
