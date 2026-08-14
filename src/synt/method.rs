use std::str;

use sorted_code::sorted_match;

use super::{
    aux::{self, List, State},
    data,
    node::{Command, DoubleSelectorNode, Node, SelectorListNode, SelectorTextNode},
    selector::{self, Selector},
    text::{self, Text},
};

use crate::{
    NeniyError::Syntax,
    Result,
    lexic::token::{BaseToken, Index, Token, TokenCategory, TokenKind},
};

macro_rules! make_check_kind {
    ($types:pat) => {
        (|token: Token| matches!(token.kind, $types)) as fn(Token) -> bool
    };
}

pub fn choose_parse(state: &mut State) -> Result<Node> {
    sorted_match! { match state[0].kind {
        TokenKind::Advancement => advancement_parse(state),
        TokenKind::Attribute => attribute_parse(state),
        TokenKind::Bossbar => bossbar_parse(state),
        TokenKind::Clear => clear_parse(state),
        TokenKind::Clone => clone_parse(state),
        TokenKind::Damage => damage_parse(state),
        TokenKind::Data => data_parse(state),
        TokenKind::Effect => effect_parse(state),
        TokenKind::Ex => ex_parse(state),
        TokenKind::Fill => fill_parse(state),
        TokenKind::Fn => fn_parse(state),
        TokenKind::Gamerule => gamerule_parse(state),
        TokenKind::Give => give_parse(state),
        TokenKind::Gm => gm_parse(state),
        TokenKind::Kill => kill_parse(state),
        TokenKind::Native => native_parse(state),
        TokenKind::Pls => pls_parse(state),
        TokenKind::Ptc => ptc_parse(state),
        TokenKind::Say => say_parse(state),
        TokenKind::Scb => scb_parse(state),
        TokenKind::Setblock => setblock_parse(state),
        TokenKind::Sm => sm_parse(state),
        TokenKind::Spawnpoint => spawnpoint_parse(state),
        TokenKind::Spectate => spectate_parse(state),
        TokenKind::Stopsound => stopsound_parse(state),
        TokenKind::Tag => tag_parse(state),
        TokenKind::Team => team_parse(state),
        TokenKind::Tellraw => tellraw_parse(state),
        TokenKind::Time => time_parse(state),
        TokenKind::Title => title_parse(state),
        TokenKind::Tp => tp_parse(state),

        _ => Err(Syntax(["unknown command ", state.extract(0)].concat())),
    }}
}

pub fn advancement_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "advancement";
    let mut args = Vec::new();

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    aux::check_token(state, 1, "name", NAME, aux::valid_id)?;
    args.push(state[0].base);

    Ok(Node::Selector {
        args,
        command: Command::Advancement,
        selector,
    })
}

pub fn attribute_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "attribute";
    let mut args = Vec::new();

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    aux::check_token(state, 1, "name", NAME, aux::valid_id)?;
    args.push(state[0].base);

    aux::check_token(state, 1, "value", NAME, aux::valid_numeric)?;
    args.push(state[0].base);

    Ok(Node::Selector {
        args,
        command: Command::Attribute,
        selector,
    })
}

pub fn bossbar_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "bossbar";

    aux::check_presence(state, 1, "mode", NAME)?;

    match state[0].kind {
        TokenKind::Add => bossbar_add_parse(state),
        TokenKind::Set => bossbar_set_parse(state),
        TokenKind::Remove => bossbar_remove_parse(state),

        _ => Err(Syntax(
            ["invalid mode ", state.extract(0), " for bossbar"].concat(),
        )),
    }
}

pub fn clear_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "clear";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "player", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    aux::check_token(state, 1, "item", NAME, aux::valid_id)?;
    let id_with_data_ptr = data::parse_id_with_data(state)?;

    if !state.exceed(1) && state[1].kind == TokenKind::Numeric {
        *state += 1;
        args.push(state[0].base);
    }

    Ok(Node::SelectorIdWithDataPtr(Box::new(
        crate::synt::node::SelectorIdWithDataPtrNode {
            args,
            command: Command::Clear,
            selector,
            id_with_data_ptr,
        },
    )))
}

pub fn clone_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "clone";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "first coordinate", NAME)?;
    capture_coords(state, &mut args, 9, NAME)?;

    aux::check_token(
        state,
        1,
        "mode",
        NAME,
        make_check_kind!(TokenKind::Replace | TokenKind::Masked),
    )?;
    args.push(state[0].base);

    if !state.exceed(1) && state[1].kind == TokenKind::Move {
        *state += 1;
        args.push(state[0].base);
    }

    Ok(Node::Base {
        args,
        command: Command::Clone,
    })
}

