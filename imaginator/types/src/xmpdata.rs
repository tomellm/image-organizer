use serde::{Deserialize, Serialize};
#[allow(dead_code)]
use std::str::FromStr;

use uuid::Uuid;

use crate::{
    args::XmpCreateArgs,
    database::*,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct XmpData {
    pub uuid: Uuid,
    pub key: String,
    pub val: String,
}

impl XmpData {
    pub fn from_key_val(key: String, val: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            key,
            val,
        }
    }

    pub fn from_args(args: XmpCreateArgs) -> Self {
        Self::from_key_val(args.key, args.val)
    }

    #[cfg(feature = "backend")]
    pub fn from_dir_entry(contents: String) -> Vec<Self> {
        match xmp_toolkit::XmpMeta::from_str(&contents) {
            Err(_) => vec![],
            Ok(xmp) => xmp
                .iter(xmp_toolkit::IterOptions::default())
                .map(|d| {
                    if d.name.eq("") && d.value.value.eq("") {
                        return None;
                    }
                    Some(Self::from_key_val(d.name, d.value.value))
                })
                .filter_map(|e| e)
                .collect::<Vec<_>>(),
        }
    }

    pub fn to_db(self, image_uuid: &String) -> MediaXmpData {
        MediaXmpData {
            uuid: self.uuid.simple().to_string(),
            media_uuid: image_uuid.to_owned(),
            data_key: self.key,
            data_val: self.val,
        }
    }
}
