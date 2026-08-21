// All functions write data without braces

use sorted_code::sorted_match;

use super::{
    aux::{self, NodeView},
    text,
};
use crate::{
    ErrorKind::Translation,
    NeniyError, Result,
    lexic::token::{BaseToken, Index, Token, TokenKind},
    synt::{
        aux::ListUnit,
        data::{DataUnit, DataValue, IdWithData},
        text::Text,
    },
};

#[derive(Debug)]
struct SimpleUnit {
    key: Token,
    value: BaseToken,
}

impl SimpleUnit {
    fn new(key: Token, value: BaseToken) -> Self {
        SimpleUnit { key, value }
    }
}

#[derive(Debug)]
struct Equipment<'a> {
    key: Token,
    value: &'a IdWithData,
}

#[derive(Debug)]
struct Storage<'a> {
    attributes: Vec<SimpleUnit>,
    equipment: Vec<Equipment<'a>>,
    tags: Vec<BaseToken>,
    chances: Vec<SimpleUnit>,
}

fn translate_enchantments(node_view: &mut NodeView, list: &[ListUnit]) {
    if list.is_empty() {
        node_view.push_str("{}");
    }

    let mut iter = list.iter();
    let first_unit = iter.next().unwrap();

    node_view.extend([
        "{\"minecraft:",
        node_view.extract(first_unit.key),
        "\":",
        node_view.extract(first_unit.value),
    ]);

    for unit in iter {
        node_view.push(',');
        node_view.extend([
            "\"minecraft:",
            node_view.extract(unit.key),
            "\":",
            node_view.extract(unit.value),
        ]);
    }

    node_view.push('}');
}

fn translate_attribute_unit(node_view: &mut NodeView, unit: &SimpleUnit) -> Result<()> {
    node_view.extend([
        "{id:\"minecraft:",
        sorted_match!(match unit.key.kind {
            TokenKind::AttackDamage => "attack_damage",
            TokenKind::AttackSpeed => "attack_speed",
            TokenKind::Health => "max_health",
            TokenKind::Stability => "knockback_resistance",

            _ =>
                return Err(NeniyError::new(
                    [
                        "unknown attribute \"",
                        node_view.extract(unit.key.base),
                        "\" (internal)",
                    ]
                    .concat(),
                    Translation,
                    node_view.source_code,
                    unit.key.base.start,
                    unit.key.base.end,
                )),
        }),
        "\",base:",
        node_view.extract(unit.value),
        "}",
    ]);

    Ok(())
}

fn translate_attributes(node_view: &mut NodeView, attributes: &[SimpleUnit]) -> Result<()> {
    if attributes.is_empty() {
        node_view.push_str("attributes:[]");
        return Ok(());
    }

    let mut iter = attributes.iter();

    node_view.push_str("attributes:[");
    translate_attribute_unit(node_view, iter.next().unwrap())?;

    for unit in iter {
        node_view.push(',');
        translate_attribute_unit(node_view, unit)?;
    }

    node_view.push(']');
    Ok(())
}

fn translate_equipment_unit(node_view: &mut NodeView, unit: &Equipment) -> Result<()> {
    node_view.extend([
        sorted_match!(match unit.key.kind {
            TokenKind::Chest => "chest",
            TokenKind::Feet => "feet",
            TokenKind::Head => "head",
            TokenKind::LeftHand => "offhand",
            TokenKind::Legs => "legs",
            TokenKind::RightHand => "mainhand",

            _ => {
                return Err(NeniyError::new(
                    [
                        "unknown equipment key \"",
                        node_view.extract(unit.key.base),
                        "\" (internal)",
                    ]
                    .concat(),
                    Translation,
                    node_view.source_code,
                    unit.key.base.start,
                    unit.key.base.end,
                ));
            }
        }),
        ":{id:\"minecraft:",
        node_view.extract(unit.value.id),
        "\"",
    ]);

    if unit.value.data.is_empty() {
        node_view.push('}');
    } else {
        node_view.push_str(",components:{");
        translate_item_data(node_view, &unit.value.data, ":")?;
        node_view.push_str("}}");
    }

    Ok(())
}

fn translate_equipment(node_view: &mut NodeView, equipment: &[Equipment]) -> Result<()> {
    node_view.push_str("equipment:{");

    let mut iter = equipment.iter();
    translate_equipment_unit(node_view, iter.next().unwrap())?;

    for unit in iter {
        node_view.push(',');
        translate_equipment_unit(node_view, unit)?;
    }

    node_view.push('}');

    Ok(())
}