pub fn damage_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "damage";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    aux::check_token(state, 1, "damage amount", NAME, aux::valid_numeric)?;
    args.push(state[0].base);

    Ok(Node::Selector {
        args,
        command: Command::Damage,
        selector,
    })
}

pub fn data_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "data";

    aux::check_presence(state, 1, "mode", NAME)?;

    match state[0].kind {
        TokenKind::Get => data_get_parse(state),
        TokenKind::Modify => data_modify_parse(state),

        _ => Err(Syntax(
            ["invalid mode ", state.extract(0), " for ", NAME].concat(),
        )),
    }
}

pub fn effect_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "effect";

    aux::check_presence(state, 1, "mode", NAME)?;

    match state[0].kind {
        TokenKind::Give => effect_give_parse(state),
        TokenKind::Clear => effect_clear_parse(state),

        _ => Err(Syntax(
            ["invalid mode ", state.extract(0), " for ", NAME].concat(),
        )),
    }
}

pub fn ex_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "execute";

    let mut subnodes = Vec::new();

    while !state.exceed(0) {
        aux::check_presence(state, 1, "mode", NAME)?;

        match state[0].kind {
            TokenKind::Align => subnodes.push(ex_align_parse(state)?),
            TokenKind::Anchored => subnodes.push(ex_anchored_parse(state)?),
            TokenKind::As => subnodes.push(ex_ent_parse(state, Command::ExAs, false)?),
            TokenKind::At => subnodes.push(ex_ent_parse(state, Command::ExAt, false)?),
            TokenKind::Facing => subnodes.push(ex_facing_parse(state)?),
            TokenKind::If => subnodes.push(ex_condition_parse(state)?),
            TokenKind::Pos => subnodes.push(ex_pos_parse(state)?),
            TokenKind::Run => {
                return Ok(Node::Execute {
                    args: Vec::new(),
                    command: Command::Ex,
                    subnodes,
                    run_node: Box::new(ex_run_parse(state)?),
                });
            }
            TokenKind::Store => subnodes.push(ex_store_parse(state)?),
            TokenKind::Uninited => subnodes.push(ex_uninited_parse(state)?),
            TokenKind::Unless => subnodes.push(ex_condition_parse(state)?),

            _ => {
                return Err(Syntax(
                    ["invalid ", NAME, " mode", state.extract(0)].concat(),
                ));
            }
        }
    }

    Err(Syntax(["run command not found for ", NAME].concat()))
}

pub fn fill_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "fill";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "first coordinate", NAME)?;
    capture_coords(state, &mut args, 6, "fill")?;

    aux::check_token(state, 1, "block", NAME, aux::valid_id)?;
    let id_with_data_ptr = data::parse_id_with_data(state)?;

    aux::check_token(
        state,
        1,
        "mode",
        NAME,
        make_check_kind!(TokenKind::Keep | TokenKind::Replace),
    )?;
    args.push(state[0].base);

    if !state.exceed(1) && state[1].kind == TokenKind::Id {
        *state += 1;
        args.push(state[0].base);
    }

    Ok(Node::IdWithDataPtr {
        args,
        command: Command::Fill,
        id_with_data_ptr,
    })
}

pub fn fn_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "fn";

    aux::check_token(state, 1, "name", NAME, aux::valid_id)?;
    let mut args = vec![state[0].base];

    if !state.exceed(1) && state[1].kind == TokenKind::Id {
        *state += 1;
        args.push(state[0].base);
    }

    Ok(Node::Base {
        args,
        command: Command::Fn,
    })
}

pub fn gm_parse(state: &mut State) -> Result<Node> {
    use TokenKind::*;

    const NAME: &str = "gm";

    aux::check_token(
        state,
        1,
        "mode",
        NAME,
        make_check_kind!(Adventure | Creative | Spectator | Survival),
    )?;
    let mut args = vec![state[0].base];

    aux::check_presence(state, 1, "player", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    Ok(Node::Selector {
        args,
        command: Command::Gm,
        selector,
    })
}

pub fn gamerule_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "gamerule";

    aux::check_token(state, 1, "rule", NAME, aux::valid_id)?;
    aux::check_token(state, 1, "value", NAME, aux::valid_value)?;

    let args = vec![state[-1].base, state[0].base];

    Ok(Node::Base {
        args,
        command: Command::Gamerule,
    })
}

