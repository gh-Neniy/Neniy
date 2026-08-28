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
    Align,
    Alt,
    Anchored,
    As,
    At,
    Attribute,
    Axis,
    Billboard,
    Block,
    Bold,
    Bossbar,
    CanBreak,
    CanPlaceOn,
    Caret,
    Chest,
    ChestChance,
    Clear,
    Clone,
    Color,
    Comma,
    Crit,
    Damage,
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
    Head,
    HeadChance,
    Health,
    Height,
    Hide,
    HurtTime,
    Id,
    If,
    InGround,
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
    Masked,
    Max,
    Modify,
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
    PotionColor,
    Powered,
    Profession,
    Ptc,
    Random,
    Range,
    Remove,
    Replace,
    Reset,
    RightHand,
    RightHandChance,
    Rotation,
    Run,
    Say,
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
    Stack,
    Stopsound,
    Storage,
    Store,
    String,
    Subtitle,
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

    #[sort_start] // attribute
    AttackDamage,
    AttackSpeed,
    MaxHealth,
    Stability,

    #[sort_start] // block
    AcaciaButton,
    Air,
    Chain,
    CoalOre,
    Cobblestone,
    CobblestoneWall,
    CrimsonButton,
    Dirt,
    Fire,
    IronBlock,
    Light,
    MagmaBlock,
    NetheriteBlock,
    Netherrack,
    OakButton,
    OakWallSign,
    SpruceButton,
    Stone,
    StoneButton,
    WallTorch,
    WarpedButton,
    WarpedNylium,
    Water,

    #[sort_start] // data field
    Scale,

    #[sort_start] // effect
    NightVision,

    #[sort_start] // enchantments
    Knockback,

    #[sort_start] // entity
    ArmorStand,
    BlockDisplay,
    FallingBlock,
    Interaction,
    ItemDisplay,
    MagmaCube,
    Marker,
    Phantom,
    PiglinBrute,
    Skeleton,
    Stray,
    TextDisplay,
    Villager,
    WanderingTrader,
    Zombie,

    #[sort_start] // game mode
    Adventure,
    Spectator,

    #[sort_start] // game rule
    NaturalRegeneration,

    #[sort_start] // item
    Arrow,
    BlazeRod,
    Bone,
    Book,
    Bow,
    Coal,
    CrimsonNylium,
    Crossbow,
    Egg,
    FireCharge,
    FlintAndSteel,
    IronBoots,
    IronNugget,
    IronPickaxe,
    IronSword,
    LeatherBoots,
    LeatherHelmet,
    NetherStar,
    NetheriteHoe,
    Potion,
    RawCopper,
    RawGold,
    RawIron,
    Shield,
    Snowball,
    SplashPotion,
    StonePickaxe,
    StoneSword,
    Trident,
    WoodenHoe,
    WoodenPickaxe,
    WoodenSword,

    #[sort_start] // particle
    Ash,
    CampfireCosySmoke,
    Cloud,
    DrippingLava,
    DustColorTransition,
    ElectricSpark,
    EndRod,
    FallingWater,
    Flame,
    GlowSquidInk,
    HappyVillager,
    Lava,
    ReversePortal,
    Scrape,
    Smoke,
    SoulFlame,

    #[sort_start] // sound
    EggThrow,
    StonePlace,

    #[sort_start] // time mode
    Night,
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

    pub fn new_empty() -> Self {
        Token {
            base: BaseToken::new_empty(),
            kind: TokenKind::Id,
            category: TokenCategory::Id,
        }
    }

    pub fn is_falling_block(&self) -> bool {
        self.kind == TokenKind::FallingBlock
    }

    pub fn is_wall(&self) -> bool {
        self.kind == TokenKind::CobblestoneWall
    }
}

