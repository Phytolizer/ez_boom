use byteorder::{LittleEndian, ReadBytesExt};
use io::Read;
use serde_derive::{Deserialize, Serialize};
use std::{io, mem, path::Path, path::PathBuf};

#[derive(Debug)]
pub struct WadInfo {
    pub identification: [u8; 4],
    pub numlumps: i32,
    pub infotableofs: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WadFileInfo {
    pub name: PathBuf,
    pub src: WadSource,
    pub handle: i32,
}

#[derive(Debug)]
pub struct FileLump {
    pub filepos: i32,
    pub size: i32,
    pub name: [u8; 8],
}

pub trait ReadWadExt {
    fn read_wadinfo(&mut self) -> Result<WadInfo, io::Error>;
    fn read_filelump(&mut self) -> Result<FileLump, io::Error>;
}

impl<T> ReadWadExt for T
where
    T: Read,
{
    fn read_wadinfo(&mut self) -> Result<WadInfo, io::Error> {
        let mut info: WadInfo = unsafe { mem::zeroed() };
        for i in 0..4 {
            info.identification[i] = self.read_u8()?;
        }
        info.numlumps = self.read_i32::<LittleEndian>()?;
        info.infotableofs = self.read_i32::<LittleEndian>()?;
        Ok(info)
    }
    fn read_filelump(&mut self) -> Result<FileLump, io::Error> {
        let mut lump: FileLump = unsafe { mem::zeroed() };
        lump.filepos = self.read_i32::<LittleEndian>()?;
        lump.size = self.read_i32::<LittleEndian>()?;
        for i in 0..8 {
            lump.name[i] = self.read_u8()?;
        }
        Ok(lump)
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum WadSource {
    Iwad = 0,
    Pre,
    AutoLoad,
    Pwad,
    Lmp,
    Net,
    Deh,
    Err,
}

pub fn add_default_extension<P: AsRef<Path>>(path: P, ext: &str) -> PathBuf {
    let path = path.as_ref().to_str().unwrap();
    let pb = path.as_bytes();
    let mut p = path.len();
    // check for . in last path component
    while p > 0 && pb[p - 1] != b'/' && pb[p - 1] != b'\\' {
        p -= 1;
        if pb[p] == b'.' {
            // file already has an extension
            return PathBuf::from(path.to_string());
        }
    }
    // must add the extension ourselves
    let mut path = path.to_string();
    if !ext.starts_with('.') {
        path.push('.');
    }
    path.push_str(ext);
    PathBuf::from(path)
}
