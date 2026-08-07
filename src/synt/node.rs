use crate::lexic::token::BaseToken;

use super::{aux::ListType, data::IdWithDataPtr, selector::Selector, text::Text};

pub enum Command {
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
    SelectorListType(Box<SelectorListTypeNode>),           // 8 bytes (instead of 81)
    SelectorText(Box<SelectorTextNode>),                   // 8 bytes (instead of 81)

    Text {
        args: Vec<BaseToken>,
        command: Command,
        text: Text,
    }, // 49 bytes
}

pub struct DoubleSelectorNode {
    pub args: Vec<BaseToken>,
    pub command: Command,
    pub selector1: Selector,
    pub selector2: Selector,
}

pub struct SelectorIdWithDataPtrNode {
    pub args: Vec<BaseToken>,
    pub command: Command,
    pub selector: Selector,
    pub id_with_data_ptr: IdWithDataPtr,
}

pub struct SelectorListTypeNode {
    pub args: Vec<BaseToken>,
    pub command: Command,
    pub selector: Selector,
    pub list: ListType,
}

pub struct SelectorTextNode {
    pub args: Vec<BaseToken>,
    pub command: Command,
    pub selector: Selector,
    pub text: Text,
}
