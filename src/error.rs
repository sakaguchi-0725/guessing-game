use std::{fmt, io};

#[derive(Debug)]
pub enum GameError {
    IoError(io::Error),
    ParseError,
    OutOfRange { min: u32, max: u32 },
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::IoError(e) => write!(f, "入力エラーが発生しました: {}", e),
            GameError::ParseError => write!(f, "数字を入力してください"),
            GameError::OutOfRange { min, max } => {
                write!(f, "{}から{}の間で入力してください", min, max)
            }
        }
    }
}
