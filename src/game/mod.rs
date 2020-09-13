use crate::configuration::Configuration;

pub(crate) fn reload_defaults(configuration: &mut Configuration) {
    configuration.weapon_recoil = configuration.defaults.weapon_recoil;
    configuration.player_bobbing = configuration.defaults.player_bobbing;
    configuration.variable_friction = configuration.defaults.variable_friction;
}
