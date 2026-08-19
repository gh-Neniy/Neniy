pub mod lexic;
pub mod synt;
pub mod trans;

use sorted_code::sorted_enum;
use std::io;
use thiserror::Error;

#[sorted_enum]
#[derive(Debug, Error)]
pub enum NeniyError {
    #[error("Input/output error - {0}")]
    Io(String),

    #[error("Lexic error - {0}")]
    Lexic(String),

    #[error("Logic error - {0}")]
    Logic(String),

    #[error("Syntax error - {0}")]
    Syntax(String),

    #[error("Translation error - {0}")]
    Translation(String),
}

impl From<io::Error> for NeniyError {
    fn from(error: io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, NeniyError>;
