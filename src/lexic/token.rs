use sorted_code::{sorted_consts, sorted_enum, sorted_match};

#[sorted_enum]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TokenCategory {
    Control,
    Id,
    Invalid,
    Keyword,
    Numeric,
    Operator,
    Selector,
    Special,
    String,
}

#[sorted_enum]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TokenKind {
    About,
    Add,
    Advancement,
    Adventure,
    Align,
    Alt,
    Anchored,
    As,
    At,
    AttackDamage,
    AttackSpeed,
    Attribute,
    Axis,
    Billboard,
    Block,
    Blue,
    Bold,
    Bossbar,
    CanBreak,
    CanGrab,
    CanPlaceOn,
    Caret,
    Chest,
    ChestChance,
    Clear,
    Clone,
    Color,
    Comma,
    Creative,
    Crit,
    Damage,
    DarkBlue,
    DarkGreen,
    DarkRed,
    Data,
    Destroy,
    Distance,
    Dx,
    Dy,
    Dz,
    East,
    Effect,
    Enchantments,
    Ent,
    Ex,
    Eyes,
    Facing,
    Feet,
    FeetChance,
    Fill,
    Fn,
    Force,
    FromColor,
    Gamerule,
    Get,
    Give,
    Gm,
    Gold,
    Gray,
    Green,
    Half,
    Head,
    HeadChance,
    Health,
    Height,
    Hide,
    HurtTime,
    Id,
    If,
    InGround,
    Interaction,
    Invisible,
    Invulnerable,
    Italic,
    Item,
    Items,
    Join,
    Keep,
    Kill,
    LeftHand,
    LeftHandChance,
    Legs,
    LegsChance,
    Level,
    Limit,
    Lit,
    Loot,
    LootTable,
    Lore,
    Marker,
    Masked,
    Max,
    Modify,
    Motion,
    Move,
    Name,
    NameVisible,
    Native,
    NoAI,
    NoDespawn,
    NoGravity,
    NoTrade,
    Normal,
    North,
    Numeric,
    Obj,
    Open,
    Opr,
    Passenger,
    PickupDelay,
    Players,
    Pls,
    Pos,
    Potion,
    PotionColor,
    Powered,
    Profession,
    Ptc,
    Random,
    Range,
    Red,
    Remove,
    Replace,
    Reset,
    RightHand,
    RightHandChance,
    Rotation,
    Run,
    Say,
    Scale,
    Scb,
    Score,
    SelectedItem,
    Set,
    Setblock,
    Shine,
    Sign,
    Silent,
    Size,
    Sm,
    Sort,
    South,
    Spawnpoint,
    Spectate,
    Spectator,
    Stability,
    Stack,
    Stopsound,
    Storage,
    Store,
    String,
    Subtitle,
    Survival,
    Tag,
    Team,
    Tellraw,
    Text,
    Tilda,
    Time,
    Title,
    ToColor,
    Tp,
    TpTime,
    Type,
    Unbreakable,
    Uninited,
    Unless,
    West,
    Width,
    XRotation,
    YRotation,
    Yellow,

    #[sort_start]
    AllPlayerSelector,
    AllSelector,
    CurrentSelector,
    NearestPlayerSelector,
    RandomPlayerSelector,

    #[sort_start]
    ClosingCurlyBrace,
    ClosingSquareBrace,
    OpeningCurlyBrace,
    OpeningSquareBrace,

    #[sort_start]
    DivideEqualOperator,
    EqualOperator,
    GreaterOperator,
    GreaterOrEqualOperator,
    LessOperator,
    LessOrEqualOperator,
    MinusEqualOperator,
    MultEqualOperator,
    PlusEqualOperator,
}

pub type Index = u16; // enough for Valter's Going

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct BaseToken {
    pub start: Index,
    pub end: Index,
}

