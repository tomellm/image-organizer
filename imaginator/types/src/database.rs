use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
#[allow(dead_code)]
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::image::Media;
use crate::mediatypes::MediaType;
use crate::metadata::*;
use crate::xmpdata::*;

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MediaData{
    pub uuid: String,
    pub original_name: String,
    pub current_name: String,
    pub extension: String,
    pub media_type: i8,
    pub datetime_created: Option<DateTime<Utc>>
}

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MediaMetaData {
    pub uuid: String,
    pub media_uuid: String,
    pub data_key: String,
    pub data_val: String
}

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MediaXmpData {
    pub uuid: String,
    pub media_uuid: String,
    pub data_key: String,
    pub data_val: String
}


impl MediaData {

    pub fn to_struct(
        self,
        meta_data: Vec<MediaMetaData>,
        xmp_data: Vec<MediaXmpData>
    ) -> Media {
        let meta_data = meta_data.into_iter()
            .map(|md| md.to_struct().unwrap())
            .collect::<Vec<_>>();

        let xmp_data = xmp_data.into_iter()
            .map(|xd| xd.to_struct().unwrap())
            .collect::<Vec<_>>();


        Media { 
            uuid: Uuid::parse_str(&self.uuid).unwrap(), 
            original_name: self.original_name, 
            current_name: self.current_name, 
            extension: self.extension, 
            meta_data, 
            xmp_data,
            media_type: MediaType::from_i8(self.media_type).unwrap(),
            datetime_created: self.datetime_created
        }
    }
}

impl MediaMetaData {
    pub fn to_struct(self) -> Result<MetaData, ()> {
        Ok(MetaData { 
            uuid: Uuid::parse_str(&self.uuid).or(Err(()))?,
            key: self.data_key,
            val: self.data_val
        })
    }
}

impl MediaXmpData {
    pub fn to_struct(self) -> Result<XmpData, ()> {
        Ok(XmpData { 
            uuid: Uuid::parse_str(&self.uuid).or(Err(()))?,
            key: self.data_key,
            val: self.data_val
        })
    }
}
