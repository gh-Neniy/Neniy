pub mod compile;
pub mod input;
pub mod lexic;
pub mod synt;
pub mod trans;

use sorted_code::{sorted_enum, sorted_match};
use std::{
    env, io,
    path::{Path, PathBuf},
};

use lexic::token::Index;

#[sorted_enum]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ErrorKind {
    Io,
    Lexic,
    Logic,
    Syntax,
    Translation,
    Warning,
}

#[derive(Debug)]
pub struct NeniyError {
    pub msg: String,
    pub kind: ErrorKind,
    pub start_row: Index,
    pub start_col: Index,
    pub end_row: Index,
    pub end_col: Index,
}

impl NeniyError {
    pub fn new(
        msg: String,
        kind: ErrorKind,
        source_code: &[u8],
        start_pos: Index,
        end_pos: Index,
    ) -> Self {
        let (start_row, start_col, end_row, end_col) =
            Self::calculate_row_col(source_code, start_pos, end_pos);

        NeniyError {
            msg,
            kind,
            start_row,
            start_col,
            end_row,
            end_col,
        }
    }

    pub fn new_io(msg: String) -> Self {
        NeniyError {
            msg,
            kind: ErrorKind::Io,
            start_row: 0,
            start_col: 0,
            end_row: 0,
            end_col: 0,
        }
    }

    pub fn new_warning(msg: String) -> Self {
        NeniyError {
            msg,
            kind: ErrorKind::Warning,
            start_row: 0,
            start_col: 0,
            end_row: 0,
            end_col: 0,
        }
    }

    pub fn as_json(&self, relative_path: &Path) -> String {
        let severity = if self.kind == ErrorKind::Warning {
            "warning"
        } else {
            "error"
        };

        let absolute_path: PathBuf = env::current_dir()
            .unwrap()
            .join(relative_path)
            .components()
            .collect();

        let escaped_msg = self.msg.replace('"', "\\\"");

        format!(
            r#"{{"file":"{}","msg":"{}","severity":"{}","range":{{"start_row":{},"start_col":{},"end_row":{},"end_col":{}}}}}"#,
            absolute_path.to_str().unwrap(),
            escaped_msg,
            severity,
            self.start_row,
            self.start_col,
            self.end_row,
            self.end_col
        )
    }

    pub fn as_error(&self, relative_path: &Path) -> String {
        format!(
            "{} - {}\n{}: {}:{}",
            self.str_kind(),
            self.msg,
            relative_path
                .components()
                .collect::<PathBuf>()
                .to_str()
                .unwrap(),
            self.start_row + 1,
            self.start_col + 1,
        )
    }

    pub fn as_error_base(&self) -> String {
        format!("{} - {}", self.str_kind(), self.msg)
    }

    fn str_kind(&self) -> &str {
        sorted_match!(match self.kind {
            ErrorKind::Io => "Input/output error",
            ErrorKind::Lexic => "Lexic error",
            ErrorKind::Logic => "Logic error",
            ErrorKind::Syntax => "Syntax error",
            ErrorKind::Translation => "Translation error",
            ErrorKind::Warning => "Warning",
        })
    }

    fn calculate_row_col(
        source_code: &[u8],
        start_pos: Index,
        end_pos: Index,
    ) -> (Index, Index, Index, Index) {
        let mut start_row = 0;
        let mut start_col = 0;
        let mut pos = 0;

        while pos < start_pos {
            let byte = source_code[pos as usize];

            if byte == b'\n' {
                start_row += 1;
                start_col = 0;
            } else if (byte & 0b1100_0000) != 0b1000_0000 {
                start_col += 1;
            }

            pos += 1;
        }

        let mut end_row = start_row;
        let mut end_col = start_col;

        while pos < end_pos {
            let byte = source_code[pos as usize];

            if byte == b'\n' {
                end_row += 1;
                end_col = 0;
            } else if (byte & 0b1100_0000) != 0b1000_0000 {
                end_col += 1;
            }

            pos += 1;
        }

        (start_row, start_col, end_row, end_col)
    }
}

impl From<io::Error> for NeniyError {
    fn from(error: io::Error) -> Self {
        Self::new_io(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, NeniyError>;
