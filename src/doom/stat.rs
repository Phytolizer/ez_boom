use crate::configuration::Configuration;

fn comp(configuration: &mut Configuration) -> &[&mut bool] {
    &[
        &mut configuration.comp_zombie,
    ]
}