pub fn give_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "give";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    aux::check_token(state, 1, "item name", NAME, aux::valid_id)?;
    let id_with_data_ptr = data::parse_id_with_data(state)?;

    if !state.exceed(1) && state[1].kind == TokenKind::Numeric {
        *state += 1;
        args.push(state[0].base);
    }

    Ok(Node::SelectorIdWithDataPtr(Box::new(
        crate::synt::node::SelectorIdWithDataPtrNode {
            args,
            command: Command::Give,
            selector,
            id_with_data_ptr,
        },
    )))
}

pub fn kill_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "kill";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    Ok(Node::Selector {
        args,
        command: Command::Kill,
        selector,
    })
}

pub fn native_parse(state: &mut State) -> Result<Node> {
    simple_command(state, "native", aux::valid_string, Command::Native)
}

pub fn ptc_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "particle";

    aux::check_token(state, 1, "particle name", NAME, aux::valid_id)?;
    let id_with_data_ptr = data::parse_id_with_data(state)?;

    let mut args = Vec::new();

    aux::check_presence(state, 1, "first coordinate", NAME)?;
    capture_coords(state, &mut args, 8, NAME)?;

    aux::check_token(
        state,
        1,
        "mode",
        NAME,
        make_check_kind!(TokenKind::Normal | TokenKind::Force),
    )?;
    args.push(state[0].base);

    Ok(Node::IdWithDataPtr {
        args,
        command: Command::Ptc,
        id_with_data_ptr,
    })
}

pub fn pls_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "pls";

    aux::check_token(state, 1, "sound name", NAME, aux::valid_id)?;
    let mut args = vec![state[0].base];

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    aux::check_presence(state, 1, "first coordinate", NAME)?;
    capture_coords(state, &mut args, 3, NAME)?;

    aux::check_token(state, 1, "volume", NAME, aux::valid_numeric)?;
    args.push(state[0].base);

    aux::check_token(state, 1, "pitch", NAME, aux::valid_numeric)?;
    args.push(state[0].base);

    Ok(Node::Selector {
        args,
        command: Command::Pls,
        selector,
    })
}

pub fn say_parse(state: &mut State) -> Result<Node> {
    simple_command(state, "say", aux::valid_value, Command::Say)
}

pub fn scb_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "scb";

    aux::check_presence(state, 1, "mode", NAME)?;

    match state[0].kind {
        TokenKind::Players => scb_players_parse(state),
        TokenKind::Obj => scb_objectives_parse(state),

        _ => Err(Syntax(["invalid ", NAME, " mode"].concat())),
    }
}

pub fn setblock_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "setblock";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "first coordinate", NAME)?;
    capture_coords(state, &mut args, 3, NAME)?;

    aux::check_token(state, 1, "block", NAME, aux::valid_id)?;
    let id_with_data_ptr = data::parse_id_with_data(state)?;

    aux::check_token(
        state,
        1,
        "mode",
        NAME,
        make_check_kind!(TokenKind::Destroy | TokenKind::Keep | TokenKind::Replace),
    )?;
    args.push(state[0].base);

    Ok(Node::IdWithDataPtr {
        args,
        command: Command::Setblock,
        id_with_data_ptr,
    })
}

pub fn spawnpoint_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "spawnpoint";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    aux::check_presence(state, 1, "first coordinate", NAME)?;
    capture_coords(state, &mut args, 3, NAME)?;

    Ok(Node::Selector {
        args,
        command: Command::Spawnpoint,
        selector,
    })
}

pub fn spectate_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "spectate";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    Ok(Node::Selector {
        args,
        command: Command::Spectate,
        selector,
    })
}

pub fn stopsound_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "stopsound";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    aux::check_token(state, 1, "sound name", NAME, aux::valid_id)?;
    args.push(state[0].base);

    Ok(Node::Selector {
        args,
        command: Command::Stopsound,
        selector,
    })
}

pub fn sm_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "sm";

    aux::check_token(state, 1, "entity", NAME, aux::valid_id)?;
    let id_with_data_ptr = data::parse_id_with_data(state)?;

    let mut args = Vec::new();

    aux::check_presence(state, 1, "first coordinate", NAME)?;
    capture_coords(state, &mut args, 3, NAME)?;

    Ok(Node::IdWithDataPtr {
        args,
        command: Command::Sm,
        id_with_data_ptr,
    })
}

