pub(crate) mod xmpdata;
pub(crate) mod metadata;
pub(crate) mod mediatype;


use chrono::{DateTime, Utc};
use imaginator_types::media::Media;
use mediatype::DBEnum;
use metadata::MediaMetaData;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use xmpdata::MediaXmpData;

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MediaData {
    pub uuid: String,
    pub original_name: String,
    pub current_name: String,
    pub extension: String,
    pub media_type: i8,
    pub datetime_created: Option<DateTime<Utc>>,
}



pub struct MediaUnwrapped(pub MediaData, pub Vec<MediaMetaData>, pub Vec<MediaXmpData>);

impl From<MediaUnwrapped> for Media {
    fn from(MediaUnwrapped(media, meta_data, xmp_data): MediaUnwrapped) -> Self {
        let meta_data = meta_data
            .into_iter()
            .map(|md| md.try_into().unwrap())
            .collect::<Vec<_>>();

        let xmp_data = xmp_data
            .into_iter()
            .map(|xd| xd.try_into().unwrap())
            .collect::<Vec<_>>();

        Media {
            uuid: Uuid::from_db(&media.uuid).unwrap(),
            original_name: media.original_name,
            current_name: media.current_name,
            extension: media.extension,
            meta_data,
            xmp_data,
            media_type: DBEnum::from(media.media_type).try_into().unwrap(),
            datetime_created: media.datetime_created,
        }
    }
}

impl From<Media> for MediaUnwrapped {
    fn from(media: Media) -> Self {
        let Media {
            uuid,
            original_name,
            current_name,
            extension,
            meta_data,
            xmp_data,
            media_type,
            datetime_created,
        } = media;
        let str_image_uuid = uuid.into_db();

        let meta_data = meta_data
            .into_iter()
            .map(|m| MediaMetaData::from((m, &str_image_uuid)))
            .collect::<Vec<_>>();
        let xmp_data = xmp_data
            .into_iter()
            .map(|x| MediaXmpData::from((x, &str_image_uuid)))
            .collect::<Vec<_>>();

        let image_data = MediaData {
            uuid: str_image_uuid,
            original_name,
            current_name,
            extension,
            media_type: *DBEnum::from(media_type),
            datetime_created,
        };

        MediaUnwrapped(image_data, meta_data, xmp_data)
    }
}

pub(crate) trait FromDBUuid {
    fn from_db(value: &String) -> Result<Uuid, ()>;
}

impl FromDBUuid for Uuid {
    fn from_db(value: &String) -> Result<Uuid, ()> {
        Uuid::parse_str(value).or(Err(()))
    }
}

pub(crate) trait IntoDBUuid {
    fn into_db(self) -> String;
}

impl IntoDBUuid for Uuid {
    fn into_db(self) -> String {
        self.simple().to_string()
    }
}
