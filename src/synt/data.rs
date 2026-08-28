use sorted_code::{sorted_match, sorted_methods};

use super::{
    aux::{self, List, ListUnit, State},
    text::{self, Text, TextUnit},
};
use crate::{
    ErrorKind::{Logic, Syntax},
    NeniyError, Result,
    lexic::token::{Token, TokenKind},
};

#[derive(Debug)]
pub struct DataUnit {
    pub key: Token,
    pub value: DataValue,
}

impl DataUnit {
    pub fn new(key: Token, value: DataValue) -> Self {
        DataUnit { key, value }
    }
}

pub type Data = Vec<DataUnit>;

#[derive(Debug)]
pub struct IdWithData {
    pub data: Data,
    pub id: Token,
}

pub type DataPtr = Box<Data>;
pub type IdWithDataPtr = Box<IdWithData>;

#[derive(Debug)]
pub enum DataValue {
    Nothing,
    Id(Token),
    Data(DataPtr),
    IdWithData(IdWithDataPtr),
    List(List),
    Text(Text),
    Lore(Vec<Text>),
}

#[sorted_methods]
impl DataValue {
    pub fn as_data(&self) -> Result<&Data> {
        if let DataValue::Data(data_ptr) = self {
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

    pub fn as_id(&self) -> Result<Token> {
        if let DataValue::Id(id) = self {
            Ok(*id)
        } else {
            Err(NeniyError {
                msg: self.error("Id"),
                kind: Logic,
                start_row: 0,
                start_col: 0,
                end_row: 0,
                end_col: 0,
            })
        }
    }

    pub fn as_id_with_data(&self) -> Result<&IdWithData> {
        if let DataValue::IdWithData(id_with_data_ptr) = self {
            Ok(id_with_data_ptr)
        } else {
            Err(NeniyError {
                msg: self.error("IdWithData"),
                kind: Logic,
                start_row: 0,
                start_col: 0,
                end_row: 0,
                end_col: 0,
            })
        }
    }

    pub fn as_list(&self) -> Result<&[ListUnit]> {
        if let DataValue::List(list) = self {
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

    pub fn as_lore(&self) -> Result<&[Text]> {
        if let DataValue::Lore(lore) = self {
            Ok(lore)
        } else {
            Err(NeniyError {
                msg: self.error("Lore"),
                kind: Logic,
                start_row: 0,
                start_col: 0,
                end_row: 0,
                end_col: 0,
            })
        }
    }

    pub fn as_text(&self) -> Result<&[TextUnit]> {
        if let DataValue::Text(text) = self {
            Ok(text)
        } else {
            Err(NeniyError {
                msg: self.error("Text"),
                kind: Logic,
                start_row: 0,
                start_col: 0,
                end_row: 0,
                end_col: 0,
            })
        }
    }

    #[sort_end]
    fn error(&self, tried: &str) -> String {
        let actual = match self {
            DataValue::Nothing => "Nothing",
            DataValue::Id(_) => "Id",
            DataValue::Data(_) => "Data",
            DataValue::IdWithData(_) => "IdWithData",
            DataValue::List(_) => "List",
            DataValue::Text(_) => "Text",
            DataValue::Lore(_) => "Lore",
        };

        [
            "tried to extract DataValue as ",
            tried,
            ", but actual variant was ",
            actual,
            " (internal)",
        ]
        .concat()
    }
}

// state[0] == '['
pub fn parse_data(state: &mut State) -> Result<DataPtr> {
    Ok(DataPtr::new(parse_data_units(state)?))
}

// state[0] - id
pub fn parse_id_with_data(state: &mut State) -> Result<IdWithDataPtr> {
    let id = state[0];

    if state.exceed(1) || !aux::valid_data(state[1]) {
        return Ok(IdWithDataPtr::new(IdWithData {
            data: Vec::new(),
            id,
        }));
    }

    *state += 1;

    Ok(IdWithDataPtr::new(IdWithData {
        data: parse_data_units(state)?,
        id,
    }))
}

fn capture_mono_item(state: &mut State) -> DataUnit {
    DataUnit {
        key: state[0],
        value: DataValue::Nothing,
    }
}

fn capture_text_item(state: &mut State) -> Result<DataUnit> {
    aux::unit_check(state, "text unit", aux::valid_text)?;

    Ok(DataUnit::new(
        state[-2],
        DataValue::Text(text::parse_text(state)?),
    ))
}

fn capture_numeric_item(state: &mut State) -> Result<DataUnit> {
    aux::unit_check(state, "numeric unit", aux::valid_numeric)?;

    Ok(DataUnit::new(state[-2], DataValue::Id(state[0])))
}

// state[0] == '['
fn capture_lore(state: &mut State) -> Result<Vec<Text>> {
    let mut lore = Vec::new();

    *state += 1;
    while !state.is_empty() && state[0].kind != TokenKind::ClosingSquareBrace {
        if state[0].kind == TokenKind::Comma {
            *state += 1;
            continue;
        }

        if !aux::valid_text(state[0]) {
            return Err(NeniyError::new(
                ["invalid text bound \"", state.extract(0), "\" in lore"].concat(),
                Syntax,
                state.source_code,
                state[0].base.start,
                state[0].base.end,
            ));
        }

        lore.push(text::parse_text(state)?);

        *state += 1;
    }

    if state.is_empty() {
        return Err(NeniyError::new(
            "']' not found for lore".to_string(),
            Syntax,
            state.source_code,
            state[-1].base.end,
            state[-1].base.end,
        ));
    }

    Ok(lore)
}

fn capture_lore_item(state: &mut State) -> Result<DataUnit> {
    aux::unit_check(state, "lore unit", aux::valid_data)?;

    Ok(DataUnit::new(
        state[-2],
        DataValue::Lore(capture_lore(state)?),
    ))
}

fn capture_data_item(state: &mut State) -> Result<DataUnit> {
    aux::unit_check(state, "data unit", aux::valid_data)?;

    Ok(DataUnit::new(
        state[-2],
        DataValue::Data(parse_data(state)?),
    ))
}

fn capture_id_with_data_item(state: &mut State) -> Result<DataUnit> {
    aux::unit_check(state, "id with data unit", aux::valid_id)?;

    Ok(DataUnit::new(
        state[-2],
        DataValue::IdWithData(parse_id_with_data(state)?),
    ))
}

fn capture_id_item(state: &mut State) -> Result<DataUnit> {
    aux::unit_check(state, "id unit", aux::valid_id)?;

    Ok(DataUnit::new(state[-2], DataValue::Id(state[0])))
}

fn capture_value_item(state: &mut State) -> Result<DataUnit> {
    aux::unit_check(state, "value unit", aux::valid_value)?;

    Ok(DataUnit::new(state[-2], DataValue::Id(state[0])))
}

fn capture_list_type_item(state: &mut State) -> Result<DataUnit> {
    aux::unit_check(state, "list unit", aux::valid_data)?;

    Ok(DataUnit::new(
        state[-2],
        DataValue::List(aux::capture_list(state)?),
    ))
}

fn capture_numeric_or_list_item(state: &mut State) -> Result<DataUnit> {
    aux::unit_check(state, "numeric or list unit", aux::valid_numeric_or_list)?;

    if aux::valid_numeric(state[0]) {
        return Ok(DataUnit::new(state[-2], DataValue::Id(state[0])));
    }

    Ok(DataUnit::new(
        state[-2],
        DataValue::List(aux::capture_list(state)?),
    ))
}

fn capture_data_unit(state: &mut State) -> Result<DataUnit> {
    use TokenKind::*;

    sorted_match!(match state[0].kind {
        About | Block | Chest | Feet | Head | Item | LeftHand | Legs | Passenger | RightHand
        | SelectedItem => capture_id_with_data_item(state),
        AttackDamage | AttackSpeed | ChestChance | FeetChance | HeadChance | Health | Height
        | HurtTime | LeftHandChance | LegsChance | Level | PickupDelay | PotionColor
        | RightHandChance | Size | Stability | Stack | TpTime | Width => {
            capture_numeric_item(state)
        }
        Axis | Billboard | CanBreak | CanPlaceOn | Effect | Facing | LootTable | Potion
        | Profession | Type => capture_id_item(state),
        Crit | East | Hide | InGround | Interaction | Invisible | Invulnerable | Lit | Marker
        | NameVisible | NoAI | NoDespawn | NoGravity | NoTrade | North | Open | Powered | Shine
        | Silent | South | Unbreakable | West => Ok(capture_mono_item(state)),
        Data => capture_data_item(state),
        Enchantments | FromColor | Rotation | ToColor => capture_list_type_item(state),
        Lore | Sign => capture_lore_item(state),
        Name | Text => capture_text_item(state),
        Scale => capture_numeric_or_list_item(state),
        Tag => capture_value_item(state),

        _ => Err(NeniyError::new(
            ["unknown key \"", state.extract(0), "\" in data unit"].concat(),
            Syntax,
            state.source_code,
            state[0].base.start,
            state[0].base.end,
        )),
    })
}

// state[0] == '['
fn parse_data_units(state: &mut State) -> Result<Vec<DataUnit>> {
    let mut units = Vec::new();

    *state += 1;
    while !state.is_empty() && state[0].kind != TokenKind::ClosingSquareBrace {
        if state[0].kind == TokenKind::Comma {
            *state += 1;
            continue;
        }

        units.push(capture_data_unit(state)?);

        *state += 1;
    }

    if state.is_empty() {
        return Err(NeniyError::new(
            "']' not found for data".to_string(),
            Syntax,
            state.source_code,
            state[-1].base.end,
            state[-1].base.end,
        ));
    }

    Ok(units)
}
