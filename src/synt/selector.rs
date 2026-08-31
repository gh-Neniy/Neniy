use sorted_code::{sorted_enum, sorted_match, sorted_methods};

use super::{
    aux::{self, List, ListUnit, State},
    data::{self, DataPtr, DataUnit},
};
use crate::{
    ErrorKind::{Logic, Syntax},
    NeniyError, Result,
    lexic::token::{Token, TokenKind},
};

#[sorted_enum]
#[derive(Debug)]
pub enum SelectorValue {
    Data(DataPtr),
    List(List),
    Value(Token),
}

#[sorted_methods]
impl SelectorValue {
    pub fn as_data(&self) -> Result<&[DataUnit]> {
        if let SelectorValue::Data(data_ptr) = self {
            Ok(data_ptr)
        } else {
            Err(NeniyError {
                msg: self.error("Data"),
                kind: Logic,
                start_row: 0,
                start_col: 0,
                end_row: 0,
                end_col: 0,
            })
        }
    }

    pub fn as_list(&self) -> Result<&[ListUnit]> {
        if let SelectorValue::List(list) = self {
            Ok(list)
        } else {
            Err(NeniyError {
                msg: self.error("List"),
                kind: Logic,
                start_row: 0,
                start_col: 0,
                end_row: 0,
                end_col: 0,
            })
        }
    }

    pub fn as_value(&self) -> Result<Token> {
        if let SelectorValue::Value(value) = self {
            Ok(*value)
        } else {
            Err(NeniyError {
                msg: self.error("Value"),
                kind: Logic,
                start_row: 0,
                start_col: 0,
                end_row: 0,
                end_col: 0,
            })
        }
    }

    fn error(&self, tried: &str) -> String {
        let actual = match self {
            SelectorValue::Data(_) => "Data",
            SelectorValue::List(_) => "List",
            SelectorValue::Value(_) => "Value",
        };

        [
            "tried to extract SelectorValue as ",
            tried,
            ", but actual variant was ",
            actual,
            " (internal)",
        ]
        .concat()
    }
}

#[derive(Debug)]
pub struct SelectorUnit {
    pub key: Token,
    pub value: SelectorValue,
}

impl SelectorUnit {
    fn new(key: Token, value: SelectorValue) -> Self {
        SelectorUnit { key, value }
    }
}

#[derive(Debug)]
pub struct Selector {
    pub kind: TokenKind,
    pub units: Vec<SelectorUnit>,
}

impl Selector {
    pub fn new_empty() -> Self {
        Selector {
            kind: TokenKind::Id,
            units: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.kind == TokenKind::Id
    }
}

// "type" always comes last
pub fn parse_selector(state: &mut State, look_ahead: bool) -> Result<Selector> {
    let kind = state[0].kind;

    if state.exceed(1)
        || state[1].kind != TokenKind::OpeningSquareBrace
        || (look_ahead && !have_next_text_block(state)?)
    {
        return Ok(Selector {
            kind,
            units: Vec::new(),
        });
    }

    let mut units = Vec::new();
    let mut type_field: Option<SelectorUnit> = None;

    *state += 2; // on the first token after '['

    while !state.is_empty() && state[0].kind != TokenKind::ClosingSquareBrace {
        if state[0].kind == TokenKind::Comma {
            *state += 1;
            continue;
        }

        let unit = capture_item(state)?;

        if unit.key.kind == TokenKind::Type {
            type_field = Some(unit);
        } else {
            units.push(unit);
        }

        *state += 1;
    }

    if state.is_empty() {
        return Err(NeniyError::new(
            "']' not found in selector parse".to_string(),
            Syntax,
            state.source_code,
            state[-1].base.end,
            state[-1].base.end,
        ));
    }

    if let Some(type_field) = type_field {
        units.push(type_field);
    }

    Ok(Selector { kind, units })
}

fn capture_id_item(state: &mut State) -> Result<SelectorUnit> {
    aux::unit_check(state, "id selector unit", aux::valid_id)?;

    Ok(SelectorUnit::new(state[-2], SelectorValue::Value(state[0])))
}

fn capture_range_item(state: &mut State) -> Result<SelectorUnit> {
    aux::unit_check(state, "range selector unit", aux::valid_range)?;

    Ok(SelectorUnit::new(
        state[-2],
        SelectorValue::Value(aux::capture_range(state)?),
    ))
}

fn capture_data_item(state: &mut State) -> Result<SelectorUnit> {
    aux::unit_check(state, "data selector unit", aux::valid_data)?;

    Ok(SelectorUnit::new(
        state[-2],
        SelectorValue::Data(data::parse_data(state)?),
    ))
}

fn capture_value_item(state: &mut State) -> Result<SelectorUnit> {
    aux::unit_check(state, "value selector unit", aux::valid_value)?;

    Ok(SelectorUnit::new(state[-2], SelectorValue::Value(state[0])))
}

fn capture_numeric_item(state: &mut State) -> Result<SelectorUnit> {
    aux::unit_check(state, "numeric selector unit", aux::valid_numeric)?;

    Ok(SelectorUnit::new(state[-2], SelectorValue::Value(state[0])))
}

fn capture_list_item(state: &mut State) -> Result<SelectorUnit> {
    aux::unit_check(state, "list unit", aux::valid_data)?;

    Ok(SelectorUnit::new(
        state[-2],
        SelectorValue::List(aux::capture_list(state)?),
    ))
}

fn capture_item(state: &mut State) -> Result<SelectorUnit> {
    use TokenKind::*;

    sorted_match!(match state[0].kind {
        Data => capture_data_item(state),
        Distance => capture_range_item(state),
        Dx | Dy | Dz | Limit => capture_numeric_item(state),
        Gm | Sort | Team | Type => capture_id_item(state),
        Score => capture_list_item(state),
        Tag => capture_value_item(state),

        _ => Err(NeniyError::new(
            ["unknown key \"", state.extract(0), "\" in selector unit"].concat(),
            Syntax,
            state.source_code,
            state[0].base.start,
            state[0].base.end,
        )),
    })
}

// state[0] on selector
fn have_next_text_block(state: &mut State) -> Result<bool> {
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
        return Err(NeniyError::new(
            [
                "invalid square brace sequence: \"",
                state.extract_segment(0, offset - 1),
                "\"",
            ]
            .concat(),
            Syntax,
            state.source_code,
            state[0].base.start,
            state[offset as i16 - 1].base.end,
        ));
    }

    // state[offset] on ']' of the first block
    Ok(!state.exceed(offset + 1) && aux::valid_text(state[offset as i16 + 1]))
}
