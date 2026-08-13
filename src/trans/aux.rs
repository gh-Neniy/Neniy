use std::{iter::Extend, ops::Index, str};

use crate::{
    lexic::token::{BaseToken, IndexType},
    synt::{
        aux::ListUnit,
        node::{Command, Node},
    },
};

pub struct NodeView<'a> {
    result: &'a mut String,
    node: Node,
    source_code: &'a [u8],
}

impl<'a> NodeView<'a> {
    pub fn new(result: &'a mut String, node: Node, source_code: &'a [u8]) -> Self {
        NodeView {
            result,
            node,
            source_code,
        }
    }

    pub fn push_str(&mut self, string: &str) {
        self.result.push_str(string);
    }

    pub fn push(&mut self, ch: char) {
        self.result.push(ch);
    }

    pub fn extract_token(&self, token: BaseToken) -> &'a str {
        str::from_utf8(&self.source_code[token.start as usize..=token.end as usize]).unwrap()
    }

    pub fn extract(&self, i: IndexType) -> &'a str {
        self.extract_token(self[i])
    }

    pub fn result(&self) -> &str {
        self.result
    }

    pub fn source_code(&self) -> &[u8] {
        self.source_code
    }

    pub fn args_len(&self) -> usize {
        self.args().len()
    }

    pub fn command(&self) -> Command {
        match &self.node {
            Node::Base { command, .. }
            | Node::Execute { command, .. }
            | Node::IdWithDataPtr { command, .. }
            | Node::Selector { command, .. }
            | Node::Text { command, .. } => *command,

            Node::DoubleSelector(node) => node.command,
            Node::SelectorIdWithDataPtr(node) => node.command,
            Node::SelectorList(node) => node.command,
            Node::SelectorText(node) => node.command,
        }
    }

    fn args(&self) -> &Vec<BaseToken> {
        match &self.node {
            Node::Base { args, .. }
            | Node::Execute { args, .. }
            | Node::IdWithDataPtr { args, .. }
            | Node::Selector { args, .. }
            | Node::Text { args, .. } => args,

            Node::DoubleSelector(node) => &node.args,
            Node::SelectorIdWithDataPtr(node) => &node.args,
            Node::SelectorList(node) => &node.args,
            Node::SelectorText(node) => &node.args,
        }
    }
}

impl<'a> Index<IndexType> for NodeView<'a> {
    type Output = BaseToken;

    fn index(&self, i: IndexType) -> &Self::Output {
        &self.args()[i as usize]
    }
}

impl<'a, A> Extend<A> for NodeView<'a>
where
    String: Extend<A>,
{
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        self.result.extend(iter)
    }
}

pub fn translate_bool(cond: bool) -> &'static str {
    if cond { "true" } else { "false" }
}

pub fn translate_numeric_list(node_view: &mut NodeView, list: &[ListUnit], suffix: &str) {
    node_view.push('[');

    let mut iter = list.iter();
    node_view.extend([node_view.extract_token(iter.next().unwrap().key), suffix]);

    for unit in iter {
        node_view.push(',');
        node_view.extend([node_view.extract_token(unit.key), suffix]);
    }

    node_view.push(']');
}
