use std::{
    fs::File,
    io::{self, Read},
};

pub(crate) mod args;
pub(crate) mod fixed;
pub(crate) mod lprint;

pub fn read_file(file_name: &str) -> Result<Vec<u8>, io::Error> {
    File::open(file_name).and_then(|mut f| {
        let mut buf = Vec::<u8>::new();
        f.read_to_end(&mut buf)?;
        Ok(buf)
    })
}
