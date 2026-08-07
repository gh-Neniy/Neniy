use std::str;

use super::{
    aux::{self, State},
    data,
    node::{Command, Node},
    selector::{self, Selector},
};

use crate::{
    NeniyError, Result,
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

    Err(NeniyError::Syntax(
        ["invalid coordinate ", state.extract(0)].concat(),
    ))
}

fn capture_entity(
    state: &mut State,
    args: &mut Vec<BaseToken>,
    name: &str,
    look_ahead: bool,
) -> Result<Selector> {
    if aux::valid_identifier(state[0]) {
        args.push(state[0].base);
        Ok(Selector::new_empty())
    } else if state[0].category == TokenCategory::Selector {
        selector::parse_selector(state, look_ahead)
    } else {
        Err(NeniyError::Syntax(
            ["invalid entity ", state.extract(0), " for ", name].concat(),
        ))
    }
}

fn capture_execute_align(state: &mut State) -> Result<Node> {
    aux::check_token(state, 1, "argument", "execute align", aux::valid_identifier)?;

    Ok(Node::Base {
        args: vec![state[0].base],
        command: Command::ExecuteAlign,
    })
}

fn capture_execute_anchored(state: &mut State) -> Result<Node> {
    aux::check_token(
        state,
        1,
        "mode",
        "execute anchored",
        make_check_kind!(TokenKind::Eyes | TokenKind::Feet),
    )?;

    Ok(Node::Base {
        args: vec![state[0].base],
        command: Command::ExecuteAnchored,
    })
}

// state[0] on last token
fn capture_execute_entity(state: &mut State, command: Command, is_if: bool) -> Result<Node> {
    let mut args = Vec::new();

    let name = match command {
        Command::ExecuteAs => "ex as",
        Command::ExecuteAt => "ex at",
        Command::ExecuteEntity => {
            args.push(state[-1].base);

            if is_if { "ex if ent" } else { "ex unless ent" }
        }

        _ => {
            return Err(NeniyError::Syntax(
                "unknown token type in capture_execute_entity() (internal)".to_string(),
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

fn capture_execute_facing(state: &mut State) -> Result<Node> {
    const NAME: &str = "execute facing";

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
        command: Command::ExecuteFacing,
        selector,
    })
}

// state[0] on "block"
fn capture_execute_block(state: &mut State, is_if: bool) -> Result<Node> {
    let name = ["execute ", if is_if { "if" } else { "unless" }, " block"].concat();

    let mut args = vec![state[-1].base];

    aux::check_presence(state, 1, "first coordinate", &name)?;
    capture_coords(state, &mut args, 3, &name)?;

    aux::check_token(state, 1, "block", &name, aux::valid_identifier)?;

    Ok(Node::IdWithDataPtr {
        args,
        command: Command::ExecuteBlock,
        id_with_data_ptr: data::parse_id_with_data(state)?,
    })
}

// state[0] on "score"
fn capture_execute_score(state: &mut State, is_if: bool) -> Result<Node> {
    let name = ["execute ", if is_if { "if" } else { "unless" }, " score"].concat();

    let mut args = vec![state[-1].base];

    aux::check_presence(state, 1, "entity", &name)?;
    let selector = capture_entity(state, &mut args, &name, false)?;

    aux::check_token(state, 1, "objective", &name, aux::valid_identifier)?;
    args.push(state[0].base);

    aux::check_presence(state, 1, "range or operator", &name)?;

    if aux::valid_range(state[0]) {
        args.push(aux::capture_range(state)?);
    } else if aux::valid_operator(state[0]) {
        args.push(state[0].base);

        aux::check_token(state, 1, "second entity", &name, aux::valid_identifier)?;
        args.push(state[0].base);
    } else {
        return Err(NeniyError::Syntax(
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
        command: Command::ExecuteScore,
        selector,
    })
}

fn capture_execute_positioned(state: &mut State) -> Result<Node> {
    const NAME: &str = "ex pos";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "first coordinate", NAME)?;
    capture_coords(state, &mut args, 3, NAME)?;

    Ok(Node::Base {
        args,
        command: Command::ExecutePositioned,
    })
}

// state[0] on "score"
fn capture_execute_store_score(state: &mut State) -> Result<Node> {
    const NAME: &str = "ex store score";

    let mut args = Vec::new();

    aux::check_presence(state, 1, "entity", NAME)?;
    let selector = capture_entity(state, &mut args, NAME, false)?;

    aux::check_token(state, 1, "objective", NAME, aux::valid_identifier)?;
    args.push(state[0].base);

    Ok(Node::Selector {
        args,
        command: Command::ExecuteStoreScore,
        selector,
    })
}

// fn capture_data_field(state: &mut State) -> Result<BaseToken> {}
