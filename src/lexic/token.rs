use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TokenCategory {
    Control,
    Identifier,
    Invalid,
    Keyword,
    NumericLiteral,
    Operator,
    Selector,
    Special,
    StringLiteral,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TokenKind {
    About,
    Add,
    Advancement,
    Adventure,
    Align,
    Anchored,
    As,
    At,
    AttackDamage,
    AttackSpeed,
    Attribute,
    Axis,
    Block,
    Blue,
    Bold,
    Bossbar,
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
    Entity,
    Execute,
    Eyes,
    Facing,
    Feet,
    FeetChance,
    Fill,
    Force,
    FromColor,
    Function,
    Gamemode,
    Gamerule,
    Get,
    Give,
    Gold,
    Gray,
    Green,
    Half,
    Head,
    HeadChance,
    Health,
    Height,
    Hieroglyph,
    Hide,
    HurtTime,
    Id,
    Identifier,
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
    LootTable,
    Lore,
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
    Normal,
    North,
    NumericLiteral,
    Objectives,
    Open,
    Operation,
    Particle,
    PickupDelay,
    Players,
    Playsound,
    Positioned,
    Potion,
    PotionColor,
    Powered,
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
    Score,
    Scoreboard,
    SelectedItem,
    Set,
    Setblock,
    Shine,
    Sign,
    Silent,
    Size,
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
    StringLiteral,
    Subtitle,
    Summon,
    Survival,
    Tag,
    Team,
    TeleportDuration,
    Tellraw,
    Text,
    Tilda,
    Time,
    Title,
    ToColor,
    Tp,
    Type,
    Unbreakable,
    Uninited,
    Unless,
    West,
    Width,
    XRotation,
    Yellow,
    YRotation,

    AllPlayerSelector,
    AllSelector,
    CurrentSelector,
    NearestPlayerSelector,
    RandomPlayerSelector,

    ClosingCurlyBrace,
    ClosingSquareBrace,
    OpeningCurlyBrace,
    OpeningSquareBrace,

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

pub type IndexType = u16; // enough for Valter's Going

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct BaseToken {
    pub start: IndexType,
    pub end: IndexType,
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
    pub fn new(start: IndexType, end: IndexType, kind: TokenKind, category: TokenCategory) -> Self {
        Token {
            base: BaseToken { start, end },
            kind,
            category,
        }
    }
}

pub fn short_token_kind(token_body: &[u8]) -> TokenKind {
    // token_body.len() <= 8

    const CARET: u64 = hash(b"^");
    const CLOSING_SQUARE_BRACE: u64 = hash(b"]");
    const CLOSING_CURLY_BRACE: u64 = hash(b"}");
    const COMMA: u64 = hash(b",");
    const EQUAL_OPERATOR: u64 = hash(b"=");
    const GREATER_OPERATOR: u64 = hash(b">");
    const LESS_OPERATOR: u64 = hash(b"<");
    const OPENING_SQUARE_BRACE: u64 = hash(b"[");
    const OPENING_CURLY_BRACE: u64 = hash(b"{");
    const TILDA: u64 = hash(b"~");

    const ABOUT: u64 = hash(b"about");
    const ADD: u64 = hash(b"add");
    const ALIGN: u64 = hash(b"align");
    const ANCHORED: u64 = hash(b"anchored");
    const AS: u64 = hash(b"as");
    const AT: u64 = hash(b"at");
    const AXIS: u64 = hash(b"axis");
    const BLOCK: u64 = hash(b"block");
    const BLUE: u64 = hash(b"blue");
    const BOLD: u64 = hash(b"bold");
    const BOSSBAR: u64 = hash(b"bossbar");
    const CAN_GRAB: u64 = hash(b"can_grab");
    const CHEST: u64 = hash(b"chest");
    const CLEAR: u64 = hash(b"clear");
    const CLONE: u64 = hash(b"clone");
    const COLOR: u64 = hash(b"color");
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
    const EX: u64 = hash(b"ex");
    const EYES: u64 = hash(b"eyes");
    const FACING: u64 = hash(b"facing");
    const FEET: u64 = hash(b"feet");
    const FILL: u64 = hash(b"fill");
    const FORCE: u64 = hash(b"force");
    const FN: u64 = hash(b"fn");
    const GM: u64 = hash(b"gm");
    const GAMERULE: u64 = hash(b"gamerule");
    const GET: u64 = hash(b"get");
    const GIVE: u64 = hash(b"give");
    const GOLD: u64 = hash(b"gold");
    const GRAY: u64 = hash(b"gray");
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
    const LEVEL: u64 = hash(b"level");
    const LIMIT: u64 = hash(b"limit");
    const LIT: u64 = hash(b"lit");
    const LORE: u64 = hash(b"lore");
    const MAX: u64 = hash(b"max");
    const MASKED: u64 = hash(b"masked");
    const MODIFY: u64 = hash(b"modify");
    const MOTION: u64 = hash(b"motion");
    const MOVE: u64 = hash(b"move");
    const NAME: u64 = hash(b"name");
    const NATIVE: u64 = hash(b"native");
    const NO_AI: u64 = hash(b"no_ai");
    const NORMAL: u64 = hash(b"normal");
    const NORTH: u64 = hash(b"north");
    const OBJ: u64 = hash(b"obj");
    const OPEN: u64 = hash(b"open");
    const OPR: u64 = hash(b"opr");
    const PTC: u64 = hash(b"ptc");
    const PLAYERS: u64 = hash(b"players");
    const PLS: u64 = hash(b"pls");
    const POS: u64 = hash(b"pos");
    const POTION: u64 = hash(b"potion");
    const POWERED: u64 = hash(b"powered");
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
    const SORT: u64 = hash(b"sort");
    const SOUTH: u64 = hash(b"south");
    const SPECTATE: u64 = hash(b"spectate");
    const STACK: u64 = hash(b"stack");
    const STORAGE: u64 = hash(b"storage");
    const STORE: u64 = hash(b"store");
    const SUBTITLE: u64 = hash(b"subtitle");
    const SM: u64 = hash(b"sm");
    const SURVIVAL: u64 = hash(b"survival");
    const TAG: u64 = hash(b"tag");
    const TEAM: u64 = hash(b"team");
    const TELLRAW: u64 = hash(b"tellraw");
    const TEXT: u64 = hash(b"text");
    const TIME: u64 = hash(b"time");
    const TITLE: u64 = hash(b"title");
    const TO_COLOR: u64 = hash(b"to_color");
    const TP: u64 = hash(b"tp");
    const TYPE: u64 = hash(b"type");
    const UNINITED: u64 = hash(b"uninited");
    const UNLESS: u64 = hash(b"unless");
    const WEST: u64 = hash(b"west");
    const WIDTH: u64 = hash(b"width");
    const YELLOW: u64 = hash(b"yellow");

    const ALL_PLAYER_SELECTOR: u64 = hash(b"@a");
    const ALL_SELECTOR: u64 = hash(b"@e");
    const CURRENT_SELECTOR: u64 = hash(b"@s");
    const NEAREST_PLAYER_SELECTOR: u64 = hash(b"@p");
    const RANDOM_PLAYER_SELECTOR: u64 = hash(b"@r");

    const DIVIDE_EQUAL_OPERATOR: u64 = hash(b"/=");
    const GREATER_OR_EQUAL_OPERATOR: u64 = hash(b">=");
    const LESS_OR_EQUAL_OPERATOR: u64 = hash(b"<=");
    const MINUS_EQUAL_OPERATOR: u64 = hash(b"-=");
    const MULT_EQUAL_OPERATOR: u64 = hash(b"*=");
    const PLUS_EQUAL_OPERATOR: u64 = hash(b"+=");

    match hash(token_body) {
        CARET => TokenKind::Caret,
        CLOSING_SQUARE_BRACE => TokenKind::ClosingSquareBrace,
        CLOSING_CURLY_BRACE => TokenKind::ClosingCurlyBrace,
        COMMA => TokenKind::Comma,
        EQUAL_OPERATOR => TokenKind::EqualOperator,
        GREATER_OPERATOR => TokenKind::GreaterOperator,
        LESS_OPERATOR => TokenKind::LessOperator,
        OPENING_SQUARE_BRACE => TokenKind::OpeningSquareBrace,
        OPENING_CURLY_BRACE => TokenKind::OpeningCurlyBrace,
        TILDA => TokenKind::Tilda,

        ABOUT => TokenKind::About,
        ADD => TokenKind::Add,
        ALIGN => TokenKind::Align,
        ANCHORED => TokenKind::Anchored,
        AS => TokenKind::As,
        AT => TokenKind::At,
        AXIS => TokenKind::Axis,
        BLOCK => TokenKind::Block,
        BLUE => TokenKind::Blue,
        BOLD => TokenKind::Bold,
        BOSSBAR => TokenKind::Bossbar,
        CAN_GRAB => TokenKind::CanGrab,
        CHEST => TokenKind::Chest,
        CLEAR => TokenKind::Clear,
        CLONE => TokenKind::Clone,
        COLOR => TokenKind::Color,
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
        ENT => TokenKind::Entity,
        EX => TokenKind::Execute,
        EYES => TokenKind::Eyes,
        FACING => TokenKind::Facing,
        FEET => TokenKind::Feet,
        FILL => TokenKind::Fill,
        FORCE => TokenKind::Force,
        FN => TokenKind::Function,
        GM => TokenKind::Gamemode,
        GAMERULE => TokenKind::Gamerule,
        GET => TokenKind::Get,
        GIVE => TokenKind::Give,
        GOLD => TokenKind::Gold,
        GRAY => TokenKind::Gray,
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
        LEVEL => TokenKind::Level,
        LIMIT => TokenKind::Limit,
        LIT => TokenKind::Lit,
        LORE => TokenKind::Lore,
        MAX => TokenKind::Max,
        MASKED => TokenKind::Masked,
        MODIFY => TokenKind::Modify,
        MOTION => TokenKind::Motion,
        MOVE => TokenKind::Move,
        NAME => TokenKind::Name,
        NATIVE => TokenKind::Native,
        NO_AI => TokenKind::NoAI,
        NORMAL => TokenKind::Normal,
        NORTH => TokenKind::North,
        OBJ => TokenKind::Objectives,
        OPEN => TokenKind::Open,
        OPR => TokenKind::Operation,
        PTC => TokenKind::Particle,
        PLAYERS => TokenKind::Players,
        PLS => TokenKind::Playsound,
        POS => TokenKind::Positioned,
        POTION => TokenKind::Potion,
        POWERED => TokenKind::Powered,
        RANGE => TokenKind::Range,
        RED => TokenKind::Red,
        REMOVE => TokenKind::Remove,
        REPLACE => TokenKind::Replace,
        RESET => TokenKind::Reset,
        ROTATION => TokenKind::Rotation,
        RUN => TokenKind::Run,
        SAY => TokenKind::Say,
        SCALE => TokenKind::Scale,
        SCB => TokenKind::Scoreboard,
        SCORE => TokenKind::Score,
        SET => TokenKind::Set,
        SETBLOCK => TokenKind::Setblock,
        SHINE => TokenKind::Shine,
        SIGN => TokenKind::Sign,
        SILENT => TokenKind::Silent,
        SIZE => TokenKind::Size,
        SORT => TokenKind::Sort,
        SOUTH => TokenKind::South,
        SPECTATE => TokenKind::Spectate,
        STACK => TokenKind::Stack,
        STORAGE => TokenKind::Storage,
        STORE => TokenKind::Store,
        SUBTITLE => TokenKind::Subtitle,
        SM => TokenKind::Summon,
        SURVIVAL => TokenKind::Survival,
        TAG => TokenKind::Tag,
        TEAM => TokenKind::Team,
        TELLRAW => TokenKind::Tellraw,
        TEXT => TokenKind::Text,
        TIME => TokenKind::Time,
        TITLE => TokenKind::Title,
        TO_COLOR => TokenKind::ToColor,
        TP => TokenKind::Tp,
        TYPE => TokenKind::Type,
        UNINITED => TokenKind::Uninited,
        UNLESS => TokenKind::Unless,
        WEST => TokenKind::West,
        WIDTH => TokenKind::Width,
        YELLOW => TokenKind::Yellow,

        ALL_PLAYER_SELECTOR => TokenKind::AllPlayerSelector,
        ALL_SELECTOR => TokenKind::AllSelector,
        CURRENT_SELECTOR => TokenKind::CurrentSelector,
        NEAREST_PLAYER_SELECTOR => TokenKind::NearestPlayerSelector,
        RANDOM_PLAYER_SELECTOR => TokenKind::RandomPlayerSelector,

        DIVIDE_EQUAL_OPERATOR => TokenKind::DivideEqualOperator,
        GREATER_OR_EQUAL_OPERATOR => TokenKind::GreaterOrEqualOperator,
        LESS_OR_EQUAL_OPERATOR => TokenKind::LessOrEqualOperator,
        MINUS_EQUAL_OPERATOR => TokenKind::MinusEqualOperator,
        MULT_EQUAL_OPERATOR => TokenKind::MultEqualOperator,
        PLUS_EQUAL_OPERATOR => TokenKind::PlusEqualOperator,

        _ => TokenKind::Identifier,
    }
}

pub fn long_token_kind(token_body: &[u8]) -> TokenKind {
    match LONG_TOKEN_TYPE.binary_search_by_key(&token_body, |item| item.str_token) {
        Ok(index) => LONG_TOKEN_TYPE[index].kind,
        Err(_) => TokenKind::Identifier,
    }
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

#[derive(Debug, PartialEq, Eq)]
struct StrTokenAndTokenKind<'a> {
    str_token: &'a [u8],
    kind: TokenKind,
}

impl<'a> StrTokenAndTokenKind<'a> {
    pub const fn new(str_token: &'a [u8], kind: TokenKind) -> Self {
        Self { str_token, kind }
    }
}

impl<'a> PartialOrd for StrTokenAndTokenKind<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for StrTokenAndTokenKind<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.str_token.cmp(other.str_token)
    }
}