fn translate_tags(node_view: &mut NodeView, tags: &[BaseToken]) {
    node_view.push_str("Tags:[");

    let mut iter = tags.iter();
    node_view.extend(["\"", node_view.extract(*iter.next().unwrap()), "\""]);

    for tag in iter {
        node_view.push(',');
        node_view.extend(["\"", node_view.extract(*tag), "\""]);
    }

    node_view.push(']');
}

fn translate_chance_unit(node_view: &mut NodeView, unit: &SimpleUnit) -> Result<()> {
    node_view.extend([
        sorted_match!(match unit.key.kind {
            TokenKind::ChestChance => "chest:",
            TokenKind::FeetChance => "feet:",
            TokenKind::HeadChance => "head:",
            TokenKind::LeftHandChance => "offhand:",
            TokenKind::LegsChance => "legs:",
            TokenKind::RightHandChance => "mainhand:",

            _ => {
                return Err(NeniyError::new(
                    [
                        "unknown chance unit \"",
                        node_view.extract(unit.key.base),
                        "\" (internal)",
                    ]
                    .concat(),
                    Translation,
                    node_view.source_code,
                    unit.key.base.start,
                    unit.key.base.end,
                ));
            }
        }),
        node_view.extract(unit.value),
    ]);

    Ok(())
}

fn translate_chances(node_view: &mut NodeView, chances: &[SimpleUnit]) -> Result<()> {
    node_view.push_str("drop_chances:{");

    let mut iter = chances.iter();
    translate_chance_unit(node_view, iter.next().unwrap())?;

    for unit in iter {
        node_view.push(',');
        translate_chance_unit(node_view, unit)?;
    }

    node_view.push('}');
    Ok(())
}

fn block_data_match<'a>(
    node_view: &mut NodeView,
    unit: &'a DataUnit,
    separator: &str,
    with_comma: bool,
) -> Result<Option<(&'a [Text], Index, Index)>> {
    if unit.key.kind == TokenKind::Sign {
        let DataValue::Lore(msgs) = &unit.value else {
            return Err(NeniyError {
                msg: "invalid DataValue variant in block_data_match() for sign: not a lore (internal)".to_string(),
                kind: Translation,
                start_row: 0,
                start_col: 0,
                end_row: 0,
                end_col: 0
            });
        };

        return Ok(Some((msgs, unit.key.base.start, unit.key.base.end)));
    }

    if with_comma {
        node_view.push(',');
    }

    sorted_match!(match unit.key.kind {
        TokenKind::East => node_view.extend(["east", separator, "\"true\""]),
        TokenKind::Lit => node_view.extend(["lit", separator, "\"true\""]),
        TokenKind::North => node_view.extend(["north", separator, "\"true\""]),
        TokenKind::Open => node_view.extend(["open", separator, "\"true\""]),
        TokenKind::Powered => node_view.extend(["powered", separator, "\"true\""]),
        TokenKind::South => node_view.extend(["south", separator, "\"true\""]),
        TokenKind::West => node_view.extend(["west", separator, "\"true\""]),

        other_kind => {
            let DataValue::Id(value) = unit.value else {
                return Err(NeniyError::new(
                    [
                        "unknown key \"",
                        node_view.extract(unit.key.base),
                        "\" in block data with non-id value (internal)",
                    ]
                    .concat(),
                    Translation,
                    node_view.source_code,
                    unit.key.base.start,
                    unit.key.base.end,
                ));
            };

            sorted_match!(match other_kind {
                TokenKind::Axis => node_view.extend(["axis", separator, node_view.extract(value)]),
                TokenKind::Facing =>
                    node_view.extend(["facing", separator, "\"", node_view.extract(value), "\"",]),
                TokenKind::Half =>
                    node_view.extend(["half", separator, "\"", node_view.extract(value), "\"",]),
                TokenKind::Level =>
                    node_view.extend(["level", separator, "\"", node_view.extract(value), "\"",]),

                _ => {
                    return Err(NeniyError::new(
                        [
                            "unknown key \"",
                            node_view.extract(unit.key.base),
                            "\" in block data",
                        ]
                        .concat(),
                        Translation,
                        node_view.source_code,
                        unit.key.base.start,
                        unit.key.base.end,
                    ));
                }
            });
        }
    });

    Ok(None)
}

fn translate_item(node_view: &mut NodeView, id_with_data: &IdWithData) -> Result<()> {
    node_view.extend(["{id:\"minecraft:", node_view.extract(id_with_data.id), "\""]);

    if id_with_data.data.is_empty() {
        node_view.push('}');
    } else {
        node_view.push_str(",components:{");
        translate_item_data(node_view, &id_with_data.data, ":")?;
        node_view.push_str("}}");
    }

    Ok(())
}

