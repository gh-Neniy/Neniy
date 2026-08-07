use crate::lexic::token::BaseToken;

use super::{aux::ListType, data::IdWithDataPtr, selector::Selector, text::Text};

pub enum Node {
    Base {
        args: Vec<BaseToken>,
    }, // 24 bytes

    DoubleSelector(Box<DoubleSelectorNode>), // 8 bytes (instead of 88)

    Execute {
        args: Vec<BaseToken>,
        subnodes: Vec<Node>,
        run_node: Box<Node>,
    }, // 56 bytes

    IdWithDataPtr {
        args: Vec<BaseToken>,
        id_with_data_ptr: IdWithDataPtr,
    }, // 32 bytes

    Selector {
        args: Vec<BaseToken>,
        selector: Selector,
    }, // 56 bytes

    SelectorIdWithDataPtr(Box<SelectorIdWithDataPtrNode>), // 8 bytes (instead of 64)
    SelectorListType(Box<SelectorListTypeNode>),           // 8 bytes (instead of 80)
    SelectorText(Box<SelectorTextNode>),                   // 8 bytes (instead of 80)

    Text {
        args: Vec<BaseToken>,
        text: Text,
    }, // 48 bytes
}

pub struct DoubleSelectorNode {
    pub args: Vec<BaseToken>,
    pub selector1: Selector,
    pub selector2: Selector,
}

pub struct SelectorIdWithDataPtrNode {
    pub args: Vec<BaseToken>,
    pub selector: Selector,
    pub id_with_data_ptr: IdWithDataPtr,
}

pub struct SelectorListTypeNode {
    pub args: Vec<BaseToken>,
    pub selector: Selector,
    pub list: ListType,
}

pub struct SelectorTextNode {
    pub args: Vec<BaseToken>,
    pub selector: Selector,
    pub text: Text,
}
