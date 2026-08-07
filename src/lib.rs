mod lexic;
mod synt;
mod trans;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum NeniyError {
    #[error("Lexic error - {0}")]
    Lexic(String),

    #[error("Syntax error - {0}")]
    Syntax(String),

    #[error("Translation error - {0}")]
    Translation(String),
}

pub type Result<T> = std::result::Result<T, NeniyError>;
