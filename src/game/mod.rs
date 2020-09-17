use crate::args::ArgList;
use crate::configuration::{CompatibilityLevel, Configuration, PlayerHelpers, SkillLevel};
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