pub fn short_token_kind(token_body: &[u8]) -> TokenKind {
    // token_body.len() <= 8

    sorted_consts!(
        const ABOUT: u64 = hash(b"about");
        const ADD: u64 = hash(b"add");
        const AIR: u64 = hash(b"air");
        const ALIGN: u64 = hash(b"align");
        const ALT: u64 = hash(b"alt");
        const ANCHORED: u64 = hash(b"anchored");
        const ARROW: u64 = hash(b"arrow");
        const AS: u64 = hash(b"as");
        const ASH: u64 = hash(b"ash");
        const AT: u64 = hash(b"at");
        const AXIS: u64 = hash(b"axis");
        const BLOCK: u64 = hash(b"block");
        const BOLD: u64 = hash(b"bold");
        const BONE: u64 = hash(b"bone");
        const BOOK: u64 = hash(b"book");
        const BOSSBAR: u64 = hash(b"bossbar");
        const BOW: u64 = hash(b"bow");
        const CARET: u64 = hash(b"^");
        const CHAIN: u64 = hash(b"chain");
        const CHEST: u64 = hash(b"chest");
        const CLEAR: u64 = hash(b"clear");
        const CLONE: u64 = hash(b"clone");
        const CLOSING_CURLY_BRACE: u64 = hash(b"}");
        const CLOSING_SQUARE_BRACE: u64 = hash(b"]");
        const CLOUD: u64 = hash(b"cloud");
        const COAL: u64 = hash(b"coal");
        const COAL_ORE: u64 = hash(b"coal_ore");
        const COLOR: u64 = hash(b"color");
        const COMMA: u64 = hash(b",");
        const CRIT: u64 = hash(b"crit");
        const CROSSBOW: u64 = hash(b"crossbow");
        const DAMAGE: u64 = hash(b"damage");
        const DATA: u64 = hash(b"data");
        const DESTROY: u64 = hash(b"destroy");
        const DIRT: u64 = hash(b"dirt");
        const DISTANCE: u64 = hash(b"distance");
        const DX: u64 = hash(b"dx");
        const DY: u64 = hash(b"dy");
        const DZ: u64 = hash(b"dz");
        const EAST: u64 = hash(b"east");
        const EFFECT: u64 = hash(b"effect");
        const EGG: u64 = hash(b"egg");
        const END_ROD: u64 = hash(b"end_rod");
        const ENT: u64 = hash(b"ent");
        const EQUAL_OPERATOR: u64 = hash(b"=");
        const EX: u64 = hash(b"ex");
        const EYES: u64 = hash(b"eyes");
        const FACING: u64 = hash(b"facing");
        const FEET: u64 = hash(b"feet");
        const FILL: u64 = hash(b"fill");
        const FIRE: u64 = hash(b"fire");
        const FLAME: u64 = hash(b"flame");
        const FN: u64 = hash(b"fn");
        const FORCE: u64 = hash(b"force");
        const GAMERULE: u64 = hash(b"gamerule");
        const GET: u64 = hash(b"get");
        const GIVE: u64 = hash(b"give");
        const GM: u64 = hash(b"gm");
        const GREATER_OPERATOR: u64 = hash(b">");
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
        const LAVA: u64 = hash(b"lava");
        const LEGS: u64 = hash(b"legs");
        const LESS_OPERATOR: u64 = hash(b"<");
        const LEVEL: u64 = hash(b"level");
        const LIGHT: u64 = hash(b"light");
        const LIMIT: u64 = hash(b"limit");
        const LIT: u64 = hash(b"lit");
        const LOOT: u64 = hash(b"loot");
        const LORE: u64 = hash(b"lore");
        const MARKER: u64 = hash(b"marker");
        const MASKED: u64 = hash(b"masked");
        const MAX: u64 = hash(b"max");
        const MODIFY: u64 = hash(b"modify");
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
        const PHANTOM: u64 = hash(b"phantom");
        const PLAYERS: u64 = hash(b"players");
        const PLS: u64 = hash(b"pls");
        const POS: u64 = hash(b"pos");
        const POTION: u64 = hash(b"potion");
        const POWERED: u64 = hash(b"powered");
        const PTC: u64 = hash(b"ptc");
        const RANDOM: u64 = hash(b"random");
        const RANGE: u64 = hash(b"..");
        const RAW_GOLD: u64 = hash(b"raw_gold");
        const RAW_IRON: u64 = hash(b"raw_iron");
        const REMOVE: u64 = hash(b"remove");
        const REPLACE: u64 = hash(b"replace");
        const RESET: u64 = hash(b"reset");
        const ROTATION: u64 = hash(b"rotation");
        const RUN: u64 = hash(b"run");
        const SAY: u64 = hash(b"say");
        const SCALE: u64 = hash(b"scale");
        const SCB: u64 = hash(b"scb");
        const SCORE: u64 = hash(b"score");
        const SCRAPE: u64 = hash(b"scrape");
        const SET: u64 = hash(b"set");
        const SETBLOCK: u64 = hash(b"setblock");
        const SHIELD: u64 = hash(b"shield");
        const SHINE: u64 = hash(b"shine");
        const SIGN: u64 = hash(b"sign");
        const SILENT: u64 = hash(b"silent");
        const SIZE: u64 = hash(b"size");
        const SKELETON: u64 = hash(b"skeleton");
        const SM: u64 = hash(b"sm");
        const SMOKE: u64 = hash(b"smoke");
        const SNOWBALL: u64 = hash(b"snowball");
        const SORT: u64 = hash(b"sort");
        const SOUTH: u64 = hash(b"south");
        const SPECTATE: u64 = hash(b"spectate");
        const STACK: u64 = hash(b"stack");
        const STONE: u64 = hash(b"stone");
        const STORAGE: u64 = hash(b"storage");
        const STORE: u64 = hash(b"store");
        const STRAY: u64 = hash(b"stray");
        const SUBTITLE: u64 = hash(b"subtitle");
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
        const TRIDENT: u64 = hash(b"trident");
        const TYPE: u64 = hash(b"type");
        const UNINITED: u64 = hash(b"uninited");
        const UNLESS: u64 = hash(b"unless");
        const VILLAGER: u64 = hash(b"villager");
        const WATER: u64 = hash(b"water");
        const WEST: u64 = hash(b"west");
        const WIDTH: u64 = hash(b"width");
        const ZOMBIE: u64 = hash(b"zombie");

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
        AIR => TokenKind::Air,
        ALIGN => TokenKind::Align,
        ALT => TokenKind::Alt,
        ANCHORED => TokenKind::Anchored,
        ARROW => TokenKind::Arrow,
        AS => TokenKind::As,
        ASH => TokenKind::Ash,
        AT => TokenKind::At,
        AXIS => TokenKind::Axis,
        BLOCK => TokenKind::Block,
        BOLD => TokenKind::Bold,
        BONE => TokenKind::Bone,
        BOOK => TokenKind::Book,
        BOSSBAR => TokenKind::Bossbar,
        BOW => TokenKind::Bow,
        CARET => TokenKind::Caret,
        CHAIN => TokenKind::Chain,
        CHEST => TokenKind::Chest,
        CLEAR => TokenKind::Clear,
        CLONE => TokenKind::Clone,
        CLOSING_CURLY_BRACE => TokenKind::ClosingCurlyBrace,
        CLOSING_SQUARE_BRACE => TokenKind::ClosingSquareBrace,
        CLOUD => TokenKind::Cloud,
        COAL => TokenKind::Coal,
        COAL_ORE => TokenKind::CoalOre,
        COLOR => TokenKind::Color,
        COMMA => TokenKind::Comma,
        CRIT => TokenKind::Crit,
        CROSSBOW => TokenKind::Crossbow,
        DAMAGE => TokenKind::Damage,
        DATA => TokenKind::Data,
        DESTROY => TokenKind::Destroy,
        DIRT => TokenKind::Dirt,
        DISTANCE => TokenKind::Distance,
        DX => TokenKind::Dx,
        DY => TokenKind::Dy,
        DZ => TokenKind::Dz,
        EAST => TokenKind::East,
        EFFECT => TokenKind::Effect,
        EGG => TokenKind::Egg,
        END_ROD => TokenKind::EndRod,
        ENT => TokenKind::Ent,
        EQUAL_OPERATOR => TokenKind::EqualOperator,
        EX => TokenKind::Ex,
        EYES => TokenKind::Eyes,
        FACING => TokenKind::Facing,
        FEET => TokenKind::Feet,
        FILL => TokenKind::Fill,
        FIRE => TokenKind::Fire,
        FLAME => TokenKind::Flame,
        FN => TokenKind::Fn,
        FORCE => TokenKind::Force,
        GAMERULE => TokenKind::Gamerule,
        GET => TokenKind::Get,
        GIVE => TokenKind::Give,
        GM => TokenKind::Gm,
        GREATER_OPERATOR => TokenKind::GreaterOperator,
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
        LAVA => TokenKind::Lava,
        LEGS => TokenKind::Legs,
        LESS_OPERATOR => TokenKind::LessOperator,
        LEVEL => TokenKind::Level,
        LIGHT => TokenKind::Light,
        LIMIT => TokenKind::Limit,
        LIT => TokenKind::Lit,
        LOOT => TokenKind::Loot,
        LORE => TokenKind::Lore,
        MARKER => TokenKind::Marker,
        MASKED => TokenKind::Masked,
        MAX => TokenKind::Max,
        MODIFY => TokenKind::Modify,
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
        PHANTOM => TokenKind::Phantom,
        PLAYERS => TokenKind::Players,
        PLS => TokenKind::Pls,
        POS => TokenKind::Pos,
        POTION => TokenKind::Potion,
        POWERED => TokenKind::Powered,
        PTC => TokenKind::Ptc,
        RANDOM => TokenKind::Random,
        RANGE => TokenKind::Range,
        RAW_GOLD => TokenKind::RawGold,
        RAW_IRON => TokenKind::RawIron,
        REMOVE => TokenKind::Remove,
        REPLACE => TokenKind::Replace,
        RESET => TokenKind::Reset,
        ROTATION => TokenKind::Rotation,
        RUN => TokenKind::Run,
        SAY => TokenKind::Say,
        SCALE => TokenKind::Scale,
        SCB => TokenKind::Scb,
        SCORE => TokenKind::Score,
        SCRAPE => TokenKind::Scrape,
        SET => TokenKind::Set,
        SETBLOCK => TokenKind::Setblock,
        SHIELD => TokenKind::Shield,
        SHINE => TokenKind::Shine,
        SIGN => TokenKind::Sign,
        SILENT => TokenKind::Silent,
        SIZE => TokenKind::Size,
        SKELETON => TokenKind::Skeleton,
        SM => TokenKind::Sm,
        SMOKE => TokenKind::Smoke,
        SNOWBALL => TokenKind::Snowball,
        SORT => TokenKind::Sort,
        SOUTH => TokenKind::South,
        SPECTATE => TokenKind::Spectate,
        STACK => TokenKind::Stack,
        STONE => TokenKind::Stone,
        STORAGE => TokenKind::Storage,
        STORE => TokenKind::Store,
        STRAY => TokenKind::Stray,
        SUBTITLE => TokenKind::Subtitle,
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
        TRIDENT => TokenKind::Trident,
        TYPE => TokenKind::Type,
        UNINITED => TokenKind::Uninited,
        UNLESS => TokenKind::Unless,
        VILLAGER => TokenKind::Villager,
        WATER => TokenKind::Water,
        WEST => TokenKind::West,
        WIDTH => TokenKind::Width,
        ZOMBIE => TokenKind::Zombie,

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
        b"acacia_button" => TokenKind::AcaciaButton,
        b"advancement" => TokenKind::Advancement,
        b"adventure" => TokenKind::Adventure,
        b"armor_stand" => TokenKind::ArmorStand,
        b"attack_damage" => TokenKind::AttackDamage,
        b"attack_speed" => TokenKind::AttackSpeed,
        b"attribute" => TokenKind::Attribute,
        b"billboard" => TokenKind::Billboard,
        b"blaze_rod" => TokenKind::BlazeRod,
        b"block_display" => TokenKind::BlockDisplay,
        b"campfire_cosy_smoke" => TokenKind::CampfireCosySmoke,
        b"can_break" => TokenKind::CanBreak,
        b"can_place_on" => TokenKind::CanPlaceOn,
        b"chest_chance" => TokenKind::ChestChance,
        b"cobblestone" => TokenKind::Cobblestone,
        b"cobblestone_wall" => TokenKind::CobblestoneWall,
        b"crimson_button" => TokenKind::CrimsonButton,
        b"crimson_nylium" => TokenKind::CrimsonNylium,
        b"dripping_lava" => TokenKind::DrippingLava,
        b"dust_color_transition" => TokenKind::DustColorTransition,
        b"egg_throw" => TokenKind::EggThrow,
        b"electric_spark" => TokenKind::ElectricSpark,
        b"enchantments" => TokenKind::Enchantments,
        b"falling_block" => TokenKind::FallingBlock,
        b"falling_water" => TokenKind::FallingWater,
        b"feet_chance" => TokenKind::FeetChance,
        b"fire_charge" => TokenKind::FireCharge,
        b"flint_and_steel" => TokenKind::FlintAndSteel,
        b"from_color" => TokenKind::FromColor,
        b"glow_squid_ink" => TokenKind::GlowSquidInk,
        b"happy_villager" => TokenKind::HappyVillager,
        b"head_chance" => TokenKind::HeadChance,
        b"hurt_time" => TokenKind::HurtTime,
        b"in_ground" => TokenKind::InGround,
        b"interaction" => TokenKind::Interaction,
        b"invisible" => TokenKind::Invisible,
        b"invulnerable" => TokenKind::Invulnerable,
        b"iron_block" => TokenKind::IronBlock,
        b"iron_boots" => TokenKind::IronBoots,
        b"iron_nugget" => TokenKind::IronNugget,
        b"iron_pickaxe" => TokenKind::IronPickaxe,
        b"iron_sword" => TokenKind::IronSword,
        b"item_display" => TokenKind::ItemDisplay,
        b"leather_boots" => TokenKind::LeatherBoots,
        b"leather_helmet" => TokenKind::LeatherHelmet,
        b"left_hand" => TokenKind::LeftHand,
        b"left_hand_chance" => TokenKind::LeftHandChance,
        b"legs_chance" => TokenKind::LegsChance,
        b"loot_table" => TokenKind::LootTable,
        b"magma_block" => TokenKind::MagmaBlock,
        b"magma_cube" => TokenKind::MagmaCube,
        b"name_visible" => TokenKind::NameVisible,
        b"nether_star" => TokenKind::NetherStar,
        b"netherite_block" => TokenKind::NetheriteBlock,
        b"netherite_hoe" => TokenKind::NetheriteHoe,
        b"netherrack" => TokenKind::Netherrack,
        b"no_despawn" => TokenKind::NoDespawn,
        b"no_gravity" => TokenKind::NoGravity,
        b"oak_button" => TokenKind::OakButton,
        b"oak_wall_sign" => TokenKind::OakWallSign,
        b"passenger" => TokenKind::Passenger,
        b"pickup_delay" => TokenKind::PickupDelay,
        b"piglin_brute" => TokenKind::PiglinBrute,
        b"potion_color" => TokenKind::PotionColor,
        b"profession" => TokenKind::Profession,
        b"raw_copper" => TokenKind::RawCopper,
        b"reverse_portal" => TokenKind::ReversePortal,
        b"right_hand" => TokenKind::RightHand,
        b"right_hand_chance" => TokenKind::RightHandChance,
        b"selected_item" => TokenKind::SelectedItem,
        b"soul_flame" => TokenKind::SoulFlame,
        b"spawnpoint" => TokenKind::Spawnpoint,
        b"spectator" => TokenKind::Spectator,
        b"splash_potion" => TokenKind::SplashPotion,
        b"spruce_button" => TokenKind::SpruceButton,
        b"stability" => TokenKind::Stability,
        b"stone_button" => TokenKind::StoneButton,
        b"stone_pickaxe" => TokenKind::StonePickaxe,
        b"stone_place" => TokenKind::StonePlace,
        b"stone_sword" => TokenKind::StoneSword,
        b"stopsound" => TokenKind::Stopsound,
        b"text_display" => TokenKind::TextDisplay,
        b"unbreakable" => TokenKind::Unbreakable,
        b"wall_torch" => TokenKind::WallTorch,
        b"wandering_trader" => TokenKind::WanderingTrader,
        b"warped_button" => TokenKind::WarpedButton,
        b"warped_nylium" => TokenKind::WarpedNylium,
        b"wooden_hoe" => TokenKind::WoodenHoe,
        b"wooden_pickaxe" => TokenKind::WoodenPickaxe,
        b"wooden_sword" => TokenKind::WoodenSword,

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
