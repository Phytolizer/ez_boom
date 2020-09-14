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
        configuration.defaults = serde_yaml::from_reader(f).unwrap_or_else(|e| {
            crate::error(format!(
                "Error: reading {}: {}",
                configuration.default_file.to_str().unwrap(),
                e
            ))
        });
    }

    dbg!(&configuration.defaults);
}
