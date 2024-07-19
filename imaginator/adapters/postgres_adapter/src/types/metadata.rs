use imaginator_types::metadata::MetaData;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::util::DatabaseUtilities;

use super::{FromDBUuid, IntoDBUuid};

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MediaMetaData {
    pub uuid: String,
    pub media_uuid: String,
    pub data_key: String,
    pub data_val: String,
}

pub struct MetaDataWithParent(pub MetaData, pub String);

impl MetaDataWithParent {
    fn from_ref((data, parent): (MetaData, &String)) -> Self {
        Self(data, parent.to_owned())
    }
}

impl From<(MetaData, &String)> for MediaMetaData {
    fn from((data, parent): (MetaData, &String)) -> Self {
        Self {
            uuid: data.uuid.into_db(),
            media_uuid: parent.to_owned(),
            data_key: data.key,
            data_val: data.val,
        }
    }
}

impl TryFrom<MediaMetaData> for MetaData {
    type Error = ();
    fn try_from(data: MediaMetaData) -> Result<Self, Self::Error> {
        Ok(Self {
            uuid: Uuid::from_db(&data.uuid)?,
            key: data.data_key,
            val: data.data_val,
        })
    }
}

impl DatabaseUtilities for MediaMetaData {
    fn db_table_name() -> &'static str {
        "meta_data"
    }
    fn db_column_names() -> &'static [&'static str] {
        &["uuid", "media_uuid", "data_key", "data_val"]
    }
    fn db_push_touple_fn(
    ) -> impl FnMut(sqlx::query_builder::Separated<'_, '_, sqlx::MySql, &'static str>, Self) {
        |mut b, meta| {
            b.push_bind(meta.uuid);
            b.push_bind(meta.media_uuid);
            b.push_bind(meta.data_key);
            b.push_bind(meta.data_val);
        }
    }
}
