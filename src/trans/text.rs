use super::aux::{self, NodeView};
use crate::synt::text::{Text, TextUnit};

pub fn translate_text(node_view: &mut NodeView, text: &Text) {
    if text.is_empty() {
        node_view.push_str("{text:\"\"}");
        return;
    }

    let mut iter = text.units.iter();
    node_view.push('{');
    translate_unit(node_view, iter.next().unwrap());

    if text.units.len() == 1 {
        node_view.push('}');
        return;
    }

    node_view.push_str(",extra:[{");
    translate_unit(node_view, iter.next().unwrap());
    node_view.push('}');

    for unit in iter {
        node_view.push_str(",{");
        translate_unit(node_view, unit);
        node_view.push('}');
    }

    node_view.push_str("]}");
}

pub fn translate_lore(node_view: &mut NodeView, lore: &[Text]) {
    if lore.is_empty() {
        node_view.push_str("[]");
        return;
    }

    node_view.push('[');
    let mut iter = lore.iter();
    translate_text(node_view, iter.next().unwrap());

    for text in iter {
        node_view.push(',');
        translate_text(node_view, text);
    }

    node_view.push(']');
}

fn translate_unit(node_view: &mut NodeView, unit: &TextUnit) {
    node_view.extend(["text:\"", node_view.extract_token(unit.source), "\""]);

    if !unit.color.is_empty() {
        node_view.extend([",color:\"", node_view.extract_token(unit.color), "\""]);
    }

    let font = if unit.hieroglyph {
        "\"minecraft:alt\""
    } else {
        "\"minecraft:uniform\""
    };

    node_view.extend([
        ",italic:",
        aux::translate_bool(unit.italic),
        ",bold:",
        aux::translate_bool(unit.bold),
        ",font:",
        font,
    ]);
}
