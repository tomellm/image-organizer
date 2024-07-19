use imaginator_types::xmpdata::XmpData;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::util::DatabaseUtilities;

use super::{FromDBUuid, IntoDBUuid};

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MediaXmpData {
    pub uuid: String,
    pub media_uuid: String,
    pub data_key: String,
    pub data_val: String,
}

pub struct XmpDataWithParent(pub XmpData, pub String);

impl XmpDataWithParent {
    fn from_ref((data, parent): (XmpData, &String)) -> Self {
        Self(data, parent.to_owned())
    }
}

impl From<(XmpData, &String)> for MediaXmpData {
    fn from((data, parent): (XmpData, &String)) -> Self {
        MediaXmpData {
            uuid: data.uuid.into_db(),
            media_uuid: parent.to_owned(),
            data_key: data.key,
            data_val: data.val,
        }
    }
}

impl TryFrom<MediaXmpData> for XmpData {
    type Error = ();
    fn try_from(value: MediaXmpData) -> Result<Self, Self::Error> {
        Ok(XmpData {
            uuid: Uuid::from_db(&value.uuid)?,
            key: value.data_key,
            val: value.data_val,
        })
    }
}

impl DatabaseUtilities for MediaXmpData {
    fn db_table_name() -> &'static str {
        "xmp_data"
    }
    fn db_column_names() -> &'static [&'static str] {
        &["uuid", "media_uuid", "data_key", "data_val"]
    }
    fn db_push_touple_fn(
    ) -> impl FnMut(sqlx::query_builder::Separated<'_, '_, sqlx::MySql, &'static str>, Self) {
        |mut b, xmp| {
            b.push_bind(xmp.uuid);
            b.push_bind(xmp.media_uuid);
            b.push_bind(xmp.data_key);
            b.push_bind(xmp.data_val);
        }
    }
}
