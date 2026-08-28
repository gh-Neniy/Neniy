pub mod aux;
pub mod data;
pub mod id;
pub mod method;
pub mod selector;
pub mod text;

use crate::{Result, synt::node::Node};
use aux::NodeView;
use std::path::Path;

pub fn translate(nodes: &[Node], source_code: &[u8], path: &Path) -> Result<String> {
    let mut result = String::with_capacity(source_code.len());

    for node in nodes {
        let mut node_view = NodeView::new(&mut result, node, source_code);

        method::choose_translate(&mut node_view, path)?;
        result.push('\n');
    }

    Ok(result)
}
