use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum CharacterError {
    DisallowedCharCode(u8),
}

impl Display for CharacterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Self::DisallowedCharCode(char_code) => {
                write!(f, "Character code {:#X} is not allowed", char_code)
            }
        }
    }
}
