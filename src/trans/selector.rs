use super::{aux::NodeView, data};
use crate::{
    NeniyError::Translation,
    Result,
    lexic::token::TokenKind,
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
        node_view.extract(first_unit.key),
        "=",
        node_view.extract(first_unit.value),
    ]);

    for unit in iter {
        node_view.push(',');
        node_view.extend([
            node_view.extract(unit.key),
            "=",
            node_view.extract(unit.value),
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

            node_view.extend([key, "=", node_view.extract(*value)]);
        }
        SelectorValue::Data(data) => {
            node_view.push_str("nbt={");
            data::translate_entity_data(node_view, data)?;
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
            return Err(Translation(
                "empty selector in translate_selector() (internal)".to_string(),
            ));
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
