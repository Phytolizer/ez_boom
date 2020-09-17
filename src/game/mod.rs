use crate::configuration::{
    CompatibilityLevel, Configuration, MonsterInfightingLevel, PlayerHelpers, SkillLevel,
};
use crate::{args::ArgList, configuration::DemoInsurance};
use num_enum::TryFromPrimitive;
use std::str::FromStr;

pub fn reload_defaults(configuration: &mut Configuration) {
    configuration.weapon_recoil = configuration.defaults.weapon_recoil;
    configuration.player_bobbing = configuration.defaults.player_bobbing;
    configuration.variable_friction = configuration.defaults.variable_friction;
    configuration.allow_pushers = configuration.defaults.allow_pushers;
    configuration.monster_infighting = configuration.defaults.monster_infighting;
    configuration.dogs = if configuration.netgame {
        PlayerHelpers::new(0).unwrap()
    } else {
        get_helpers(configuration)
    };

    configuration.friend_distance = configuration.defaults.friend_distance;
    configuration.monster_backing = configuration.defaults.monster_backing;
    configuration.monster_avoid_hazards = configuration.defaults.monster_avoid_hazards;
    configuration.monster_friction = configuration.defaults.monster_friction;
    configuration.help_friends = configuration.defaults.help_friends;
    configuration.monkeys = configuration.defaults.monkeys;
    configuration.respawnparm = configuration.arg_meta.respawnparm;
    configuration.fastparm = configuration.arg_meta.fastparm;
    configuration.nomonsters = configuration.arg_meta.nomonsters;

    if configuration.start_skill == SkillLevel::None {
        configuration.start_skill = configuration.defaults.default_skill;
    }

    configuration.demo_playback = false;
    configuration.single_demo = false;
    configuration.net_demo = false;

    for p in &mut configuration.player_in_game[1..] {
        *p = false;
    }

    configuration.console_player = 0;

    configuration.compatibility_level = configuration.defaults.default_compatibility_level;
    if let Some(i) = configuration.args.check_parm("-complevel") {
        if i < configuration.args.len() - 1 {
            // try as integer
            match &configuration.args[i + 1].parse::<i32>() {
                Ok(complevel) => {
                    configuration.compatibility_level =
                        CompatibilityLevel::try_from_primitive(*complevel).unwrap_or_else(|_| {
                            // not in the enum, -1 is special though
                            if *complevel == -1 {
                                CompatibilityLevel::PrBoomLatest
                            } else {
                                crate::error(format!(
                                    "Unknown compatibility level {}",
                                    configuration.args[i + 1]
                                ));
                            }
                        })
                }
                Err(_) => {
                    // try as enum variant string
                    configuration.compatibility_level = CompatibilityLevel::from_str(
                        &configuration.args[i + 1],
                    )
                    .unwrap_or_else(|_| {
                        crate::error(format!(
                            "Unknown compatibility level {}",
                            configuration.args[i + 1]
                        ))
                    })
                }
            }
        }
    }

    if configuration.compatibility_level >= CompatibilityLevel::Mbf {
        configuration.comp_zombie = configuration.defaults.comp_zombie;
        configuration.comp_telefrag = configuration.defaults.comp_telefrag;
        configuration.comp_dropoff = configuration.defaults.comp_dropoff;
        configuration.comp_vile = configuration.defaults.comp_vile;
        configuration.comp_pain = configuration.defaults.comp_pain;
        configuration.comp_skull = configuration.defaults.comp_skull;
        configuration.comp_blazing = configuration.defaults.comp_blazing;
        configuration.comp_doorlight = configuration.defaults.comp_doorlight;
        configuration.comp_model = configuration.defaults.comp_model;
        configuration.comp_god = configuration.defaults.comp_god;
        configuration.comp_falloff = configuration.defaults.comp_falloff;
        configuration.comp_floors = configuration.defaults.comp_floors;
        configuration.comp_skymap = configuration.defaults.comp_skymap;
        configuration.comp_pursuit = configuration.defaults.comp_pursuit;
        configuration.comp_doorstuck = configuration.defaults.comp_doorstuck;
        configuration.comp_staylift = configuration.defaults.comp_staylift;
        configuration.comp_stairs = configuration.defaults.comp_stairs;
        configuration.comp_infcheat = configuration.defaults.comp_infcheat;
        configuration.comp_zerotags = configuration.defaults.comp_zerotags;
        configuration.comp_moveblock = configuration.defaults.comp_moveblock;
        configuration.comp_respawn = false;
        configuration.comp_sound = configuration.defaults.comp_sound;
        configuration.comp_666 = configuration.defaults.comp_666;
        configuration.comp_soul = configuration.defaults.comp_soul;
        configuration.comp_maskedanim = configuration.defaults.comp_maskedanim;
        configuration.comp_ouchface = configuration.defaults.comp_ouchface;
        configuration.comp_maxhealth = configuration.defaults.comp_maxhealth;
        configuration.comp_translucency = configuration.defaults.comp_translucency;
    }

    compatibility(configuration);

    configuration.demo_insurance = if configuration.defaults.demo_insurance == DemoInsurance::Always
    {
        DemoInsurance::Always
    } else {
        DemoInsurance::None
    };

    configuration.rngseed = configuration.rngseed.wrapping_add(
        ((crate::system::get_random_time_seed() % (std::u32::MAX as u64 + 1)) as u32)
            .wrapping_add(configuration.gametic as u32),
    );
}

