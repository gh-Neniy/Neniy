use std::{
    ops::{AddAssign, Index},
    str,
};

use crate::{
    NeniyError,
    lexic::token::{BaseToken, IndexType, Token, TokenCategory, TokenKind},
};

pub struct State<'a> {
    tokens: &'a [Token],
    source_code: &'a [u8],
    pos: IndexType,
}

impl<'a> State<'a> {
    pub fn new(tokens: &'a [Token], source_code: &'a [u8], pos: IndexType) -> Self {
        State {
            tokens,
            source_code,
            pos,
        }
    }

    pub fn exceed(&self, offset: IndexType) -> bool {
        self.pos + offset >= self.tokens.len() as IndexType
    }

    pub fn extract(&self, offset: IndexType) -> &str {
        let token = &self.tokens[(self.pos + offset) as usize].base;

        str::from_utf8(&self.source_code[token.start as usize..=token.end as usize]).unwrap()
    }

    pub fn extract_segment(&self, i1: IndexType, i2: IndexType) -> &str {
        let start = self.tokens[(self.pos + i1) as usize].base.start as usize;
        let end = self.tokens[(self.pos + i2) as usize].base.end as usize;

        str::from_utf8(&self.source_code[start..=end]).unwrap()
    }
}

impl<'a> AddAssign<IndexType> for State<'a> {
    fn add_assign(&mut self, offset: IndexType) {
        self.pos += offset;
    }
}

impl<'a> Index<i16> for State<'a> {
    type Output = Token;

    fn index(&self, offset: i16) -> &Self::Output {
        &self.tokens[((self.pos as i16).saturating_add(offset)) as usize]
    }
}

pub struct ListUnit {
    key: BaseToken,
    value: BaseToken,
}

pub type ListType = Vec<ListUnit>;

pub fn capture_range(state: &mut State) -> Result<BaseToken, NeniyError> {
    let mut range = state[0].base;

    if valid_numeric(state[0]) {
        if state.exceed(1) || state[1].kind != TokenKind::Range {
            return Ok(state[0].base);
        }

        if !consecutive(state[0], state[1]) {
            return Err(NeniyError::Syntax(
                ["invalid range", state.extract_segment(0, 1)].concat(),
            ));
        }

        *state += 1; // on ".." token
    }

    range.end = capture_range_impl(state)?;
    Ok(range)
}

pub fn check_presence(
    state: &mut State,
    offset: IndexType,
    token_name: &str,
    command: &str,
) -> Result<(), NeniyError> {
    if state.exceed(offset) {
        return Err(NeniyError::Syntax(
            [token_name, " not found for ", command].concat(),
        ));
    }

    *state += offset;
    Ok(())
}

pub fn check_token(
    state: &mut State,
    offset: IndexType,
    token_name: &str,
    command: &str,
    valid_token: fn(Token) -> bool,
) -> Result<(), NeniyError> {
    check_presence(state, offset, token_name, command)?;

    if !valid_token(state[0]) {
        return Err(NeniyError::Syntax(
            [
                "invalid",
                token_name,
                " ",
                state.extract(0),
                " in ",
                command,
            ]
            .concat(),
        ));
    }

    Ok(())
}

pub fn unit_check(
    state: &mut State,
    unit_name: &str,
    valid_value: fn(Token) -> bool,
) -> Result<(), NeniyError> {
    if state.exceed(2) {
        return Err(NeniyError::Syntax(
            ["not enough tokens for ", unit_name].concat(),
        ));
    }
    if state[1].kind != TokenKind::EqualOperator {
        return Err(NeniyError::Syntax(
            ["'=' not found for ", unit_name].concat(),
        ));
    }
    if !valid_value(state[2]) {
        return Err(NeniyError::Syntax(
            ["invalid value ", state.extract(2), " for ", unit_name].concat(),
        ));
    }

    *state += 2;
    Ok(())
}

pub fn consecutive(t1: Token, t2: Token) -> bool {
    t1.base.end + 1 == t2.base.start
}

pub fn valid_coordinate(token: Token) -> bool {
    valid_numeric(token) || matches!(token.kind, TokenKind::Tilda | TokenKind::Caret)
}

pub fn valid_data(token: Token) -> bool {
    token.kind == TokenKind::OpeningSquareBrace
}

pub fn valid_entity(token: Token) -> bool {
    valid_identifier(token) || token.category == TokenCategory::Selector
}

pub fn valid_identifier(token: Token) -> bool {
    matches!(
        token.category,
        TokenCategory::Identifier | TokenCategory::Keyword
    )
}

pub fn valid_numeric(token: Token) -> bool {
    token.kind == TokenKind::NumericLiteral
}

pub fn valid_numeric_or_list(token: Token) -> bool {
    valid_numeric(token) || token.kind == TokenKind::OpeningSquareBrace
}

pub fn valid_operator(token: Token) -> bool {
    token.category == TokenCategory::Operator
}

pub fn valid_range(token: Token) -> bool {
    matches!(token.kind, TokenKind::Range | TokenKind::NumericLiteral)
}

pub fn valid_string(token: Token) -> bool {
    token.kind == TokenKind::StringLiteral
}

pub fn valid_text(token: Token) -> bool {
    matches!(
        token.kind,
        TokenKind::OpeningSquareBrace | TokenKind::OpeningCurlyBrace
    )
}

pub fn valid_value(token: Token) -> bool {
    valid_identifier(token) || valid_numeric(token)
}

// state[0] == '['
pub fn capture_list(state: &mut State) -> Result<ListType, NeniyError> {
    let mut list = ListType::new();

    *state += 1;
    while !state.exceed(0) && state[0].kind != TokenKind::ClosingSquareBrace {
        if state[0].kind == TokenKind::Comma {
            *state += 1;
            continue;
        }

        if valid_numeric(state[0]) {
            list.push(ListUnit {
                key: state[0].base,
                value: BaseToken::new_empty(),
            });

            *state += 1;
            continue;
        }

        if !valid_identifier(state[0]) {
            return Err(NeniyError::Syntax(
                ["invalid key ", state.extract(0), " in list"].concat(),
            ));
        }

        list.push(capture_list_item(state)?);

        *state += 1;
    }

    if state.exceed(0) {
        return Err(NeniyError::Syntax(
            "']' not found for list in selector".to_string(),
        ));
    }

    Ok(list)
}

fn capture_range_impl(state: &mut State) -> Result<IndexType, NeniyError> {
    if state.exceed(1) || !valid_numeric(state[1]) {
        return Ok(state[0].base.end);
    }

    if !consecutive(state[0], state[1]) {
        return Err(NeniyError::Syntax(
            ["invalid range ", state.extract_segment(0, 1)].concat(),
        ));
    }

    *state += 1;

    Ok(state[0].base.end)
}

fn capture_list_item(state: &mut State) -> Result<ListUnit, NeniyError> {
    if state.exceed(1) || state[1].kind != TokenKind::EqualOperator {
        return Ok(ListUnit {
            key: state[0].base,
            value: BaseToken::new_empty(),
        });
    }

    check_token(state, 2, "value", "list", valid_value)?;

    if valid_range(state[0]) {
        return Ok(ListUnit {
            key: state[-2].base,
            value: capture_range(state)?,
        });
    }

    Ok(ListUnit {
        key: state[-2].base,
        value: state[0].base,
    })
}
