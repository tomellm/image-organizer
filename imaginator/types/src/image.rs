use serde::{Serialize, Deserialize};
use sqlx::{query_builder::Separated, MySql};
#[allow(dead_code)]

use uuid::Uuid;

use crate::{database::*, args::{ImageCreateArgs, MetaCreateArgs, XmpCreateArgs}};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    pub uuid: Uuid,
    pub original_name: String,
    pub current_name: String,
    pub extension: String,
    pub meta_data: Vec<MetaData>,
    pub xmp_data: Vec<XmpData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetaData {
    pub uuid: Uuid,
    pub key: String,
    pub val: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XmpData {
    pub uuid: Uuid,
    pub key: String,
    pub val: String
}

impl Image {
    pub fn from_args(args: ImageCreateArgs) -> Self {
        Self { 
            uuid: Uuid::new_v4(),
            original_name: args.original_name,
            current_name: args.current_name,
            extension: args.extension,
            meta_data: args.meta_data.into_iter().map(MetaData::from_args).collect(),
            xmp_data: args.xmp_data.into_iter().map(XmpData::from_args).collect()
        }
    }
    pub fn to_db(
        self
    ) -> (ImageData, Vec<ImageMetaData>,Vec<ImageXmpData>) {
        let str_image_uuid = self.uuid.simple().to_string();

        let meta_data = self.meta_data.into_iter()
            .map(|m| m.to_db(&str_image_uuid))
            .collect::<Vec<_>>();
        let xmp_data = self.xmp_data.into_iter()
            .map(|x| x.to_db(&str_image_uuid))
            .collect::<Vec<_>>();


        let image_data = ImageData {
            uuid: self.uuid.simple().to_string(),
            original_name: self.original_name,
            current_name: self.current_name,
            extension: self.extension,
        };

        (image_data, meta_data, xmp_data)
    }
}

impl MetaData {
    pub fn from_args(args: MetaCreateArgs) -> Self {
        Self { 
            uuid: Uuid::new_v4(), 
            key: args.key, 
            val: args.val
        }
    }
    pub fn to_db(self, image_uuid: &String) -> ImageMetaData {
        ImageMetaData { 
            uuid: self.uuid.simple().to_string(),
            image_uuid: image_uuid.to_owned(),
            data_key: self.key,
            data_val: self.val
        }
    }
}

impl XmpData {
    pub fn from_args(args: XmpCreateArgs) -> Self {
        Self { 
            uuid: Uuid::new_v4(),
            key: args.key,
            val: args.val
        }
    }
    pub fn to_db(self, image_uuid: &String) -> ImageXmpData {
        ImageXmpData { 
            uuid: self.uuid.simple().to_string(),
            image_uuid: image_uuid.to_owned(),
            data_key: self.key,
            data_val: self.val
        }
    }
}
