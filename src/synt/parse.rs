use super::{aux::State, method, node::Node};
use crate::{Result, lexic::token::Token};

pub fn parse(tokens: Vec<Token>, source_code: &[u8]) -> Result<Vec<Node>> {
    let mut state = State::new(&tokens, source_code, 0);
    let mut nodes = Vec::with_capacity(tokens.len() / 5);

    while !state.is_empty() {
        nodes.push(method::choose_parse(&mut state)?);

        state += 1;
    }

    Ok(nodes)
}
