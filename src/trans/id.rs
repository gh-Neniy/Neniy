use sorted_code::{sorted_fns, sorted_match};

use crate::{
    ErrorKind, NeniyError, Result,
    lexic::token::{Token, TokenKind},
    trans::aux::NodeView,
};

fn unknown_id(node_view: &NodeView, id: Token, id_name: &str) -> NeniyError {
    NeniyError::new(
        ["unknown ", id_name, " \"", node_view.extract(id.base), "\""].concat(),
        ErrorKind::Translation,
        node_view.source_code,
        id.base.start,
        id.base.end,
    )
}

sorted_fns!(
    use TokenKind::*;

    pub fn attribute_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
        let result = sorted_match!(match id.kind {
            AttackDamage => "attack_damage",
            AttackSpeed => "attack_speed",
            MaxHealth => "max_health",
            Stability => "knockback_resistance",

            _ => {
                return Err(unknown_id(node_view, id, "attribute"));
            }
        });

        Ok(result)
    }

    pub fn block_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
        let result = sorted_match!(match id.kind {
            AcaciaButton => "acacia_button",
            Air => "air",
            Barrier => "barrier",
            Bedrock => "bedrock",
            BoneBlock => "bone_block",
            Candle => "candle",
            Cauldron => "cauldron",
            Chain => "iron_chain",
            Chest => "chest",
            CoalOre => "coal_ore",
            CobbledDeepslate => "cobbled_deepslate",
            Cobblestone => "cobblestone",
            CobblestoneWall => "cobblestone_wall",
            CopperOre => "copper_ore",
            CrackedStoneBricks => "cracked_stone_bricks",
            CrimsonButton => "crimson_button",
            CrimsonDoor => "crimson_door",
            Dirt => "dirt",
            Fire => "fire",
            GoldOre => "gold_ore",
            GrayCandle => "gray_candle",
            Ice => "ice",
            IronBars => "iron_bars",
            IronBlock => "iron_block",
            IronOre => "iron_ore",
            Lantern => "lantern",
            LapisBlock => "lapis_block",
            Light => "light",
            MagmaBlock => "magma_block",
            MushroomStem => "mushroom_stem",
            NetherBrickFence => "nether_brick_fence",
            NetheriteBlock => "netherite_block",
            Netherrack => "netherrack",
            OakButton => "oak_button",
            OakWallSign => "oak_wall_sign",
            OxidizedCopper => "oxidized_copper",
            PottedWitherRose => "potted_wither_rose",
            RawIronBlock => "raw_iron_block",
            SoulSoil => "soul_soil",
            SpruceButton => "spruce_button",
            SpruceLog => "spruce_log",
            SpruceWallSign => "spruce_wall_sign",
            Stone => "stone",
            StoneButton => "stone_button",
            WallTorch => "wall_torch",
            WarpedButton => "warped_button",
            WarpedNylium => "warped_nylium",
            WarpedPlanks => "warped_planks",
            Water => "water",
            WhiteStainedGlass => "white_stained_glass",

            _ => return Err(unknown_id(node_view, id, "block")),
        });

        Ok(result)
    }

    pub fn data_field_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
        let result = sorted_match!(match id.kind {
            Block => "block_state.Name",
            Count => "Item.count",
            Fire => "Fire",
            Health => "Health",
            LootTable => "DeathLootTable",
            NameVisible => "CustomNameVisible",
            NoAI => "NoAI",
            Pos => "Pos",
            Scale => "transformation.scale",
            TpTime => "teleport_duration",
            Translation => "transformation.translation",

            _ => return Err(unknown_id(node_view, id, "data field")),
        });

        Ok(result)
    }

    pub fn effect_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
        let result = sorted_match!(match id.kind {
            Blindness => "blindness",
            Invisibility => "invisibility",
            NightVision => "night_vision",
            Saturation => "saturation",

            _ => return Err(unknown_id(node_view, id, "effect")),
        });

        Ok(result)
    }

    pub fn enchantment_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
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
            Adventure => "adventure",
            Spectator => "spectator",

            _ => return Err(unknown_id(node_view, id, "game mode")),
        });

        Ok(result)
    }

    pub fn game_rule_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
        let result = sorted_match!(match id.kind {
            NaturalRegeneration => "natural_health_regeneration",

            _ => return Err(unknown_id(node_view, id, "game rule")),
        });

        Ok(result)
    }

    pub fn item_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
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
            MushroomStem => "mushroom_stem",
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
        let result = sorted_match!(match id.kind {
            Ash => "ash",
            Block => "block",
            CampfireCosySmoke => "campfire_cosy_smoke",
            Cloud => "cloud",
            DrippingLava => "dripping_lava",
            DustColorTransition => "dust_color_transition",
            ElectricSpark => "electric_spark",
            Enchant => "enchant",
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
            AmethystBlockPlace => "block.amethyst_block.place",
            AmethystBlockStep => "block.amethyst_block.step",
            AncientDebrisBreak => "block.ancient_debris.break",
            ArrowHit => "entity.arrow.hit",
            AxeScrape => "item.axe.scrape",
            BasaltDeltasMood => "ambient.basalt_deltas.mood",
            BeaconPowerSelect => "block.beacon.power_select",
            BellResonate => "block.bell.resonate",
            BellUse => "block.bell.use",
            BlazeShoot => "entity.blaze.shoot",
            BucketEmptyLava => "item.bucket.empty_lava",
            CandleExtinguish => "block.candle.extinguish",
            Cave => "ambient.cave",
            ChestOpen => "block.chest.open",
            CrimsonForestLoop => "ambient.crimson_forest.loop",
            CrossbowLoadingEnd => "item.crossbow.loading_end",
            EggThrow => "entity.egg.throw",
            EvokerPrepareSummon => "entity.evoker.prepare_summon",
            ExperienceOrbPickup => "entity.experience_orb.pickup",
            FireExtinguish => "block.fire.extinguish",
            FireworkRocketBlast => "entity.firework_rocket.blast",
            FireworkRocketLargeBlast => "entity.firework_rocket.large_blast",
            FireworkRocketLaunch => "entity.firework_rocket.launch",
            GenericExplode => "entity.generic.explode",
            GlassBreak => "block.glass.break",
            GravelBreak => "block.gravel.break",
            HuskConvertedToZombie => "entity.husk.converted_to_zombie",
            LavaExtinguish => "block.lava.extinguish",
            LightningBoltThunder => "entity.lightning_bolt.thunder",
            MinecartRiding => "entity.minecart.riding",
            NoteBlockXylophone => "block.note_block.xylophone",
            PiglinBruteAmbient => "entity.piglin_brute.ambient",
            PiglinBruteAngry => "entity.piglin_brute.angry",
            PlayerAttackCrit => "entity.player.attack.crit",
            PlayerLevelup => "entity.player.levelup",
            RespawnAnchorCharge => "block.respawn_anchor.charge",
            RespawnAnchorDeplete => "block.respawn_anchor.deplete",
            SkeletonAmbient => "entity.skeleton.ambient",
            SnowballThrow => "entity.snowball.throw",
            SoulSandStep => "block.soul_sand.step",
            SoulSandValleyMood => "ambient.soul_sand_valley.mood",
            StoneButtonClickOn => "block.stone_button.click_on",
            StonePlace => "block.stone.place",
            TuffBreak => "block.tuff.break",
            VillagerTrade => "entity.villager.trade",
            WanderingTraderAmbient => "entity.wandring_trader.ambient",
            WanderingTraderYes => "entity.wandering_trader.yes",
            WarpedForestMood => "ambient.warped_forest.mood",
            WitherSkeletonStep => "entity.wither_skeleton.step",
            WitherSpawn => "entity.wither.spawn",
            WoodPlace => "block.wood.place",

            _ => return Err(unknown_id(node_view, id, "sound")),
        });

        Ok(result)
    }

    pub fn time_match(node_view: &NodeView, id: Token) -> Result<&'static str> {
        let result = sorted_match!(match id.kind {
            Night => "night",

            _ => return Err(unknown_id(node_view, id, "time mode")),
        });

        Ok(result)
    }
);
