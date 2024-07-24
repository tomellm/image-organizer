use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    path::PathBuf,
    str::FromStr,
};

use crate::{
    errors::{ImportErr, MediaReadErr},
    ReadMediaDirectory,
};
use imaginator_types::media::Media;

pub fn get_media_with_xmp(dir_path: &str) -> Result<ReadMediaDirectory, ImportErr> {
    let path_buf = PathBuf::from_str(dir_path).unwrap();
    if !path_buf.is_dir() {
        return Err(ImportErr::NotDir(dir_path.to_string()));
    }

    let read_dir = fs::read_dir(path_buf).map_err(|err| ImportErr::ReadDirErr(err))?;

    let (read_dirs, err_read_dirs): (Vec<_>, Vec<_>) =
        read_dir.into_iter().partition(Result::is_ok);

    let read_dirs = read_dirs
        .into_iter()
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    let mut all_media_errors = err_read_dirs
        .into_iter()
        .map(|err| MediaReadErr::read_dir(err.unwrap_err()))
        .collect::<Vec<_>>();

    let mut files_map = HashMap::<String, (Option<DirEntry>, Option<DirEntry>)>::new();

    for reading_dir in read_dirs {
        let current_path_buf = reading_dir.path();

        let Some(stem_os_str) = current_path_buf.file_stem() else {
            all_media_errors.push(MediaReadErr::path_buf(current_path_buf));
            continue;
        };
        let Some(stem) = stem_os_str.to_str() else {
            all_media_errors.push(MediaReadErr::string_fmt(stem_os_str));
            continue;
        };
        let Some(os_str_extension) = current_path_buf.extension() else {
            all_media_errors.push(MediaReadErr::ext(current_path_buf));
            continue;
        };
        let Some(extension) = os_str_extension.to_str() else {
            all_media_errors.push(MediaReadErr::string_fmt(os_str_extension));
            continue;
        };
        let element = match files_map.remove(stem) {
            Some(element) => element,
            None => (None, None),
        };

        match extension.to_lowercase().as_str() {
            "xmp" => {
                files_map.insert(stem.to_string(), (element.0, Some(reading_dir)));
            }
            _ => {
                files_map.insert(stem.to_string(), (Some(reading_dir), element.1));
            }
        };
    }

    let media = files_map
        .into_iter()
        .filter(|(name, group)| match group {
            (Some(_), _) => true,
            (None, Some(dir_entry)) => {
                all_media_errors.push(MediaReadErr::xmp_but_no_file(name, dir_entry));
                false
            }
            (None, None) => {
                all_media_errors.push(MediaReadErr::neither(name));
                false
            }
        })
        .map(|(_, (entry, xmp_file))| Media::from_dir_entry(entry.unwrap(), xmp_file))
        .collect::<Vec<_>>();

    Ok(ReadMediaDirectory {
        media,
        errors: all_media_errors,
    })
}