fn translate_block_state(
    node_view: &mut NodeView,
    unit: &DataUnit,
    is_falling_block: bool,
) -> Result<()> {
    let id_with_data = unit.value.as_id_with_data()?;
    let block_state = if is_falling_block {
        "BlockState"
    } else {
        "block_state"
    };

    node_view.extend([
        block_state,
        ":{Name:\"minecraft:",
        node_view.extract(id_with_data.id),
        "\"",
    ]);

    if id_with_data.data.is_empty() {
        node_view.push('}');
    } else {
        node_view.push_str(",Properties:{");
        translate_block_data(node_view, &id_with_data.data, ":")?;
        node_view.push_str("}}");
    }

    Ok(())
}

fn entity_data_match<'a>(
    node_view: &mut NodeView,
    unit: &'a DataUnit,
    storage: &mut Storage<'a>,
    is_falling_block: bool,
    with_comma: bool,
) -> Result<bool> {
    use TokenKind::*;

    let mut is_changed = false;

    sorted_match!(match unit.key.kind {
        AttackDamage | AttackSpeed | Stability => {
            storage
                .attributes
                .push(SimpleUnit::new(unit.key, unit.value.as_id()?));
        }
        Chest | Feet | Head | LeftHand | Legs | RightHand => {
            storage.equipment.push(Equipment {
                key: unit.key,
                value: unit.value.as_id_with_data()?,
            });
        }
        ChestChance | FeetChance | HeadChance | LeftHandChance | LegsChance | RightHandChance => {
            storage
                .chances
                .push(SimpleUnit::new(unit.key, unit.value.as_id()?));
        }
        Tag => storage.tags.push(unit.value.as_id()?),

        other_kind => {
            is_changed = true;

            if with_comma {
                node_view.push(',');
            }

            sorted_match!(match other_kind {
                About => {
                    node_view.push_str("item:");
                    translate_item(node_view, unit.value.as_id_with_data()?)?;
                }
                Block => translate_block_state(node_view, unit, is_falling_block)?,
                CanGrab => node_view.push_str("CanPickUpLoot:1b"),
                Crit => node_view.push_str("crit:1b"),
                Health => {
                    let value = unit.value.as_id()?;

                    storage.attributes.push(SimpleUnit {
                        key: unit.key,
                        value,
                    });
                    node_view.extend(["Health:", node_view.extract(value)]);
                }
                Height => node_view.extend(["height:", node_view.extract(unit.value.as_id()?)]),
                HurtTime =>
                    node_view.extend(["HurtTime:", node_view.extract(unit.value.as_id()?), "s"]),
                InGround => node_view.push_str("inGround:1b"),
                Interaction => node_view.push_str("interaction:{}"),
                Invisible => node_view.push_str("Invisible:1b"),
                Invulnerable => node_view.push_str("Invulnerable:1b"),
                Item => {
                    node_view.push_str("Item:");
                    translate_item(node_view, unit.value.as_id_with_data()?)?;
                }
                LootTable => node_view.extend([
                    "DeathLootTable:\"",
                    node_view.extract(unit.value.as_id()?),
                    "\""
                ]),
                Name => {
                    node_view.push_str("CustomName:");
                    text::translate_text(node_view, unit.value.as_text()?);
                }
                NameVisible => node_view.push_str("CustomNameVisible:1b"),
                NoAI => node_view.push_str("NoAI:1b"),
                NoDespawn => node_view.push_str("PersistenceRequired:1b"),
                NoGravity => node_view.push_str("NoGravity:1b"),
                PickupDelay =>
                    node_view.extend(["PickupDelay:", node_view.extract(unit.value.as_id()?)]),
                Rotation => {
                    node_view.push_str("Rotation:");
                    aux::translate_numeric_list(node_view, unit.value.as_list()?, "f");
                }
                Scale => {
                    node_view.push_str("transformation:{scale:");
                    aux::translate_numeric_list(node_view, unit.value.as_list()?, "f");
                    node_view.push('}');
                }
                SelectedItem => {
                    node_view.push_str("SelectedItem:");
                    translate_item(node_view, unit.value.as_id_with_data()?)?
                }
                Shine => node_view.push_str("Glowing:1b"),
                Silent => node_view.push_str("Silent:1b"),
                Size => node_view.extend(["Size:", node_view.extract(unit.value.as_id()?)]),
                Text => {
                    node_view.push_str("text:");
                    text::translate_text(node_view, unit.value.as_text()?);
                }
                TpTime =>
                    node_view.extend(["teleport_duration:", node_view.extract(unit.value.as_id()?)]),
                Width => node_view.extend(["width:", node_view.extract(unit.value.as_id()?)]),

                _ =>
                    return Err(NeniyError::new(
                        [
                            "unknown key \"",
                            node_view.extract(unit.key.base),
                            "\" in entity data"
                        ]
                        .concat(),
                        Translation,
                        node_view.source_code,
                        unit.key.base.start,
                        unit.key.base.end,
                    )),
            })
        }
    });

    Ok(is_changed)
}