impl BaseToken {
    pub fn new_empty() -> Self {
        BaseToken { start: 1, end: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.start == 1 && self.end == 0
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Token {
    pub base: BaseToken,
    pub kind: TokenKind,
    pub category: TokenCategory,
}

impl Token {
    pub fn new(start: Index, end: Index, kind: TokenKind, category: TokenCategory) -> Self {
        Token {
            base: BaseToken { start, end },
            kind,
            category,
        }
    }
}

pub fn short_token_kind(token_body: &[u8]) -> TokenKind {
    // token_body.len() <= 8

    sorted_consts!(
        const ABOUT: u64 = hash(b"about");
        const ADD: u64 = hash(b"add");
        const ALIGN: u64 = hash(b"align");
        const ALT: u64 = hash(b"alt");
        const ANCHORED: u64 = hash(b"anchored");
        const AS: u64 = hash(b"as");
        const AT: u64 = hash(b"at");
        const AXIS: u64 = hash(b"axis");
        const BLOCK: u64 = hash(b"block");
        const BLUE: u64 = hash(b"blue");
        const BOLD: u64 = hash(b"bold");
        const BOSSBAR: u64 = hash(b"bossbar");
        const CAN_GRAB: u64 = hash(b"can_grab");
        const CARET: u64 = hash(b"^");
        const CHEST: u64 = hash(b"chest");
        const CLEAR: u64 = hash(b"clear");
        const CLONE: u64 = hash(b"clone");
        const CLOSING_CURLY_BRACE: u64 = hash(b"}");
        const CLOSING_SQUARE_BRACE: u64 = hash(b"]");
        const COLOR: u64 = hash(b"color");
        const COMMA: u64 = hash(b",");
        const CREATIVE: u64 = hash(b"creative");
        const CRIT: u64 = hash(b"crit");
        const DAMAGE: u64 = hash(b"damage");
        const DARK_RED: u64 = hash(b"dark_red");
        const DATA: u64 = hash(b"data");
        const DESTROY: u64 = hash(b"destroy");
        const DISTANCE: u64 = hash(b"distance");
        const DX: u64 = hash(b"dx");
        const DY: u64 = hash(b"dy");
        const DZ: u64 = hash(b"dz");
        const EAST: u64 = hash(b"east");
        const EFFECT: u64 = hash(b"effect");
        const ENT: u64 = hash(b"ent");
        const EQUAL_OPERATOR: u64 = hash(b"=");
        const EX: u64 = hash(b"ex");
        const EYES: u64 = hash(b"eyes");
        const FACING: u64 = hash(b"facing");
        const FEET: u64 = hash(b"feet");
        const FILL: u64 = hash(b"fill");
        const FN: u64 = hash(b"fn");
        const FORCE: u64 = hash(b"force");
        const GAMERULE: u64 = hash(b"gamerule");
        const GET: u64 = hash(b"get");
        const GIVE: u64 = hash(b"give");
        const GM: u64 = hash(b"gm");
        const GOLD: u64 = hash(b"gold");
        const GRAY: u64 = hash(b"gray");
        const GREATER_OPERATOR: u64 = hash(b">");
        const GREEN: u64 = hash(b"green");
        const HALF: u64 = hash(b"half");
        const HEAD: u64 = hash(b"head");
        const HEALTH: u64 = hash(b"health");
        const HEIGHT: u64 = hash(b"height");
        const HIDE: u64 = hash(b"hide");
        const ID: u64 = hash(b"id");
        const IF: u64 = hash(b"if");
        const ITALIC: u64 = hash(b"italic");
        const ITEM: u64 = hash(b"item");
        const ITEMS: u64 = hash(b"items");
        const JOIN: u64 = hash(b"join");
        const KEEP: u64 = hash(b"keep");
        const KILL: u64 = hash(b"kill");
        const LEGS: u64 = hash(b"legs");
        const LESS_OPERATOR: u64 = hash(b"<");
        const LEVEL: u64 = hash(b"level");
        const LIMIT: u64 = hash(b"limit");
        const LIT: u64 = hash(b"lit");
        const LOOT: u64 = hash(b"loot");
        const LORE: u64 = hash(b"lore");
        const MARKER: u64 = hash(b"marker");
        const MASKED: u64 = hash(b"masked");
        const MAX: u64 = hash(b"max");
        const MODIFY: u64 = hash(b"modify");
        const MOTION: u64 = hash(b"motion");
        const MOVE: u64 = hash(b"move");
        const NAME: u64 = hash(b"name");
        const NATIVE: u64 = hash(b"native");
        const NORMAL: u64 = hash(b"normal");
        const NORTH: u64 = hash(b"north");
        const NO_AI: u64 = hash(b"no_ai");
        const NO_TRADE: u64 = hash(b"no_trade");
        const OBJ: u64 = hash(b"obj");
        const OPEN: u64 = hash(b"open");
        const OPENING_CURLY_BRACE: u64 = hash(b"{");
        const OPENING_SQUARE_BRACE: u64 = hash(b"[");
        const OPR: u64 = hash(b"opr");
        const PLAYERS: u64 = hash(b"players");
        const PLS: u64 = hash(b"pls");
        const POS: u64 = hash(b"pos");
        const POTION: u64 = hash(b"potion");
        const POWERED: u64 = hash(b"powered");
        const PTC: u64 = hash(b"ptc");
        const RANDOM: u64 = hash(b"random");
        const RANGE: u64 = hash(b"..");
        const RED: u64 = hash(b"red");
        const REMOVE: u64 = hash(b"remove");
        const REPLACE: u64 = hash(b"replace");
        const RESET: u64 = hash(b"reset");
        const ROTATION: u64 = hash(b"rotation");
        const RUN: u64 = hash(b"run");
        const SAY: u64 = hash(b"say");
        const SCALE: u64 = hash(b"scale");
        const SCB: u64 = hash(b"scb");
        const SCORE: u64 = hash(b"score");
        const SET: u64 = hash(b"set");
        const SETBLOCK: u64 = hash(b"setblock");
        const SHINE: u64 = hash(b"shine");
        const SIGN: u64 = hash(b"sign");
        const SILENT: u64 = hash(b"silent");
        const SIZE: u64 = hash(b"size");
        const SM: u64 = hash(b"sm");
        const SORT: u64 = hash(b"sort");
        const SOUTH: u64 = hash(b"south");
        const SPECTATE: u64 = hash(b"spectate");
        const STACK: u64 = hash(b"stack");
        const STORAGE: u64 = hash(b"storage");
        const STORE: u64 = hash(b"store");
        const SUBTITLE: u64 = hash(b"subtitle");
        const SURVIVAL: u64 = hash(b"survival");
        const TAG: u64 = hash(b"tag");
        const TEAM: u64 = hash(b"team");
        const TELLRAW: u64 = hash(b"tellraw");
        const TEXT: u64 = hash(b"text");
        const TILDA: u64 = hash(b"~");
        const TIME: u64 = hash(b"time");
        const TITLE: u64 = hash(b"title");
        const TO_COLOR: u64 = hash(b"to_color");
        const TP: u64 = hash(b"tp");
        const TP_TIME: u64 = hash(b"tp_time");
        const TYPE: u64 = hash(b"type");
        const UNINITED: u64 = hash(b"uninited");
        const UNLESS: u64 = hash(b"unless");
        const WEST: u64 = hash(b"west");
        const WIDTH: u64 = hash(b"width");
        const YELLOW: u64 = hash(b"yellow");

        #[sort_start]
        const ALL_PLAYER_SELECTOR: u64 = hash(b"@a");
        const ALL_SELECTOR: u64 = hash(b"@e");
        const CURRENT_SELECTOR: u64 = hash(b"@s");
        const NEAREST_PLAYER_SELECTOR: u64 = hash(b"@p");
        const RANDOM_PLAYER_SELECTOR: u64 = hash(b"@r");

        #[sort_start]
        const DIVIDE_EQUAL_OPERATOR: u64 = hash(b"/=");
        const GREATER_OR_EQUAL_OPERATOR: u64 = hash(b">=");
        const LESS_OR_EQUAL_OPERATOR: u64 = hash(b"<=");
        const MINUS_EQUAL_OPERATOR: u64 = hash(b"-=");
        const MULT_EQUAL_OPERATOR: u64 = hash(b"*=");
        const PLUS_EQUAL_OPERATOR: u64 = hash(b"+=");
    );

    sorted_match!(match hash(token_body) {
        ABOUT => TokenKind::About,
        ADD => TokenKind::Add,
        ALIGN => TokenKind::Align,
        ALT => TokenKind::Alt,
        ANCHORED => TokenKind::Anchored,
        AS => TokenKind::As,
        AT => TokenKind::At,
        AXIS => TokenKind::Axis,
        BLOCK => TokenKind::Block,
        BLUE => TokenKind::Blue,
        BOLD => TokenKind::Bold,
        BOSSBAR => TokenKind::Bossbar,
        CAN_GRAB => TokenKind::CanGrab,
        CARET => TokenKind::Caret,
        CHEST => TokenKind::Chest,
        CLEAR => TokenKind::Clear,
        CLONE => TokenKind::Clone,
        CLOSING_CURLY_BRACE => TokenKind::ClosingCurlyBrace,
        CLOSING_SQUARE_BRACE => TokenKind::ClosingSquareBrace,
        COLOR => TokenKind::Color,
        COMMA => TokenKind::Comma,
        CREATIVE => TokenKind::Creative,
        CRIT => TokenKind::Crit,
        DAMAGE => TokenKind::Damage,
        DARK_RED => TokenKind::DarkRed,
        DATA => TokenKind::Data,
        DESTROY => TokenKind::Destroy,
        DISTANCE => TokenKind::Distance,
        DX => TokenKind::Dx,
        DY => TokenKind::Dy,
        DZ => TokenKind::Dz,
        EAST => TokenKind::East,
        EFFECT => TokenKind::Effect,
        ENT => TokenKind::Ent,
        EQUAL_OPERATOR => TokenKind::EqualOperator,
        EX => TokenKind::Ex,
        EYES => TokenKind::Eyes,
        FACING => TokenKind::Facing,
        FEET => TokenKind::Feet,
        FILL => TokenKind::Fill,
        FN => TokenKind::Fn,
        FORCE => TokenKind::Force,
        GAMERULE => TokenKind::Gamerule,
        GET => TokenKind::Get,
        GIVE => TokenKind::Give,
        GM => TokenKind::Gm,
        GOLD => TokenKind::Gold,
        GRAY => TokenKind::Gray,
        GREATER_OPERATOR => TokenKind::GreaterOperator,
        GREEN => TokenKind::Green,
        HALF => TokenKind::Half,
        HEAD => TokenKind::Head,
        HEALTH => TokenKind::Health,
        HEIGHT => TokenKind::Height,
        HIDE => TokenKind::Hide,
        ID => TokenKind::Id,
        IF => TokenKind::If,
        ITALIC => TokenKind::Italic,
        ITEM => TokenKind::Item,
        ITEMS => TokenKind::Items,
        JOIN => TokenKind::Join,
        KEEP => TokenKind::Keep,
        KILL => TokenKind::Kill,
        LEGS => TokenKind::Legs,
        LESS_OPERATOR => TokenKind::LessOperator,
        LEVEL => TokenKind::Level,
        LIMIT => TokenKind::Limit,
        LIT => TokenKind::Lit,
        LOOT => TokenKind::Loot,
        LORE => TokenKind::Lore,
        MARKER => TokenKind::Marker,
        MASKED => TokenKind::Masked,
        MAX => TokenKind::Max,
        MODIFY => TokenKind::Modify,
        MOTION => TokenKind::Motion,
        MOVE => TokenKind::Move,
        NAME => TokenKind::Name,
        NATIVE => TokenKind::Native,
        NORMAL => TokenKind::Normal,
        NORTH => TokenKind::North,
        NO_AI => TokenKind::NoAI,
        NO_TRADE => TokenKind::NoTrade,
        OBJ => TokenKind::Obj,
        OPEN => TokenKind::Open,
        OPENING_CURLY_BRACE => TokenKind::OpeningCurlyBrace,
        OPENING_SQUARE_BRACE => TokenKind::OpeningSquareBrace,
        OPR => TokenKind::Opr,
        PLAYERS => TokenKind::Players,
        PLS => TokenKind::Pls,
        POS => TokenKind::Pos,
        POTION => TokenKind::Potion,
        POWERED => TokenKind::Powered,
        PTC => TokenKind::Ptc,
        RANDOM => TokenKind::Random,
        RANGE => TokenKind::Range,
        RED => TokenKind::Red,
        REMOVE => TokenKind::Remove,
        REPLACE => TokenKind::Replace,
        RESET => TokenKind::Reset,
        ROTATION => TokenKind::Rotation,
        RUN => TokenKind::Run,
        SAY => TokenKind::Say,
        SCALE => TokenKind::Scale,
        SCB => TokenKind::Scb,
        SCORE => TokenKind::Score,
        SET => TokenKind::Set,
        SETBLOCK => TokenKind::Setblock,
        SHINE => TokenKind::Shine,
        SIGN => TokenKind::Sign,
        SILENT => TokenKind::Silent,
        SIZE => TokenKind::Size,
        SM => TokenKind::Sm,
        SORT => TokenKind::Sort,
        SOUTH => TokenKind::South,
        SPECTATE => TokenKind::Spectate,
        STACK => TokenKind::Stack,
        STORAGE => TokenKind::Storage,
        STORE => TokenKind::Store,
        SUBTITLE => TokenKind::Subtitle,
        SURVIVAL => TokenKind::Survival,
        TAG => TokenKind::Tag,
        TEAM => TokenKind::Team,
        TELLRAW => TokenKind::Tellraw,
        TEXT => TokenKind::Text,
        TILDA => TokenKind::Tilda,
        TIME => TokenKind::Time,
        TITLE => TokenKind::Title,
        TO_COLOR => TokenKind::ToColor,
        TP => TokenKind::Tp,
        TP_TIME => TokenKind::TpTime,
        TYPE => TokenKind::Type,
        UNINITED => TokenKind::Uninited,
        UNLESS => TokenKind::Unless,
        WEST => TokenKind::West,
        WIDTH => TokenKind::Width,
        YELLOW => TokenKind::Yellow,

        #[sort_start]
        ALL_PLAYER_SELECTOR => TokenKind::AllPlayerSelector,
        ALL_SELECTOR => TokenKind::AllSelector,
        CURRENT_SELECTOR => TokenKind::CurrentSelector,
        NEAREST_PLAYER_SELECTOR => TokenKind::NearestPlayerSelector,
        RANDOM_PLAYER_SELECTOR => TokenKind::RandomPlayerSelector,

        #[sort_start]
        DIVIDE_EQUAL_OPERATOR => TokenKind::DivideEqualOperator,
        GREATER_OR_EQUAL_OPERATOR => TokenKind::GreaterOrEqualOperator,
        LESS_OR_EQUAL_OPERATOR => TokenKind::LessOrEqualOperator,
        MINUS_EQUAL_OPERATOR => TokenKind::MinusEqualOperator,
        MULT_EQUAL_OPERATOR => TokenKind::MultEqualOperator,
        PLUS_EQUAL_OPERATOR => TokenKind::PlusEqualOperator,

        _ => TokenKind::Id,
    })
}

pub fn long_token_kind(token_body: &[u8]) -> TokenKind {
    sorted_match!(match token_body {
        b"advancement" => TokenKind::Advancement,
        b"adventure" => TokenKind::Adventure,
        b"attack_damage" => TokenKind::AttackDamage,
        b"attack_speed" => TokenKind::AttackSpeed,
        b"attribute" => TokenKind::Attribute,
        b"billboard" => TokenKind::Billboard,
        b"can_break" => TokenKind::CanBreak,
        b"can_place_on" => TokenKind::CanPlaceOn,
        b"chest_chance" => TokenKind::ChestChance,
        b"dark_blue" => TokenKind::DarkBlue,
        b"dark_green" => TokenKind::DarkGreen,
        b"enchantments" => TokenKind::Enchantments,
        b"feet_chance" => TokenKind::FeetChance,
        b"from_color" => TokenKind::FromColor,
        b"head_chance" => TokenKind::HeadChance,
        b"hurt_time" => TokenKind::HurtTime,
        b"in_ground" => TokenKind::InGround,
        b"interaction" => TokenKind::Interaction,
        b"invisible" => TokenKind::Invisible,
        b"invulnerable" => TokenKind::Invulnerable,
        b"left_hand" => TokenKind::LeftHand,
        b"left_hand_chance" => TokenKind::LeftHandChance,
        b"legs_chance" => TokenKind::LegsChance,
        b"loot_table" => TokenKind::LootTable,
        b"name_visible" => TokenKind::NameVisible,
        b"no_despawn" => TokenKind::NoDespawn,
        b"no_gravity" => TokenKind::NoGravity,
        b"passenger" => TokenKind::Passenger,
        b"pickup_delay" => TokenKind::PickupDelay,
        b"potion_color" => TokenKind::PotionColor,
        b"profession" => TokenKind::Profession,
        b"right_hand" => TokenKind::RightHand,
        b"right_hand_chance" => TokenKind::RightHandChance,
        b"selected_item" => TokenKind::SelectedItem,
        b"spawnpoint" => TokenKind::Spawnpoint,
        b"spectator" => TokenKind::Spectator,
        b"stability" => TokenKind::Stability,
        b"stopsound" => TokenKind::Stopsound,
        b"unbreakable" => TokenKind::Unbreakable,
        b"x_rotation" => TokenKind::XRotation,
        b"y_rotation" => TokenKind::YRotation,

        _ => TokenKind::Id,
    })
}

const fn hash(token_body: &[u8]) -> u64 {
    let mut bytes = [0u8; 8];
    let mut i = 0;

    while i < token_body.len() {
        bytes[i] = token_body[i];
        i += 1;
    }

    u64::from_ne_bytes(bytes)
}
