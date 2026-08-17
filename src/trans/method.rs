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
) -> Result<usize> {
    if selector.kind == TokenKind::Id {
        node_view.push_str(node_view.extract(args[entity_pos as usize]));
        entity_pos += 1;
    } else {
        selector::translate_selector(node_view, selector)?;
    }

    Ok(entity_pos as usize)
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

fn choose_translate(node_view: &mut NodeView) -> Result<()> {
    match node_view.command() {
        Command::Advancement => translate_advancement(node_view),
        Command::Attribute => translate_attribute(node_view),
        Command::BossbarAdd => translate_bossbar_add(node_view),
        Command::BossbarSet => translate_bossbar_set(node_view),
        Command::BossbarRemove => translate_bossbar_remove(node_view),
        Command::Clear => translate_clear(node_view),
        Command::Clone => translate_clone(node_view),
        Command::Damage => translate_damage(node_view),
        Command::DataGet | Command::DataModify => translate_data(node_view),
        Command::Effect => translate_effect(node_view),
        Command::Ex => translate_ex(node_view),
        Command::Fill => translate_fill(node_view),
        //Command::Fn => translate_fn(node_view),
        Command::Gm => translate_gm(node_view),
        Command::Gamerule => translate_gamerule(node_view),
        Command::Give => translate_give(node_view),
        Command::Kill => translate_kill(node_view),
        Command::Native => translate_native(node_view),
        Command::Ptc => translate_ptc(node_view),
        Command::Pls => translate_pls(node_view),
        Command::Say => translate_say(node_view),
        Command::ScbObjAdd => translate_scb_obj_add(node_view),
        Command::ScbObjSet => translate_scb_obj_set(node_view),
        Command::ScbPlayers => translate_scb_players(node_view),
        Command::Setblock => translate_setblock(node_view),
        Command::Spawnpoint => translate_spawnpoint(node_view),
        Command::Spectate => translate_spectate(node_view),
        Command::Stopsound => translate_stopsound(node_view),
        Command::Sm => translate_sm(node_view),
        Command::Tag => translate_tag(node_view),
        Command::TeamAdd => translate_team_add(node_view),
        Command::TeamJoin => translate_team_join(node_view),
        Command::TeamModify => translate_team_modify(node_view),
        Command::Tellraw => translate_tellraw(node_view),
        Command::Time => translate_time(node_view),
        Command::Title => translate_title(node_view),
        Command::Tp => translate_tp(node_view),

        _ => Err(Translation(
            "unknown command in choose_translate() (internal)".to_string(),
        )),
    }
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
    let (args, _, selector) = node_view.as_selector()?;
    let mode = node_view.extract(args[0]);

    node_view.extend(["effect ", mode, " "]);

    let current_arg = translate_entity(node_view, args, selector, 1)? as usize;

    node_view.extend([" ", node_view.extract(args[current_arg])]);

    if mode == "clear" {
        return Ok(());
    }

    node_view.extend([
        " ",
        node_view.extract(args[current_arg + 1]), // duration
        " ",
        node_view.extract(args[current_arg + 2]), // amplifier
        " true",
    ]);

    Ok(())
}

fn translate_ex_align(node_view: &mut NodeView) -> Result<()> {
    let (args, _) = node_view.as_base()?;

    node_view.extend(["align ", node_view.extract(args[0])]);
    Ok(())
}

fn translate_ex_anchored(node_view: &mut NodeView) -> Result<()> {
    let (args, _) = node_view.as_base()?;

    node_view.extend(["anchored ", node_view.extract(args[0])]);
    Ok(())
}

fn translate_ex(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("execute");
    let (args, _, subnodes, run_node) = node_view.as_ex()?;

    for subnode in subnodes {
        node_view.push(' ');

        let mut subnode_view = NodeView::new(node_view.result, subnode, node_view.source_code);

        match subnode_view.command() {
            Command::ExAlign => translate_ex_align(&mut subnode_view)?,
            Command::ExAnchored => translate_ex_anchored(&mut subnode_view)?,
            Command::ExAs => translate_ex_as(&mut subnode_view)?,
            Command::ExAt => translate_ex_at(&mut subnode_view)?,
            Command::ExBlock => translate_ex_block(&mut subnode_view)?,
            Command::ExEnt => translate_ex_ent(&mut subnode_view)?,
            Command::ExFacing => translate_ex_facing(&mut subnode_view)?,
            Command::ExItemsBlock => translate_ex_items_block(&mut subnode_view)?,
            Command::ExItemsEnt => translate_ex_items_ent(&mut subnode_view)?,
            Command::ExScore => translate_ex_score(&mut subnode_view)?,
            Command::ExPos => translate_ex_pos(&mut subnode_view)?,
            Command::ExStoreBossbar => translate_ex_store_bossbar(&mut subnode_view)?,
            Command::ExStoreEnt => translate_ex_store_ent(&mut subnode_view)?,
            Command::ExStoreStorage => translate_ex_store_storage(&mut subnode_view)?,
            Command::ExStoreScore => translate_ex_store_score(&mut subnode_view)?,
            Command::ExUninited => translate_ex_uninited(&mut subnode_view)?,

            _ => {
                return Err(Translation(
                    "unknown execute subcommand in translate_ex() (internal)".to_string(),
                ));
            }
        }
    }

    node_view.push_str(" run ");

    let mut run_view = NodeView::new(node_view.result, run_node, node_view.source_code);

    choose_translate(&mut run_view)
}

fn translate_fill(node_view: &mut NodeView) -> Result<()> {
    let (args, _, id_with_data) = node_view.as_id_with_data()?;

    node_view.extend([
        "fill",
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
        node_view.extract(id_with_data.id), // block
    ]);

    if !id_with_data.data.is_empty() {
        translate_block_data(node_view, &id_with_data.data)?;
    }

    node_view.extend([" ", node_view.extract(args[6])]); // mode

    if args.len() == 8 {
        node_view.extend([" ", node_view.extract(args[7])]); // block to be replaced
    }

    Ok(())
}

//fn translate_fn(node_view: &mut NodeView) -> Result<()> {
//    node_view.push_str("function ");

//}

fn translate_gm(node_view: &mut NodeView) -> Result<()> {
    let (args, _, selector) = node_view.as_selector()?;

    node_view.extend([
        "gamemode",
        node_view.extract(args[0]), // mode
        " ",
    ]);

    translate_entity(node_view, args, selector, 1)?;

    Ok(())
}

fn translate_gamerule(node_view: &mut NodeView) -> Result<()> {
    let (args, _) = node_view.as_base()?;

    let mut rule = node_view.extract(args[0]);

    if rule == "natural_regeneration" {
        rule = "natural_health_regeneration";
    }

    node_view.extend([
        "gamerule ",
        rule,
        " ",
        node_view.extract(args[1]), // value
    ]);

    Ok(())
}

fn translate_give(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("give ");

    let (args, _, selector, id_with_data) = node_view.as_selector_id_with_data()?;
    let current_arg = translate_entity(node_view, args, selector, 0)? as usize;

    node_view.extend([" ", node_view.extract(id_with_data.id)]);

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

fn translate_kill(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("kill ");

    let (args, _, selector) = node_view.as_selector()?;

    translate_entity(node_view, args, selector, 0)?;

    Ok(())
}

fn translate_native(node_view: &mut NodeView) -> Result<()> {
    let (args, _) = node_view.as_base()?;
    let native_command = BaseToken {
        start: args[0].start + 1,
        end: args[0].end - 1,
    };

    node_view.push_str(node_view.extract(native_command));

    Ok(())
}

fn translate_ptc(node_view: &mut NodeView) -> Result<()> {
    let (args, _, id_with_data) = node_view.as_id_with_data()?;

    node_view.extend(["particle ", node_view.extract(id_with_data.id)]);

    if !id_with_data.data.is_empty() {
        node_view.push('{');
        data::translate_particle_data(node_view, &id_with_data.data)?;
        node_view.push('}');
    }

    node_view.extend([
        " ",
        node_view.extract(args[0]), // x
        " ",
        node_view.extract(args[1]), // y
        " ",
        node_view.extract(args[2]), // z
        " ",
        node_view.extract(args[3]), // dx
        " ",
        node_view.extract(args[4]), // dy
        " ",
        node_view.extract(args[5]), // dz
        " ",
        node_view.extract(args[6]), // speed
        " ",
        node_view.extract(args[7]), // count
        " ",
        node_view.extract(args[8]), // mode
    ]);

    Ok(())
}

fn translate_pls(node_view: &mut NodeView) -> Result<()> {
    let (args, _, selector) = node_view.as_selector()?;

    node_view.extend([
        "playsound ",
        node_view.extract(args[0]), // sound id
        " neutral",
    ]);

    let current_arg = translate_entity(node_view, args, selector, 1)? as usize;

    node_view.extend([
        " ",
        node_view.extract(args[current_arg]), // x
        " ",
        node_view.extract(args[current_arg + 1]), // y
        " ",
        node_view.extract(args[current_arg + 2]), // z
        " ",
        node_view.extract(args[current_arg + 3]), // volume
        " ",
        node_view.extract(args[current_arg + 4]), // pitch
    ]);

    Ok(())
}

fn translate_say(node_view: &mut NodeView) -> Result<()> {
    let (args, _) = node_view.as_base()?;

    node_view.extend(["say ", node_view.extract(args[0])]);

    Ok(())
}

fn translate_scb_obj_add(node_view: &mut NodeView) -> Result<()> {
    let (args, _, text) = node_view.as_text()?;

    node_view.extend([
        "scoreboard objectives add ",
        node_view.extract(args[0]), // objective
        " ",
        node_view.extract(args[1]), // objective kind
    ]);

    if text.is_empty() {
        return Ok(());
    }

    node_view.push(' ');
    text::translate_text(node_view, text); // objective name

    Ok(())
}

fn translate_scb_obj_set(node_view: &mut NodeView) -> Result<()> {
    let (args, _) = node_view.as_base()?;

    node_view.extend([
        "scoreboard objectives setdisplay ",
        node_view.extract(args[0]),
        " ",
        node_view.extract(args[1]),
    ]);

    Ok(())
}

fn translate_scb_players(node_view: &mut NodeView) -> Result<()> {
    let (args, _, selector) = node_view.as_selector()?;
    let mut mode = node_view.extract(args[0]);

    if mode == "opr" {
        mode = "operation";
    }

    node_view.extend(["scoreboard players ", mode, " "]);

    let current_arg = translate_entity(node_view, args, selector, 1)?;

    node_view.extend([
        " ",
        node_view.extract(args[current_arg]), // objective
    ]);

    if matches!(mode, "reset" | "get") {
        return Ok(());
    }

    node_view.extend([" ", node_view.extract(args[current_arg + 1])]);

    if mode == "operation" {
        node_view.extend([
            " ",
            node_view.extract(args[current_arg + 2]), // second entity
            node_view.extract(args[current_arg]),     // same objective
        ]);
    }

    Ok(())
}

fn translate_setblock(node_view: &mut NodeView) -> Result<()> {
    let (args, _, id_with_data) = node_view.as_id_with_data()?;

    node_view.extend([
        "setblock ",
        node_view.extract(args[0]), // x
        " ",
        node_view.extract(args[1]), // y
        " ",
        node_view.extract(args[2]), // z
        " ",
        node_view.extract(id_with_data.id), // block
    ]);

    if !id_with_data.data.is_empty() {
        translate_block_data(node_view, &id_with_data.data)?;
    }

    node_view.extend([" ", node_view.extract(args[3])]);

    Ok(())
}

fn translate_spawnpoint(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("spawnpoint ");

    let (args, _, selector) = node_view.as_selector()?;
    let current_arg = translate_entity(node_view, args, selector, 0)?;

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

fn translate_spectate(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("spectate ");

    let (args, _, selector) = node_view.as_selector()?;
    translate_entity(node_view, args, selector, 0);

    Ok(())
}

fn translate_stopsound(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("stopsound ");

    let (args, _, selector) = node_view.as_selector()?;
    let current_arg = translate_entity(node_view, args, selector, 0)?;

    node_view.extend([" * ", node_view.extract(args[current_arg])]); // sound id
    Ok(())
}

fn translate_sm(node_view: &mut NodeView) -> Result<()> {
    let (args, _, id_with_data) = node_view.as_id_with_data()?;

    node_view.extend([
        "summon ",
        node_view.extract(id_with_data.id), // entity name
        " ",
        node_view.extract(args[0]), // x
        " ",
        node_view.extract(args[1]), // y
        " ",
        node_view.extract(args[2]), // z
    ]);

    if !id_with_data.data.is_empty() {
        node_view.push_str(" {");
        data::translate_entity_data(node_view, &id_with_data.data)?;
        node_view.push('}');
    }

    Ok(())
}

fn translate_tag(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("tag ");

    let (args, _, selector) = node_view.as_selector()?;
    let current_arg = translate_entity(node_view, args, selector, 0)?;

    node_view.extend([
        " ",
        node_view.extract(args[current_arg]),
        " ",
        node_view.extract(args[current_arg + 1]),
    ]);

    Ok(())
}

fn translate_team_add(node_view: &mut NodeView) -> Result<()> {
    let (args, _) = node_view.as_base()?;
    node_view.extend(["team add ", node_view.extract(args[0])]);

    Ok(())
}

fn translate_team_join(node_view: &mut NodeView) -> Result<()> {
    let (args, _, selector) = node_view.as_selector()?;

    node_view.extend(["team join ", node_view.extract(args[0]), " "]);
    translate_entity(node_view, args, selector, 1)?;

    Ok(())
}

fn translate_team_modify(node_view: &mut NodeView) -> Result<()> {
    let (args, _) = node_view.as_base()?;
    let mut rule = node_view.extract(args[1]);

    if rule == "friendly_fire" {
        rule = "friendlyFire";
    } else if rule == "collision" {
        rule = "collisionRule";
    }

    node_view.extend([
        "team modify ",
        node_view.extract(args[0]),
        " ",
        rule,
        " ",
        node_view.extract(args[2]),
    ]);

    Ok(())
}

fn translate_tellraw(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("tellraw ");

    let (args, _, selector, text) = node_view.as_selector_text()?;

    translate_entity(node_view, args, selector, 0)?;

    node_view.push(' ');

    text::translate_text(node_view, text);

    Ok(())
}

fn translate_time(node_view: &mut NodeView) -> Result<()> {
    let (args, _) = node_view.as_base()?;

    node_view.extend([
        "time ",
        node_view.extract(args[0]), // mode
        " ",
        node_view.extract(args[1]), // value
    ]);

    Ok(())
}

fn translate_title(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("title ");

    let (args, _, selector, text) = node_view.as_selector_text()?;
    let current_arg = translate_entity(node_view, args, selector, 0)?;

    node_view.extend([" ", node_view.extract(args[current_arg]), " "]);

    text::translate_text(node_view, text);

    Ok(())
}

fn translate_tp(node_view: &mut NodeView) -> Result<()> {
    node_view.push_str("tp ");

    let (args, _, selector1, selector2) = node_view.as_double_selector()?;

    match args.len() {
        0 => {
            // one or two selectors
            selector::translate_selector(node_view, selector1)?;

            if selector2.kind != TokenKind::Id {
                node_view.push(' ');
                selector::translate_selector(node_view, selector2)?;
            }
        }
        1 => {
            // exactly one entity name
            if selector1.kind == TokenKind::Id {
                node_view.push_str(node_view.extract(args[0]));

                if selector2.kind != TokenKind::Id {
                    node_view.push(' ');
                    selector::translate_selector(node_view, selector2)?;
                }
            } else {
                selector::translate_selector(node_view, selector1)?;
                node_view.extend([" ", node_view.extract(args[0])]);
            }
        }
        2 => {
            // both arguments are entity names
            node_view.extend([node_view.extract(args[0]), " ", node_view.extract(args[1])]);
        }
        3 => {
            // only coordinates
            if selector1.kind != TokenKind::Id {
                selector::translate_selector(node_view, selector1)?;
                node_view.push(' ');
            }

            node_view.extend([
                node_view.extract(args[0]),
                " ",
                node_view.extract(args[1]),
                " ",
                node_view.extract(args[2]),
            ]);
        }
        4 => {
            // entity name and coordinates
            node_view.extend([
                node_view.extract(args[0]),
                " ",
                node_view.extract(args[1]),
                " ",
                node_view.extract(args[2]),
                " ",
                node_view.extract(args[3]),
            ])
        }
        5 => {
            // only coordinates
            if selector1.kind != TokenKind::Id {
                selector::translate_selector(node_view, selector1)?;
                node_view.push(' ');
            }

            node_view.extend([
                node_view.extract(args[0]),
                " ",
                node_view.extract(args[1]),
                " ",
                node_view.extract(args[2]),
                " ",
                node_view.extract(args[3]),
                " ",
                node_view.extract(args[4]),
            ]);
        }
        6 => {
            // one entity name and 5 coordinates
            node_view.extend([
                node_view.extract(args[0]),
                " ",
                node_view.extract(args[1]),
                " ",
                node_view.extract(args[2]),
                " ",
                node_view.extract(args[3]),
                " ",
                node_view.extract(args[4]),
                " ",
                node_view.extract(args[5]),
            ]);
        }

        _ => {
            return Err(Translation(
                "impossible case reached in translate_tp() (internal)".to_string(),
            ));
        }
    }

    Ok(())
}
