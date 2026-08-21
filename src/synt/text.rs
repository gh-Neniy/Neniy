use sorted_code::sorted_match;

use super::aux::{self, State};
use crate::{
    ErrorKind::Syntax,
    NeniyError, Result,
    lexic::token::{BaseToken, TokenKind},
};

#[derive(Debug)]
pub struct TextUnit {
    pub source: BaseToken,
    pub color: BaseToken,
    pub bold: bool,
    pub italic: bool,
    pub alt: bool,
}

impl TextUnit {
    pub fn new(source: BaseToken, color: BaseToken, bold: bool, italic: bool, alt: bool) -> Self {
        TextUnit {
            source,
            color,
            bold,
            italic,
            alt,
        }
    }
}

pub type Text = Vec<TextUnit>;

// state[0] == '{' or '['
pub fn parse_text(state: &mut State) -> Result<Text> {
    if state[0].kind == TokenKind::OpeningCurlyBrace {
        let unit = capture_text_unit(state)?;

        if unit.source.is_empty() {
            return Ok(Text::new());
        }

        return Ok(vec![unit]);
    }

    let mut text = Text::new();
    *state += 1;

    while !state.is_empty() && state[0].kind != TokenKind::ClosingSquareBrace {
        if state[0].kind == TokenKind::Comma {
            *state += 1;
            continue;
        }

        if state[0].kind == TokenKind::OpeningCurlyBrace {
            let unit = capture_text_unit(state)?;

            if !unit.source.is_empty() {
                text.push(unit);
            }
        } else {
            return Err(NeniyError::new(
                [
                    "invalid token: \"",
                    state.extract(0),
                    "\" instead of '{' in text",
                ]
                .concat(),
                Syntax,
                state.source_code,
                state[0].base.start,
                state[0].base.end,
            ));
        }

        *state += 1;
    }

    if state.is_empty() {
        return Err(NeniyError::new(
            "']' not found in text".to_string(),
            Syntax,
            state.source_code,
            state[-1].base.end,
            state[-1].base.end,
        ));
    }

    Ok(text)
}

fn capture_color(state: &mut State) -> Result<BaseToken> {
    aux::unit_check(state, "color in text unit", aux::valid_id)?;

    Ok(state[0].base)
}

fn capture_source(state: &mut State) -> Result<BaseToken> {
    aux::unit_check(state, "source in text unit", aux::valid_string)?;

    // text assumed to be in quotations
    Ok(BaseToken {
        start: state[0].base.start + 1,
        end: state[0].base.end - 1,
    })
}

// state[0] == '{'
fn capture_text_unit(state: &mut State) -> Result<TextUnit> {
    let mut unit = TextUnit::new(
        BaseToken::new_empty(),
        BaseToken::new_empty(),
        false,
        false,
        false,
    );

    *state += 1;

    while !state.is_empty() && state[0].kind != TokenKind::ClosingCurlyBrace {
        sorted_match!(match state[0].kind {
            TokenKind::Alt => unit.alt = true,
            TokenKind::Bold => unit.bold = true,
            TokenKind::Color => unit.color = capture_color(state)?,
            TokenKind::Comma => (),
            TokenKind::Italic => unit.italic = true,
            TokenKind::Text => unit.source = capture_source(state)?,

            _ => {
                return Err(NeniyError::new(
                    ["unknown key \"", state.extract(0), "\" in text"].concat(),
                    Syntax,
                    state.source_code,
                    state[0].base.start,
                    state[0].base.end,
                ));
            }
        });

        *state += 1;
    }

    if state.is_empty() {
        return Err(NeniyError::new(
            "'}' not found in text unit".to_string(),
            Syntax,
            state.source_code,
            state[-1].base.end,
            state[-1].base.end,
        ));
    }

    Ok(unit)
}