pub fn tag_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "tag";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    aux::check_token(
        state,
        1,
        "mode",
        NAME,
        make_check_kind!(TokenKind::Add | TokenKind::Remove),
    )?;
    args.push(state[0].base);

    aux::check_token(state, 1, "tag name", NAME, aux::valid_value)?;
    args.push(state[0].base);

    Ok(Node::Selector {
        args,
        command: Command::Tag,
        selector,
    })
}

pub fn team_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "team";

    aux::check_presence(state, 1, "mode", NAME)?;

    match state[0].kind {
        TokenKind::Add => team_add_parse(state),
        TokenKind::Join => team_join_parse(state),
        TokenKind::Modify => team_modify_parse(state),

        _ => Err(Syntax(
            ["invalid mode ", state.extract(0), " for ", NAME].concat(),
        )),
    }
}

pub fn tellraw_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "tellraw";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, true)?;

    aux::check_token(state, 1, "text", NAME, aux::valid_text)?;
    let text = text::parse_text(state)?;

    Ok(Node::SelectorText(Box::new(SelectorTextNode {
        args,
        command: Command::Tellraw,
        selector,
        text,
    })))
}

pub fn time_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "time";

    aux::check_token(state, 1, "mode", NAME, aux::valid_id)?;
    aux::check_token(state, 1, "value", NAME, aux::valid_value)?;

    Ok(Node::Base {
        args: vec![state[-1].base, state[0].base],
        command: Command::Time,
    })
}

pub fn title_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "title";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    aux::check_token(
        state,
        1,
        "mode",
        NAME,
        make_check_kind!(TokenKind::Subtitle | TokenKind::Title),
    )?;
    args.push(state[0].base);

    aux::check_token(state, 1, "text", NAME, aux::valid_text)?;
    let text = text::parse_text(state)?;

    Ok(Node::SelectorText(Box::new(SelectorTextNode {
        args,
        command: Command::Title,
        selector,
        text,
    })))
}

pub fn tp_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "tp";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "first entity or first coordinate", NAME)?;
    let selector1 = capture_entity_or_coords(state, &mut args, NAME)?;

    let selector2 = if args.len() < 3 // entity captured
        && !state.exceed(1)
        && (state[1].category == TokenCategory::Selector
            || state[1].kind == TokenKind::Id
            || aux::valid_coordinate(state[1]))
    {
        aux::check_presence(state, 1, "second entity or first coordinate", NAME)?;
        capture_entity_or_coords(state, &mut args, NAME)?
    } else {
        Selector::new_empty()
    };

    Ok(Node::DoubleSelector(Box::new(DoubleSelectorNode {
        args,
        command: Command::Tp,
        selector1,
        selector2,
    })))
}

fn simple_command(
    state: &mut State,
    name: &str,
    valid_token: fn(Token) -> bool,
    command: Command,
) -> Result<Node> {
    aux::check_token(state, 1, "argument", name, valid_token)?;

    Ok(Node::Base {
        args: vec![state[0].base],
        command,
    })
}

fn capture_coordinate(state: &mut State) -> Result<BaseToken> {
    if aux::valid_numeric(state[0]) {
        return Ok(state[0].base);
    }

    if matches!(state[0].kind, TokenKind::Tilda | TokenKind::Caret) {
        if state.exceed(1) || !aux::valid_numeric(state[1]) || !aux::consecutive(state[0], state[1])
        {
            return Ok(state[1].base);
        }

        *state += 1;

        return Ok(BaseToken {
            start: state[-1].base.start,
            end: state[0].base.end,
        });
    }

    Err(Syntax(["invalid coordinate ", state.extract(0)].concat()))
}

fn capture_entity(
    state: &mut State,
    args: &mut Vec<BaseToken>,
    name: &str,
    look_ahead: bool,
) -> Result<Selector> {
    if aux::valid_id(state[0]) {
        args.push(state[0].base);
        Ok(Selector::new_empty())
    } else if state[0].category == TokenCategory::Selector {
        selector::parse_selector(state, look_ahead)
    } else {
        Err(Syntax(
            ["invalid entity ", state.extract(0), " for ", name].concat(),
        ))
    }
}

fn ex_align_parse(state: &mut State) -> Result<Node> {
    aux::check_token(state, 1, "argument", "ex align", aux::valid_id)?;

    Ok(Node::Base {
        args: vec![state[0].base],
        command: Command::ExAlign,
    })
}

