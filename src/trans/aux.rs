use std::{iter::Extend, str};

use sorted_code::{sorted_match, sorted_methods};

use crate::{
    NeniyError::Logic,
    Result,
    lexic::token::BaseToken,
    synt::{
        aux::ListUnit,
        data::IdWithData,
        node::{Command, Node},
        selector::Selector,
        text::TextUnit,
    },
};

#[derive(Debug)]
pub struct NodeView<'a> {
    pub result: &'a mut String,
    pub node: &'a Node,
    pub source_code: &'a [u8],
}

#[sorted_methods]
impl<'a> NodeView<'a> {
    pub fn as_base(&self) -> Result<&'a [BaseToken]> {
        if let Node::Base { args, .. } = &self.node {
            Ok(args)
        } else {
            Err(Logic(self.error("Base")))
        }
    }

    pub fn as_double_selector(&self) -> Result<(&'a [BaseToken], &'a Selector, &'a Selector)> {
        if let Node::DoubleSelector(node) = &self.node {
            Ok((&node.args, &node.selector1, &node.selector2))
        } else {
            Err(Logic(self.error("DoubleSelector")))
        }
    }

    pub fn as_ex(&self) -> Result<(&'a [Node], &'a Node)> {
        if let Node::Ex {
            subnodes, run_node, ..
        } = &self.node
        {
            Ok((subnodes, run_node))
        } else {
            Err(Logic(self.error("Ex")))
        }
    }

    pub fn as_id_with_data(&self) -> Result<(&'a [BaseToken], &'a IdWithData)> {
        if let Node::IdWithData {
            args,
            id_with_data_ptr,
            ..
        } = &self.node
        {
            Ok((args, id_with_data_ptr))
        } else {
            Err(Logic(self.error("IdWithData")))
        }
    }

    pub fn as_selector(&self) -> Result<(&'a [BaseToken], &'a Selector)> {
        if let Node::Selector { args, selector, .. } = &self.node {
            Ok((args, selector))
        } else {
            Err(Logic(self.error("Selector")))
        }
    }

    pub fn as_selector_id_with_data(
        &self,
    ) -> Result<(&'a [BaseToken], &'a Selector, &'a IdWithData)> {
        if let Node::SelectorIdWithData(node) = &self.node {
            Ok((&node.args, &node.selector, &node.id_with_data_ptr))
        } else {
            Err(Logic(self.error("SelectorIdWithData")))
        }
    }

    pub fn as_selector_list(&self) -> Result<(&'a [BaseToken], &'a Selector, &'a [ListUnit])> {
        if let Node::SelectorList(node) = &self.node {
            Ok((&node.args, &node.selector, &node.list))
        } else {
            Err(Logic(self.error("SelectorList")))
        }
    }

    pub fn as_selector_text(&self) -> Result<(&'a [BaseToken], &'a Selector, &'a [TextUnit])> {
        if let Node::SelectorText(node) = &self.node {
            Ok((&node.args, &node.selector, &node.text))
        } else {
            Err(Logic(self.error("SelectorText")))
        }
    }

    pub fn as_text(&self) -> Result<(&'a [BaseToken], &'a [TextUnit])> {
        if let Node::Text { args, text, .. } = &self.node {
            Ok((args, text))
        } else {
            Err(Logic(self.error("Text")))
        }
    }

    pub fn command(&self) -> Command {
        sorted_match!(match self.node {
            Node::Base { command, .. }
            | Node::Ex { command, .. }
            | Node::IdWithData { command, .. }
            | Node::Selector { command, .. }
            | Node::Text { command, .. } => *command,

            Node::DoubleSelector(node) => node.command,
            Node::SelectorIdWithData(node) => node.command,
            Node::SelectorList(node) => node.command,
            Node::SelectorText(node) => node.command,
        })
    }

    pub fn extract(&self, token: BaseToken) -> &'a str {
        str::from_utf8(&self.source_code[token.start as usize..=token.end as usize]).unwrap()
    }

    pub fn new(result: &'a mut String, node: &'a Node, source_code: &'a [u8]) -> Self {
        NodeView {
            result,
            node,
            source_code,
        }
    }

    pub fn push(&mut self, ch: char) {
        self.result.push(ch);
    }

    pub fn push_str(&mut self, string: &str) {
        self.result.push_str(string);
    }

    #[sort_end]
    fn error(&self, tried: &str) -> String {
        let actual = sorted_match!(match self.node {
            Node::Base { .. } => "Base",
            Node::DoubleSelector(_) => "DoubleSelector",
            Node::Ex { .. } => "Ex",
            Node::IdWithData { .. } => "IdWithData",
            Node::Selector { .. } => "Selector",
            Node::SelectorIdWithData(_) => "SelectorIdWithData",
            Node::SelectorList(_) => "SelectorList",
            Node::SelectorText(_) => "SelectorText",
            Node::Text { .. } => "Text",
        });

        [
            "tried to extract node from NodeView as ",
            tried,
            ", but actual variant was ",
            actual,
            " (internal)",
        ]
        .concat()
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
    if list.is_empty() {
        node_view.push_str("[]");
        return;
    }

    node_view.push('[');

    let mut iter = list.iter();
    node_view.extend([node_view.extract(iter.next().unwrap().key), suffix]);

    for unit in iter {
        node_view.push(',');
        node_view.extend([node_view.extract(unit.key), suffix]);
    }

    node_view.push(']');
}
