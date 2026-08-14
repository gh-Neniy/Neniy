use sorted_code::sorted_enum;

use crate::lexic::token::BaseToken;

use super::{aux::List, data::IdWithDataPtr, selector::Selector, text::Text};

#[sorted_enum]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Command {
    Advancement,
    Attribute,
    BossbarAdd,
    BossbarRemove,
    BossbarSet,
    Clear,
    Clone,
    Damage,
    DataGet,
    DataModify,
    Effect,
    Ex,
    ExAlign,
    ExAnchored,
    ExAs,
    ExAt,
    ExBlock,
    ExEnt,
    ExFacing,
    ExItemsBlock,
    ExItemsEnt,
    ExPos,
    ExScore,
    ExStoreBossbar,
    ExStoreEnt,
    ExStoreScore,
    ExStoreStorage,
    ExUninited,
    Fill,
    Fn,
    Gamerule,
    Give,
    Gm,
    Kill,
    Native,
    Pls,
    Ptc,
    Say,
    ScbObjAdd,
    ScbObjSet,
    ScbPlayers,
    Setblock,
    Sm,
    Spawnpoint,
    Spectate,
    Stopsound,
    Tag,
    TeamAdd,
    TeamJoin,
    TeamModify,
    Tellraw,
    Time,
    Title,
    Tp,
}

#[sorted_enum]
#[derive(Debug)]
pub enum Node {
    Base {
        args: Vec<BaseToken>,
        command: Command,
    }, // 25 bytes

    DoubleSelector(Box<DoubleSelectorNode>), // 8 bytes (instead of 89)

    Execute {
        args: Vec<BaseToken>,
        command: Command,
        subnodes: Vec<Node>,
        run_node: Box<Node>,
    }, // 57 bytes

    IdWithDataPtr {
        args: Vec<BaseToken>,
        command: Command,
        id_with_data_ptr: IdWithDataPtr,
    }, // 33 bytes

    Selector {
        args: Vec<BaseToken>,
        command: Command,
        selector: Selector,
    }, // 57 bytes

    SelectorIdWithDataPtr(Box<SelectorIdWithDataPtrNode>), // 8 bytes (instead of 65)
    SelectorList(Box<SelectorListNode>),                   // 8 bytes (instead of 81)
    SelectorText(Box<SelectorTextNode>),                   // 8 bytes (instead of 81)

    Text {
        args: Vec<BaseToken>,
        command: Command,
        text: Text,
    }, // 49 bytes
}

#[derive(Debug)]
pub struct DoubleSelectorNode {
    pub args: Vec<BaseToken>,
    pub command: Command,
    pub selector1: Selector,
    pub selector2: Selector,
}

#[derive(Debug)]
pub struct SelectorIdWithDataPtrNode {
    pub args: Vec<BaseToken>,
    pub command: Command,
    pub selector: Selector,
    pub id_with_data_ptr: IdWithDataPtr,
}

#[derive(Debug)]
pub struct SelectorListNode {
    pub args: Vec<BaseToken>,
    pub command: Command,
    pub selector: Selector,
    pub list: List,
}

#[derive(Debug)]
pub struct SelectorTextNode {
    pub args: Vec<BaseToken>,
    pub command: Command,
    pub selector: Selector,
    pub text: Text,
}
