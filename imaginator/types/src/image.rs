use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::mediatypes::*;
use crate::metadata::*;
use crate::xmpdata::*;
use crate::{
    args::ImageCreateArgs,
    database::*,
};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash)]
pub struct Media {
    pub uuid: Uuid,
    pub original_name: String,
    pub current_name: String,
    pub extension: String,
    pub meta_data: Vec<MetaData>,
    pub xmp_data: Vec<XmpData>,
    pub media_type: MediaType,
    pub datetime_created: Option<DateTime<Utc>>,
}

impl Media {
    pub fn from_args(args: ImageCreateArgs) -> Self {
        let meta_data = args
            .meta_data
            .into_iter()
            .map(MetaData::from_args)
            .collect();
        let xmp_data = args.xmp_data.into_iter().map(XmpData::from_args).collect();
        let media_type = MediaType::from_ext(args.extension.clone().as_str());
        let datetime_created = Self::get_most_likely_date(&meta_data, &xmp_data);
        Self {
            uuid: Uuid::new_v4(),
            original_name: args.original_name,
            current_name: args.current_name,
            extension: args.extension,
            meta_data,
            xmp_data,
            media_type,
            datetime_created,
        }
    }

    #[cfg(feature = "backend")]
    pub fn from_dir_entry(entry: DirEntry, xmp_file: Option<DirEntry>) -> Self {
        let original_name = entry.file_name().into_string().unwrap();
        let current_name = entry.file_name().into_string().unwrap();
        let extension = entry
            .path()
            .extension()
            .and_then(OsStr::to_str)
            .unwrap()
            .to_string();
        let meta_data = MetaData::from_dir_entry(entry);
        let xmp_data = match xmp_file {
            Some(xmp) => {
                let contents = fs::read_to_string(xmp.path()).unwrap();
                XmpData::from_dir_entry(contents)
            }
            None => vec![],
        };
        let media_type = MediaType::from_ext(&extension);
        let datetime_created = Self::get_most_likely_date(&meta_data, &xmp_data);

        Self {
            uuid: Uuid::new_v4(),
            original_name,
            current_name,
            extension,
            meta_data,
            xmp_data,
            media_type,
            datetime_created,
        }
    }

    pub fn to_db(self) -> (MediaData, Vec<MediaMetaData>, Vec<MediaXmpData>) {
        let str_image_uuid = self.uuid.simple().to_string();

        let meta_data = self
            .meta_data
            .into_iter()
            .map(|m| m.to_db(&str_image_uuid))
            .collect::<Vec<_>>();
        let xmp_data = self
            .xmp_data
            .into_iter()
            .map(|x| x.to_db(&str_image_uuid))
            .collect::<Vec<_>>();

        let image_data = MediaData {
            uuid: self.uuid.simple().to_string(),
            original_name: self.original_name,
            current_name: self.current_name,
            extension: self.extension,
            media_type: self.media_type.to_db(),
            datetime_created: self.datetime_created,
        };

        (image_data, meta_data, xmp_data)
    }

    pub fn get_linkable_name(&self) -> String {
        self.original_name
            .replace(" ", "%20")
            .replace("(", "\\(")
            .replace(")", "\\)")
    }

    fn get_most_likely_date(
        meta_data: &Vec<MetaData>,
        xmp_data: &Vec<XmpData>,
    ) -> Option<DateTime<Utc>> {
        let meta_min = meta_data
            .iter()
            .map(|m| dateparser::parse(m.val.as_str()))
            .filter_map(Result::ok)
            .min();

        let xmp_min = xmp_data
            .iter()
            .map(|x| dateparser::parse(x.val.as_str()))
            .filter_map(Result::ok)
            .min();

        match (meta_min, xmp_min) {
            (Some(meta), None) => Some(meta),
            (None, Some(xmp)) => Some(xmp),
            (None, None) => None,
            (Some(meta), Some(xmp)) => Some(match meta.cmp(&xmp) {
                std::cmp::Ordering::Less => meta,
                std::cmp::Ordering::Equal => meta,
                std::cmp::Ordering::Greater => xmp,
            }),
        }
    }
}

impl PartialEq for Media {
    fn eq(&self, other: &Self) -> bool {
        self.uuid.eq(&other.uuid)
    }
}
