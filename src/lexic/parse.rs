use super::capture_token;
use super::categorize;
use super::token::Token;

fn lexic_parse(source_code: &[u8]) -> Vec<Token> {
    let mut tokens = Vec::with_capacity(source_code.len() / 4);

    for mut i in 0..source_code.len() {
        if source_code[i].is_ascii_whitespace() {
            continue;
        }

        if i + 1 != source_code.len() && &source_code[i..i + 2] == b"//" {
            while i < source_code.len() && source_code[i] != b'\n' {
                i += 1;
            }

            continue;
        }

        tokens.push(capture_token::capture_token(
            source_code,
            categorize::categorize(source_code[i]),
            i,
        ));
        i = tokens.last().unwrap().base.end;
    }

    tokens
}
