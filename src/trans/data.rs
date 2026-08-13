// All functions write data without braces
// Units are passed because of ambiguity

use super::{text, aux::{self, NodeView}};
use crate::{
    NeniyError::Translation,
    Result,
    lexic::token::{BaseToken, Token, TokenKind},
    synt::{
        aux::ListUnit,
        data::{DataUnit, DataValue, IdWithDataPtr},
        text::Text,
    },
};

struct SimpleUnit {
    key: Token,
    value: BaseToken,
}

impl SimpleUnit {
    fn new(key: Token, value: BaseToken) -> Self {
        SimpleUnit {key, value}
    }
}

struct Equipment<'a> {
    key: Token,
    value: &'a IdWithDataPtr,
}

fn translate_enchantments(node_view: &mut NodeView, list: &[ListUnit]) {
    if list.is_empty() {
        node_view.push_str("{}");
    }

    let mut iter = list.iter();
    let first_unit = iter.next().unwrap();

    node_view.extend([
        "{\"minecraft:",
        node_view.extract_token(first_unit.key),
        "\":",
        node_view.extract_token(first_unit.value),
    ]);

    for unit in iter {
        node_view.push(',');
        node_view.extend([
            "\"minecraft:",
            node_view.extract_token(unit.key),
            "\":",
            node_view.extract_token(unit.value),
        ]);
    }

    node_view.push('{');
}

