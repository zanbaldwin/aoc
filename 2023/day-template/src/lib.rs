pub mod part1;
pub mod part2;

#[derive(Debug)]
pub enum Error {
    NotYetImplemented,
    Other(String),
    InvalidCardCharacter(char),
    WrongNumberOfCards(usize),
}
impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Other(value.to_string())
    }
}