fn ex_anchored_parse(state: &mut State) -> Result<Node> {
    aux::check_token(
        state,
        1,
        "mode",
        "ex anchored",
        make_check_kind!(TokenKind::Eyes | TokenKind::Feet),
    )?;

    Ok(Node::Base {
        args: vec![state[0].base],
        command: Command::ExAnchored,
    })
}

// state[0] on last token
fn ex_ent_parse(state: &mut State, command: Command, is_if: bool) -> Result<Node> {
    let mut args = Vec::new();

    let name = match command {
        Command::ExAs => "ex as",
        Command::ExAt => "ex at",
        Command::ExEnt => {
            args.push(state[-1].base);

            if is_if { "ex if ent" } else { "ex unless ent" }
        }

        _ => {
            return Err(Syntax(
                "unknown token type in ex_ent_parse() (internal)".to_string(),
            ));
        }
    };

    aux::check_presence(state, 1, "entity", name)?;
    let selector = capture_entity(state, &mut args, name, false)?;

    Ok(Node::Selector {
        args,
        command,
        selector,
    })
}

// state[0] on first coordinate
fn capture_coords(
    state: &mut State,
    args: &mut Vec<BaseToken>,
    count: Index,
    name: &str,
) -> Result<()> {
    args.push(capture_coordinate(state)?);

    for _ in 0..count - 1 {
        aux::check_presence(state, 1, "coordinate", name)?;

        args.push(capture_coordinate(state)?);
    }

    Ok(())
}

fn ex_facing_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "ex facing";

    aux::check_presence(state, 1, "first coordinate or entity", NAME)?;

    let mut args = Vec::new();
    let selector = if aux::valid_coordinate(state[0]) {
        capture_coords(state, &mut args, 3, NAME)?;
        Selector::new_empty()
    } else {
        capture_entity(state, &mut args, NAME, false)?
    };

    Ok(Node::Selector {
        args,
        command: Command::ExFacing,
        selector,
    })
}

// state[0] on "block"
fn ex_block_parse(state: &mut State, is_if: bool) -> Result<Node> {
    let name = ["ex ", if is_if { "if" } else { "unless" }, " block"].concat();

    let mut args = vec![state[-1].base];

    aux::check_presence(state, 1, "first coordinate", &name)?;
    capture_coords(state, &mut args, 3, &name)?;

    aux::check_token(state, 1, "block", &name, aux::valid_id)?;

    Ok(Node::IdWithDataPtr {
        args,
        command: Command::ExBlock,
        id_with_data_ptr: data::parse_id_with_data(state)?,
    })
}

// state[0] on "score"
fn ex_score_parse(state: &mut State, is_if: bool) -> Result<Node> {
    let name = ["ex ", if is_if { "if" } else { "unless" }, " score"].concat();

    let mut args = vec![state[-1].base];

    aux::check_presence(state, 1, "entity", &name)?;
    let selector = capture_entity(state, &mut args, &name, false)?;

    aux::check_token(state, 1, "objective", &name, aux::valid_id)?;
    args.push(state[0].base);

    aux::check_presence(state, 1, "range or operator", &name)?;

    if aux::valid_range(state[0]) {
        args.push(aux::capture_range(state)?);
    } else if aux::valid_operator(state[0]) {
        args.push(state[0].base);

        aux::check_token(state, 1, "second entity", &name, aux::valid_id)?;
        args.push(state[0].base);
    } else {
        return Err(Syntax(
            [
                "invalid range or operator ",
                state.extract(0),
                " in ",
                &name,
            ]
            .concat(),
        ));
    }

    Ok(Node::Selector {
        args,
        command: Command::ExScore,
        selector,
    })
}

fn ex_pos_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "ex pos";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "first coordinate", NAME)?;
    capture_coords(state, &mut args, 3, NAME)?;

    Ok(Node::Base {
        args,
        command: Command::ExPos,
    })
}

// state[0] on "score"
fn ex_store_score_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "ex store score";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    aux::check_token(state, 1, "objective", NAME, aux::valid_id)?;
    args.push(state[0].base);

    Ok(Node::Selector {
        args,
        command: Command::ExStoreScore,
        selector,
    })
}

fn capture_data_field(state: &mut State) -> Result<BaseToken> {
    let mut result = state[0].base;

    while !state.exceed(1) && state[1].kind == TokenKind::OpeningSquareBrace {
        if state.exceed(3)
            || !aux::valid_numeric(state[2])
            || state[3].kind != TokenKind::ClosingSquareBrace
            || !aux::consecutive3(state[1], state[2], state[3])
        {
            return Err(Syntax(
                [
                    "invalid indexing ",
                    state.extract_segment(1, 3),
                    " in data-field",
                ]
                .concat(),
            ));
        }

        result.end = state[3].base.end;
        *state += 3;
    }

    Ok(result)
}