fn translate_attribute_unit(node_view: &mut NodeView, unit: &SimpleUnit) -> Result<()> {
    match unit.key.kind {
        TokenKind::AttackDamage => Ok(node_view.extend([
            "{id:\"minecraft:attack_damage\",base:",
            node_view.extract_token(unit.value),
            "}",
        ])),
        TokenKind::AttackSpeed => Ok(node_view.extend([
            "{id:\"minecraft:attack_speed\",base:",
            node_view.extract_token(unit.value),
            "}",
        ])),
        TokenKind::Health => Ok(node_view.extend([
            "{id:\"minecraft:max_health\",base:",
            node_view.extract_token(unit.value),
            "}",
        ])),
        TokenKind::Stability => Ok(node_view.extend([
            "{id:\"minecraft:knockback_resistance\",base:",
            node_view.extract_token(unit.value),
            "}",
        ])),

        _ => Err(Translation(
            [
                "unknown attribute ",
                node_view.extract_token(unit.key.base),
                " (internal)",
            ]
            .concat(),
        )),
    }
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

fn translate_equipment_unit(
    node_view: &mut NodeView,
    id_with_data_ptr: &IdWithDataPtr,
    name: &str,
) {
    node_view.extend([
        name,
        ":{id:\"minecraft:",
        node_view.extract_token(id_with_data_ptr.id),
        "\"",
    ]);

    if id_with_data_ptr.data.units.is_empty() {
        node_view.push('}');
    } else {
        node_view.push_str(",components:{");
        translate_item_data(node_view, &id_with_data_ptr.data.units, ":");
        node_view.push_str("}}");
    }
}

fn choose_equipment_unit(node_view: &mut NodeView, unit: &Equipment) -> Result<()> {
    translate_equipment_unit(
        node_view,
        unit.value,
        match unit.key.kind {
            TokenKind::Chest => "chest",
            TokenKind::Feet => "feet",
            TokenKind::Head => "head",
            TokenKind::LeftHand => "offhand",
            TokenKind::Legs => "legs",
            TokenKind::RightHand => "mainhand",

            _ => {
                return Err(Translation(
                    [
                        "unknown equipment key ",
                        node_view.extract_token(unit.key.base),
                        " (internal)",
                    ]
                    .concat(),
                ));
            }
        },
    );

    Ok(())
}

fn translate_equipment(node_view: &mut NodeView, equipment: &[Equipment]) -> Result<()> {
    node_view.push_str("equipment:{");

    let mut iter = equipment.iter();
    choose_equipment_unit(node_view, iter.next().unwrap())?;

    for unit in iter {
        node_view.push(',');
        choose_equipment_unit(node_view, unit)?;
    }

    node_view.push('}');

    Ok(())
}

fn translate_tags(node_view: &mut NodeView, tags: &[BaseToken]) {
    node_view.push_str("Tags:[");

    let mut iter = tags.iter();
    node_view.push_str(node_view.extract_token(*iter.next().unwrap()));

    for tag in iter {
        node_view.push(',');
        node_view.push_str(node_view.extract_token(*tag));
    }

    node_view.push(']');
}

fn choose_chance_unit(node_view: &mut NodeView, unit: &SimpleUnit) -> Result<()> {
    node_view.extend([
        match unit.key.kind {
            TokenKind::ChestChance => "chest:",
            TokenKind::FeetChance => "feet:",
            TokenKind::HeadChance => "head:",
            TokenKind::LeftHandChance => "offhand:",
            TokenKind::LegsChance => "legs:",
            TokenKind::RightHandChance => "mainhand:",

            _ => {
                return Err(Translation(
                    [
                        "unknown chance unit ",
                        node_view.extract_token(unit.key.base),
                        " (internal)",
                    ]
                    .concat(),
                ));
            }
        },
        node_view.extract_token(unit.value),
    ]);

    Ok(())
}

fn translate_chances(node_view: &mut NodeView, chances: &[SimpleUnit]) -> Result<()> {
    node_view.push_str("drop_chances:{");

    let mut iter = chances.iter();
    choose_chance_unit(node_view, iter.next().unwrap());

    for unit in iter {
        node_view.push(',');
        choose_chance_unit(node_view, unit)?;
    }

    Ok(())
}

fn block_data_match<'a: 'b, 'b>(
    node_view: &mut NodeView,
    unit: &'a DataUnit,
    separator: &str,
    mut _sign_msgs: &'b [Text],
    with_comma: bool,
) -> Result<()> {
    if unit.key.kind == TokenKind::Sign {
        let DataValue::Lore(msgs) = &unit.value else {
            return Err(Translation(
                "invalid enum variant in block_data_match(): not a lore (internal)".to_string(),
            ));
        };

        _sign_msgs = msgs;
        return Ok(());
    }

    if with_comma {
        node_view.push(',');
    }

    let DataValue::Identifier(value) = unit.value else {
        return Err(Translation(
            "invalid enum variant in block_data_match(): not a id (internal)".to_string(),
        ));
    };

    match unit.key.kind {
        TokenKind::Axis => node_view.extend(["axis", separator, node_view.extract_token(value)]),
        TokenKind::East => node_view.extend(["east", separator, "\"true\""]),
        TokenKind::Facing => node_view.extend([
            "facing",
            separator,
            "\"",
            node_view.extract_token(value),
            "\"",
        ]),
        TokenKind::Half => node_view.extend([
            "half",
            separator,
            "\"",
            node_view.extract_token(value),
            "\"",
        ]),
        TokenKind::Level => node_view.extend([
            "level",
            separator,
            "\"",
            node_view.extract_token(value),
            "\"",
        ]),
        TokenKind::Lit => node_view.extend(["lit", separator, "\"true\""]),
        TokenKind::North => node_view.extend(["north", separator, "\"true\""]),
        TokenKind::Open => node_view.extend(["open", separator, "\"true\""]),
        TokenKind::Powered => node_view.extend(["powered", separator, "\"true\""]),
        TokenKind::South => node_view.extend(["south", separator, "\"true\""]),
        TokenKind::West => node_view.extend(["west", separator, "\"true\""]),

        _ => {
            return Err(Translation(
                [
                    "unknown key ",
                    node_view.extract_token(unit.key.base),
                    " in block data",
                ]
                .concat(),
            ));
        }
    }

    Ok(())
}

fn translate_item(node_view: &mut NodeView, id_with_data_ptr: &IdWithDataPtr) {
    node_view.extend([
        "{id:\"minecraft:",
        node_view.extract_token(id_with_data_ptr.id),
        "\"",
    ]);

    if !id_with_data_ptr.data.units.is_empty() {
        node_view.push_str(",components:{");
        translate_item_data(node_view, &id_with_data_ptr.data.units, ":");
        node_view.push('}');
    }

    node_view.push('}');
}

fn translate_block_state(
    node_view: &mut NodeView,
    unit: &DataUnit,
    is_falling_block: bool,
) -> Result<()> {
    let id_with_data_ptr = unit.value.as_id_with_data()?;
    let block_state = if is_falling_block {
        "BlockState"
    } else {
        "block_state"
    };

    node_view.extend([
        block_state,
        ":{Name:\"minecraft:",
        node_view.extract_token(id_with_data_ptr.id),
        "\"",
    ]);

    if !id_with_data_ptr.data.units.is_empty() {
        node_view.push_str(",Properties:{");
        translate_block_data(node_view, &id_with_data_ptr.data.units, ":");
        node_view.push('}');
    }

    node_view.push('}');
    Ok(())
}