fn item_data_match(
    node_view: &mut NodeView,
    unit: &DataUnit,
    separator: &str,
    potion_contents: &mut Vec<SimpleUnit>,
    attribute_modifiers: &mut Vec<SimpleUnit>,
    with_comma: bool,
) -> Result<bool> {
    let mut is_changed = false;

    sorted_match!(match unit.key.kind {
        TokenKind::AttackDamage | TokenKind::AttackSpeed => {
            attribute_modifiers.push(SimpleUnit::new(unit.key, unit.value.as_id()?))
        }
        TokenKind::Potion | TokenKind::PotionColor => {
            potion_contents.push(SimpleUnit::new(unit.key, unit.value.as_id()?))
        }

        other_kind => {
            is_changed = true;

            if with_comma {
                node_view.push(',');
            }

            sorted_match!(match other_kind {
                TokenKind::CanPlaceOn => node_view.extend([
                    "can_place_on", separator, "{\"blocks\":\"", node_view.extract(unit.value.as_id()?), "\"}"
                ]),
                TokenKind::Enchantments => {
                    node_view.extend(["enchantments", separator]);
                    translate_enchantments(node_view, unit.value.as_list()?);
                },
                TokenKind::Hide => node_view.extend([
                    "tooltip_display",
                    separator,
                    "{hidden_components:[\"attribute_modifiers\",\"enchantments\",\"unbreakable\",\"can_place_on\",\"potion_contents\"]}"
                ]),
                TokenKind::Lore => {
                    node_view.extend(["lore", separator]);
                    text::translate_lore(node_view, unit.value.as_lore()?);
                },
                TokenKind::Name => {
                    node_view.extend(["custom_name", separator]);
                    text::translate_text(node_view, unit.value.as_text()?);
                },
                TokenKind::Shine => node_view.extend(["enchantment_glint_override", separator, "1b"]),
                TokenKind::Stack => node_view.extend(["max_stack_size", separator, node_view.extract(unit.value.as_id()?)]),
                TokenKind::Tag => {
                    let custom_data = if separator == "=" { "custom_data" } else { "\"minecraft:custom_data\"" };

                    node_view.extend([custom_data, separator, "{tag:", node_view.extract(unit.value.as_id()?), "}"]);
                },
                TokenKind::Unbreakable => node_view.extend(["unbreakable", separator, "{}"]),

                _ => return Err(NeniyError::new(
                    ["unknown key \"", node_view.extract(unit.key.base), "\" in item data"].concat(),
                    Translation,
                    node_view.source_code,
                    unit.key.base.start,
                    unit.key.base.end
                )),
            })
        }
    });

    Ok(is_changed)
}

fn particle_data_match(node_view: &mut NodeView, unit: &DataUnit, with_comma: bool) -> Result<()> {
    if with_comma {
        node_view.push(',');
    }

    sorted_match!(match unit.key.kind {
        TokenKind::Block => node_view.extend([
            "block_state:",
            node_view.extract(unit.value.as_id_with_data()?.id),
        ]),
        TokenKind::FromColor => {
            node_view.push_str("from_color:");
            aux::translate_numeric_list(node_view, unit.value.as_list()?, "f");
        }
        TokenKind::Item => {
            node_view.extend(["item:", node_view.extract(unit.value.as_id_with_data()?.id)])
        }
        TokenKind::Scale => node_view.extend(["scale:", node_view.extract(unit.value.as_id()?)]),
        TokenKind::ToColor => {
            node_view.push_str("to_color:");
            aux::translate_numeric_list(node_view, unit.value.as_list()?, "f");
        }

        _ => {
            return Err(NeniyError::new(
                [
                    "unknown key \"",
                    node_view.extract(unit.key.base),
                    "\" in particle data",
                ]
                .concat(),
                Translation,
                node_view.source_code,
                unit.key.base.start,
                unit.key.base.end,
            ));
        }
    });

    Ok(())
}

