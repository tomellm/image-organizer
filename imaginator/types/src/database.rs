#[allow(dead_code)]
use serde::{Deserialize, Serialize};
use sqlx::{query_builder::Separated, MySql};
use uuid::Uuid;

use crate::image::{Image, XmpData, MetaData};

/*
create table images_data (
    uuid varchar(32) primary key not null, 
    original_name varchar(255) not null, 
    current_name varchar(255) not null, 
    extension varchar(20) not null
);
*/

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageData{
    pub uuid: String,
    pub original_name: String,
    pub current_name: String,
    pub extension: String
}

/*
create table image_metadata ( 
    uuid varchar(32) primary key not null, 
    image_uuid varchar(32) not null,
    data_key varchar(255) not null, 
    data_val tinytext not null
);
*/

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageMetaData {
    pub uuid: String,
    pub image_uuid: String,
    pub data_key: String,
    pub data_val: String
}

/*
create table image_xmpdata ( 
    uuid varchar(32) primary key not null, 
    image_uuid varchar(32) not null, 
    data_key varchar(255) not null, 
    data_val tinytext not null
);
 */

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageXmpData {
    pub uuid: String,
    pub image_uuid: String,
    pub data_key: String,
    pub data_val: String
}


impl ImageData {

    pub fn to_struct(
        self,
        meta_data: Vec<ImageMetaData>,
        xmp_data: Vec<ImageXmpData>
    ) -> Image {
        let meta_data = meta_data.into_iter()
            .map(|md| md.to_struct().unwrap())
            .collect::<Vec<_>>();

        let xmp_data = xmp_data.into_iter()
            .map(|xd| xd.to_struct().unwrap())
            .collect::<Vec<_>>();

        println!("this is the uuid {}", self.uuid);

        Image { 
            uuid: Uuid::parse_str(&self.uuid).unwrap(), 
            original_name: self.original_name, 
            current_name: self.current_name, 
            extension: self.extension, 
            meta_data, 
            xmp_data 
        }
    }
}

impl ImageMetaData {
    pub fn to_struct(self) -> Result<MetaData, ()> {
        Ok(MetaData { 
            uuid: Uuid::parse_str(&self.uuid).or(Err(()))?,
            key: self.data_key,
            val: self.data_val
        })
    }


}

impl ImageXmpData {
    pub fn to_struct(self) -> Result<XmpData, ()> {
        Ok(XmpData { 
            uuid: Uuid::parse_str(&self.uuid).or(Err(()))?,
            key: self.data_key,
            val: self.data_val
        })
    }
}
