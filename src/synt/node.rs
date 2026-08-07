use crate::lexic::token::BaseToken;

use super::{aux::ListType, data::IdWithDataPtr, selector::Selector, text::Text};

pub enum NodeKind {
    Base,
    DoubleSelector,
    Execute,
    IdWithDataPtr,
    Selector,
    SelectorIdWithDataPtr,
    SelectorListType,
    SelectorText,
    Text,
}

pub enum CommandKind {
    Advancement,
    Attribute,
    BossbarAdd,
    BossbarSet,
    BossbarRemove,
    Clear,
    Clone,
    Damage,
    DataGet,
    DataModify,
    Effect,
    Execute,
    ExecuteAlign,
    ExecuteAnchored,
    ExecuteAs,
    ExecuteAt,
    ExecuteFacing,
    ExecutePositioned,
    ExecuteStoreBossbar,
    ExecuteStoreEntity,
    ExecuteStoreScore,
    ExecuteStoreStorage,
    ExecuteUninited,
    ExecuteBlock,
    ExecuteEntity,
    ExecuteItemsBlock,
    ExecuteItemsEntity,
    ExecuteScore,
    Fill,
    Function,
    Gamemode,
    Gamerule,
    Give,
    Kill,
    Native,
    Particle,
    Playsound,
    Say,
    ScoreboardObjectivesAdd,
    ScoreboardObjectivesSet,
    ScoreboardPlayers,
    Setblock,
    Spawnpoint,
    Spectate,
    Stopsound,
    Summon,
    Tag,
    TeamAdd,
    TeamJoin,
    TeamModify,
    Tellraw,
    Time,
    Title,
    Tp,
}

pub struct Node {
    args: Vec<BaseToken>,
}

pub struct SelectorNode {
    node: Node,
    selector: Selector,
}

pub struct DoubleSelectorNode {
    node: Node,
    selector1: Selector,
    selector2: Selector,
}

pub struct IdWithDataPtrNode {
    node: Node,
    id_with_data_ptr: IdWithDataPtr,
}

pub struct TextNode {
    node: Node,
    text: Text,
}

pub struct SelectorIdWithDataPtrNode {
    node: Node,
    selector: Selector,
    id_with_data_ptr: IdWithDataPtr,
}

pub struct SelectorListTypeNode {
    node: Node,
    selector: Selector,
    list: ListType,
}

pub struct SelectorTextNode {
    node: Node,
    selector: Selector,
    text: Text,
}

pub struct ExecuteNode {
    node: Node,
    subnodes: NodePtrs,
    run_node: NodePtr,
}

pub type NodePtr = Box<Node>;
pub type NodePtrs = Vec<NodePtr>;
