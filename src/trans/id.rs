use sorted_code::{sorted_fns, sorted_match};

use crate::{
    ErrorKind::Translation,
    NeniyError, Result,
    lexic::token::{Token, TokenKind},
    trans::aux::NodeView,
};

fn unknown_id(node_view: &NodeView, id: Token, id_name: &str) -> NeniyError {
    NeniyError::new(
        ["unknown ", id_name, " \"", node_view.extract(id.base), "\""].concat(),
        Translation,
        node_view.source_code,
        id.base.start,
        id.base.end,
    )
}

sorted_fns!(
    pub fn attribute_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
        let result = sorted_match!(match id.kind {
            TokenKind::AttackDamage => "attack_damage",
            TokenKind::AttackSpeed => "attack_speed",
            TokenKind::MaxHealth => "max_health",
            TokenKind::Stability => "knockback_resistance",

            _ => {
                return Err(unknown_id(node_view, id, "attribute"));
            }
        });

        Ok(result)
    }

    pub fn block_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
        use TokenKind::*;

        let result = sorted_match!(match id.kind {
            AcaciaButton => "acacia_button",
            Air => "air",
            Barrier => "barrier",
            Chain => "iron_chain",
            CoalOre => "coal_ore",
            Cobblestone => "cobblestone",
            CobblestoneWall => "cobblestone_wall",
            CopperOre => "copper_ore",
            CrimsonButton => "crimson_button",
            Dirt => "dirt",
            Fire => "fire",
            GoldOre => "gold_ore",
            Ice => "ice",
            IronBlock => "iron_block",
            IronOre => "iron_ore",
            Light => "light",
            MagmaBlock => "magma_block",
            MushroomStem => "mushroom_stem",
            NetheriteBlock => "netherite_block",
            Netherrack => "netherrack",
            OakButton => "oak_button",
            OakWallSign => "oak_wall_sign",
            SpruceButton => "spruce_button",
            Stone => "stone",
            StoneButton => "stone_button",
            WallTorch => "wall_torch",
            WarpedButton => "warped_button",
            WarpedNylium => "warped_nylium",
            Water => "water",
            WhiteStainedGlass => "white_stained_glass",

            _ => return Err(unknown_id(node_view, id, "block")),
        });

        Ok(result)
    }

    pub fn data_field_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
        let result = sorted_match!(match id.kind {
            TokenKind::Scale => "transformation.scale",

            _ => return Err(unknown_id(node_view, id, "data field")),
        });

        Ok(result)
    }

    pub fn effect_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
        let result = sorted_match!(match id.kind {
            TokenKind::NightVision => "night_vision",

            _ => return Err(unknown_id(node_view, id, "effect")),
        });

        Ok(result)
    }

    pub fn enchantment_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
        use TokenKind::*;

        let result = sorted_match!(match id.kind {
            Knockback => "knockback",
            Looting => "looting",
            Power => "power",
            Protection => "protection",
            QuickCharge => "quick_charge",
            Sharpness => "sharpness",

            _ => return Err(unknown_id(node_view, id, "enchantment")),
        });

        Ok(result)
    }

    pub fn entity_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
        use TokenKind::*;

        let result = sorted_match!(match id.kind {
            ArmorStand => "armor_stand",
            BlockDisplay => "block_display",
            FallingBlock => "falling_block",
            Interaction => "interaction",
            Item => "item",
            ItemDisplay => "item_display",
            MagmaCube => "magma_cube",
            Marker => "marker",
            MushroomStem => "mushroom_stem",
            Phantom => "phantom",
            PiglinBrute => "piglin_brute",
            Skeleton => "skeleton",
            Stray => "stray",
            TextDisplay => "text_display",
            Villager => "villager",
            WanderingTrader => "wandering_trader",
            Zombie => "zombie",

            _ => return Err(unknown_id(node_view, id, "entity")),
        });

        Ok(result)
    }

    pub fn game_mode_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
        let result = sorted_match!(match id.kind {
            TokenKind::Adventure => "adventure",
            TokenKind::Spectator => "spectator",

            _ => return Err(unknown_id(node_view, id, "game mode")),
        });

        Ok(result)
    }

    pub fn game_rule_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
        let result = sorted_match!(match id.kind {
            TokenKind::NaturalRegeneration => "natural_health_regeneration",

            _ => return Err(unknown_id(node_view, id, "game rule")),
        });

        Ok(result)
    }

    pub fn item_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
        use TokenKind::*;

        let result = sorted_match!(match id.kind {
            Arrow => "arrow",
            Barrier => "barrier",
            BlazeRod => "blaze_rod",
            Bone => "bone",
            Book => "book",
            Bow => "bow",
            Coal => "coal",
            CrimsonNylium => "crimson_nylium",
            Crossbow => "crossbow",
            Egg => "egg",
            FireCharge => "fire_charge",
            FlintAndSteel => "flint_and_steel",
            GoldIngot => "gold_ingot",
            IronBoots => "iron_boots",
            IronChestplate => "iron_chestplate",
            IronLeggings => "iron_leggings",
            IronNugget => "iron_nugget",
            IronPickaxe => "iron_pickaxe",
            IronSword => "iron_sword",
            LeatherBoots => "leather_boots",
            LeatherChestplate => "leather_chestplate",
            LeatherHelmet => "leather_helmet",
            LeatherLeggings => "leather_leggings",
            NetherStar => "nether_star",
            NetheriteHoe => "netherite_hoe",
            Potion => "potion",
            RawCopper => "raw_copper",
            RawGold => "raw_gold",
            RawIron => "raw_iron",
            RottenFlesh => "rotten_flesh",
            Shield => "shield",
            Snowball => "snowball",
            SplashPotion => "splash_potion",
            StoneButton => "stone_button",
            StonePickaxe => "stone_pickaxe",
            StoneSword => "stone_sword",
            Trident => "trident",
            WoodenHoe => "wooden_hoe",
            WoodenPickaxe => "wooden_pickaxe",
            WoodenSword => "wooden_sword",

            _ => return Err(unknown_id(node_view, id, "item")),
        });

        Ok(result)
    }

    pub fn particle_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
        use TokenKind::*;

        let result = sorted_match!(match id.kind {
            Ash => "ash",
            Block => "block",
            CampfireCosySmoke => "campfire_cosy_smoke",
            Cloud => "cloud",
            DrippingLava => "dripping_lava",
            DustColorTransition => "dust_color_transition",
            ElectricSpark => "electric_spark",
            EndRod => "end_rod",
            FallingWater => "falling_water",
            Flame => "flame",
            GlowSquidInk => "glow_squid_ink",
            HappyVillager => "happy_villager",
            Item => "item",
            Lava => "lava",
            ReversePortal => "reverse_portal",
            Scrape => "scrape",
            Smoke => "smoke",
            SoulFlame => "soul_fire_flame",

            _ => return Err(unknown_id(node_view, id, "particle")),
        });

        Ok(result)
    }

    pub fn sound_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
        let result = sorted_match!(match id.kind {
            TokenKind::EggThrow => "entity.egg.throw",
            TokenKind::StonePlace => "block.stone.place",

            _ => return Err(unknown_id(node_view, id, "sound")),
        });

        Ok(result)
    }

    pub fn time_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
        let result = sorted_match!(match id.kind {
            TokenKind::Night => "night",

            _ => return Err(unknown_id(node_view, id, "time mode")),
        });

        Ok(result)
    }
);