fn entity_data_match<'a>(
    node_view: &mut NodeView,
    unit: &'a DataUnit,
    attributes: &mut Vec<SimpleUnit>,
    equipment: &mut Vec<Equipment<'a>>,
    tags: &mut Vec<BaseToken>,
    chances: &mut Vec<SimpleUnit>,
    with_comma: bool,
) -> Result<()> {
    match unit.key.kind {
        TokenKind::AttackDamage | TokenKind::AttackSpeed | TokenKind::Stability => {
            attributes.push(SimpleUnit {
                key: unit.key,
                value: unit.value.as_id()?,
            });
        },
        TokenKind::Chest | TokenKind::Feet | TokenKind::Head | TokenKind::LeftHand | TokenKind::Legs | TokenKind::RightHand => {
            equipment.push(Equipment {
                key: unit.key,
                value: unit.value.as_id_with_data()?,
            });
        },
        TokenKind::ChestChance | TokenKind::FeetChance | TokenKind::HeadChance | TokenKind::LeftHandChance | TokenKind::LegsChance | TokenKind::RightHandChance => {
            chances.push(SimpleUnit {
                key: unit.key,
                value: unit.value.as_id()?,
            })
        },
        TokenKind::Tag => tags.push(unit.value.as_id()?),

        other_kind => {
            if with_comma {
                node_view.push(',');
            }

            match other_kind {
                TokenKind::About => {
                    node_view.push_str("item:");
                    translate_item(node_view, unit.value.as_id_with_data()?);
                }
                TokenKind::Block => translate_block_state(node_view, unit, true)?,
                TokenKind::CanGrab => node_view.push_str("CanPickUpLoot:1b"),
                TokenKind::Crit => node_view.push_str("crit:1b"),
                TokenKind::Health => {
                    let value = unit.value.as_id()?;
        
                    attributes.push(SimpleUnit {
                        key: unit.key,
                        value,
                    });
                    node_view.extend(["Health:", node_view.extract_token(value)]);
                },
                TokenKind::Height => node_view.extend(["height:", node_view.extract_token(unit.value.as_id()?)]),
                TokenKind::HurtTime => node_view.extend(["HurtTime:", node_view.extract_token(unit.value.as_id()?), "s"]),
                TokenKind::Id /*id for block_display*/ => 
                    translate_block_state(node_view, unit, false)?,
                TokenKind::InGround => node_view.push_str("inGround:1b"),
                TokenKind::Interaction => node_view.push_str("interaction:{}"),
                TokenKind::Invisible => node_view.push_str("Invisible:1b"),
                TokenKind::Invulnerable => node_view.push_str("Invulnerable:1b"),
                TokenKind::Item => {
                    node_view.push_str("Item:");
                    translate_item(node_view, unit.value.as_id_with_data()?);
                }
                TokenKind::LootTable => node_view.extend(["DeathLootTable:\"", node_view.extract_token(unit.value.as_id()?), "\""]),
                TokenKind::Name => {
                    node_view.push_str("CustomName:");
                    text::translate_text(node_view, unit.value.as_text()?);
                },
                TokenKind::NameVisible => node_view.push_str("CustomNameVisible:1b"),
                TokenKind::NoAI => node_view.push_str("NoAI:1b"),
                TokenKind::NoDespawn => node_view.push_str("PersistenceRequired:1b"),
                TokenKind::NoGravity => node_view.push_str("NoGravity:1b"),
                TokenKind::PickupDelay => node_view.extend(["PickupDelay:", node_view.extract_token(unit.value.as_id()?)]),
                TokenKind::Scale => {
                    node_view.push_str("transformation:{scale:");
                    aux::translate_numeric_list(node_view, unit.value.as_list()?, "f");
                    node_view.push('}');
                },
                TokenKind::Rotation => {
                    node_view.push_str("Rotation:");
                    aux::translate_numeric_list(node_view, unit.value.as_list()?, "f");
                },
                TokenKind::SelectedItem => {
                    node_view.push_str("SelectedItem:");
                    translate_item(node_view, unit.value.as_id_with_data()?)
                },
                TokenKind::Shine => node_view.push_str("Glowing:1b"),
                TokenKind::Silent => node_view.push_str("Silent:1b"),
                TokenKind::Size => node_view.extend(["Size:", node_view.extract_token(unit.value.as_id()?)]),
                TokenKind::TeleportDuration => node_view.extend(["teleport_duration:", node_view.extract_token(unit.value.as_id()?)]),
                TokenKind::Text => {
                    node_view.push_str("text:");
                    text::translate_text(node_view, unit.value.as_text()?);
                }
                TokenKind::Width => node_view.extend(["width:", node_view.extract_token(unit.value.as_id()?)]),

                _ => return Err(Translation(["unknown key ", node_view.extract_token(unit.key.base), " in entity data"].concat())),
            }
        }
    }

    Ok(())
}

