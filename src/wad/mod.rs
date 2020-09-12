use byteorder::{LittleEndian, ReadBytesExt};
use io::Read;
use std::{io, mem, path::PathBuf};

#[derive(Debug)]
pub(crate) struct WadInfo {
    pub(crate) identification: [u8; 4],
    pub(crate) numlumps: i32,
    pub(crate) infotableofs: i32,
}

#[derive(Debug)]
pub(crate) struct WadFileInfo {
    pub(crate) name: PathBuf,
    pub(crate) src: WadSource,
    pub(crate) handle: i32,
}

#[derive(Debug)]
pub(crate) struct FileLump {
    pub(crate) filepos: i32,
    pub(crate) size: i32,
    pub(crate) name: [u8; 8],
}

pub(crate) trait ReadWadExt {
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
#[derive(Debug, Copy, Clone)]
pub(crate) enum WadSource {
    Iwad = 0,
    Pre,
    AutoLoad,
    Pwad,
    Lmp,
    Net,
    Deh,
    Err,
}

pub(crate) fn add_default_extension(path: &str, ext: &str) -> String {
    let pb = path.as_bytes();
    let mut p = path.len();
    // check for . in last path component
    while p > 0 && pb[p - 1] != b'/' && pb[p - 1] != b'\\' {
        p -= 1;
        if pb[p] == b'.' {
            // file already has an extension
            return path.to_string();
        }
    }
    // must add the extension ourselves
    let mut path = path.to_string();
    if !ext.starts_with('.') {
        path.push('.');
    }
    path.push_str(ext);
    path
}
