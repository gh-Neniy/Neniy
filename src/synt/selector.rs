use std::str;

use super::{
    aux::{self, ListType, State},
    data::{self, DataPtr},
};
use crate::{
    NeniyError,
    lexic::token::{BaseToken, Token, TokenKind},
};

pub enum SelectorValue {
    Value(BaseToken),
    Data(DataPtr),
    List(ListType),
}

pub struct SelectorUnit {
    key: Token,
    value: SelectorValue,
}

impl SelectorUnit {
    fn new(key: Token, value: SelectorValue) -> Self {
        SelectorUnit { key, value }
    }
}

pub struct Selector {
    stem: Token,
    units: Vec<SelectorUnit>,
}

pub fn parse_selector(state: &mut State, look_ahead: bool) -> Result<Selector, NeniyError> {
    let stem = state[0];

    if state.exceed(1)
        || state[1].kind != TokenKind::OpeningSquareBrace
        || (look_ahead && !have_next_text_block(state)?)
    {
        return Ok(Selector {
            stem,
            units: Vec::new(),
        });
    }

    let mut units = Vec::new();

    *state += 2; // on first token after '['
    while !state.exceed(0) && state[0].kind != TokenKind::ClosingSquareBrace {
        if state[0].kind == TokenKind::Comma {
            *state += 1;
            continue;
        }

        units.push(capture_item(state)?);

        *state += 1;
    }

    if state.exceed(0) {
        return Err(NeniyError::Syntax(
            "']' not found in selector parse".to_string(),
        ));
    }

    Ok(Selector { stem, units })
}

fn capture_id_item(state: &mut State) -> Result<SelectorUnit, NeniyError> {
    aux::unit_check(state, "id selector unit", aux::valid_identifier)?;

    Ok(SelectorUnit::new(
        state[-2],
        SelectorValue::Value(state[0].base),
    ))
}

fn capture_range_item(state: &mut State) -> Result<SelectorUnit, NeniyError> {
    aux::unit_check(state, "range selector unit", aux::valid_range)?;

    Ok(SelectorUnit::new(
        state[-2],
        SelectorValue::Value(aux::capture_range(state)?),
    ))
}

fn capture_data_item(state: &mut State) -> Result<SelectorUnit, NeniyError> {
    aux::unit_check(state, "data selector unit", aux::valid_data)?;

    Ok(SelectorUnit::new(
        state[-2],
        SelectorValue::Data(data::parse_data(state)?),
    ))
}

fn capture_value_item(state: &mut State) -> Result<SelectorUnit, NeniyError> {
    aux::unit_check(state, "value selector unit", aux::valid_value)?;

    Ok(SelectorUnit::new(
        state[-2],
        SelectorValue::Value(state[0].base),
    ))
}

fn capture_numeric_item(state: &mut State) -> Result<SelectorUnit, NeniyError> {
    aux::unit_check(state, "numeric selector unit", aux::valid_numeric)?;

    Ok(SelectorUnit::new(
        state[-2],
        SelectorValue::Value(state[0].base),
    ))
}

fn capture_list_type_item(state: &mut State) -> Result<SelectorUnit, NeniyError> {
    aux::unit_check(state, "list unit", aux::valid_data)?;

    Ok(SelectorUnit::new(
        state[-2],
        SelectorValue::List(aux::capture_list(state)?),
    ))
}

fn capture_item(state: &mut State) -> Result<SelectorUnit, NeniyError> {
    match state[0].kind {
        TokenKind::Distance => capture_range_item(state),
        TokenKind::Data => capture_data_item(state),
        TokenKind::Dx | TokenKind::Dy | TokenKind::Dz => capture_numeric_item(state),
        TokenKind::Gamemode => capture_id_item(state),
        TokenKind::Limit => capture_numeric_item(state),
        TokenKind::Score => capture_list_type_item(state),
        TokenKind::Sort => capture_id_item(state),
        TokenKind::Tag => capture_value_item(state),
        TokenKind::Team | TokenKind::Type => capture_id_item(state),
        TokenKind::XRotation | TokenKind::YRotation => capture_range_item(state),

        _ => Err(NeniyError::Syntax(
            [
                "unknown key ",
                str::from_utf8(state.extract(0)).unwrap(),
                " in selector unit",
            ]
            .concat(),
        )),
    }
}

// state[0] on selector
fn have_next_text_block(state: &mut State) -> Result<bool, NeniyError> {
    let mut offset = 2; // after '['
    let mut balance = 1;

    while !state.exceed(offset) {
        if state[offset as i16].kind == TokenKind::OpeningSquareBrace {
            balance += 1;
        } else if state[offset as i16].kind == TokenKind::ClosingSquareBrace {
            balance -= 1;

            if balance == 0 {
                break;
            }
        }

        offset += 1;
    }

    if state.exceed(offset) && balance > 0 {
        return Err(NeniyError::Syntax(
            [
                "invalid square brace sequence: ",
                str::from_utf8(state.extract_segment(0, offset - 1)).unwrap(),
            ]
            .concat(),
        ));
    }

    // state[offset] on ']' of the first block
    Ok(!state.exceed(offset + 1) && aux::valid_text(state[offset as i16 + 1]))
}
