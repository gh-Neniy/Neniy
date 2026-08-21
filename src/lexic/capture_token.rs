use sorted_code::sorted_match;

use super::token::{self, Index, Token, TokenCategory, TokenKind};
use crate::{ErrorKind::Lexic, NeniyError, Result};

pub fn capture_token(
    source_code: &[u8],
    category: TokenCategory,
    start_pos: usize,
) -> Result<Token> {
    if matches!(
        category,
        TokenCategory::Control
            | TokenCategory::Operator
            | TokenCategory::Selector
            | TokenCategory::Special
    ) {
        return capture_short_token(source_code, category, start_pos);
    }

    capture_long_token(source_code, category, start_pos)
}

fn capture_short_token(
    source_code: &[u8],
    category: TokenCategory,
    start_pos: usize,
) -> Result<Token> {
    let start = start_pos as Index;

    sorted_match!(match category {
        TokenCategory::Control => Ok(Token::new(
            start,
            start,
            token::short_token_kind(&source_code[start_pos..start_pos + 1]),
            category,
        )),
        TokenCategory::Operator => Ok(capture_operator(source_code, start_pos)),
        TokenCategory::Selector => capture_selector(source_code, start_pos),
        TokenCategory::Special => capture_special(source_code, start_pos),

        _ => Err(NeniyError::new(
            "invalid token category in capture_short_token() (internal)".to_string(),
            Lexic,
            source_code,
            start,
            start,
        )),
    })
}

fn capture_operator(source_code: &[u8], start_pos: usize) -> Token {
    let mut offset = 0;

    if start_pos + 1 != source_code.len() && source_code[start_pos + 1] == b'=' {
        offset = 1;
    }

    Token::new(
        start_pos as Index,
        start_pos as Index + offset,
        token::short_token_kind(&source_code[start_pos..start_pos + 1 + offset as usize]),
        TokenCategory::Operator,
    )
}

fn capture_selector(source_code: &[u8], start_pos: usize) -> Result<Token> {
    let start = start_pos as Index;

    if start_pos + 1 == source_code.len() {
        return Err(NeniyError::new(
            "@ instead of selector".to_string(),
            Lexic,
            source_code,
            start,
            start,
        ));
    }

    Ok(Token::new(
        start,
        start + 1,
        token::short_token_kind(&source_code[start_pos..start_pos + 2]),
        TokenCategory::Selector,
    ))
}

fn capture_special(source_code: &[u8], start_pos: usize) -> Result<Token> {
    let start = start_pos as Index;
    let mut offset = 0;

    if source_code[start_pos] == b'.' {
        if start_pos + 1 == source_code.len() || source_code[start_pos + 1] != b'.' {
            return Err(NeniyError::new(
                "@ instead of selector".to_string(),
                Lexic,
                source_code,
                start,
                start,
            ));
        }

        offset += 1;
    }

    Ok(Token::new(
        start,
        start + offset,
        token::short_token_kind(&source_code[start_pos..start_pos + 1 + offset as usize]),
        TokenCategory::Special,
    ))
}

fn valid_keyword_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}

fn valid_id_char(c: u8) -> bool {
    valid_keyword_char(c) || matches!(c, b'.' | b':' | b'/' | b'!' | b'#')
}

fn token_kind(token_body: &[u8]) -> TokenKind {
    if token_body.len() <= 8 {
        return token::short_token_kind(token_body);
    }

    token::long_token_kind(token_body)
}

fn capture_id(source_code: &[u8], mut end_pos: usize) -> usize {
    while end_pos < source_code.len() {
        if !valid_id_char(source_code[end_pos]) {
            return end_pos - 1;
        }

        end_pos += 1;
    }

    end_pos - 1
}

fn capture_invalid(source_code: &[u8], mut end_pos: usize) -> usize {
    while end_pos < source_code.len() {
        if source_code[end_pos].is_ascii_whitespace() {
            return end_pos - 1;
        }

        end_pos += 1;
    }

    end_pos - 1
}