const LONG_TOKEN_TYPE: [StrTokenAndTokenKind; 39] = [
    StrTokenAndTokenKind::new(b"advancement", TokenKind::Advancement),
    StrTokenAndTokenKind::new(b"adventure", TokenKind::Adventure),
    StrTokenAndTokenKind::new(b"attack_damage", TokenKind::AttackDamage),
    StrTokenAndTokenKind::new(b"attack_speed", TokenKind::AttackSpeed),
    StrTokenAndTokenKind::new(b"attribute", TokenKind::Attribute),
    StrTokenAndTokenKind::new(b"can_place_on", TokenKind::CanPlaceOn),
    StrTokenAndTokenKind::new(b"chest_chance", TokenKind::ChestChance),
    StrTokenAndTokenKind::new(b"dark_blue", TokenKind::DarkBlue),
    StrTokenAndTokenKind::new(b"dark_green", TokenKind::DarkGreen),
    StrTokenAndTokenKind::new(b"enchantments", TokenKind::Enchantments),
    StrTokenAndTokenKind::new(b"feet_chance", TokenKind::FeetChance),
    StrTokenAndTokenKind::new(b"from_color", TokenKind::FromColor),
    StrTokenAndTokenKind::new(b"head_chance", TokenKind::HeadChance),
    StrTokenAndTokenKind::new(b"hieroglyph", TokenKind::Hieroglyph),
    StrTokenAndTokenKind::new(b"hurt_time", TokenKind::HurtTime),
    StrTokenAndTokenKind::new(b"in_ground", TokenKind::InGround),
    StrTokenAndTokenKind::new(b"interaction", TokenKind::Interaction),
    StrTokenAndTokenKind::new(b"invisible", TokenKind::Invisible),
    StrTokenAndTokenKind::new(b"invulnerable", TokenKind::Invulnerable),
    StrTokenAndTokenKind::new(b"left_hand", TokenKind::LeftHand),
    StrTokenAndTokenKind::new(b"left_hand_chance", TokenKind::LeftHandChance),
    StrTokenAndTokenKind::new(b"legs_chance", TokenKind::LegsChance),
    StrTokenAndTokenKind::new(b"loot_table", TokenKind::LootTable),
    StrTokenAndTokenKind::new(b"name_visible", TokenKind::NameVisible),
    StrTokenAndTokenKind::new(b"no_despawn", TokenKind::NoDespawn),
    StrTokenAndTokenKind::new(b"no_gravity", TokenKind::NoGravity),
    StrTokenAndTokenKind::new(b"pickup_delay", TokenKind::PickupDelay),
    StrTokenAndTokenKind::new(b"potion_color", TokenKind::PotionColor),
    StrTokenAndTokenKind::new(b"right_hand", TokenKind::RightHand),
    StrTokenAndTokenKind::new(b"right_hand_chance", TokenKind::RightHandChance),
    StrTokenAndTokenKind::new(b"selected_item", TokenKind::SelectedItem),
    StrTokenAndTokenKind::new(b"spawnpoint", TokenKind::Spawnpoint),
    StrTokenAndTokenKind::new(b"spectator", TokenKind::Spectator),
    StrTokenAndTokenKind::new(b"stability", TokenKind::Stability),
    StrTokenAndTokenKind::new(b"stopsound", TokenKind::Stopsound),
    StrTokenAndTokenKind::new(b"teleport_duration", TokenKind::TeleportDuration),
    StrTokenAndTokenKind::new(b"unbreakable", TokenKind::Unbreakable),
    StrTokenAndTokenKind::new(b"x_rotation", TokenKind::XRotation),
    StrTokenAndTokenKind::new(b"y_rotation", TokenKind::YRotation),
];
