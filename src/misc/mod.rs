use crate::lprint;
use crate::misc::lprint::OutputLevel;
use regex::bytes::Regex;
use std::{
    fs,
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

use args::ArgList;

use crate::configuration::Configuration;

pub(crate) mod args;
pub(crate) mod fixed;
pub(crate) mod lprint;

pub const BOOM_CFG: &str = "ezboom.cfg";

pub fn read_file(file_name: &str) -> Result<Vec<u8>, io::Error> {
    File::open(file_name).and_then(|mut f| {
        let mut buf = Vec::<u8>::new();
        f.read_to_end(&mut buf)?;
        Ok(buf)
    })
}

pub(crate) fn load_defaults(configuration: &mut Configuration) {
    let mut defaults = &mut configuration.defaults;

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

    if let Ok(mut f) = fs::File::open(&configuration.default_file) {
        let config_param_regex = Regex::new(r"^\s*(\S+)\s+(.+?)\s*$").unwrap();
        loop {
            let mut line = Vec::<u8>::new();
            f.read(&mut line).unwrap_or_else(|_| {
                crate::error("Failed reading a line from the configuration file!")
            });
            if line[0].is_ascii_alphanumeric() {
                // not a comment
                if let Some(caps) = config_param_regex.captures(&line) {
                    // valid config item
                    let def = caps.get(1).unwrap();
                    let strparm = caps.get(2).unwrap();
                    let mut is_string = false;

                    let new_string: &[u8];
                    if def.as_bytes().starts_with(br#"""#) {
                        is_string = true;
                        new_string = &def.as_bytes()[1..def.range().len() - 1];
                    } else if strparm.as_bytes().starts_with(b"0x") {
                        let strparm = &strparm.as_bytes()[2..];
                        let parm = usize::from_str_radix(&String::from_utf8_lossy(strparm), 16)
                            .unwrap_or_else(|_| {
                                crate::error(format!(
                                    "Error: bad hexadecimal integer {}",
                                    String::from_utf8_lossy(strparm),
                                ))
                            });
                    }
                }
            }
        }
    }
}
