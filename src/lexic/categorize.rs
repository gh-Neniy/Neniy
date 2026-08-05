use super::token::TokenCategory;

fn is_control(c: u8) -> bool {
    c == b',' || c == b'[' || c == b']' || c == b'{' || c == b'}'
}

fn is_operator(c: u8) -> bool {
    c == b'+' || c == b'-' || c == b'*' || c == b'/' || c == b'<' || c == b'=' || c == b'>'
}

fn is_special(c: u8) -> bool {
    c == b'.' || c == b'~' || c == b'^'
}

pub fn categorize(c: u8) -> TokenCategory {
    match c {
        b'\'' => TokenCategory::StringLiteral,
        b'@' => TokenCategory::Selector,
        b'#' | b'!' => TokenCategory::Identifier,
        _ if is_special(c) => TokenCategory::Special,
        b'-' | _ if c.is_ascii_digit() => TokenCategory::NumericLiteral,
        b'_' | _ if c.is_ascii_alphabetic() => TokenCategory::Keyword,
        _ if is_operator(c) => TokenCategory::Operator,
        _ if is_control(c) => TokenCategory::Control,
        _ => TokenCategory::Invalid,
    }
}