fn capture_keyword(source_code: &[u8], mut end_pos: usize) -> usize {
    while end_pos < source_code.len() {
        if !valid_keyword_char(source_code[end_pos]) {
            if valid_id_char(source_code[end_pos]) {
                return capture_id(source_code, end_pos + 1);
            }

            return end_pos - 1;
        }

        end_pos += 1;
    }

    end_pos - 1
}

fn capture_numeric(source_code: &[u8], mut end_pos: usize, with_minus: bool) -> usize {
    let mut was_dot = false;

    while end_pos < source_code.len() {
        if source_code[end_pos] == b'.' {
            if (end_pos + 1 != source_code.len() && source_code[end_pos + 1] == b'.') /*separating from ranges*/ || was_dot
            {
                return end_pos - 1;
            }

            was_dot = true;
        } else if !source_code[end_pos].is_ascii_digit() {
            if valid_id_char(source_code[end_pos]) && !with_minus {
                return capture_id(source_code, end_pos + 1);
            }

            return end_pos - 1;
        }

        end_pos += 1;
    }

    end_pos - 1
}

fn capture_string(source_code: &[u8], mut end_pos: usize) -> Result<usize> {
    while end_pos < source_code.len() {
        if source_code[end_pos] == b'\\' {
            end_pos += 2;
            continue;
        }

        if source_code[end_pos] == b'\'' {
            return Ok(end_pos);
        }

        end_pos += 1;
    }

    Err(NeniyError::new(
        "string literal quotation is not closed".to_string(),
        Lexic,
        source_code,
        end_pos as Index - 1,
        end_pos as Index - 1,
    ))
}

fn capture_long_token(
    source_code: &[u8],
    category: TokenCategory,
    start_pos: usize,
) -> Result<Token> {
    let start = start_pos as Index;

    let end_pos = match category {
        TokenCategory::Id => capture_id(source_code, start_pos + 1),
        TokenCategory::Invalid => capture_invalid(source_code, start_pos + 1),
        TokenCategory::Keyword => capture_keyword(source_code, start_pos + 1),
        TokenCategory::Numeric => {
            if source_code[start_pos] == b'-' && start_pos + 1 != source_code.len() {
                let second_sym = source_code[start_pos + 1];

                match second_sym {
                    b'=' => {
                        return Ok(Token::new(
                            start,
                            start + 1,
                            TokenKind::MinusEqualOperator,
                            TokenCategory::Operator,
                        ));
                    }
                    b'.' => {
                        return Err(NeniyError::new(
                            "found \"-.\" instead of valid numeric token".to_string(),
                            Lexic,
                            source_code,
                            start,
                            start + 1,
                        ));
                    }
                    _ if !second_sym.is_ascii_digit() => {
                        return Err(NeniyError::new(
                            "numeric consists of minus only".to_string(),
                            Lexic,
                            source_code,
                            start,
                            start,
                        ));
                    }
                    _ => capture_numeric(source_code, start_pos + 2, true),
                }
            } else {
                capture_numeric(source_code, start_pos + 1, false)
            }
        }
        TokenCategory::String => capture_string(source_code, start_pos + 1)?,

        _ => {
            return Err(NeniyError::new(
                "invalid token category in capture_long_token() (internal)".to_string(),
                Lexic,
                source_code,
                start,
                start,
            ));
        }
    };

    let token_body = &source_code[start_pos..=end_pos];
    let end = end_pos as Index;

    Ok(Token::new(
        start,
        end,
        sorted_match!(match category {
            TokenCategory::Id => TokenKind::Id,
            TokenCategory::Keyword => token_kind(token_body),
            TokenCategory::Numeric => TokenKind::Numeric,
            TokenCategory::String => TokenKind::String,

            _ => {
                return Err(NeniyError::new(
                    [
                        "invalid token \"",
                        std::str::from_utf8(token_body).unwrap(),
                        "\"",
                    ]
                    .concat(),
                    Lexic,
                    source_code,
                    start,
                    end,
                ));
            }
        }),
        category,
    ))
}
