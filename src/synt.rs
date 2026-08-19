// Все функции, принимающие State, после успешного завершения переводят его на последний токен, который был обработан
// Кроме функции HaveNextTextBlock в selector

pub mod aux;
pub mod data;
pub mod method;
pub mod node;
pub mod selector;
pub mod text;

use crate::{Result, lexic::token::Token};
use {aux::State, node::Node};

pub fn parse(tokens: &[Token], source_code: &[u8]) -> Result<Vec<Node>> {
    let mut state = State::new(tokens, source_code, 0);
    let mut nodes = Vec::with_capacity(tokens.len() / 5);

    while !state.is_empty() {
        nodes.push(method::choose_parse(&mut state)?);

        state += 1;
    }

    Ok(nodes)
}