fn item_data_match(node_view: &mut NodeView, unit: &DataUnit, separator: &str, potion_contents: &mut Vec<SimpleUnit>, attribute_modifiers: &mut Vec<SimpleUnit>, with_comma: bool) -> Result<()> {
    match unit.key.kind {
        TokenKind::AttackDamage | TokenKind::AttackSpeed => attribute_modifiers.push(SimpleUnit::new(unit.key, unit.value.as_id()?)),
        TokenKind::Potion | TokenKind::PotionColor => potion_contents.push(SimpleUnit::new(unit.key, unit.value.as_id()?)),

        other_kind => {
            if with_comma {
                node_view.push(',');
            }

            match other_kind {
                TokenKind::CanPlaceOn => node_view.extend(["can_place_on", separator, "{\"blocks\":\"", node_view.extract_token(unit.value.as_id()?), "\"}"]),
                TokenKind::Enchantments => {
                    node_view.extend(["enchantments", separator]);
                    translate_enchantments(node_view, unit.value.as_list()?);
                },
                TokenKind::Hide => node_view.extend(["tooltip_display", separator, "{hidden_components:[\"attribute_modifiers\",\"enchantments\",\"unbreakable\",\"can_place_on\",\"potion_contents\"]}"]),
                TokenKind::Lore => {
                    node_view.extend(["lore", separator]);
                    text::translate_lore(node_view, unit.value.as_lore()?);
                },
                TokenKind::Name => {
                    node_view.extend(["custom_name", separator]);
                    text::translate_text(node_view, unit.value.as_text()?);
                },
                TokenKind::Shine => node_view.extend(["enchantment_glint_override", separator, "1b"]),
                TokenKind::Stack => node_view.extend(["max_stack_size", separator, node_view.extract_token(unit.value.as_id()?)]),
                TokenKind::Tag => {
                    let custom_data = if separator == "=" { "custom_data" } else { "\"minecraft:custom_data\"" };

                    node_view.extend([custom_data, separator, "{tag:", node_view.extract_token(unit.value.as_id()?), "}"]);
                },
                TokenKind::Unbreakable => node_view.extend(["unbreakable", separator, "{}"]),

                _ => return Err(Translation(["unknown key ", node_view.extract_token(unit.key.base), " in item data"].concat())),
            }
        }
    }

    Ok(())
}

fn particle_data_match(node_view: &mut NodeView, unit: &DataUnit, with_comma: bool) -> Result<()> {
    if with_comma {
        node_view.push(',');
    }

    match unit.key.kind {
        TokenKind::Block => node_view.extend(["block_state:", node_view.extract_token(unit.value.as_id_with_data()?.id)]),
        TokenKind::FromColor | TokenKind::ToColor => {
            node_view.push_str(if unit.key.kind == TokenKind::FromColor { "from_color:" } else { "to_color:" });
            aux::translate_numeric_list(node_view, unit.value.as_list()?, "f");
        },
        TokenKind::Item => node_view.extend(["item:", node_view.extract_token(unit.value.as_id_with_data()?.id)]),
        TokenKind::Scale => node_view.extend(["scale:", node_view.extract_token(unit.value.as_id()?)]),

        _ => return Err(Translation(["unknown key ", node_view.extract_token(unit.key.base), " in particle data"].concat())),
    }

    Ok(())
}

fn potion_contents_match(node_view: &mut NodeView, unit: &SimpleUnit) -> Result<()> {
    match unit.key.kind {
        TokenKind::Potion => node_view.extend(["potion:", node_view.extract_token(unit.value)]),
        TokenKind::PotionColor => node_view.extend(["custom_color:", node_view.extract_token(unit.value)]),

        _ => return Err(Translation(["unknown potion unit ", node_view.extract_token(unit.value)].concat())),
    }

    Ok(())
}