// state[0] on "entity"
fn ex_store_entity_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "ex store ent";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    aux::check_token(state, 1, "data-field", NAME, aux::valid_id)?;
    args.push(capture_data_field(state)?);

    aux::check_token(state, 1, "data type", NAME, aux::valid_id)?;
    args.push(capture_data_field(state)?);

    aux::check_token(state, 1, "multiplier", NAME, aux::valid_numeric)?;
    args.push(state[0].base);

    Ok(Node::Selector {
        args,
        command: Command::ExStoreEnt,
        selector,
    })
}

fn ex_store_storage_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "ex store storage";

    let mut args = Vec::new();

    aux::check_token(state, 1, "name", NAME, aux::valid_id)?;
    args.push(state[0].base);

    aux::check_token(state, 1, "variable", NAME, aux::valid_id)?;
    args.push(state[0].base);

    aux::check_token(state, 1, "data type", NAME, aux::valid_id)?;
    args.push(state[0].base);

    aux::check_token(state, 1, "multiplier", NAME, aux::valid_numeric)?;
    args.push(state[0].base);

    Ok(Node::Base {
        args,
        command: Command::ExStoreStorage,
    })
}

fn ex_store_bossbar_parse(state: &mut State) -> Result<Node> {
    simple_command(
        state,
        "ex store bossbar",
        aux::valid_value,
        Command::ExStoreBossbar,
    )
}

fn ex_store_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "ex store";

    aux::check_presence(state, 1, "mode", NAME)?;

    match state[0].kind {
        TokenKind::Bossbar => ex_store_bossbar_parse(state),
        TokenKind::Ent => ex_store_entity_parse(state),
        TokenKind::Score => ex_store_score_parse(state),
        TokenKind::Storage => ex_store_storage_parse(state),

        _ => Err(Syntax(
            ["invalid mode ", state.extract(0), " for ", NAME].concat(),
        )),
    }
}

fn ex_uninited_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "ex uninited";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    aux::check_token(state, 1, "objective", NAME, aux::valid_id)?;
    args.push(state[0].base);

    Ok(Node::Selector {
        args,
        command: Command::ExUninited,
        selector,
    })
}

// state[0] on "players"
fn scb_players_parse(state: &mut State) -> Result<Node> {
    use TokenKind::*;

    const NAME: &str = "scb players";

    aux::check_token(
        state,
        1,
        "mode",
        NAME,
        make_check_kind!(Set | Add | Get | Opr | Remove | Reset),
    )?;

    let mode = state[0].kind;
    let mut args = vec![state[0].base];

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    aux::check_token(state, 1, "objective", NAME, aux::valid_id)?;
    args.push(state[0].base);

    if mode == Opr {
        const NAME: &str = "scb players operation";

        aux::check_token(state, 1, "operator", NAME, aux::valid_operator)?;
        args.push(state[0].base);

        // second entity is never being selector
        aux::check_token(state, 1, "second entity", NAME, aux::valid_id)?;
        args.push(state[0].base);
    } else if !matches!(mode, Get | Reset) {
        aux::check_token(state, 1, "value", NAME, aux::valid_numeric)?;
        args.push(state[0].base);
    }

    Ok(Node::Selector {
        args,
        command: Command::ScbPlayers,
        selector,
    })
}

// state[0] on "add"
fn scb_obj_add_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "scb obj add";

    aux::check_token(state, 1, "objective", NAME, aux::valid_id)?;
    let mut args = vec![state[0].base];

    aux::check_token(state, 1, "objective type", NAME, aux::valid_id)?;
    args.push(state[0].base);

    let mut text = Text::new();

    if !state.exceed(1) && aux::valid_text(state[1]) {
        *state += 1;

        text = text::parse_text(state)?;
    }

    Ok(Node::Text {
        args,
        command: Command::ScbObjAdd,
        text,
    })
}

// state[0] on "set"
fn scb_obj_set_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "scb obj set";

    aux::check_token(state, 1, "action", NAME, aux::valid_id)?;
    let mut args = vec![state[0].base];

    aux::check_token(state, 1, "objective", NAME, aux::valid_id)?;
    args.push(state[0].base);

    Ok(Node::Base {
        args,
        command: Command::ScbObjSet,
    })
}

