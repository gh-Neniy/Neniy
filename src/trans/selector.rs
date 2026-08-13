use crate::synt::{
    aux::ListUnit,
    selector::{SelectorUnit, SelectorValue},
};

use super::{aux::NodeView, data};

fn translate_list(node_view: &mut NodeView, list: &[ListUnit]) {
    // without braces
    if list.is_empty() {
        return;
    }

    let mut iter = list.iter();
    let first_unit = iter.next().unwrap();

    node_view.extend([
        node_view.extract_token(first_unit.key),
        "=",
        node_view.extract_token(first_unit.value),
    ]);

    for unit in iter {
        node_view.push(',');
        node_view.extend([
            node_view.extract_token(unit.key),
            "=",
            node_view.extract_token(unit.value),
        ]);
    }
}

fn translate_selector_unit(node_view: &mut NodeView, unit: &SelectorUnit) {
    match &unit.value {
        SelectorValue::Value(value) => {
            let mut key = node_view.extract_token(unit.key.base);

            if key == "gm" {
                key = "gamemode";
            }

            node_view.extend([key, "=", node_view.extract_token(*value)]);
        }
        SelectorValue::Data(data) => {
            node_view.push_str("nbt={");
            data::translate_entity_data(node_view, &data.units);
            node_view.push('}');
        }
        SelectorValue::List(list) => {
            node_view.push_str("scores={");
            translate_list(node_view, &list);
            node_view.push('}');
        }
    }
}
