use crate::configuration::ProcessPriority;
use crate::lprint;
use crate::misc::lprint::OutputLevel;
use regex::bytes::Regex;
use std::str::FromStr;
use std::{
    fs,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::Path,
    path::PathBuf,
};

use args::ArgList;

use crate::configuration::{self, Configuration, Defaults, PositiveInt};
use configuration::{CompatibilityLevel, OptionalLimit};

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

#[derive(Debug)]
pub enum ConfigParam {
    String(Vec<u8>),
    Integer(i32),
    EnumVariant(String),
    Bool(bool),
    Array(Vec<u8>),
}

impl ConfigParam {
    pub fn is_string(&self) -> bool {
        match self {
            Self::String(_) => true,
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Self::Integer(_) => true,
            _ => false,
        }
    }

    pub fn is_enum_variant(&self) -> bool {
        match self {
            Self::EnumVariant(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Self::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Self::Array(_) => true,
            _ => false,
        }
    }

    pub fn as_string(&self) -> &Vec<u8> {
        match self {
            Self::String(s) => s,
            _ => panic!("not a string"),
        }
    }

    pub fn as_integer(&self) -> i32 {
        match self {
            Self::Integer(i) => *i,
            _ => panic!("not an integer"),
        }
    }

    pub fn as_enum_variant(&self) -> &String {
        match self {
            Self::EnumVariant(v) => v,
            _ => panic!("not an enum variant"),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Self::Bool(b) => *b,
            _ => panic!("not a bool"),
        }
    }

    pub fn as_array(&self) -> &Vec<u8> {
        match self {
            Self::Array(a) => a,
            _ => panic!("not an array"),
        }
    }
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
        let config_param_regex = Regex::new(r#"^\s*(\S+)\s+(\S+|"(?:.+)")\s*$"#).unwrap();
        let mut r = BufReader::new(f);
        configuration.defaults = serde_yaml::from_reader(f).unwrap_or_else(|e| {
            crate::error(format!(
                "Error: reading {}: {}",
                configuration.default_file.to_str().unwrap(),
                e
            ))
        });
        // loop {
        //     let mut line = Vec::<u8>::new();
        //     r.read_until(b'\n', &mut line).unwrap_or_else(|_| {
        //         crate::error("Failed reading a line from the configuration file!")
        //     });
        //     if line.is_empty() {
        //         continue;
        //     }
        //     let fname = configuration.default_file.to_str().unwrap();
        //     if line[0].is_ascii_alphanumeric() {
        //         // not a comment

        //         // if let Some(caps) = config_param_regex.captures(&line) {
        //         //     // valid config item
        //         //     let def = caps.get(1).unwrap();
        //         //     let strparm = caps.get(2).unwrap();

        //         //     let parm = if def.as_bytes().starts_with(br#"""#) {
        //         //         ConfigParam::String(def.as_bytes()[1..def.range().len() - 1].to_owned())
        //         //     } else if strparm.as_bytes().starts_with(b"0x") {
        //         //         let strparm = &strparm.as_bytes()[2..];
        //         //         ConfigParam::Integer(
        //         //             i32::from_str_radix(&String::from_utf8_lossy(strparm), 16)
        //         //                 .unwrap_or_else(|_| {
        //         //                     crate::error(format!(
        //         //                         "Error: in {}: bad hexadecimal integer {}",
        //         //                         fname,
        //         //                         String::from_utf8_lossy(strparm),
        //         //                     ))
        //         //                 }),
        //         //         )
        //         //     } else if strparm.as_bytes()[0].is_ascii_alphabetic() {
        //         //         // enum variant, probably
        //         //         if strparm.as_bytes() == b"true" || strparm.as_bytes() == b"false" {
        //         //             ConfigParam::Bool(strparm.as_bytes() == b"true")
        //         //         } else {
        //         //             ConfigParam::EnumVariant(
        //         //                 String::from_utf8(strparm.as_bytes().to_owned()).unwrap_or_else(
        //         //                     |_| {
        //         //                         crate::error(format!(
        //         //                             "Error: in {}: parameter to {} is not valid UTF-8",
        //         //                             fname,
        //         //                             String::from_utf8_lossy(def.as_bytes())
        //         //                         ))
        //         //                     },
        //         //                 ),
        //         //             )
        //         //         }
        //         //     } else if strparm.as_bytes()[0] == b'[' {
        //         //         // array
        //         //         let mut res = strparm.as_bytes()[1..].to_owned();
        //         //         let mut buf = Vec::<u8>::new();
        //         //         r.read_until(b']', &mut buf).unwrap_or_else(|e| {
        //         //             crate::error(format!("Error: reading {}: {}", fname, e))
        //         //         });
        //         //         res.append(&mut buf);
        //         //         // remove closing ']' since read_until includes it
        //         //         res.remove(res.len() - 1);
        //         //         ConfigParam::Array(res)
        //         //     } else {
        //         //         ConfigParam::Integer(
        //         //             String::from_utf8_lossy(strparm.as_bytes())
        //         //                 .parse()
        //         //                 .unwrap_or_else(|_| {
        //         //                     crate::error(format!(
        //         //                         "Error: in {}: bad integer {}",
        //         //                         fname,
        //         //                         String::from_utf8_lossy(strparm.as_bytes())
        //         //                     ))
        //         //                 }),
        //         //         )
        //         //     };

        //         //     let def = String::from_utf8_lossy(def.as_bytes()).to_string();

        //         //     let parm_is_valid = Defaults::get_basic_validator(&def)(&parm);
        //         //     if !parm_is_valid {
        //         //         crate::error(format!(
        //         //             "Error: in {}: value for {} is wrong type",
        //         //             fname, def
        //         //         ));
        //         //     }
        //         //     let parm_err = |why: &str| -> ! {
        //         //         crate::error(format!("Error: in {}: value for {} is {}", fname, def, why));
        //         //     };
        //         //     let mut defaults = &mut configuration.defaults;
        //         //     match def.as_str() {
        //         //         "process_priority" => {
        //         //             if let Some(pri) = ProcessPriority::new(parm.as_integer()) {
        //         //                 defaults.process_priority = pri;
        //         //             } else {
        //         //                 parm_err("out of bounds");
        //         //             }
        //         //         }
        //         //         "default_compatibility_level" => {
        //         //             if let Ok(lev) = CompatibilityLevel::from_str(parm.as_enum_variant()) {
        //         //                 defaults.default_compatibility_level = lev;
        //         //             } else {
        //         //                 parm_err("not a known compatibility level");
        //         //             }
        //         //         }
        //         //         "realtic_clock_rate" => {
        //         //             if let Some(rate) = PositiveInt::new(parm.as_integer()) {
        //         //                 defaults.realtic_clock_rate = rate;
        //         //             } else {
        //         //                 parm_err("out of bounds");
        //         //             }
        //         //         }
        //         //         "menu_background" => defaults.menu_background = parm.as_bool(),
        //         //         "body_queue_size" => {
        //         //             if parm.is_enum_variant() {
        //         //                 if parm.as_enum_variant() == "NoLimit" {
        //         //                     defaults.body_queue_size = OptionalLimit::NoLimit;
        //         //                 } else {
        //         //                     parm_err("not a valid optional limit (can be 'NoLimit' or an integer limit)");
        //         //                 }
        //         //             } else {
        //         //                 defaults.body_queue_size = OptionalLimit::Limit(
        //         //                     PositiveInt::new(parm.as_integer())
        //         //                         .unwrap_or_else(|| parm_err("out of bounds")),
        //         //                 );
        //         //             }
        //         //         }
        //         //         "weapon_attack_alignment" => {}
        //         //         "player_helpers" => {}
        //         //         "friend_distance" => {}
        //         //         _ => {
        //         //             lprint!(
        //         //                 OutputLevel::WARN,
        //         //                 "Skipping unknown config key {}.\n",
        //         //                 String::from_utf8_lossy(def.as_bytes())
        //         //             );
        //         //         }
        //         //     }
        //         // }
        //     }
        //     if let Ok(b"") = r.fill_buf() {
        //         break;
        //     }
        // }
    }
}
