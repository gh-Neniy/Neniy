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
    Barrier,
    BoneBlock,
    Chain,
    CoalOre,
    Cobblestone,
    CobblestoneWall,
    CopperOre,
    CrimsonButton,
    Dirt,
    Fire,
    GoldOre,
    Ice,
    IronBlock,
    IronOre,
    Light,
    MagmaBlock,
    MushroomStem,
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
    WhiteStainedGlass,

    #[sort_start] // data field
    Scale,

    #[sort_start] // effect
    NightVision,

    #[sort_start] // enchantments
    Knockback,
    Looting,
    Power,
    Protection,
    QuickCharge,
    Sharpness,

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
    GoldIngot,
    IronBoots,
    IronChestplate,
    IronLeggings,
    IronNugget,
    IronPickaxe,
    IronSword,
    LeatherBoots,
    LeatherChestplate,
    LeatherHelmet,
    LeatherLeggings,
    NetherStar,
    NetheriteHoe,
    Potion,
    RawCopper,
    RawGold,
    RawIron,
    RottenFlesh,
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
    Enchant,
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
    AmethystBlockPlace,
    AmethystBlockStep,
    AncientDebrisBreak,
    ArrowHit,
    AxeScrape,
    BasaltDeltasMood,
    BeaconPowerSelect,
    BellResonate,
    BellUse,
    BlazeShoot,
    BucketEmptyLava,
    CandleExtinguish,
    Cave,
    ChestOpen,
    CrimsonForestLoop,
    CrossbowLoadingEnd,
    EggThrow,
    EvokerPrepareSummon,
    ExperienceOrbPickup,
    FireExtinguish,
    FireworkRocketBlast,
    FireworkRocketLargeBlast,
    FireworkRocketLaunch,
    GenericExplode,
    GlassBreak,
    GravelBreak,
    HuskConvertedToZombie,
    LavaExtinguish,
    LightningBoltThunder,
    MinecartRiding,
    NoteBlockXylophone,
    PiglinBruteAmbient,
    PiglinBruteAngry,
    PlayerAttackCrit,
    PlayerLevelup,
    RespawnAnchorCharge,
    RespawnAnchorDeplete,
    SkeletonAmbient,
    SnowballThrow,
    SoulSandStep,
    SoulSandValleyMood,
    StoneButtonClickOn,
    StonePlace,
    TuffBreak,
    VillagerTrade,
    WanderingTraderAmbient,
    WanderingTraderYes,
    WarpedForestMood,
    WitherSkeletonStep,
    WitherSpawn,
    WoodPlace,

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
        const BARRIER: u64 = hash(b"barrier");
        const BELL_USE: u64 = hash(b"bell.use");
        const BLOCK: u64 = hash(b"block");
        const BOLD: u64 = hash(b"bold");
        const BONE: u64 = hash(b"bone");
        const BOOK: u64 = hash(b"book");
        const BOSSBAR: u64 = hash(b"bossbar");
        const BOW: u64 = hash(b"bow");
        const CARET: u64 = hash(b"^");
        const CAVE: u64 = hash(b"cave");
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
        const ENCHANT: u64 = hash(b"enchant");
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
        const GOLD_ORE: u64 = hash(b"gold_ore");
        const GREATER_OPERATOR: u64 = hash(b">");
        const HEAD: u64 = hash(b"head");
        const HEALTH: u64 = hash(b"health");
        const HEIGHT: u64 = hash(b"height");
        const HIDE: u64 = hash(b"hide");
        const ICE: u64 = hash(b"ice");
        const ID: u64 = hash(b"id");
        const IF: u64 = hash(b"if");
        const IRON_ORE: u64 = hash(b"iron_ore");
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
        const LOOTING: u64 = hash(b"looting");
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
        const POWER: u64 = hash(b"power");
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

    use TokenKind::*;

    sorted_match!(match hash(token_body) {
        ABOUT => About,
        ADD => Add,
        AIR => Air,
        ALIGN => Align,
        ALT => Alt,
        ANCHORED => Anchored,
        ARROW => Arrow,
        AS => As,
        ASH => Ash,
        AT => At,
        AXIS => Axis,
        BARRIER => Barrier,
        BELL_USE => BellUse,
        BLOCK => Block,
        BOLD => Bold,
        BONE => Bone,
        BOOK => Book,
        BOSSBAR => Bossbar,
        BOW => Bow,
        CARET => Caret,
        CAVE => Cave,
        CHAIN => Chain,
        CHEST => Chest,
        CLEAR => Clear,
        CLONE => Clone,
        CLOSING_CURLY_BRACE => ClosingCurlyBrace,
        CLOSING_SQUARE_BRACE => ClosingSquareBrace,
        CLOUD => Cloud,
        COAL => Coal,
        COAL_ORE => CoalOre,
        COLOR => Color,
        COMMA => Comma,
        CRIT => Crit,
        CROSSBOW => Crossbow,
        DAMAGE => Damage,
        DATA => Data,
        DESTROY => Destroy,
        DIRT => Dirt,
        DISTANCE => Distance,
        DX => Dx,
        DY => Dy,
        DZ => Dz,
        EAST => East,
        EFFECT => Effect,
        EGG => Egg,
        ENCHANT => Enchant,
        END_ROD => EndRod,
        ENT => Ent,
        EQUAL_OPERATOR => EqualOperator,
        EX => Ex,
        EYES => Eyes,
        FACING => Facing,
        FEET => Feet,
        FILL => Fill,
        FIRE => Fire,
        FLAME => Flame,
        FN => Fn,
        FORCE => Force,
        GAMERULE => Gamerule,
        GET => Get,
        GIVE => Give,
        GM => Gm,
        GOLD_ORE => GoldOre,
        GREATER_OPERATOR => GreaterOperator,
        HEAD => Head,
        HEALTH => Health,
        HEIGHT => Height,
        HIDE => Hide,
        ICE => Ice,
        ID => Id,
        IF => If,
        IRON_ORE => IronOre,
        ITALIC => Italic,
        ITEM => Item,
        ITEMS => Items,
        JOIN => Join,
        KEEP => Keep,
        KILL => Kill,
        LAVA => Lava,
        LEGS => Legs,
        LESS_OPERATOR => LessOperator,
        LEVEL => Level,
        LIGHT => Light,
        LIMIT => Limit,
        LIT => Lit,
        LOOT => Loot,
        LOOTING => Looting,
        LORE => Lore,
        MARKER => Marker,
        MASKED => Masked,
        MAX => Max,
        MODIFY => Modify,
        MOVE => Move,
        NAME => Name,
        NATIVE => Native,
        NORMAL => Normal,
        NORTH => North,
        NO_AI => NoAI,
        NO_TRADE => NoTrade,
        OBJ => Obj,
        OPEN => Open,
        OPENING_CURLY_BRACE => OpeningCurlyBrace,
        OPENING_SQUARE_BRACE => OpeningSquareBrace,
        OPR => Opr,
        PHANTOM => Phantom,
        PLAYERS => Players,
        PLS => Pls,
        POS => Pos,
        POTION => Potion,
        POWER => Power,
        POWERED => Powered,
        PTC => Ptc,
        RANDOM => Random,
        RANGE => Range,
        RAW_GOLD => RawGold,
        RAW_IRON => RawIron,
        REMOVE => Remove,
        REPLACE => Replace,
        RESET => Reset,
        ROTATION => Rotation,
        RUN => Run,
        SAY => Say,
        SCALE => Scale,
        SCB => Scb,
        SCORE => Score,
        SCRAPE => Scrape,
        SET => Set,
        SETBLOCK => Setblock,
        SHIELD => Shield,
        SHINE => Shine,
        SIGN => Sign,
        SILENT => Silent,
        SIZE => Size,
        SKELETON => Skeleton,
        SM => Sm,
        SMOKE => Smoke,
        SNOWBALL => Snowball,
        SORT => Sort,
        SOUTH => South,
        SPECTATE => Spectate,
        STACK => Stack,
        STONE => Stone,
        STORAGE => Storage,
        STORE => Store,
        STRAY => Stray,
        SUBTITLE => Subtitle,
        TAG => Tag,
        TEAM => Team,
        TELLRAW => Tellraw,
        TEXT => Text,
        TILDA => Tilda,
        TIME => Time,
        TITLE => Title,
        TO_COLOR => ToColor,
        TP => Tp,
        TP_TIME => TpTime,
        TRIDENT => Trident,
        TYPE => Type,
        UNINITED => Uninited,
        UNLESS => Unless,
        VILLAGER => Villager,
        WATER => Water,
        WEST => West,
        WIDTH => Width,
        ZOMBIE => Zombie,

        #[sort_start]
        ALL_PLAYER_SELECTOR => AllPlayerSelector,
        ALL_SELECTOR => AllSelector,
        CURRENT_SELECTOR => CurrentSelector,
        NEAREST_PLAYER_SELECTOR => NearestPlayerSelector,
        RANDOM_PLAYER_SELECTOR => RandomPlayerSelector,

        #[sort_start]
        DIVIDE_EQUAL_OPERATOR => DivideEqualOperator,
        GREATER_OR_EQUAL_OPERATOR => GreaterOrEqualOperator,
        LESS_OR_EQUAL_OPERATOR => LessOrEqualOperator,
        MINUS_EQUAL_OPERATOR => MinusEqualOperator,
        MULT_EQUAL_OPERATOR => MultEqualOperator,
        PLUS_EQUAL_OPERATOR => PlusEqualOperator,

        _ => Id,
    })
}

