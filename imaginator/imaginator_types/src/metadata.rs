use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[allow(dead_code)]
use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;

use uuid::Uuid;

use crate::args::MetaCreateArgs;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MetaData {
    pub uuid: Uuid,
    pub key: String,
    pub val: String,
}

impl MetaData {
    pub fn from_key_val(key: String, val: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            key,
            val,
        }
    }
    pub fn from_args(args: MetaCreateArgs) -> Self {
        Self::from_key_val(args.key, args.val)
    }
    pub fn from_dir_entry(entry: DirEntry) -> Vec<Self> {
        let mut meta = Self::from_meta_data(entry.metadata().unwrap());
        meta.extend(Self::from_exif(entry.path()));
        meta
    }
    pub fn from_meta_data(meta_data: fs::Metadata) -> Vec<Self> {
        let mut pairs: Vec<(String, String)> = vec![];
        if let Ok(accessed) = meta_data.accessed() {
            let time: DateTime<Utc> = accessed.into();
            pairs.push((
                String::from("accessed"),
                time.format("%d-%m-%Y %T").to_string(),
            ));
        }
        if let Ok(created) = meta_data.created() {
            let time: DateTime<Utc> = created.into();
            pairs.push((
                String::from("created"),
                time.format("%d-%m-%Y %T").to_string(),
            ));
        }
        if let Ok(modified) = meta_data.modified() {
            let time: DateTime<Utc> = modified.into();
            pairs.push((
                String::from("modified"),
                time.format("%d-%m-%Y %T").to_string(),
            ));
        }
        pairs.push((String::from("filesize"), meta_data.len().to_string()));

        pairs
            .into_iter()
            .map(|(key, val)| Self::from_key_val(key, val))
            .collect()
    }
    pub fn from_exif(path: PathBuf) -> Vec<Self> {
        let file = std::fs::File::open(path);
        if let Err(_) = file {
            return vec![];
        }
        let file = file.unwrap();
        let mut buffer = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        match exifreader.read_from_container(&mut buffer) {
            Err(_) => vec![],
            Ok(exif_data) => exif_data
                .fields()
                .into_iter()
                .map(|f| Self::from_key_val(f.tag.to_string(), f.display_value().to_string()))
                .collect::<Vec<_>>(),
        }
    }
}
