use super::{
    aux::{self, NodeView},
    data, selector, text,
};
use crate::{
    NeniyError::Translation,
    Result,
    lexic::token::{BaseToken, Index, TokenKind},
    synt::{data::DataUnit, node::Command, selector::Selector},
};

fn translate_block_data(node_view: &mut NodeView, units: &[DataUnit]) -> Result<()> {
    node_view.push('[');
    let sign_msgs = data::translate_block_data(node_view, units, "=")?;
    node_view.push(']');

    if !sign_msgs.is_empty() {
        if sign_msgs.len() != 4 {
            return Err(Translation("sign messages length is not 4".to_string()));
        }

        node_view.push_str("{front_text:{messages:");
        text::translate_lore(node_view, &sign_msgs);
        node_view.push_str("}}");
    }

    Ok(())
}

fn translate_entity(
    node_view: &mut NodeView,
    args: &[BaseToken],
    selector: &Selector,
    mut entity_pos: Index,
) -> Result<Index> {
    if selector.kind == TokenKind::Id {
        node_view.push_str(node_view.extract(args[entity_pos as usize]));
        entity_pos += 1;
    } else {
        selector::translate_selector(node_view, selector)?;
    }

    Ok(entity_pos)
}

fn translate_ex_as(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("as ");

    let (args, _, selector) = node_view.as_selector()?;
    translate_entity(node_view, args, selector, 0)?;

    Ok(())
}

fn translate_ex_at(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("at ");

    let (args, _, selector) = node_view.as_selector()?;
    translate_entity(node_view, args, selector, 0)?;

    Ok(())
}

fn translate_ex_block(node_view: &mut NodeView) -> Result<()> {
    let (args, _, id_with_data) = node_view.as_id_with_data()?;

    node_view.extend([
        node_view.extract(args[0]), // if | unless
        " block ",
        node_view.extract(args[1]), // x
        " ",
        node_view.extract(args[2]), // y
        " ",
        node_view.extract(args[3]), // z
        " ",
        node_view.extract(id_with_data.id), // block
    ]);

    if !id_with_data.data.is_empty() {
        translate_block_data(node_view, &id_with_data.data)?;
    }

    Ok(())
}

fn translate_ex_ent(node_view: &mut NodeView) -> Result<()> {
    let (args, _, selector) = node_view.as_selector()?;

    node_view.extend([
        node_view.extract(args[0]), // condition
        " entity ",
    ]);

    translate_entity(node_view, args, selector, 1)?;

    Ok(())
}

fn translate_ex_items_block(node_view: &mut NodeView) -> Result<()> {
    let (args, _) = node_view.as_base()?;

    node_view.extend([
        node_view.extract(args[0]), // condition
        " items block ~ ~ ~ ",
        node_view.extract(args[1]), // container
        node_view.extract(args[2]), // item name
    ]);

    Ok(())
}

fn translate_ex_items_ent(node_view: &mut NodeView) -> Result<()> {
    let (args, _, selector) = node_view.as_selector()?;

    node_view.extend([
        node_view.extract(args[0]), // condition
        " items entity ",
    ]);

    let current_arg = translate_entity(node_view, args, selector, 1)?;

    node_view.extend([
        " ",
        node_view.extract(args[current_arg as usize]), // container
        " ",
        node_view.extract(args[current_arg as usize]), // item name
    ]);

    Ok(())
}

fn translate_ex_score(node_view: &mut NodeView) -> Result<()> {
    let (args, _, selector) = node_view.as_selector()?;

    node_view.extend([
        node_view.extract(args[0]), // condition
        " score ",
    ]);

    if args.len() == 5 {
        // for operators
        node_view.extend([
            node_view.extract(args[1]), // entity
            node_view.extract(args[2]), // objective
            node_view.extract(args[3]), // operator
            node_view.extract(args[4]), // second entity
            node_view.extract(args[2]), // same objective
        ]);

        return Ok(());
    }

    let current_arg = translate_entity(node_view, args, selector, 1)?;

    node_view.extend([
        " ",
        node_view.extract(args[current_arg as usize]), // objective
        " matches ",
        node_view.extract(args[current_arg as usize + 1]), // value or range
    ]);

    Ok(())
}

fn translate_ex_pos(node_view: &mut NodeView) -> Result<()> {
    let (args, _) = node_view.as_base()?;

    node_view.extend([
        "positioned",
        node_view.extract(args[0]), // x
        " ",
        node_view.extract(args[1]), // y
        " ",
        node_view.extract(args[2]), // z
    ]);

    Ok(())
}

fn translate_ex_store_score(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("store result score ");

    let (args, _, selector) = node_view.as_selector()?;
    let current_arg = translate_entity(node_view, args, selector, 0)?;

    node_view.extend([" ", node_view.extract(args[current_arg as usize])]);

    Ok(())
}

fn translate_ex_store_bossbar(node_view: &mut NodeView) -> Result<()> {
    let (args, _) = node_view.as_base()?;

    node_view.extend([
        "store result bossbar ",
        node_view.extract(args[0]), // bossbar name
        " value",
    ]);

    Ok(())
}

fn translate_ex_store_ent(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("store result entity ");

    let (args, _, selector) = node_view.as_selector()?;
    let current_arg = translate_entity(node_view, args, selector, 0)? as usize;

    node_view.extend([
        " ",
        node_view.extract(args[current_arg]),
        " ",
        node_view.extract(args[current_arg + 1]),
        " ",
        node_view.extract(args[current_arg + 2]),
    ]);

    Ok(())
}

