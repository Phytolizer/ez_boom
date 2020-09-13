use crate::lprint;
use crate::misc::lprint::OutputLevel;
use regex::bytes::Regex;
use std::{
    fs,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::Path,
    path::PathBuf,
};

use args::ArgList;

use crate::configuration::{Configuration, Defaults};

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

    // dbg!(&defaults);
    // if let Ok(f) = fs::File::create(&configuration.default_file) {
    //     serde_yaml::to_writer(f, &defaults).unwrap();
    // }

    #[derive(Debug)]
    enum ConfigParam {
        String(Vec<u8>),
        Integer(i32),
        EnumVariant(String),
        Bool(bool),
        Array(Vec<u8>),
    }

    if let Ok(f) = fs::File::open(&configuration.default_file) {
        let config_param_regex = Regex::new(r#"^\s*(\S+)\s+(\S+|"(?:.+)")\s*$"#).unwrap();
        let mut r = BufReader::new(f);
        loop {
            let mut line = Vec::<u8>::new();
            r.read_until(b'\n', &mut line).unwrap_or_else(|_| {
                crate::error("Failed reading a line from the configuration file!")
            });
            if line.is_empty() {
                continue;
            }
            if line[0].is_ascii_alphanumeric() {
                // not a comment
                if let Some(caps) = config_param_regex.captures(&line) {
                    // valid config item
                    let def = caps.get(1).unwrap();
                    let strparm = caps.get(2).unwrap();
                    let mut is_string = false;

                    let parm = if def.as_bytes().starts_with(br#"""#) {
                        is_string = true;
                        ConfigParam::String(def.as_bytes()[1..def.range().len() - 1].to_owned())
                    } else if strparm.as_bytes().starts_with(b"0x") {
                        let strparm = &strparm.as_bytes()[2..];
                        ConfigParam::Integer(
                            i32::from_str_radix(&String::from_utf8_lossy(strparm), 16)
                                .unwrap_or_else(|_| {
                                    crate::error(format!(
                                        "Error: in {}: bad hexadecimal integer {}",
                                        configuration.default_file.to_str().unwrap(),
                                        String::from_utf8_lossy(strparm),
                                    ))
                                }),
                        )
                    } else if strparm.as_bytes()[0].is_ascii_alphabetic() {
                        // enum variant, probably
                        if strparm.as_bytes() == b"true" || strparm.as_bytes() == b"false" {
                            ConfigParam::Bool(strparm.as_bytes() == b"true")
                        } else {
                            ConfigParam::EnumVariant(
                                String::from_utf8(strparm.as_bytes().to_owned()).unwrap_or_else(
                                    |_| {
                                        crate::error(format!(
                                            "Error: in {}: parameter to {} is not valid UTF-8",
                                            configuration.default_file.to_str().unwrap(),
                                            String::from_utf8_lossy(def.as_bytes())
                                        ))
                                    },
                                ),
                            )
                        }
                    } else if strparm.as_bytes()[0] == b'[' {
                        // array
                        let mut res = strparm.as_bytes()[1..].to_owned();
                        let mut buf = Vec::<u8>::new();
                        r.read_until(b']', &mut buf).unwrap_or_else(|e| {
                            crate::error(format!(
                                "Error: reading {}: {}",
                                configuration.default_file.to_str().unwrap(),
                                e
                            ))
                        });
                        res.append(&mut buf);
                        // remove closing ']' since read_until includes it
                        res.remove(res.len() - 1);
                        ConfigParam::Array(res)
                    } else {
                        ConfigParam::Integer(
                            String::from_utf8_lossy(strparm.as_bytes())
                                .parse()
                                .unwrap_or_else(|_| {
                                    crate::error(format!(
                                        "Error: in {}: bad integer {}",
                                        configuration.default_file.to_str().unwrap(),
                                        String::from_utf8_lossy(strparm.as_bytes())
                                    ))
                                }),
                        )
                    };

                    dbg!(&parm);

                    match def.as_bytes() {}
                }
            }
            if let Ok(b"") = r.fill_buf() {
                break;
            }
        }
    }
}
