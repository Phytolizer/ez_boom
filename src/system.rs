use crate::defs::PACKAGE_NAME;
use crate::defs::PACKAGE_VERSION;

pub fn version_string() -> String {
    format!("{} v{}", PACKAGE_NAME, PACKAGE_VERSION)
}
