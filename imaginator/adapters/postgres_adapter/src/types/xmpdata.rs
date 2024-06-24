use imaginator_types::xmpdata::XmpData;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
