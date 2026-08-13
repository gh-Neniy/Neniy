use super::token::TokenCategory;

pub fn categorize(c: u8) -> TokenCategory {
    match c {
        b'\'' => TokenCategory::String,
        b'@' => TokenCategory::Selector,
        b'#' | b'!' => TokenCategory::Id,
        _ if is_special(c) => TokenCategory::Special,
        b'-' | b'0'..=b'9' => TokenCategory::Numeric,
        b'_' | b'a'..=b'z' | b'A'..=b'Z' => TokenCategory::Keyword,
        _ if is_operator(c) => TokenCategory::Operator,
        _ if is_control(c) => TokenCategory::Control,
        _ => TokenCategory::Invalid,
    }
}

fn is_control(c: u8) -> bool {
    matches!(c, b',' | b'[' | b']' | b'{' | b'}')
}

fn is_operator(c: u8) -> bool {
    matches!(c, b'+' | b'-' | b'*' | b'/' | b'<' | b'=' | b'>')
}

fn is_special(c: u8) -> bool {
    matches!(c, b'.' | b'~' | b'^')
}
