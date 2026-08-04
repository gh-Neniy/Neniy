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

pub type IndexType = u16;

#[derive(Debug, PartialEq, Eq)]
pub struct BaseToken {
    start: IndexType,
    end: IndexType,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    base: BaseToken,
    kind: TokenKind,
    category: TokenCategory,
}

pub fn short_token_type(token_body: &str) -> TokenKind {
    // token_body.len() <= 8

    const CARET: u64 = hash("^");
    const CLOSING_SQUARE_BRACE: u64 = hash("]");
    const CLOSING_CURLY_BRACE: u64 = hash("}");
    const COMMA: u64 = hash(",");
    const EQUAL_OPERATOR: u64 = hash("=");
    const GREATER_OPERATOR: u64 = hash(">");
    const LESS_OPERATOR: u64 = hash("<");
    const OPENING_SQUARE_BRACE: u64 = hash("[");
    const OPENING_CURLY_BRACE: u64 = hash("{");
    const TILDA: u64 = hash("~");

    const ABOUT: u64 = hash("about");
    const ADD: u64 = hash("add");
    const ALIGN: u64 = hash("align");
    const ANCHORED: u64 = hash("anchored");
    const AS: u64 = hash("as");
    const AT: u64 = hash("at");
    const AXIS: u64 = hash("axis");
    const BLOCK: u64 = hash("block");
    const BLUE: u64 = hash("blue");
    const BOLD: u64 = hash("bold");
    const BOSSBAR: u64 = hash("bossbar");
    const CAN_GRAB: u64 = hash("can_grab");
    const CHEST: u64 = hash("chest");
    const CLEAR: u64 = hash("clear");
    const CLONE: u64 = hash("clone");
    const COLOR: u64 = hash("color");
    const CREATIVE: u64 = hash("creative");
    const CRIT: u64 = hash("crit");
    const DAMAGE: u64 = hash("damage");
    const DARK_RED: u64 = hash("dark_red");
    const DATA: u64 = hash("data");
    const DESTROY: u64 = hash("destroy");
    const DISTANCE: u64 = hash("distance");
    const DX: u64 = hash("dx");
    const DY: u64 = hash("dy");
    const DZ: u64 = hash("dz");
    const EAST: u64 = hash("east");
    const EFFECT: u64 = hash("effect");
    const ENT: u64 = hash("ent");
    const EX: u64 = hash("ex");
    const EYES: u64 = hash("eyes");
    const FACING: u64 = hash("facing");
    const FEET: u64 = hash("feet");
    const FILL: u64 = hash("fill");
    const FORCE: u64 = hash("force");
    const FN: u64 = hash("fn");
    const GM: u64 = hash("gm");
    const GAMERULE: u64 = hash("gamerule");
    const GET: u64 = hash("get");
    const GIVE: u64 = hash("give");
    const GOLD: u64 = hash("gold");
    const GRAY: u64 = hash("gray");
    const GREEN: u64 = hash("green");
    const HALF: u64 = hash("half");
    const HEAD: u64 = hash("head");
    const HEALTH: u64 = hash("health");
    const HEIGHT: u64 = hash("height");
    const HIDE: u64 = hash("hide");
    const ID: u64 = hash("id");
    const IF: u64 = hash("if");
    const ITALIC: u64 = hash("italic");
    const ITEM: u64 = hash("item");
    const ITEMS: u64 = hash("items");
    const JOIN: u64 = hash("join");
    const KEEP: u64 = hash("keep");
    const KILL: u64 = hash("kill");
    const LEGS: u64 = hash("legs");
    const LEVEL: u64 = hash("level");
    const LIMIT: u64 = hash("limit");
    const LIT: u64 = hash("lit");
    const LORE: u64 = hash("lore");
    const MAX: u64 = hash("max");
    const MASKED: u64 = hash("masked");
    const MODIFY: u64 = hash("modify");
    const MOTION: u64 = hash("motion");
    const MOVE: u64 = hash("move");
    const NAME: u64 = hash("name");
    const NATIVE: u64 = hash("native");
    const NO_AI: u64 = hash("no_ai");
    const NORMAL: u64 = hash("normal");
    const NORTH: u64 = hash("north");
    const OBJ: u64 = hash("obj");
    const OPEN: u64 = hash("open");
    const OPR: u64 = hash("opr");
    const PTC: u64 = hash("ptc");
    const PLAYERS: u64 = hash("players");
    const PLS: u64 = hash("pls");
    const POS: u64 = hash("pos");
    const POTION: u64 = hash("potion");
    const POWERED: u64 = hash("powered");
    const RANGE: u64 = hash("..");
    const RED: u64 = hash("red");
    const REMOVE: u64 = hash("remove");
    const REPLACE: u64 = hash("replace");
    const RESET: u64 = hash("reset");
    const ROTATION: u64 = hash("rotation");
    const RUN: u64 = hash("run");
    const SAY: u64 = hash("say");
    const SCALE: u64 = hash("scale");
    const SCB: u64 = hash("scb");
    const SCORE: u64 = hash("score");
    const SET: u64 = hash("set");
    const SETBLOCK: u64 = hash("setblock");
    const SHINE: u64 = hash("shine");
    const SIGN: u64 = hash("sign");
    const SILENT: u64 = hash("silent");
    const SIZE: u64 = hash("size");
    const SORT: u64 = hash("sort");
    const SOUTH: u64 = hash("south");
    const SPECTATE: u64 = hash("spectate");
    const STACK: u64 = hash("stack");
    const STORAGE: u64 = hash("storage");
    const STORE: u64 = hash("store");
    const SUBTITLE: u64 = hash("subtitle");
    const SM: u64 = hash("sm");
    const SURVIVAL: u64 = hash("survival");
    const TAG: u64 = hash("tag");
    const TEAM: u64 = hash("team");
    const TELLRAW: u64 = hash("tellraw");
    const TEXT: u64 = hash("text");
    const TIME: u64 = hash("time");
    const TITLE: u64 = hash("title");
    const TO_COLOR: u64 = hash("to_color");
    const TP: u64 = hash("tp");
    const TYPE: u64 = hash("type");
    const UNINITED: u64 = hash("uninited");
    const UNLESS: u64 = hash("unless");
    const WEST: u64 = hash("west");
    const WIDTH: u64 = hash("width");
    const YELLOW: u64 = hash("yellow");

    const ALL_PLAYER_SELECTOR: u64 = hash("@a");
    const ALL_SELECTOR: u64 = hash("@e");
    const CURRENT_SELECTOR: u64 = hash("@s");
    const NEAREST_PLAYER_SELECTOR: u64 = hash("@p");
    const RANDOM_PLAYER_SELECTOR: u64 = hash("@r");

    const DIVIDE_EQUAL_OPERATOR: u64 = hash("/=");
    const GREATER_OR_EQUAL_OPERATOR: u64 = hash(">=");
    const LESS_OR_EQUAL_OPERATOR: u64 = hash("<=");
    const MINUS_EQUAL_OPERATOR: u64 = hash("-=");
    const MULT_EQUAL_OPERATOR: u64 = hash("*=");
    const PLUS_EQUAL_OPERATOR: u64 = hash("+=");

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

pub fn long_token_type(token_body: &str) -> TokenKind {
    match LONG_TOKEN_TYPE.binary_search_by_key(&token_body, |item| item.str_token) {
        Ok(index) => LONG_TOKEN_TYPE[index].kind,
        Err(_) => TokenKind::Identifier,
    }
}

const fn hash(token_body: &str) -> u64 {
    let bytes = token_body.as_bytes();
    let mut arr = [0u8; 8];

    let mut i = 0;

    while i < bytes.len() {
        arr[i] = bytes[i];
        i += 1;
    }

    u64::from_ne_bytes(arr)
}

#[derive(Debug, PartialEq, Eq)]
struct StrTokenAndTokenKind<'a> {
    str_token: &'a str,
    kind: TokenKind,
}