// state[0] on "objectives"
fn scb_objectives_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "scb objectives";

    aux::check_presence(state, 1, "mode", NAME)?;

    match state[0].kind {
        TokenKind::Add => scb_obj_add_parse(state),
        TokenKind::Set => scb_obj_set_parse(state),

        _ => Err(Syntax(
            ["invalid mode ", state.extract(0), " for ", NAME].concat(),
        )),
    }
}

fn ex_items_shared_parse(state: &mut State, args: &mut Vec<BaseToken>, name: &str) -> Result<()> {
    aux::check_token(state, 1, "container", name, aux::valid_id)?;
    args.push(state[0].base);

    aux::check_token(state, 1, "item name", name, aux::valid_id)?;
    args.push(state[0].base);

    Ok(())
}

fn ex_items_block_parse(state: &mut State, is_if: bool) -> Result<Node> {
    let name = ["ex", if is_if { "if" } else { "unless" }, " items block"].concat();

    let mut args = vec![state[-2].base];
    ex_items_shared_parse(state, &mut args, &name)?;

    Ok(Node::Base {
        args,
        command: Command::ExItemsBlock,
    })
}

fn ex_items_ent_parse(state: &mut State, is_if: bool) -> Result<Node> {
    let name = ["ex", if is_if { "if" } else { "unless" }, " items ent"].concat();

    let mut args = vec![state[-2].base];

    aux::check_presence(state, 1, "entity", &name)?;
    let selector = capture_entity(state, &mut args, &name, false)?;

    ex_items_shared_parse(state, &mut args, &name)?;

    Ok(Node::Selector {
        args,
        command: Command::ExItemsEnt,
        selector,
    })
}

fn ex_condition_parse(state: &mut State) -> Result<Node> {
    let is_if = state[0].kind == TokenKind::If;
    let mut name = ["ex ", if is_if { "if" } else { "unless" }].concat();

    aux::check_presence(state, 1, "mode", &name)?;

    match state[0].kind {
        TokenKind::Block => ex_block_parse(state, is_if),
        TokenKind::Ent => ex_ent_parse(state, Command::ExEnt, is_if),
        TokenKind::Items => {
            name.push_str(" items");

            aux::check_presence(state, 1, "mode", &name)?;

            match state[0].kind {
                TokenKind::Block => ex_items_block_parse(state, is_if),
                TokenKind::Ent => ex_items_ent_parse(state, is_if),

                _ => Err(Syntax(
                    ["unknown mode ", state.extract(0), " for ", &name].concat(),
                )),
            }
        }
        TokenKind::Score => ex_score_parse(state, is_if),

        _ => Err(Syntax(
            ["invalid mode ", state.extract(0), " for ", &name].concat(),
        )),
    }
}

// state[0] on data mode
fn data_shared_parse(state: &mut State, command: Command) -> Result<(Vec<BaseToken>, Selector)> {
    let name = [
        "data ",
        if command == Command::DataGet {
            "get"
        } else {
            "modify"
        },
    ]
    .concat();

    let mut args = Vec::new();

    aux::check_presence(state, 1, "entity", &name)?;
    let selector = capture_entity(state, &mut args, &name, false)?;

    aux::check_token(state, 1, "data-field", &name, aux::valid_id)?;
    args.push(capture_data_field(state)?);

    Ok((args, selector))
}

fn data_get_parse(state: &mut State) -> Result<Node> {
    let (args, selector) = data_shared_parse(state, Command::DataGet)?;

    Ok(Node::Selector {
        args,
        command: Command::DataGet,
        selector,
    })
}

fn data_modify_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "data modify";

    let (mut args, selector) = data_shared_parse(state, Command::DataModify)?;

    aux::check_token(
        state,
        1,
        "mode",
        NAME,
        make_check_kind!(TokenKind::Add | TokenKind::Set),
    )?;
    args.push(state[0].base);

    aux::check_presence(state, 1, "value", NAME)?;
    let mut list = List::new();

    if aux::valid_value(state[0]) {
        args.push(state[0].base);
    } else if state[0].kind == TokenKind::OpeningSquareBrace {
        list = aux::capture_list(state)?;
    } else {
        return Err(Syntax(
            ["invalid value ", state.extract(0), " in ", NAME].concat(),
        ));
    }

    Ok(Node::SelectorList(Box::new(SelectorListNode {
        args,
        command: Command::DataModify,
        selector,
        list,
    })))
}