fn translate_ex_store_storage(node_view: &mut NodeView) -> Result<()> {
    let (args, _) = node_view.as_base()?;

    node_view.extend([
        "store result storage ",
        node_view.extract(args[0]),
        " ",
        node_view.extract(args[1]),
        " ",
        node_view.extract(args[2]),
        " ",
        node_view.extract(args[3]),
    ]);

    Ok(())
}

fn translate_ex_uninited(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("unless score ");

    let (args, _, selector) = node_view.as_selector()?;

    let initial_len = node_view.result.len();
    let current_arg = translate_entity(node_view, args, selector, 0)? as usize;

    let entity = node_view.result[initial_len..].to_string();
    let objective = node_view.extract(args[current_arg]);

    node_view.extend([" ", objective, " = ", &entity, " ", objective]);

    Ok(())
}

fn translate_ex_facing(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("facing ");

    let (args, _, selector) = node_view.as_selector()?;

    if selector.kind != TokenKind::Id {
        selector::translate_selector(node_view, selector)?;
    } else if args.len() == 1 {
        node_view.push_str(node_view.extract(args[0]));
    } else {
        node_view.extend([
            node_view.extract(args[0]),
            " ",
            node_view.extract(args[1]),
            " ",
            node_view.extract(args[2]),
        ])
    }

    Ok(())
}

fn translate_advancement(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("advancement grant ");

    let (args, _, selector) = node_view.as_selector()?;
    let current_arg = translate_entity(node_view, args, selector, 0)? as usize;

    node_view.extend([" only ", node_view.extract(args[current_arg])]);

    Ok(())
}

fn translate_attribute(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("attribute ");

    let (args, _, selector) = node_view.as_selector()?;
    let current_arg = translate_entity(node_view, args, selector, 0)? as usize;

    node_view.extend([
        " ",
        node_view.extract(args[current_arg]), // attribute name
        " base set ",
        node_view.extract(args[current_arg + 1]), // value
    ]);

    Ok(())
}

fn translate_bossbar_add(node_view: &mut NodeView) -> Result<()> {
    let (args, _, text) = node_view.as_text()?;

    node_view.extend([
        "bossbar add ",
        node_view.extract(args[0]), // bossbar name
        " ",
    ]);

    text::translate_text(node_view, text);

    Ok(())
}

fn translate_bossbar_set(node_view: &mut NodeView) -> Result<()> {
    let (args, _, selector) = node_view.as_selector()?;
    let submode = node_view.extract(args[1]);

    node_view.extend([
        "bossbar set ",
        node_view.extract(args[0]), // bossbar name
        " ",
        submode,
    ]);

    if submode == "players" {
        translate_entity(node_view, args, selector, 2)?;
    } else {
        // color or max
        node_view.push_str(node_view.extract(args[2]));
    }

    Ok(())
}

fn translate_bossbar_remove(node_view: &mut NodeView) -> Result<()> {
    let (args, _) = node_view.as_base()?;

    node_view.extend([
        "bossbar remove ",
        node_view.extract(args[0]), // bossbar name
    ]);

    Ok(())
}

fn translate_clear(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("clear ");

    let (args, _, selector, id_with_data) = node_view.as_selector_id_with_data()?;
    let current_arg = translate_entity(node_view, args, selector, 0)? as usize;

    node_view.extend([" ", node_view.extract(id_with_data.id)]); // item

    if !id_with_data.data.is_empty() {
        node_view.push('[');
        data::translate_item_data(node_view, &id_with_data.data, "=")?;
        node_view.push(']');
    }

    if args.len() == current_arg + 1 {
        node_view.extend([" ", node_view.extract(args[current_arg])]);
    }

    Ok(())
}

fn translate_clone(node_view: &mut NodeView) -> Result<()> {
    let (args, _) = node_view.as_base()?;

    node_view.extend([
        "clone ",
        node_view.extract(args[0]), // start x
        " ",
        node_view.extract(args[1]), // start y
        " ",
        node_view.extract(args[2]), // start z
        " ",
        node_view.extract(args[3]), // end x
        " ",
        node_view.extract(args[4]), // end y
        " ",
        node_view.extract(args[5]), // end z
        " ",
        node_view.extract(args[6]), // from x
        " ",
        node_view.extract(args[7]), // from y
        " ",
        node_view.extract(args[8]), // from z
        " ",
        node_view.extract(args[9]), // mode
    ]);

    if args.len() == 11 {
        node_view.extend([" ", node_view.extract(args[10])]);
    }

    Ok(())
}

fn translate_damage(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("damage ");

    let (args, _, selector) = node_view.as_selector()?;
    let current_arg = translate_entity(node_view, args, selector, 0)? as usize;

    node_view.extend([
        " ",
        node_view.extract(args[current_arg]), // count
        " generic kill",
    ]);

    Ok(())
}

fn translate_data(node_view: &mut NodeView) -> Result<()> {
    let (args, command, selector, list) = node_view.as_selector_list()?;
    let mode = if command == Command::DataGet {
        "get"
    } else {
        "modify"
    };

    node_view.extend(["data ", mode, " entity "]);
    let current_arg = translate_entity(node_view, args, selector, 0)? as usize;
    let mut data_field = node_view.extract(args[current_arg]);

    if data_field == "loot_table" {
        data_field = "DeathLootTable";
    }

    node_view.extend([" ", data_field]);

    if command == Command::DataGet {
        return Ok(());
    }

    node_view.extend([" ", node_view.extract(args[current_arg + 1]), " value "]); // modify mode

    if list.is_empty() {
        node_view.extend(["\"", node_view.extract(args[current_arg + 2]), "\""]);
    } else {
        aux::translate_numeric_list(node_view, list, "f");
    }

    Ok(())
}

fn translate_effect(node_view: &mut NodeView) -> Result<()> {
    unimplemented!();
}