fn translate_potion_contents(node_view: &mut NodeView, potion_contents: &[SimpleUnit], separator: &str) -> Result<()> {
    node_view.extend(["potion_contents", separator, "{"]);

    let mut iter = potion_contents.iter();
    potion_contents_match(node_view, iter.next().unwrap());

    for unit in iter {
        node_view.push(',');
        potion_contents_match(node_view, unit)?;
    }

    node_view.push('}');

    Ok(())

}

fn translate_attribute_modifier(node_view: &mut NodeView, modifier: &SimpleUnit) {
    let name = node_view.extract_token(modifier.key.base);

    node_view.extend(["{type:\"minecraft:", name, "\",amount:", node_view.extract_token(modifier.value), ",operation:\"add_value\",slot:mainhand\",id:\"base_", name, "\"}"]);
}

fn translate_attribute_modifiers(node_view: &mut NodeView, attribute_modifiers: &[SimpleUnit], separator: &str) {
    node_view.extend(["attribute_modifiers", separator, "["]);

    let mut iter = attribute_modifiers.iter();
    translate_attribute_modifier(node_view, iter.next().unwrap());

    for unit in iter {
        node_view.push(',');
        translate_attribute_modifier(node_view, unit);
    }

    node_view.push(']');
}

// Vec<Text> for sign data
pub fn translate_block_data(node_view: &mut NodeView, units: &[DataUnit], separator: &str) -> Result<Vec<Text>> {
    let mut sign_msgs = Vec::new();
    let mut with_comma = false;

    for unit in units.iter() {
        if with_comma {
            block_data_match(node_view, unit, separator, &mut sign_msgs, with_comma)?;
        } else {
            let initial_len = node_view.result().len();

            block_data_match(node_view, unit, separator, &mut sign_msgs, with_comma)?;

            if node_view.result().len() > initial_len {
                with_comma = true;
            }
        }
    }

    Ok(sign_msgs)
}

pub fn translate_entity_data(node_view: &mut NodeView, units: &[DataUnit]) -> Result<()> {
    let mut attributes = Vec::new();
    let mut equipment = Vec::new();
    let mut tags = Vec::new();
    let mut chances = Vec::new();
    let mut with_comma = false;

    for unit in units.iter() {
        if with_comma {
            entity_data_match(node_view, unit, &mut attributes, &mut equipment, &mut tags, &mut chances, with_comma)?;
        } else {
            let initial_len = node_view.result().len();

            entity_data_match(node_view, unit, &mut attributes, &mut equipment, &mut tags, &mut chances, with_comma)?;

            if node_view.result().len() > initial_len {
                with_comma = true;
            }
        }
    }

    if !attributes.is_empty() {
        if with_comma {
            node_view.push(',');
        }

        translate_attributes(node_view, &attributes)?;
        with_comma = true;
    }
    if !equipment.is_empty() {
        if with_comma {
            node_view.push(',');
        }

        translate_equipment(node_view, &equipment)?;
        with_comma = true;
    }
    if !tags.is_empty() {
        if with_comma {
            node_view.push(',');
        }

        translate_tags(node_view, &tags);
        with_comma = true;
    }
    if !chances.is_empty() {
        if with_comma {
            node_view.push(',');
        }

        translate_chances(node_view, &chances)?;
    }

    Ok(())
}

pub fn translate_item_data(node_view: &mut NodeView, units: &[DataUnit], separator: &str) -> Result<()> {
    let mut potion_contents = Vec::new();
    let mut attribute_modifiers = Vec::new();
    let mut with_comma = false;

    for unit in units.iter() {
        if with_comma {
            item_data_match(node_view, unit, separator, &mut potion_contents, &mut attribute_modifiers, with_comma)?;
        } else {
            let initial_len = node_view.result().len();

            item_data_match(node_view, unit, separator, &mut potion_contents, &mut attribute_modifiers, with_comma)?;

            if node_view.result().len() > initial_len {
                with_comma = true;
            }
        }
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
        if with_comma {
            particle_data_match(node_view, unit, with_comma)?;
        } else {
            let initial_len = node_view.result().len();

            particle_data_match(node_view, unit, with_comma)?;

            if node_view.result().len() > initial_len {
                with_comma = true;
            }
        }
    }

    Ok(())
}
