use sorted_code::sorted_match;

use super::{
    aux::{self, NodeView},
    data, id,
};
use crate::{
    ErrorKind, NeniyError, Result,
    lexic::token::{Token, TokenKind},
    synt::{
        aux::ListUnit,
        selector::{Selector, SelectorUnit},
    },
};

fn translate_list(node_view: &mut NodeView, list: &[ListUnit]) {
    if list.is_empty() {
        return;
    }

    let mut iter = list.iter();
    let first_unit = iter.next().unwrap();

    node_view.extend([
        node_view.extract(first_unit.key.base),
        "=",
        node_view.extract(first_unit.value.base),
    ]);

    for unit in iter {
        node_view.push(',');
        node_view.extend([
            node_view.extract(unit.key.base),
            "=",
            node_view.extract(unit.value.base),
        ]);
    }
}

fn translate_selector_unit(
    node_view: &mut NodeView,
    unit: &SelectorUnit,
    last_unit: &SelectorUnit,
) -> Result<()> {
    use TokenKind::*;

    sorted_match!(match unit.key.kind {
        Data => {
            let entity = if last_unit.key.kind == Type {
                last_unit.value.as_value()?
            } else {
                Token::new_empty()
            };

            node_view.push_str("nbt={");
            data::translate_entity_data(node_view, entity, unit.value.as_data()?)?;
            node_view.push('}');
        }
        Gm => {
            node_view.push_str("gamemode=");
            let game_mode = unit.value.as_value()?;

            aux::translate_negation(node_view, game_mode)?;

            node_view.push_str(id::game_mode_match(node_view, game_mode)?);
        }
        Score => {
            node_view.push_str("scores={");
            translate_list(node_view, unit.value.as_list()?);
            node_view.push('}');
        }
        Type => {
            node_view.push_str("type=");
            let entity = unit.value.as_value()?;

            aux::translate_negation(node_view, entity)?;

            node_view.push_str(id::entity_match(node_view, entity)?);
        }

        other_kind => node_view.extend([
            sorted_match!(match other_kind {
                Distance => "distance=",
                Dx => "dx=",
                Dy => "dy=",
                Dz => "dz=",
                Limit => "limit=",
                Sort => "sort=",
                Tag => "tag=",
                Team => "team=",

                _ => {
                    return Err(NeniyError::new(
                        [
                            "unknown key \"",
                            node_view.extract(unit.key.base),
                            "\" in selector data",
                        ]
                        .concat(),
                        ErrorKind::Translation,
                        node_view.source_code,
                        unit.key.base.start,
                        unit.key.base.end,
                    ));
                }
            }),
            node_view.extract(unit.value.as_value()?.base),
        ]),
    });

    Ok(())
}

pub fn translate_selector(node_view: &mut NodeView, selector: &Selector) -> Result<()> {
    node_view.push_str(match selector.kind {
        TokenKind::AllSelector => "@e",
        TokenKind::AllPlayerSelector => "@a",
        TokenKind::CurrentSelector => "@s",
        TokenKind::NearestPlayerSelector => "@p",
        TokenKind::RandomPlayerSelector => "@r",

        _ => {
            return Err(NeniyError {
                msg: "empty selector in translate_selector() (internal)".to_string(),
                kind: ErrorKind::Translation,
                start_row: 0,
                start_col: 0,
                end_row: 0,
                end_col: 0,
            });
        }
    });

    if selector.units.is_empty() {
        return Ok(());
    }

    let mut iter = selector.units.iter();
    let last_unit = selector.units.last().unwrap();

    node_view.push('[');
    translate_selector_unit(node_view, iter.next().unwrap(), last_unit)?;

    for unit in iter {
        node_view.push(',');
        translate_selector_unit(node_view, unit, last_unit)?;
    }

    node_view.push(']');
    Ok(())
}
