use super::{capture_token, categorize, token::Token};
use crate::Result;

fn parse(source_code: &[u8]) -> Result<Vec<Token>> {
    let mut tokens = Vec::with_capacity(source_code.len() / 4);
    let mut i = 0;

    while i < source_code.len() {
        if source_code[i].is_ascii_whitespace() {
            i += 1;
            continue;
        }

        if i + 1 != source_code.len() && &source_code[i..i + 2] == b"//" {
            while i < source_code.len() && source_code[i] != b'\n' {
                i += 1;
            }

            i += 1;
            continue;
        }

        tokens.push(capture_token::capture_token(
            source_code,
            categorize::categorize(source_code[i]),
            i,
        )?);

        i = tokens.last().unwrap().base.end as usize + 1;
    }

    Ok(tokens)
}
