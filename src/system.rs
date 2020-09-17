use crate::defs::PACKAGE_NAME;
use crate::defs::PACKAGE_VERSION;

pub fn version_string() -> String {
    format!("{} v{}", PACKAGE_NAME, PACKAGE_VERSION)
}

pub fn get_random_time_seed() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
