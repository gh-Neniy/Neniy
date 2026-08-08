use std::str;

use super::{
    aux::{self, List, State},
    data,
    node::{Command, Node, SelectorListNode},
    selector::{self, Selector},
    text::{self, Text},
};

use crate::{
    NeniyError::Syntax,
    Result,
    lexic::token::{BaseToken, IndexType, Token, TokenCategory, TokenKind},
};

macro_rules! make_check_kind {
    ($types:pat) => {
        (|token: Token| matches!(token.kind, $types)) as fn(Token) -> bool
    };
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

fn capture_ex_align(state: &mut State) -> Result<Node> {
    aux::check_token(state, 1, "argument", "ex align", aux::valid_id)?;

    Ok(Node::Base {
        args: vec![state[0].base],
        command: Command::ExAlign,
    })
}

fn capture_ex_anchored(state: &mut State) -> Result<Node> {
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
fn capture_ex_ent(state: &mut State, command: Command, is_if: bool) -> Result<Node> {
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
                "unknown token type in capture_ex_ent() (internal)".to_string(),
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
    count: IndexType,
    name: &str,
) -> Result<()> {
    args.push(capture_coordinate(state)?);

    for _ in 0..count - 1 {
        aux::check_presence(state, 1, "coordinate", name)?;

        args.push(capture_coordinate(state)?);
    }

    Ok(())
}

fn capture_ex_facing(state: &mut State) -> Result<Node> {
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
fn capture_ex_block(state: &mut State, is_if: bool) -> Result<Node> {
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
fn capture_ex_score(state: &mut State, is_if: bool) -> Result<Node> {
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

fn capture_ex_positioned(state: &mut State) -> Result<Node> {
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
fn capture_ex_store_score(state: &mut State) -> Result<Node> {
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
fn capture_ex_store_entity(state: &mut State) -> Result<Node> {
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

fn capture_ex_store_storage(state: &mut State) -> Result<Node> {
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

fn capture_ex_store_bossbar(state: &mut State) -> Result<Node> {
    simple_command(
        state,
        "ex store bossbar",
        aux::valid_value,
        Command::ExStoreBossbar,
    )
}

fn capture_ex_store(state: &mut State) -> Result<Node> {
    const NAME: &str = "ex store";

    aux::check_presence(state, 1, "mode", NAME)?;

    match state[0].kind {
        TokenKind::Bossbar => capture_ex_store_bossbar(state),
        TokenKind::Entity => capture_ex_store_entity(state),
        TokenKind::Score => capture_ex_store_score(state),
        TokenKind::Storage => capture_ex_store_storage(state),

        _ => Err(Syntax(
            ["invalid mode ", state.extract(0), " for ", NAME].concat(),
        )),
    }
}

fn capture_ex_uninited(state: &mut State) -> Result<Node> {
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
fn capture_scb_players(state: &mut State) -> Result<Node> {
    use TokenKind::*;

    const NAME: &str = "scb players";

    aux::check_token(
        state,
        1,
        "mode",
        NAME,
        make_check_kind!(Set | Add | Get | Operation | Remove | Reset),
    )?;

    let mode = state[0].kind;
    let mut args = vec![state[0].base];

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    aux::check_token(state, 1, "objective", NAME, aux::valid_id)?;
    args.push(state[0].base);

    if mode == Operation {
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
fn capture_scb_obj_add(state: &mut State) -> Result<Node> {
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
fn capture_scb_obj_set(state: &mut State) -> Result<Node> {
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
fn capture_scb_objectives(state: &mut State) -> Result<Node> {
    const NAME: &str = "scb objectives";

    aux::check_presence(state, 1, "mode", NAME)?;

    match state[0].kind {
        TokenKind::Add => capture_scb_obj_add(state),
        TokenKind::Set => capture_scb_obj_set(state),

        _ => Err(Syntax(
            ["invalid mode ", state.extract(0), " for ", NAME].concat(),
        )),
    }
}

fn capture_ex_items_shared(state: &mut State, args: &mut Vec<BaseToken>, name: &str) -> Result<()> {
    aux::check_token(state, 1, "container", name, aux::valid_id)?;
    args.push(state[0].base);

    aux::check_token(state, 1, "item name", name, aux::valid_id)?;
    args.push(state[0].base);

    Ok(())
}

fn capture_ex_items_block(state: &mut State, is_if: bool) -> Result<Node> {
    let name = ["ex", if is_if { "if" } else { "unless" }, " items block"].concat();

    let mut args = vec![state[-2].base];
    capture_ex_items_shared(state, &mut args, &name)?;

    Ok(Node::Base {
        args,
        command: Command::ExItemsBlock,
    })
}

fn capture_ex_items_ent(state: &mut State, is_if: bool) -> Result<Node> {
    let name = ["ex", if is_if { "if" } else { "unless" }, " items ent"].concat();

    let mut args = vec![state[-2].base];

    aux::check_presence(state, 1, "entity", &name)?;
    let selector = capture_entity(state, &mut args, &name, false)?;

    capture_ex_items_shared(state, &mut args, &name)?;

    Ok(Node::Selector {
        args,
        command: Command::ExItemsEnt,
        selector,
    })
}

fn capture_ex_condition(state: &mut State, subnodes: &mut Vec<Node>) -> Result<()> {
    let is_if = state[0].kind == TokenKind::If;
    let mut name = ["ex ", if is_if { "if" } else { "unless" }].concat();

    aux::check_presence(state, 1, "mode", &name)?;

    match state[0].kind {
        TokenKind::Block => subnodes.push(capture_ex_block(state, is_if)?),
        TokenKind::Entity => subnodes.push(capture_ex_ent(state, Command::ExEnt, is_if)?),
        TokenKind::Items => {
            name.push_str(" items");

            aux::check_presence(state, 1, "mode", &name)?;

            match state[0].kind {
                TokenKind::Block => subnodes.push(capture_ex_items_block(state, is_if)?),
                TokenKind::Entity => subnodes.push(capture_ex_items_ent(state, is_if)?),

                _ => {
                    return Err(Syntax(
                        ["unknown mode ", state.extract(0), " for ", &name].concat(),
                    ));
                }
            }
        }
        TokenKind::Score => subnodes.push(capture_ex_score(state, is_if)?),

        _ => {
            return Err(Syntax(
                ["invalid mode ", state.extract(0), " for ", &name].concat(),
            ));
        }
    }

    Ok(())
}

// state[0] on data mode
fn capture_data_shared(state: &mut State, command: Command) -> Result<(Vec<BaseToken>, Selector)> {
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

fn capture_data_get(state: &mut State) -> Result<Node> {
    let (args, selector) = capture_data_shared(state, Command::DataGet)?;

    Ok(Node::Selector {
        args,
        command: Command::DataGet,
        selector,
    })
}

fn capture_data_modify(state: &mut State) -> Result<Node> {
    const NAME: &str = "data modify";

    let (mut args, selector) = capture_data_shared(state, Command::DataModify)?;

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

fn capture_effect_shared(state: &mut State, name: &str) -> Result<(Vec<BaseToken>, Selector)> {
    let mut args = vec![state[0].base];

    aux::check_presence(state, 1, "entity", name)?;
    let selector = capture_entity(state, &mut args, name, false)?;

    aux::check_token(state, 1, "effect name", name, aux::valid_id)?;
    args.push(state[0].base);

    Ok((args, selector))
}

fn capture_effect_clear(state: &mut State) -> Result<Node> {
    let (args, selector) = capture_effect_shared(state, "effect clear")?;

    Ok(Node::Selector {
        args,
        command: Command::Effect,
        selector,
    })
}

fn capture_effect_give(state: &mut State) -> Result<Node> {
    const NAME: &str = "effect give";

    let (mut args, selector) = capture_effect_shared(state, NAME)?;

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

fn capture_ex_run(state: &mut State) -> Result<Node> {
    aux::check_presence(state, 1, "run command", "ex run")?;

    choose_parse(state)
}

fn capture_team_shared(state: &mut State, name: &str) -> Result<Vec<BaseToken>> {
    aux::check_token(state, 1, "team name", name, aux::valid_id)?;
    Ok(vec![state[0].base])
}

fn capture_team_add(state: &mut State) -> Result<Node> {
    Ok(Node::Base {
        args: capture_team_shared(state, "team add")?,
        command: Command::TeamAdd,
    })
}

fn capture_team_join(state: &mut State) -> Result<Node> {
    const NAME: &str = "team join";

    let mut args = capture_team_shared(state, NAME)?;

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    Ok(Node::Selector {
        args,
        command: Command::TeamJoin,
        selector,
    })
}

fn capture_team_modify(state: &mut State) -> Result<Node> {
    const NAME: &str = "team modify";

    let mut args = capture_team_shared(state, NAME)?;

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

fn capture_bossbar_add(state: &mut State) -> Result<Node> {
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

fn capture_bossbar_set(state: &mut State) -> Result<Node> {
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

fn capture_bossbar_remove(state: &mut State) -> Result<Node> {
    simple_command(
        state,
        "bossbar remove",
        aux::valid_value,
        Command::BossbarRemove,
    )
}

pub fn choose_parse(state: &mut State) -> Result<Node> {
    match state[0].kind {
        TokenKind::Advancement => advancement(state),
        _ => Err(Syntax(["unknown command ", state.extract(0)].concat())),
    }
}

pub fn advancement(state: &mut State) -> Result<Node> {
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

pub fn attribute(state: &mut State) -> Result<Node> {
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

pub fn bossbar(state: &mut State) -> Result<Node> {
    const NAME: &str = "bossbar";

    aux::check_presence(state, 1, "mode", NAME)?;

    match state[0].kind {
        TokenKind::Add => capture_bossbar_add(state),
        TokenKind::Set => capture_bossbar_set(state),
        TokenKind::Remove => capture_bossbar_remove(state),

        _ => Err(Syntax(
            ["invalid mode ", state.extract(0), " for bossbar"].concat(),
        )),
    }
}
