use super::{aux::NodeView, data};
use crate::{
    ErrorKind::Translation,
    NeniyError, Result,
    lexic::token::{Token, TokenKind},
    synt::{
        aux::ListUnit,
        selector::{Selector, SelectorUnit, SelectorValue},
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

fn translate_selector_unit(node_view: &mut NodeView, unit: &SelectorUnit) -> Result<()> {
    match &unit.value {
        SelectorValue::Value(value) => {
            let mut key = node_view.extract(unit.key.base);

            if key == "gm" {
                key = "gamemode";
            }

            node_view.extend([key, "=", node_view.extract(value.base)]);
        }
        SelectorValue::Data(data) => {
            node_view.push_str("nbt={");
            data::translate_entity_data(node_view, Token::new_empty(), data)?;
            node_view.push('}');
        }
        SelectorValue::List(list) => {
            node_view.push_str("scores={");
            translate_list(node_view, list);
            node_view.push('}');
        }
    }

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
                kind: Translation,
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

    node_view.push('[');
    translate_selector_unit(node_view, iter.next().unwrap())?;

    for unit in iter {
        node_view.push(',');
        translate_selector_unit(node_view, unit)?;
    }

    node_view.push(']');
    Ok(())
}