pub fn compatibility(configuration: &mut Configuration) {
    let cl = configuration.compatibility_level;

    if cl < CompatibilityLevel::Mbf {
        configuration.comp_telefrag = true;
        configuration.comp_dropoff = true;
        configuration.comp_vile = cl < CompatibilityLevel::BoomV201;
        configuration.comp_pain = cl < CompatibilityLevel::BoomV201;
        configuration.comp_skull = cl < CompatibilityLevel::BoomV201;
        configuration.comp_blazing = cl < CompatibilityLevel::BoomV201;
        configuration.comp_doorlight = cl < CompatibilityLevel::BoomV201;
        configuration.comp_model = cl < CompatibilityLevel::BoomV201;
        configuration.comp_god = cl < CompatibilityLevel::BoomV201;
        configuration.comp_falloff = true;
        configuration.comp_floors = cl < CompatibilityLevel::Boom;
        configuration.comp_skymap = true;
        configuration.comp_pursuit = true;
        configuration.comp_doorstuck = cl < CompatibilityLevel::BoomV202;
        configuration.comp_staylift = true;
        configuration.comp_zombie = cl < CompatibilityLevel::LxDoomV1;
        configuration.comp_stairs = cl < CompatibilityLevel::BoomV202;
        configuration.comp_infcheat = true;
        configuration.comp_zerotags = cl < CompatibilityLevel::BoomV201;

        configuration.monster_infighting = MonsterInfightingLevel::OtherSpecies;
        configuration.monster_backing = false;
        configuration.monster_avoid_hazards = false;
        configuration.monster_friction = false;
        configuration.help_friends = false;
        configuration.dogs = PlayerHelpers::new(0).unwrap();
        configuration.dog_jumping = false;
        configuration.monkeys = false;
    }
    if cl < CompatibilityLevel::PrBoomV210211 {
        configuration.comp_moveblock = cl < CompatibilityLevel::LxDoomV1;
        configuration.comp_respawn = true;
    }
    if cl < CompatibilityLevel::PrBoomV22x {
        configuration.comp_sound = cl < CompatibilityLevel::Boom;
    }
    if cl < CompatibilityLevel::PrBoomV23x {
        configuration.comp_666 = cl < CompatibilityLevel::UltimateDoom;
        configuration.comp_soul = true;
        configuration.comp_maskedanim = cl < CompatibilityLevel::DoomV1666;
    }
    if cl < CompatibilityLevel::PrBoomLatest {
        configuration.comp_ouchface = cl < CompatibilityLevel::PrBoomV203Beta;
        configuration.comp_maxhealth = cl < CompatibilityLevel::Boom;
        configuration.comp_translucency = cl < CompatibilityLevel::Boom;
    }
}

pub fn get_helpers(configuration: &Configuration) -> PlayerHelpers {
    if let Some(j) = configuration.args.check_parms(&["-dog", "-dogs"]) {
        if j + 1 < configuration.args.len() {
            let dogs_parse_error = || -> ! {
                crate::error(format!(
                    "Invalid number of dogs {}",
                    configuration.args[j + 1]
                ));
            };
            PlayerHelpers::new(
                configuration.args[j + 1]
                    .parse()
                    .unwrap_or_else(|_| dogs_parse_error()),
            )
            .unwrap_or_else(|| dogs_parse_error())
        } else {
            PlayerHelpers::new(1).unwrap()
        }
    } else {
        configuration.defaults.player_helpers
    }
}