impl<'a> StrTokenAndTokenKind<'a> {
    pub const fn new(str_token: &'a str, kind: TokenKind) -> Self {
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
        self.str_token.cmp(&other.str_token)
    }
}

const LONG_TOKEN_TYPE: [StrTokenAndTokenKind; 39] = [
    StrTokenAndTokenKind::new("advancement", TokenKind::Advancement),
    StrTokenAndTokenKind::new("adventure", TokenKind::Adventure),
    StrTokenAndTokenKind::new("attack_damage", TokenKind::AttackDamage),
    StrTokenAndTokenKind::new("attack_speed", TokenKind::AttackSpeed),
    StrTokenAndTokenKind::new("attribute", TokenKind::Attribute),
    StrTokenAndTokenKind::new("can_place_on", TokenKind::CanPlaceOn),
    StrTokenAndTokenKind::new("chest_chance", TokenKind::ChestChance),
    StrTokenAndTokenKind::new("dark_blue", TokenKind::DarkBlue),
    StrTokenAndTokenKind::new("dark_green", TokenKind::DarkGreen),
    StrTokenAndTokenKind::new("enchantments", TokenKind::Enchantments),
    StrTokenAndTokenKind::new("feet_chance", TokenKind::FeetChance),
    StrTokenAndTokenKind::new("from_color", TokenKind::FromColor),
    StrTokenAndTokenKind::new("head_chance", TokenKind::HeadChance),
    StrTokenAndTokenKind::new("hieroglyph", TokenKind::Hieroglyph),
    StrTokenAndTokenKind::new("hurt_time", TokenKind::HurtTime),
    StrTokenAndTokenKind::new("in_ground", TokenKind::InGround),
    StrTokenAndTokenKind::new("interaction", TokenKind::Interaction),
    StrTokenAndTokenKind::new("invisible", TokenKind::Invisible),
    StrTokenAndTokenKind::new("invulnerable", TokenKind::Invulnerable),
    StrTokenAndTokenKind::new("left_hand", TokenKind::LeftHand),
    StrTokenAndTokenKind::new("left_hand_chance", TokenKind::LeftHandChance),
    StrTokenAndTokenKind::new("legs_chance", TokenKind::LegsChance),
    StrTokenAndTokenKind::new("loot_table", TokenKind::LootTable),
    StrTokenAndTokenKind::new("name_visible", TokenKind::NameVisible),
    StrTokenAndTokenKind::new("no_despawn", TokenKind::NoDespawn),
    StrTokenAndTokenKind::new("no_gravity", TokenKind::NoGravity),
    StrTokenAndTokenKind::new("pickup_delay", TokenKind::PickupDelay),
    StrTokenAndTokenKind::new("potion_color", TokenKind::PotionColor),
    StrTokenAndTokenKind::new("right_hand", TokenKind::RightHand),
    StrTokenAndTokenKind::new("right_hand_chance", TokenKind::RightHandChance),
    StrTokenAndTokenKind::new("selected_item", TokenKind::SelectedItem),
    StrTokenAndTokenKind::new("spawnpoint", TokenKind::Spawnpoint),
    StrTokenAndTokenKind::new("spectator", TokenKind::Spectator),
    StrTokenAndTokenKind::new("stability", TokenKind::Stability),
    StrTokenAndTokenKind::new("stopsound", TokenKind::Stopsound),
    StrTokenAndTokenKind::new("teleport_duration", TokenKind::TeleportDuration),
    StrTokenAndTokenKind::new("unbreakable", TokenKind::Unbreakable),
    StrTokenAndTokenKind::new("x_rotation", TokenKind::XRotation),
    StrTokenAndTokenKind::new("y_rotation", TokenKind::YRotation),
];