fn translate_potion_unit(node_view: &mut NodeView, unit: &SimpleUnit) -> Result<()> {
    sorted_match!(match unit.key.kind {
        TokenKind::Potion => node_view.extend(["potion:", node_view.extract(unit.value)]),
        TokenKind::PotionColor => {
            node_view.extend(["custom_color:", node_view.extract(unit.value)])
        }

        _ => {
            return Err(NeniyError::new(
                [
                    "unknown potion unit \"",
                    node_view.extract(unit.value),
                    "\"",
                ]
                .concat(),
                Translation,
                node_view.source_code,
                unit.value.start,
                unit.value.end,
            ));
        }
    });

    Ok(())
}

fn translate_potion_contents(
    node_view: &mut NodeView,
    potion_contents: &[SimpleUnit],
    separator: &str,
) -> Result<()> {
    node_view.extend(["potion_contents", separator, "{"]);

    let mut iter = potion_contents.iter();
    translate_potion_unit(node_view, iter.next().unwrap())?;

    for unit in iter {
        node_view.push(',');
        translate_potion_unit(node_view, unit)?;
    }

    node_view.push('}');
    Ok(())
}

fn translate_attribute_modifier(node_view: &mut NodeView, modifier: &SimpleUnit) {
    let name = node_view.extract(modifier.key.base);

    node_view.extend([
        "{type:\"minecraft:",
        name,
        "\",amount:",
        node_view.extract(modifier.value),
        ",operation:\"add_value\",slot:\"mainhand\",id:\"base_",
        name,
        "\"}",
    ]);
}

fn translate_attribute_modifiers(
    node_view: &mut NodeView,
    attribute_modifiers: &[SimpleUnit],
    separator: &str,
) {
    node_view.extend(["attribute_modifiers", separator, "["]);

    let mut iter = attribute_modifiers.iter();
    translate_attribute_modifier(node_view, iter.next().unwrap());

    for unit in iter {
        node_view.push(',');
        translate_attribute_modifier(node_view, unit);
    }

    node_view.push(']');
}

// &[Text] for sign messages
pub fn translate_block_data<'a>(
    node_view: &mut NodeView,
    units: &'a [DataUnit],
    separator: &str,
) -> Result<Option<(&'a [Text], Index, Index)>> {
    let mut sign_msgs = None;
    let mut with_comma = false;

    for unit in units.iter() {
        let msgs = block_data_match(node_view, unit, separator, with_comma)?;

        if msgs.is_none() {
            with_comma = true;
        } else {
            sign_msgs = msgs;
        }
    }

    Ok(sign_msgs)
}

pub fn translate_entity_data(
    node_view: &mut NodeView,
    units: &[DataUnit],
    is_falling_block: bool,
) -> Result<()> {
    let mut storage = Storage {
        attributes: Vec::new(),
        equipment: Vec::new(),
        tags: Vec::new(),
        chances: Vec::new(),
    };
    let mut with_comma = false;

    for unit in units.iter() {
        with_comma |=
            entity_data_match(node_view, unit, &mut storage, is_falling_block, with_comma)?;
    }

    if !storage.attributes.is_empty() {
        if with_comma {
            node_view.push(',');
        }

        translate_attributes(node_view, &storage.attributes)?;
        with_comma = true;
    }
    if !storage.equipment.is_empty() {
        if with_comma {
            node_view.push(',');
        }

        translate_equipment(node_view, &storage.equipment)?;
        with_comma = true;
    }
    if !storage.tags.is_empty() {
        if with_comma {
            node_view.push(',');
        }

        translate_tags(node_view, &storage.tags);
        with_comma = true;
    }
    if !storage.chances.is_empty() {
        if with_comma {
            node_view.push(',');
        }

        translate_chances(node_view, &storage.chances)?;
    }

    Ok(())
}

pub fn translate_item_data(
    node_view: &mut NodeView,
    units: &[DataUnit],
    separator: &str,
) -> Result<()> {
    let mut potion_contents = Vec::new();
    let mut attribute_modifiers = Vec::new();
    let mut with_comma = false;

    for unit in units.iter() {
        with_comma |= item_data_match(
            node_view,
            unit,
            separator,
            &mut potion_contents,
            &mut attribute_modifiers,
            with_comma,
        )?;
    }

    if !potion_contents.is_empty() {
        if with_comma {
            node_view.push(',');
        }

        translate_potion_contents(node_view, &potion_contents, separator)?;
        with_comma = true;
    }
    if !attribute_modifiers.is_empty() {
        if with_comma {
            node_view.push(',');
        }

        translate_attribute_modifiers(node_view, &attribute_modifiers, separator);
    }

    Ok(())
}

pub fn translate_particle_data(node_view: &mut NodeView, units: &[DataUnit]) -> Result<()> {
    let mut with_comma = false;

    for unit in units.iter() {
        particle_data_match(node_view, unit, with_comma)?;
        with_comma = true;
    }

    Ok(())
}
