use crate::{defs::PACKAGE_TARNAME, lprint, wad::WadFileInfo};
use crate::{misc::lprint::OutputLevel, wad::WadSource};
use std::{
    fs,
    fs::File,
    io::{self, Read},
    path::Path,
    path::PathBuf,
};

use args::ArgList;

use crate::configuration::Configuration;

pub(crate) mod args;
pub(crate) mod fixed;
pub(crate) mod lprint;

pub const BOOM_CFG: &str = "ezboom.cfg";

pub fn read_file<P: AsRef<Path>>(file_name: P) -> Result<Vec<u8>, io::Error> {
    File::open(file_name.as_ref()).and_then(|mut f| {
        let mut buf = Vec::<u8>::new();
        f.read_to_end(&mut buf)?;
        Ok(buf)
    })
}

pub(crate) fn load_defaults(configuration: &mut Configuration) {
    if let Some(i) = configuration.args.check_parm("-config") {
        if i < configuration.args.len() - 1 {
            configuration.default_file = PathBuf::from(&configuration.args[i + 1]);
        }
    }

    lprint!(
        OutputLevel::CONFIRM,
        " default file: {}\n",
        configuration.default_file.to_str().unwrap()
    );

    // dbg!(&defaults);
    // if let Ok(f) = fs::File::create(&configuration.default_file) {
    //     serde_yaml::to_writer(f, &defaults).unwrap();
    // }

    if let Ok(f) = fs::File::open(&configuration.default_file) {
        match serde_yaml::from_reader(f) {
            Ok(defaults) => configuration.defaults = defaults,
            Err(e) => eprintln!(
                "Error: reading {}: {}",
                configuration.default_file.to_str().unwrap(),
                e
            ),
        }
    }

    match crate::find_file(&format!("{}.wad", PACKAGE_TARNAME), "") {
        Some(wad) => configuration.wad_files.push(WadFileInfo {
            name: wad,
            src: WadSource::Pre,
            handle: 0,
        }),
        None => crate::error("Ezboom.wad not found. Can't continue."),
    }
}
