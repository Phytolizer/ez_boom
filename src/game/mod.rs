use crate::configuration::Configuration;

pub(crate) fn reload_defaults(configuration: &mut Configuration) {
    configuration.weapon_recoil.reload_default();
    configuration.player_bobbing.reload_default();
    configuration.variable_friction.reload_default();
}