fn effect_shared_parse(state: &mut State, name: &str) -> Result<(Vec<BaseToken>, Selector)> {
    let mut args = vec![state[0].base];

    aux::check_presence(state, 1, "entity", name)?;
    let selector = capture_entity(state, &mut args, name, false)?;

    aux::check_token(state, 1, "effect name", name, aux::valid_id)?;
    args.push(state[0].base);

    Ok((args, selector))
}

fn effect_clear_parse(state: &mut State) -> Result<Node> {
    let (args, selector) = effect_shared_parse(state, "effect clear")?;

    Ok(Node::Selector {
        args,
        command: Command::Effect,
        selector,
    })
}

fn effect_give_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "effect give";

    let (mut args, selector) = effect_shared_parse(state, NAME)?;

    aux::check_token(state, 1, "duration", NAME, aux::valid_value)?;
    args.push(state[0].base);

    aux::check_token(state, 1, "amplifier", NAME, aux::valid_value)?;
    args.push(state[0].base);

    Ok(Node::Selector {
        args,
        command: Command::Effect,
        selector,
    })
}

fn ex_run_parse(state: &mut State) -> Result<Node> {
    aux::check_presence(state, 1, "run command", "ex run")?;

    choose_parse(state)
}

fn team_shared_parse(state: &mut State, name: &str) -> Result<Vec<BaseToken>> {
    aux::check_token(state, 1, "team name", name, aux::valid_id)?;
    Ok(vec![state[0].base])
}

fn team_add_parse(state: &mut State) -> Result<Node> {
    Ok(Node::Base {
        args: team_shared_parse(state, "team add")?,
        command: Command::TeamAdd,
    })
}

fn team_join_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "team join";

    let mut args = team_shared_parse(state, NAME)?;

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    Ok(Node::Selector {
        args,
        command: Command::TeamJoin,
        selector,
    })
}

fn team_modify_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "team modify";

    let mut args = team_shared_parse(state, NAME)?;

    aux::check_token(state, 1, "rule", NAME, aux::valid_id)?;
    args.push(state[0].base);

    aux::check_token(state, 1, "value", NAME, aux::valid_id)?;
    args.push(state[0].base);

    Ok(Node::Base {
        args,
        command: Command::TeamModify,
    })
}

fn capture_entity_or_coords(
    state: &mut State,
    args: &mut Vec<BaseToken>,
    name: &str,
) -> Result<Selector> {
    if aux::valid_coordinate(state[0]) {
        capture_coords(state, args, 3, name)?;

        if !state.exceed(1) && aux::valid_coordinate(state[1]) {
            *state += 1;
            capture_coords(state, args, 2, name)?;
        }

        Ok(Selector::new_empty())
    } else if aux::valid_entity(state[0]) {
        capture_entity(state, args, name, false)
    } else {
        Err(Syntax(
            ["unknown argument ", state.extract(0), " for ", name].concat(),
        ))
    }
}

fn bossbar_add_parse(state: &mut State) -> Result<Node> {
    const NAME: &str = "bossbar add";

    aux::check_token(state, 1, "name", NAME, aux::valid_value)?;
    let args = vec![state[0].base];

    aux::check_token(state, 1, "text", NAME, aux::valid_text)?;

    Ok(Node::Text {
        args,
        command: Command::BossbarAdd,
        text: text::parse_text(state)?,
    })
}

fn bossbar_set_parse(state: &mut State) -> Result<Node> {
    use TokenKind::*;

    const NAME: &str = "bossbar set";

    aux::check_token(state, 1, "name", NAME, aux::valid_value)?;
    let mut args = vec![state[0].base];

    aux::check_token(
        state,
        1,
        "submode",
        NAME,
        make_check_kind!(Color | Players | Max),
    )?;
    args.push(state[0].base);

    let mut selector = Selector::new_empty();

    if state[0].kind == TokenKind::Players {
        aux::check_presence(state, 1, "entity", NAME)?;
        selector = capture_entity(state, &mut args, NAME, false)?;
    } else {
        let mut token_name = "color";
        let mut checker: fn(Token) -> bool = aux::valid_id;

        if state[0].kind == Max {
            token_name = "max";
            checker = aux::valid_numeric;
        }

        aux::check_token(state, 1, token_name, NAME, checker)?;
        args.push(state[0].base);
    }

    Ok(Node::Selector {
        args,
        command: Command::BossbarSet,
        selector,
    })
}

fn bossbar_remove_parse(state: &mut State) -> Result<Node> {
    simple_command(
        state,
        "bossbar remove",
        aux::valid_value,
        Command::BossbarRemove,
    )
}
