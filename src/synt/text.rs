use std::str;

use super::aux::{self, State};
use crate::{
    NeniyError,
    lexic::token::{BaseToken, TokenKind},
};

#[derive(Debug)]
pub struct TextUnit {
    pub source: BaseToken,
    pub color: BaseToken,
    pub bold: bool,
    pub italic: bool,
    pub hieroglyph: bool,
}

impl TextUnit {
    pub fn new(
        source: BaseToken,
        color: BaseToken,
        bold: bool,
        italic: bool,
        hieroglyph: bool,
    ) -> Self {
        TextUnit {
            source,
            color,
            bold,
            italic,
            hieroglyph,
        }
    }
}

#[derive(Debug)]
pub struct Text {
    units: Vec<TextUnit>,
}

impl Text {
    pub fn new() -> Self {
        Text { units: Vec::new() }
    }
}

// state[0] == '{' or '['
pub fn parse_text(state: &mut State) -> Result<Text, NeniyError> {
    if state[0].kind == TokenKind::OpeningCurlyBrace {
        let unit = capture_text_unit(state)?;

        if unit.source.is_empty() {
            return Ok(Text::new());
        }

        return Ok(Text { units: vec![unit] });
    }

    let mut text = Text::new();

    *state += 1;
    while !state.exceed(0) && state[0].kind != TokenKind::ClosingSquareBrace {
        if state[0].kind == TokenKind::Comma {
            *state += 1;
            continue;
        }

        if state[0].kind == TokenKind::OpeningCurlyBrace {
            let unit = capture_text_unit(state)?;

            if !unit.source.is_empty() {
                text.units.push(unit);
            }
        } else {
            return Err(NeniyError::Syntax(
                ["invalid token ", state.extract(0), " instead of '{'"].concat(),
            ));
        }

        *state += 1;
    }

    if state.exceed(0) {
        return Err(NeniyError::Syntax(
            "']' not found in text parsing".to_string(),
        ));
    }

    Ok(text)
}

fn capture_color(state: &mut State) -> Result<BaseToken, NeniyError> {
    aux::unit_check(state, "color in text unit", aux::valid_identifier)?;

    Ok(state[0].base)
}

fn capture_source(state: &mut State) -> Result<BaseToken, NeniyError> {
    aux::unit_check(state, "source in text unit", aux::valid_string)?;

    // text assumed to be in quotations
    Ok(BaseToken {
        start: state[0].base.start + 1,
        end: state[0].base.end - 1,
    })
}

// state[0] == '{'
fn capture_text_unit(state: &mut State) -> Result<TextUnit, NeniyError> {
    let mut unit = TextUnit::new(
        BaseToken::new_empty(),
        BaseToken::new_empty(),
        false,
        false,
        false,
    );

    *state += 1;

    while !state.exceed(0) && state[0].kind != TokenKind::ClosingCurlyBrace {
        match state[0].kind {
            TokenKind::Bold => unit.bold = true,
            TokenKind::Color => unit.color = capture_color(state)?,
            TokenKind::Comma => {}
            TokenKind::Hieroglyph => unit.hieroglyph = true,
            TokenKind::Italic => unit.italic = true,
            TokenKind::Text => unit.source = capture_source(state)?,
            _ => {
                return Err(NeniyError::Syntax(
                    ["unknown key ", state.extract(0), " in text"].concat(),
                ));
            }
        };

        *state += 1;
    }

    if state.exceed(0) {
        return Err(NeniyError::Syntax("'}' not found in text unit".to_string()));
    }

    Ok(unit)
}