pub fn long_token_kind(token_body: &[u8]) -> TokenKind {
    use TokenKind::*;

    sorted_match!(match token_body {
        b"acacia_button" => AcaciaButton,
        b"advancement" => Advancement,
        b"adventure" => Adventure,
        b"amethyst_block.place" => AmethystBlockPlace,
        b"amethyst_block.step" => AmethystBlockStep,
        b"ancient_debris.break" => AncientDebrisBreak,
        b"armor_stand" => ArmorStand,
        b"arrow.hit" => ArrowHit,
        b"attack_damage" => AttackDamage,
        b"attack_speed" => AttackSpeed,
        b"attribute" => Attribute,
        b"axe.scrape" => AxeScrape,
        b"basalt_deltas.mood" => BasaltDeltasMood,
        b"beacon.power_select" => BeaconPowerSelect,
        b"bell.resonate" => BellResonate,
        b"billboard" => Billboard,
        b"blaze.shoot" => BlazeShoot,
        b"blaze_rod" => BlazeRod,
        b"block_display" => BlockDisplay,
        b"bone_block" => BoneBlock,
        b"bucket.empty_lava" => BucketEmptyLava,
        b"campfire_cosy_smoke" => CampfireCosySmoke,
        b"can_break" => CanBreak,
        b"can_place_on" => CanPlaceOn,
        b"candle.extinguish" => CandleExtinguish,
        b"chest.open" => ChestOpen,
        b"chest_chance" => ChestChance,
        b"cobblestone" => Cobblestone,
        b"cobblestone_wall" => CobblestoneWall,
        b"copper_ore" => CopperOre,
        b"crimson_button" => CrimsonButton,
        b"crimson_forest.loop" => CrimsonForestLoop,
        b"crimson_nylium" => CrimsonNylium,
        b"crossbow.loading_end" => CrossbowLoadingEnd,
        b"dripping_lava" => DrippingLava,
        b"dust_color_transition" => DustColorTransition,
        b"egg.throw" => EggThrow,
        b"electric_spark" => ElectricSpark,
        b"enchantments" => Enchantments,
        b"evoker.prepare_summon" => EvokerPrepareSummon,
        b"experience_orb.pickup" => ExperienceOrbPickup,
        b"falling_block" => FallingBlock,
        b"falling_water" => FallingWater,
        b"feet_chance" => FeetChance,
        b"fire.extinguish" => FireExtinguish,
        b"fire_charge" => FireCharge,
        b"firework_rocket.blast" => FireworkRocketBlast,
        b"firework_rocket.large_blast" => FireworkRocketLargeBlast,
        b"firework_rocket.launch" => FireworkRocketLaunch,
        b"flint_and_steel" => FlintAndSteel,
        b"from_color" => FromColor,
        b"generic.explode" => GenericExplode,
        b"glass.break" => GlassBreak,
        b"glow_squid_ink" => GlowSquidInk,
        b"gold_ingot" => GoldIngot,
        b"gravel.break" => GravelBreak,
        b"happy_villager" => HappyVillager,
        b"head_chance" => HeadChance,
        b"hurt_time" => HurtTime,
        b"husk.converted_to_zombie" => HuskConvertedToZombie,
        b"in_ground" => InGround,
        b"interaction" => Interaction,
        b"invisible" => Invisible,
        b"invulnerable" => Invulnerable,
        b"iron_block" => IronBlock,
        b"iron_boots" => IronBoots,
        b"iron_chestplate" => IronChestplate,
        b"iron_leggings" => IronLeggings,
        b"iron_nugget" => IronNugget,
        b"iron_pickaxe" => IronPickaxe,
        b"iron_sword" => IronSword,
        b"item_display" => ItemDisplay,
        b"knockback" => Knockback,
        b"lava.extinguish" => LavaExtinguish,
        b"leather_boots" => LeatherBoots,
        b"leather_chestplate" => LeatherChestplate,
        b"leather_helmet" => LeatherHelmet,
        b"leather_leggings" => LeatherLeggings,
        b"left_hand" => LeftHand,
        b"left_hand_chance" => LeftHandChance,
        b"legs_chance" => LegsChance,
        b"lightning_bolt.thunder" => LightningBoltThunder,
        b"loot_table" => LootTable,
        b"magma_block" => MagmaBlock,
        b"magma_cube" => MagmaCube,
        b"minecart.riding" => MinecartRiding,
        b"mushroom_stem" => MushroomStem,
        b"name_visible" => NameVisible,
        b"nether_star" => NetherStar,
        b"netherite_block" => NetheriteBlock,
        b"netherite_hoe" => NetheriteHoe,
        b"netherrack" => Netherrack,
        b"no_despawn" => NoDespawn,
        b"no_gravity" => NoGravity,
        b"note_block.xylophone" => NoteBlockXylophone,
        b"oak_button" => OakButton,
        b"oak_wall_sign" => OakWallSign,
        b"passenger" => Passenger,
        b"pickup_delay" => PickupDelay,
        b"piglin_brute" => PiglinBrute,
        b"piglin_brute.ambient" => PiglinBruteAmbient,
        b"piglin_brute.angry" => PiglinBruteAngry,
        b"player.attack.crit" => PlayerAttackCrit,
        b"player.levelup" => PlayerLevelup,
        b"potion_color" => PotionColor,
        b"profession" => Profession,
        b"protection" => Protection,
        b"quick_charge" => QuickCharge,
        b"raw_copper" => RawCopper,
        b"respawn_anchor.charge" => RespawnAnchorCharge,
        b"respawn_anchor.deplete" => RespawnAnchorDeplete,
        b"reverse_portal" => ReversePortal,
        b"right_hand" => RightHand,
        b"right_hand_chance" => RightHandChance,
        b"rotten_flesh" => RottenFlesh,
        b"selected_item" => SelectedItem,
        b"sharpness" => Sharpness,
        b"skeleton.ambient" => SkeletonAmbient,
        b"snowball.throw" => SnowballThrow,
        b"soul_flame" => SoulFlame,
        b"soul_sand.step" => SoulSandStep,
        b"soul_sand_valley.mood" => SoulSandValleyMood,
        b"spawnpoint" => Spawnpoint,
        b"spectator" => Spectator,
        b"splash_potion" => SplashPotion,
        b"spruce_button" => SpruceButton,
        b"stability" => Stability,
        b"stone.place" => StonePlace,
        b"stone_button" => StoneButton,
        b"stone_button.click_on" => StoneButtonClickOn,
        b"stone_pickaxe" => StonePickaxe,
        b"stone_sword" => StoneSword,
        b"stopsound" => Stopsound,
        b"text_display" => TextDisplay,
        b"tuff.break" => TuffBreak,
        b"unbreakable" => Unbreakable,
        b"villager.trade" => VillagerTrade,
        b"wall_torch" => WallTorch,
        b"wandering_trader" => WanderingTrader,
        b"wandering_trader.ambient" => WanderingTraderAmbient,
        b"wandering_trader.yes" => WanderingTraderYes,
        b"warped_button" => WarpedButton,
        b"warped_forest.mood" => WarpedForestMood,
        b"warped_nylium" => WarpedNylium,
        b"white_stained_glass" => WhiteStainedGlass,
        b"wither.spawn" => WitherSpawn,
        b"wither_skeleton.step" => WitherSkeletonStep,
        b"wood.place" => WoodPlace,
        b"wooden_hoe" => WoodenHoe,
        b"wooden_pickaxe" => WoodenPickaxe,
        b"wooden_sword" => WoodenSword,

        _ => Id,
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
