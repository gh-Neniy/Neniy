use std::str;

use sorted_code::{sorted_fns, sorted_match};

use super::{
    aux::{self, List, State},
    data,
    node::{Command, DoubleSelectorNode, Node, SelectorListNode, SelectorTextNode},
    selector::{self, Selector},
    text::{self, Text},
};

use crate::{
    ErrorKind::Syntax,
    NeniyError, Result,
    lexic::token::{BaseToken, Index, Token, TokenCategory, TokenKind},
};

macro_rules! make_check_kind {
    ($types:pat) => {
        (|token: Token| matches!(token.kind, $types)) as fn(Token) -> bool
    };
}

pub fn choose_parse(state: &mut State) -> Result<Node> {
    sorted_match!(match state[0].kind {
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
        TokenKind::Loot => loot_parse(state),
        TokenKind::Native => native_parse(state),
        TokenKind::Pls => pls_parse(state),
        TokenKind::Ptc => ptc_parse(state),
        TokenKind::Random => random_parse(state),
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

        _ => Err(NeniyError::new(
            ["unknown command \"", state.extract(0), "\""].concat(),
            Syntax,
            state.source_code,
            state[0].base.start,
            state[0].base.end,
        )),
    })
}

sorted_fns!(
    pub fn advancement_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "advancement";
        let mut args = Vec::with_capacity(2);

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
        let mut args = Vec::with_capacity(3);

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

        sorted_match!(match state[0].kind {
            TokenKind::Add => bossbar_add_parse(state),
            TokenKind::Remove => bossbar_remove_parse(state),
            TokenKind::Set => bossbar_set_parse(state),

            _ => Err(invalid_mode(state, NAME)),
        })
    }

    pub fn clear_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "clear";

        let mut args = Vec::with_capacity(3);

        aux::check_presence(state, 1, "entity", NAME)?;
        let selector = capture_entity(state, &mut args, NAME, false)?;

        aux::check_token(state, 1, "item", NAME, aux::valid_id)?;
        let id_with_data_ptr = data::parse_id_with_data(state)?;

        if !state.exceed(1) && state[1].kind == TokenKind::Numeric {
            *state += 1;
            args.push(state[0].base);
        }

        Ok(Node::SelectorIdWithData(Box::new(
            crate::synt::node::SelectorIdWithDataNode {
                args,
                command: Command::Clear,
                selector,
                id_with_data_ptr,
            },
        )))
    }

    pub fn clone_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "clone";

        let mut args = Vec::with_capacity(11);

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

        let mut args = Vec::with_capacity(2);

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

        sorted_match!(match state[0].kind {
            TokenKind::Get => data_get_parse(state),
            TokenKind::Modify => data_modify_parse(state),

            _ => Err(invalid_mode(state, NAME)),
        })
    }

    pub fn effect_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "effect";

        aux::check_presence(state, 1, "mode", NAME)?;

        sorted_match!(match state[0].kind {
            TokenKind::Clear => effect_clear_parse(state),
            TokenKind::Give => effect_give_parse(state),

            _ => Err(invalid_mode(state, NAME)),
        })
    }

    pub fn ex_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "ex";

        let mut subnodes = Vec::new();

        while !state.is_empty() {
            aux::check_presence(state, 1, "mode", NAME)?;

            sorted_match!(match state[0].kind {
                TokenKind::Align => subnodes.push(ex_align_parse(state)?),
                TokenKind::Anchored => subnodes.push(ex_anchored_parse(state)?),
                TokenKind::As => subnodes.push(ex_ent_parse(state, Command::ExAs, false)?),
                TokenKind::At => subnodes.push(ex_ent_parse(state, Command::ExAt, false)?),
                TokenKind::Facing => subnodes.push(ex_facing_parse(state)?),
                TokenKind::If => subnodes.push(ex_condition_parse(state)?),
                TokenKind::Pos => subnodes.push(ex_pos_parse(state)?),
                TokenKind::Run => {
                    return Ok(Node::Ex {
                        command: Command::Ex,
                        subnodes,
                        run_node: Box::new(ex_run_parse(state)?),
                    });
                }
                TokenKind::Store => subnodes.push(ex_store_parse(state)?),
                TokenKind::Uninited => subnodes.push(ex_uninited_parse(state)?),
                TokenKind::Unless => subnodes.push(ex_condition_parse(state)?),

                _ => {
                    return Err(invalid_mode(state, NAME));
                }
            })
        }

        Err(NeniyError::new(
            ["run command not found for ", NAME].concat(),
            Syntax,
            state.source_code,
            state[-1].base.end,
            state[-1].base.end,
        ))
    }

    pub fn fill_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "fill";

        let mut args = Vec::with_capacity(9);

        aux::check_presence(state, 1, "first coordinate", NAME)?;
        capture_coords(state, &mut args, 6, "fill")?;

        aux::check_token(state, 1, "block", NAME, aux::valid_id)?;
        let id_with_data_ptr = data::parse_id_with_data(state)?;

        aux::check_token(
            state,
            1,
            "mode",
            NAME,
            make_check_kind!(TokenKind::Keep | TokenKind::Replace | TokenKind::Destroy),
        )?;
        args.push(state[0].base);

        if !state.exceed(1) && state[1].kind == TokenKind::Id {
            *state += 1;
            args.push(state[0].base);
        }

        Ok(Node::IdWithData {
            args,
            command: Command::Fill,
            id_with_data_ptr,
        })
    }

    pub fn fn_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "fn";

        let mut args = Vec::with_capacity(2);

        aux::check_token(state, 1, "name", NAME, aux::valid_id)?;
        args.push(state[0].base);

        if !state.exceed(1) && state[1].kind == TokenKind::Id {
            *state += 1;
            args.push(state[0].base);
        }

        Ok(Node::Base {
            args,
            command: Command::Fn,
        })
    }

    pub fn gamerule_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "gamerule";

        aux::check_token(state, 1, "rule", NAME, aux::valid_id)?;
        aux::check_token(state, 1, "value", NAME, aux::valid_value)?;

        Ok(Node::Base {
            args: vec![state[-1].base, state[0].base],
            command: Command::Gamerule,
        })
    }

    pub fn give_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "give";

        let mut args = Vec::with_capacity(3);

        aux::check_presence(state, 1, "entity", NAME)?;
        let selector = capture_entity(state, &mut args, NAME, false)?;

        aux::check_token(state, 1, "item name", NAME, aux::valid_id)?;
        let id_with_data_ptr = data::parse_id_with_data(state)?;

        if !state.exceed(1) && state[1].kind == TokenKind::Numeric {
            *state += 1;
            args.push(state[0].base);
        }

        Ok(Node::SelectorIdWithData(Box::new(
            crate::synt::node::SelectorIdWithDataNode {
                args,
                command: Command::Give,
                selector,
                id_with_data_ptr,
            },
        )))
    }

    pub fn gm_parse(state: &mut State) -> Result<Node> {
        use TokenKind::*;

        const NAME: &str = "gm";
        let mut args = Vec::with_capacity(2);

        aux::check_token(
            state,
            1,
            "mode",
            NAME,
            make_check_kind!(Adventure | Creative | Spectator | Survival),
        )?;
        args.push(state[0].base);

        aux::check_presence(state, 1, "player", NAME)?;
        let selector = capture_entity(state, &mut args, NAME, false)?;

        Ok(Node::Selector {
            args,
            command: Command::Gm,
            selector,
        })
    }

    pub fn kill_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "kill";

        let mut args = Vec::with_capacity(1);

        aux::check_presence(state, 1, "entity", NAME)?;
        let selector = capture_entity(state, &mut args, NAME, false)?;

        Ok(Node::Selector {
            args,
            command: Command::Kill,
            selector,
        })
    }

    pub fn loot_parse(state: &mut State) -> Result<Node> {
        simple_command(state, "loot", aux::valid_id, Command::Loot)
    }

    pub fn native_parse(state: &mut State) -> Result<Node> {
        simple_command(state, "native", aux::valid_string, Command::Native)
    }

    pub fn pls_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "pls";
        let mut args = Vec::with_capacity(7);

        aux::check_token(state, 1, "sound name", NAME, aux::valid_id)?;
        args.push(state[0].base);

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

    pub fn ptc_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "ptc";

        aux::check_token(state, 1, "particle name", NAME, aux::valid_id)?;
        let id_with_data_ptr = data::parse_id_with_data(state)?;

        let mut args = Vec::with_capacity(9);

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

        Ok(Node::IdWithData {
            args,
            command: Command::Ptc,
            id_with_data_ptr,
        })
    }

    pub fn random_parse(state: &mut State) -> Result<Node> {
        aux::check_token(state, 1, "range", "random", aux::valid_range)?;

        Ok(Node::Base {
            args: vec![aux::capture_range(state)?],
            command: Command::Random,
        })
    }

    pub fn say_parse(state: &mut State) -> Result<Node> {
        simple_command(state, "say", aux::valid_value, Command::Say)
    }

    pub fn scb_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "scb";

        aux::check_presence(state, 1, "mode", NAME)?;

        sorted_match!(match state[0].kind {
            TokenKind::Obj => scb_obj_parse(state),
            TokenKind::Players => scb_players_parse(state),

            _ => Err(invalid_mode(state, NAME)),
        })
    }

    pub fn setblock_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "setblock";

        let mut args = Vec::with_capacity(4);

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

        Ok(Node::IdWithData {
            args,
            command: Command::Setblock,
            id_with_data_ptr,
        })
    }

    pub fn sm_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "sm";

        aux::check_token(state, 1, "entity", NAME, aux::valid_id)?;
        let id_with_data_ptr = data::parse_id_with_data(state)?;

        let mut args = Vec::with_capacity(3);

        aux::check_presence(state, 1, "first coordinate", NAME)?;
        capture_coords(state, &mut args, 3, NAME)?;

        Ok(Node::IdWithData {
            args,
            command: Command::Sm,
            id_with_data_ptr,
        })
    }

    pub fn spawnpoint_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "spawnpoint";

        let mut args = Vec::with_capacity(4);

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

        let mut args = Vec::with_capacity(1);

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

        let mut args = Vec::with_capacity(2);

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

    pub fn tag_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "tag";

        let mut args = Vec::with_capacity(3);

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

        sorted_match!(match state[0].kind {
            TokenKind::Add => team_add_parse(state),
            TokenKind::Join => team_join_parse(state),
            TokenKind::Modify => team_modify_parse(state),

            _ => Err(invalid_mode(state, NAME)),
        })
    }

    pub fn tellraw_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "tellraw";

        let mut args = Vec::with_capacity(1);

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
        let mut args = Vec::with_capacity(2);

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
        let mut args = Vec::with_capacity(2);

        aux::check_presence(state, 1, "first entity or first coordinate", NAME)?;
        let selector1 = capture_entity_or_coords(state, &mut args, NAME)?;

        let selector2 = if args.len() < 3 // entity captured
            && !state.exceed(1)
            && (state[1].category == TokenCategory::Selector
                || state[1].kind == TokenKind::Id
                || aux::valid_coordinate(state[1]))
        {
            *state += 1;
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

    #[sort_start]
    fn capture_coordinate(state: &mut State) -> Result<BaseToken> {
        if aux::valid_numeric(state[0]) {
            return Ok(state[0].base);
        }

        if matches!(state[0].kind, TokenKind::Tilda | TokenKind::Caret) {
            if state.exceed(1)
                || !aux::valid_numeric(state[1])
                || !aux::consecutive(state[0], state[1])
            {
                return Ok(state[0].base);
            }

            *state += 1;

            return Ok(BaseToken {
                start: state[-1].base.start,
                end: state[0].base.end,
            });
        }

        Err(NeniyError::new(
            ["invalid coordinate \"", state.extract(0), "\""].concat(),
            Syntax,
            state.source_code,
            state[0].base.start,
            state[0].base.end,
        ))
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

    fn capture_data_field(state: &mut State, args: &mut Vec<BaseToken>) -> Result<()> {
        let mut data_field = state[0].base;

        while !state.exceed(1) && state[1].kind == TokenKind::OpeningSquareBrace {
            if state.exceed(3) {
                return Err(NeniyError::new(
                    "indexing not found for data-field".to_string(),
                    Syntax,
                    state.source_code,
                    state[1].base.start,
                    state[1].base.end,
                ));
            }

            if !aux::valid_numeric(state[2])
                || state[3].kind != TokenKind::ClosingSquareBrace
                || !aux::consecutive3(state[1], state[2], state[3])
            {
                return Err(NeniyError::new(
                    [
                        "invalid indexing \"",
                        state.extract_segment(1, 3),
                        "\" in data-field",
                    ]
                    .concat(),
                    Syntax,
                    state.source_code,
                    state[1].base.start,
                    state[3].base.end,
                ));
            }

            data_field.end = state[3].base.end;
            *state += 3;
        }

        args.push(data_field);
        Ok(())
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
            Err(NeniyError::new(
                ["invalid entity \"", state.extract(0), "\" for ", name].concat(),
                Syntax,
                state.source_code,
                state[0].base.start,
                state[0].base.end,
            ))
        }
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
            Err(NeniyError::new(
                [
                    "invalid entity or coords \"",
                    state.extract(0),
                    "\" for ",
                    name,
                ]
                .concat(),
                Syntax,
                state.source_code,
                state[0].base.start,
                state[0].base.end,
            ))
        }
    }

    fn invalid_mode(state: &State, name: &str) -> NeniyError {
        NeniyError::new(
            ["invalid mode \"", state.extract(0), "\" for ", name].concat(),
            Syntax,
            state.source_code,
            state[0].base.start,
            state[0].base.end,
        )
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

    #[sort_start]
    fn bossbar_add_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "bossbar add";

        aux::check_token(state, 1, "name", NAME, aux::valid_value)?;
        aux::check_token(state, 1, "text", NAME, aux::valid_text)?;

        Ok(Node::Text {
            args: vec![state[-1].base],
            command: Command::BossbarAdd,
            text: text::parse_text(state)?,
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

    fn bossbar_set_parse(state: &mut State) -> Result<Node> {
        use TokenKind::*;

        const NAME: &str = "bossbar set";
        let mut args = Vec::with_capacity(3);

        aux::check_token(state, 1, "name", NAME, aux::valid_value)?;
        args.push(state[0].base);

        aux::check_token(
            state,
            1,
            "submode",
            NAME,
            make_check_kind!(Color | Players | Max),
        )?;
        args.push(state[0].base);

        let mut selector = Selector::new_empty();

        if state[0].kind == Players {
            aux::check_presence(state, 1, "entity", NAME)?;
            selector = capture_entity(state, &mut args, NAME, false)?;
        } else {
            if state[0].kind == Max {
                aux::check_token(state, 1, "max", NAME, aux::valid_numeric)?;
            } else {
                aux::check_token(state, 1, "color", NAME, aux::valid_id)?;
            };

            args.push(state[0].base);
        }

        Ok(Node::Selector {
            args,
            command: Command::BossbarSet,
            selector,
        })
    }

    fn data_get_parse(state: &mut State) -> Result<Node> {
        let (args, selector) = data_shared_parse(state, "data get", 2)?;

        Ok(Node::Selector {
            args,
            command: Command::DataGet,
            selector,
        })
    }

    fn data_modify_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "data modify";

        let (mut args, selector) = data_shared_parse(state, NAME, 4)?;

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
            return Err(NeniyError::new(
                ["invalid value \"", state.extract(0), "\" for ", NAME].concat(),
                Syntax,
                state.source_code,
                state[0].base.start,
                state[0].base.end,
            ));
        }

        Ok(Node::SelectorList(Box::new(SelectorListNode {
            args,
            command: Command::DataModify,
            selector,
            list,
        })))
    }

    // state[0] on data mode
    fn data_shared_parse(
        state: &mut State,
        name: &str,
        capacity: usize,
    ) -> Result<(Vec<BaseToken>, Selector)> {
        let mut args = Vec::with_capacity(capacity);

        aux::check_presence(state, 1, "entity", name)?;
        let selector = capture_entity(state, &mut args, name, false)?;

        aux::check_token(state, 1, "data-field", name, aux::valid_id)?;
        capture_data_field(state, &mut args)?;

        Ok((args, selector))
    }

    fn effect_clear_parse(state: &mut State) -> Result<Node> {
        let (args, selector) = effect_shared_parse(state, "effect clear", 3)?;

        Ok(Node::Selector {
            args,
            command: Command::Effect,
            selector,
        })
    }

    fn effect_give_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "effect give";

        let (mut args, selector) = effect_shared_parse(state, NAME, 5)?;

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

    fn effect_shared_parse(
        state: &mut State,
        name: &str,
        capacity: usize,
    ) -> Result<(Vec<BaseToken>, Selector)> {
        let mut args = Vec::with_capacity(capacity);
        args.push(state[0].base);

        aux::check_presence(state, 1, "entity", name)?;
        let selector = capture_entity(state, &mut args, name, false)?;

        aux::check_token(state, 1, "effect name", name, aux::valid_id)?;
        args.push(state[0].base);

        Ok((args, selector))
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

    // state[0] on "block"
    fn ex_block_parse(state: &mut State, is_if: bool) -> Result<Node> {
        let name = if is_if {
            "ex if block"
        } else {
            "ex unless block"
        };

        let mut args = Vec::with_capacity(4);
        args.push(state[-1].base);

        aux::check_presence(state, 1, "first coordinate", name)?;
        capture_coords(state, &mut args, 3, name)?;

        aux::check_token(state, 1, "block", name, aux::valid_id)?;

        Ok(Node::IdWithData {
            args,
            command: Command::ExBlock,
            id_with_data_ptr: data::parse_id_with_data(state)?,
        })
    }

    fn ex_condition_parse(state: &mut State) -> Result<Node> {
        let is_if = state[0].kind == TokenKind::If;
        let name = if is_if { "ex if" } else { "ex unless" };

        aux::check_presence(state, 1, "mode", name)?;

        sorted_match!(match state[0].kind {
            TokenKind::Block => ex_block_parse(state, is_if),
            TokenKind::Ent => ex_ent_parse(state, Command::ExEnt, is_if),
            TokenKind::Items => {
                let name = if is_if {
                    "ex if items"
                } else {
                    "ex unless items"
                };

                aux::check_presence(state, 1, "mode", name)?;

                sorted_match!(match state[0].kind {
                    TokenKind::Block => ex_items_block_parse(state, is_if),
                    TokenKind::Ent => ex_items_ent_parse(state, is_if),

                    _ => Err(invalid_mode(state, name)),
                })
            }
            TokenKind::Score => ex_score_parse(state, is_if),

            _ => Err(invalid_mode(state, name)),
        })
    }

    // state[0] on last token
    fn ex_ent_parse(state: &mut State, command: Command, is_if: bool) -> Result<Node> {
        let mut args = Vec::with_capacity(2);

        let name = sorted_match!(match command {
            Command::ExAs => "ex as",
            Command::ExAt => "ex at",
            Command::ExEnt => {
                args.push(state[-1].base);

                if is_if { "ex if ent" } else { "ex unless ent" }
            }

            _ => {
                return Err(NeniyError::new(
                    "unknown command in ex_ent_parse() (internal)".to_string(),
                    Syntax,
                    state.source_code,
                    state[0].base.start,
                    state[0].base.end,
                ));
            }
        });

        aux::check_presence(state, 1, "entity", name)?;
        let selector = capture_entity(state, &mut args, name, false)?;

        Ok(Node::Selector {
            args,
            command,
            selector,
        })
    }

    fn ex_facing_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "ex facing";

        aux::check_presence(state, 1, "first coordinate or entity", NAME)?;

        let mut args = Vec::with_capacity(3);

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

    fn ex_items_block_parse(state: &mut State, is_if: bool) -> Result<Node> {
        let name = if is_if {
            "ex if items block"
        } else {
            "ex unless items block"
        };

        let mut args = Vec::with_capacity(3);
        args.push(state[-2].base);

        ex_items_shared_parse(state, &mut args, name)?;

        Ok(Node::Base {
            args,
            command: Command::ExItemsBlock,
        })
    }

    fn ex_items_ent_parse(state: &mut State, is_if: bool) -> Result<Node> {
        let name = if is_if {
            "ex if items ent"
        } else {
            "ex unless items ent"
        };

        let mut args = Vec::with_capacity(4);
        args.push(state[-2].base);

        aux::check_presence(state, 1, "entity", name)?;
        let selector = capture_entity(state, &mut args, name, false)?;

        ex_items_shared_parse(state, &mut args, name)?;

        Ok(Node::Selector {
            args,
            command: Command::ExItemsEnt,
            selector,
        })
    }

    fn ex_items_shared_parse(
        state: &mut State,
        args: &mut Vec<BaseToken>,
        name: &str,
    ) -> Result<()> {
        aux::check_token(state, 1, "container", name, aux::valid_id)?;
        args.push(state[0].base);

        aux::check_token(state, 1, "item name", name, aux::valid_id)?;
        args.push(state[0].base);

        Ok(())
    }

    fn ex_pos_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "ex pos";

        let mut args = Vec::with_capacity(3);

        aux::check_presence(state, 1, "first coordinate", NAME)?;
        capture_coords(state, &mut args, 3, NAME)?;

        Ok(Node::Base {
            args,
            command: Command::ExPos,
        })
    }

    fn ex_run_parse(state: &mut State) -> Result<Node> {
        aux::check_presence(state, 1, "run command", "ex run")?;

        choose_parse(state)
    }

    // state[0] on "score"
    fn ex_score_parse(state: &mut State, is_if: bool) -> Result<Node> {
        let name = if is_if {
            "ex if score"
        } else {
            "ex unless score"
        };

        let mut args = Vec::with_capacity(5);
        args.push(state[-1].base);

        aux::check_presence(state, 1, "entity", name)?;
        let selector = capture_entity(state, &mut args, name, false)?;

        aux::check_token(state, 1, "objective", name, aux::valid_id)?;
        args.push(state[0].base);

        aux::check_presence(state, 1, "range or operator", name)?;

        if aux::valid_range(state[0]) {
            args.push(aux::capture_range(state)?);
        } else if aux::valid_operator(state[0]) {
            args.push(state[0].base);

            aux::check_token(state, 1, "second entity", name, aux::valid_id)?;
            args.push(state[0].base);
        } else {
            return Err(NeniyError::new(
                [
                    "invalid range or operator \"",
                    state.extract(0),
                    "\" in ",
                    name,
                ]
                .concat(),
                Syntax,
                state.source_code,
                state[0].base.start,
                state[0].base.end,
            ));
        }

        Ok(Node::Selector {
            args,
            command: Command::ExScore,
            selector,
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

    // state[0] on "entity"
    fn ex_store_entity_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "ex store ent";

        let mut args = Vec::with_capacity(4);

        aux::check_presence(state, 1, "entity", NAME)?;
        let selector = capture_entity(state, &mut args, NAME, false)?;

        aux::check_token(state, 1, "data-field", NAME, aux::valid_id)?;
        capture_data_field(state, &mut args)?;

        aux::check_token(state, 1, "data type", NAME, aux::valid_id)?;
        args.push(state[0].base);

        aux::check_token(state, 1, "multiplier", NAME, aux::valid_numeric)?;
        args.push(state[0].base);

        Ok(Node::Selector {
            args,
            command: Command::ExStoreEnt,
            selector,
        })
    }

    fn ex_store_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "ex store";

        aux::check_presence(state, 1, "mode", NAME)?;

        sorted_match!(match state[0].kind {
            TokenKind::Bossbar => ex_store_bossbar_parse(state),
            TokenKind::Ent => ex_store_entity_parse(state),
            TokenKind::Score => ex_store_score_parse(state),
            TokenKind::Storage => ex_store_storage_parse(state),

            _ => Err(invalid_mode(state, NAME)),
        })
    }

    // state[0] on "score"
    fn ex_store_score_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "ex store score";
        let mut args = Vec::with_capacity(2);

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

    fn ex_store_storage_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "ex store storage";

        aux::check_token(state, 1, "name", NAME, aux::valid_id)?;
        aux::check_token(state, 1, "variable", NAME, aux::valid_id)?;
        aux::check_token(state, 1, "data type", NAME, aux::valid_id)?;
        aux::check_token(state, 1, "multiplier", NAME, aux::valid_numeric)?;

        Ok(Node::Base {
            args: vec![
                state[-3].base,
                state[-2].base,
                state[-1].base,
                state[0].base,
            ],
            command: Command::ExStoreStorage,
        })
    }

    fn ex_uninited_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "ex uninited";
        let mut args = Vec::with_capacity(2);

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

    // state[0] on "add"
    fn scb_obj_add_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "scb obj add";

        aux::check_token(state, 1, "objective", NAME, aux::valid_id)?;
        aux::check_token(state, 1, "objective type", NAME, aux::valid_id)?;

        let args = vec![state[-1].base, state[0].base];
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

    // state[0] on "obj"
    fn scb_obj_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "scb obj";

        aux::check_presence(state, 1, "mode", NAME)?;

        sorted_match!(match state[0].kind {
            TokenKind::Add => scb_obj_add_parse(state),
            TokenKind::Set => scb_obj_set_parse(state),

            _ => Err(invalid_mode(state, NAME)),
        })
    }

    // state[0] on "set"
    fn scb_obj_set_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "scb obj set";

        aux::check_token(state, 1, "action", NAME, aux::valid_id)?;
        aux::check_token(state, 1, "objective", NAME, aux::valid_id)?;

        Ok(Node::Base {
            args: vec![state[-1].base, state[0].base],
            command: Command::ScbObjSet,
        })
    }

    // state[0] on "players"
    fn scb_players_parse(state: &mut State) -> Result<Node> {
        use TokenKind::*;

        const NAME: &str = "scb players";
        let mut args = Vec::with_capacity(5);

        aux::check_token(
            state,
            1,
            "mode",
            NAME,
            make_check_kind!(Set | Add | Get | Opr | Remove | Reset),
        )?;

        let mode = state[0].kind;
        args.push(state[0].base);

        aux::check_presence(state, 1, "entity", NAME)?;
        let selector = capture_entity(state, &mut args, NAME, false)?;

        aux::check_token(state, 1, "objective", NAME, aux::valid_id)?;
        args.push(state[0].base);

        if mode == Opr {
            const NAME: &str = "scb players opr";

            aux::check_token(state, 1, "operator", NAME, aux::valid_operator)?;
            args.push(state[0].base);

            // second entity is never being selector
            aux::check_token(state, 1, "second entity", NAME, aux::valid_id)?;
            args.push(state[0].base);
        } else if matches!(mode, Add | Remove | Set) {
            aux::check_token(state, 1, "value", NAME, aux::valid_numeric)?;
            args.push(state[0].base);
        }

        Ok(Node::Selector {
            args,
            command: Command::ScbPlayers,
            selector,
        })
    }

    fn team_add_parse(state: &mut State) -> Result<Node> {
        simple_command(state, "team add", aux::valid_id, Command::TeamAdd)
    }

    fn team_join_parse(state: &mut State) -> Result<Node> {
        const NAME: &str = "team join";
        let mut args = Vec::with_capacity(2);

        aux::check_token(state, 1, "team name", NAME, aux::valid_id)?;
        args.push(state[0].base);

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

        aux::check_token(state, 1, "team name", NAME, aux::valid_id)?;
        aux::check_token(state, 1, "rule", NAME, aux::valid_id)?;
        aux::check_token(state, 1, "value", NAME, aux::valid_id)?;

        Ok(Node::Base {
            args: vec![state[-2].base, state[-1].base, state[0].base],
            command: Command::TeamModify,
        })
    }
);
