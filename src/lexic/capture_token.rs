use super::token::{self, IndexType, Token, TokenCategory, TokenKind};
use crate::{NeniyError, Result};

pub fn capture_token(
    source_code: &[u8],
    category: TokenCategory,
    start_pos: usize,
) -> Result<Token> {
    if matches!(
        category,
        TokenCategory::Selector
            | TokenCategory::Operator
            | TokenCategory::Control
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
    let start = start_pos as IndexType;

    match category {
        TokenCategory::Control => Ok(Token::new(
            start,
            start,
            token::short_token_kind(&source_code[start_pos..start_pos + 1]),
            category,
        )),
        TokenCategory::Operator => Ok(capture_operator(source_code, start_pos)),
        TokenCategory::Selector => capture_selector(source_code, start_pos),
        TokenCategory::Special => capture_special(source_code, start_pos),

        _ => Err(NeniyError::Lexic(
            "invalid token category in CaptureShortToken() (internal)".to_string(),
        )),
    }
}

fn capture_operator(source_code: &[u8], start_pos: usize) -> Token {
    let mut offset = 0;

    if start_pos + 1 != source_code.len() && source_code[start_pos + 1] == b'=' {
        offset += 1;
    }

    Token::new(
        start_pos as IndexType,
        start_pos as IndexType + offset,
        token::short_token_kind(&source_code[start_pos..start_pos + 1 + offset as usize]),
        TokenCategory::Operator,
    )
}

fn capture_selector(source_code: &[u8], start_pos: usize) -> Result<Token> {
    if start_pos + 1 == source_code.len() {
        return Err(NeniyError::Lexic("@ instead of selector".to_string()));
    }

    Ok(Token::new(
        start_pos as IndexType,
        start_pos as IndexType + 1,
        token::short_token_kind(&source_code[start_pos..start_pos + 2]),
        TokenCategory::Selector,
    ))
}

fn capture_special(source_code: &[u8], start_pos: usize) -> Result<Token> {
    let mut offset = 0;

    if source_code[start_pos] == b'.' {
        if start_pos + 1 == source_code.len() || source_code[start_pos + 1] != b'.' {
            return Err(NeniyError::Lexic("invalid range token".to_string()));
        }

        offset += 1;
    }

    Ok(Token::new(
        start_pos as IndexType,
        start_pos as IndexType + offset,
        token::short_token_kind(&source_code[start_pos..start_pos + 1 + offset as usize]),
        TokenCategory::Special,
    ))
}

fn valid_keyword_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}

fn valid_identifier_char(c: u8) -> bool {
    valid_keyword_char(c) || matches!(c, b'.' | b':' | b'/' | b'!' | b'#')
}

fn valid_numeric_literal_char(c: u8, was_dot: &mut bool) -> bool {
    if c == b'.' {
        if *was_dot {
            return false;
        }

        *was_dot = true;

        return true;
    }

    c.is_ascii_digit()
}

fn valid_string_literal_char(c: u8, finished: &mut bool) -> bool {
    if *finished {
        return false;
    }

    if c == b'\'' {
        *finished = true;
    }

    true
}

fn token_kind(token_body: &[u8]) -> TokenKind {
    if token_body.len() <= 8 {
        return token::short_token_kind(token_body);
    }

    token::long_token_kind(token_body)
}

fn capture_long_token(
    source_code: &[u8],
    mut category: TokenCategory,
    start_pos: usize,
) -> Result<Token> {
    let start = start_pos as IndexType;

    if source_code[start_pos] == b'-'
        && start_pos + 1 != source_code.len()
        && source_code[start_pos + 1] == b'='
    {
        return Ok(Token::new(
            start,
            start + 1,
            TokenKind::MinusEqualOperator,
            TokenCategory::Operator,
        ));
    }

    let mut state = false;
    let mut end_pos = start_pos + 1;

    while end_pos < source_code.len() {
        if category == TokenCategory::NumericLiteral
            && source_code[end_pos] == b'.'
            && (end_pos + 1 == source_code.len() || !source_code[end_pos + 1].is_ascii_digit())
        {
            break;
        }

        let is_valid_char = match category {
            TokenCategory::Keyword => valid_keyword_char(source_code[end_pos]),
            TokenCategory::Identifier => valid_identifier_char(source_code[end_pos]),
            TokenCategory::NumericLiteral => {
                valid_numeric_literal_char(source_code[end_pos], &mut state)
            }
            TokenCategory::StringLiteral => {
                valid_string_literal_char(source_code[end_pos], &mut state)
            }
            TokenCategory::Invalid => !source_code[end_pos].is_ascii_whitespace(),

            _ => {
                return Err(NeniyError::Lexic(
                    "invalid token category in capture_long_token() (internal)".to_string(),
                ));
            }
        };

        if !is_valid_char {
            if matches!(
                category,
                TokenCategory::Keyword | TokenCategory::NumericLiteral
            ) && valid_identifier_char(source_code[end_pos])
            {
                category = TokenCategory::Identifier;
            } else {
                break;
            }
        }

        end_pos += 1;
    }

    if category == TokenCategory::StringLiteral
        && end_pos == source_code.len()
        && *source_code.last().unwrap() != b'\''
    {
        return Err(NeniyError::Lexic(
            "string literal quotation is not closed".to_string(),
        ));
    }

    let token_body = &source_code[start_pos..end_pos];

    let end = end_pos as IndexType - 1;

    match category {
        TokenCategory::Identifier => Ok(Token::new(start, end, TokenKind::Identifier, category)),
        TokenCategory::Keyword => Ok(Token::new(start, end, token_kind(token_body), category)),
        TokenCategory::NumericLiteral => {
            Ok(Token::new(start, end, TokenKind::NumericLiteral, category))
        }
        TokenCategory::StringLiteral => {
            Ok(Token::new(start, end, TokenKind::StringLiteral, category))
        }

        _ => Err(NeniyError::Lexic(
            ["invalid token ", std::str::from_utf8(token_body).unwrap()].concat(),
        )),
    }
}
