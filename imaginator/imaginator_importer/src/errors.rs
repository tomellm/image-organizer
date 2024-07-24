use std::{
    ffi::{OsStr, OsString},
    fs::DirEntry,
    io::Error,
    path::PathBuf,
};

#[derive(Debug)]
pub enum ImportErr {
    NotDir(String),
    ReadDirErr(Error),
}

#[derive(Debug)]
pub enum MediaReadErr {
    ReadDirFailed(Error),
    FileName(PathBuf),
    StringFormat(OsString),
    Extension(PathBuf),
    XmpPresentButFileMissing(String, PathBuf),
    NeitherPresent(String),
}

impl MediaReadErr {
    pub fn read_dir(err: Error) -> Self {
        Self::ReadDirFailed(err)
    }
    pub fn path_buf(path_buf: PathBuf) -> Self {
        Self::FileName(path_buf)
    }
    pub fn string_fmt(os_str: &OsStr) -> Self {
        Self::StringFormat(os_str.to_owned())
    }
    pub fn ext(path_buf: PathBuf) -> Self {
        Self::Extension(path_buf)
    }
    pub fn xmp_but_no_file(name: &String, dir_entry: &DirEntry) -> Self {
        Self::XmpPresentButFileMissing(name.to_string(), dir_entry.path())
    }
    pub fn neither(name: &String) -> Self {
        Self::NeitherPresent(name.to_string())
    }
}
