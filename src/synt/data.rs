use super::{
    aux::{self, List, State},
    text::{self, Text},
};
use crate::{
    NeniyError::Syntax,
    Result,
    lexic::token::{BaseToken, Token, TokenKind},
};

pub struct DataUnit {
    key: Token,
    value: DataValue,
}

impl DataUnit {
    pub fn new(key: Token, value: DataValue) -> Self {
        DataUnit { key, value }
    }
}

pub struct Data {
    units: Vec<DataUnit>,
}

pub struct IdWithData {
    data: Data,
    id: BaseToken,
}

pub type DataPtr = Box<Data>;
pub type IdWithDataPtr = Box<IdWithData>;

pub enum DataValue {
    Nothing,
    Identifier(BaseToken),
    Data(DataPtr),
    IdWithData(IdWithDataPtr),
    List(List),
    Text(Text),
    Lore(Vec<Text>),
}

// state[0] == '['
pub fn parse_data(state: &mut State) -> Result<DataPtr> {
    Ok(DataPtr::new(Data {
        units: parse_data_units(state)?,
    }))
}

// state[0] - id
pub fn parse_id_with_data(state: &mut State) -> Result<IdWithDataPtr> {
    let id = state[0].base;

    if state.exceed(1) || !aux::valid_data(state[1]) {
        return Ok(IdWithDataPtr::new(IdWithData {
            data: Data { units: Vec::new() },
            id,
        }));
    }

    *state += 1;

    Ok(IdWithDataPtr::new(IdWithData {
        data: Data {
            units: parse_data_units(state)?,
        },
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

    Ok(DataUnit::new(
        state[-2],
        DataValue::Identifier(state[0].base),
    ))
}

// state[0] == '['
fn capture_lore(state: &mut State) -> Result<Vec<Text>> {
    let mut lore = Vec::new();

    *state += 1;
    while !state.exceed(0) && state[0].kind != TokenKind::ClosingSquareBrace {
        if state[0].kind == TokenKind::Comma {
            *state += 1;
            continue;
        }

        if !aux::valid_text(state[0]) {
            return Err(Syntax(
                ["invalid text bound ", state.extract(0), " in lore"].concat(),
            ));
        }

        lore.push(text::parse_text(state)?);

        *state += 1;
    }

    if state.exceed(0) {
        return Err(Syntax("']' not found for lore".to_string()));
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

    Ok(DataUnit::new(
        state[-2],
        DataValue::Identifier(state[0].base),
    ))
}

fn capture_value_item(state: &mut State) -> Result<DataUnit> {
    aux::unit_check(state, "value unit", aux::valid_value)?;

    Ok(DataUnit::new(
        state[-2],
        DataValue::Identifier(state[0].base),
    ))
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
        return Ok(DataUnit::new(
            state[-2],
            DataValue::Identifier(state[0].base),
        ));
    }

    Ok(DataUnit::new(
        state[-2],
        DataValue::List(aux::capture_list(state)?),
    ))
}

fn capture_data_unit(state: &mut State) -> Result<DataUnit> {
    match state[0].kind {
        TokenKind::About => capture_id_with_data_item(state),
        TokenKind::AttackDamage => capture_numeric_item(state),
        TokenKind::AttackSpeed => capture_numeric_item(state),
        TokenKind::Axis => capture_id_item(state),
        TokenKind::Block => capture_id_with_data_item(state),
        TokenKind::CanGrab => Ok(capture_mono_item(state)),
        TokenKind::CanPlaceOn => capture_id_item(state),
        TokenKind::Chest => capture_id_with_data_item(state),
        TokenKind::ChestChance => capture_numeric_item(state),
        TokenKind::Crit => Ok(capture_mono_item(state)),
        TokenKind::East => Ok(capture_mono_item(state)),
        TokenKind::Enchantments => capture_list_type_item(state),
        TokenKind::Facing => capture_id_item(state),
        TokenKind::Feet => capture_id_with_data_item(state),
        TokenKind::FeetChance => capture_numeric_item(state),
        TokenKind::FromColor => capture_list_type_item(state),
        TokenKind::Half => capture_id_item(state),
        TokenKind::Head => capture_id_with_data_item(state),
        TokenKind::HeadChance => capture_numeric_item(state),
        TokenKind::Health => capture_numeric_item(state),
        TokenKind::Hide => Ok(capture_mono_item(state)),
        TokenKind::Height => capture_numeric_item(state),
        TokenKind::HurtTime => capture_numeric_item(state),
        TokenKind::Id => capture_id_with_data_item(state),
        TokenKind::InGround => Ok(capture_mono_item(state)),
        TokenKind::Interaction => Ok(capture_mono_item(state)),
        TokenKind::Invisible => Ok(capture_mono_item(state)),
        TokenKind::Invulnerable => Ok(capture_mono_item(state)),
        TokenKind::Item => capture_id_with_data_item(state),
        TokenKind::LeftHand => capture_id_with_data_item(state),
        TokenKind::LeftHandChance => capture_numeric_item(state),
        TokenKind::Legs => capture_id_with_data_item(state),
        TokenKind::LegsChance => capture_numeric_item(state),
        TokenKind::Level => capture_numeric_item(state),
        TokenKind::Lit => Ok(capture_mono_item(state)),
        TokenKind::LootTable => capture_id_item(state),
        TokenKind::Lore => capture_lore_item(state),
        TokenKind::Name => capture_text_item(state),
        TokenKind::NameVisible => Ok(capture_mono_item(state)),
        TokenKind::North => Ok(capture_mono_item(state)),
        TokenKind::Data => capture_data_item(state),
        TokenKind::NoAI => Ok(capture_mono_item(state)),
        TokenKind::NoDespawn => Ok(capture_mono_item(state)),
        TokenKind::NoGravity => Ok(capture_mono_item(state)),
        TokenKind::Sign => capture_lore_item(state),
        TokenKind::Stability => capture_numeric_item(state),
        TokenKind::Open => Ok(capture_mono_item(state)),
        TokenKind::PickupDelay => capture_numeric_item(state),
        TokenKind::Potion => capture_id_item(state),
        TokenKind::PotionColor => capture_numeric_item(state),
        TokenKind::Powered => Ok(capture_mono_item(state)),
        TokenKind::Rotation => capture_list_type_item(state),
        TokenKind::RightHand => capture_id_with_data_item(state),
        TokenKind::RightHandChance => capture_numeric_item(state),
        TokenKind::Scale => capture_numeric_or_list_item(state),
        TokenKind::SelectedItem => capture_id_with_data_item(state),
        TokenKind::Stack => capture_numeric_item(state),
        TokenKind::Shine => Ok(capture_mono_item(state)),
        TokenKind::Silent => Ok(capture_mono_item(state)),
        TokenKind::Size => capture_numeric_item(state),
        TokenKind::South => Ok(capture_mono_item(state)),
        TokenKind::Tag => capture_value_item(state),
        TokenKind::TeleportDuration => capture_numeric_item(state),
        TokenKind::Text => capture_text_item(state),
        TokenKind::ToColor => capture_list_type_item(state),
        TokenKind::Unbreakable => Ok(capture_mono_item(state)),
        TokenKind::West => Ok(capture_mono_item(state)),
        TokenKind::Width => capture_numeric_item(state),

        _ => Err(Syntax(
            ["unknown key ", state.extract(0), " in data unit"].concat(),
        )),
    }
}

// state[0] == '['
fn parse_data_units(state: &mut State) -> Result<Vec<DataUnit>> {
    let mut units = Vec::new();

    *state += 1;
    while !state.exceed(0) && state[0].kind != TokenKind::ClosingSquareBrace {
        if state[0].kind == TokenKind::Comma {
            *state += 1;
            continue;
        }

        units.push(capture_data_unit(state)?);

        *state += 1;
    }

    if state.exceed(0) {
        return Err(Syntax("']' not found for data".to_string()));
    }

    Ok(units)
}
