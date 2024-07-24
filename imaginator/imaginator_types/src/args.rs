use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetManyPayload {
    pub uuids: Vec<Uuid>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
}

impl Pagination {
    pub fn get_vals(&self) -> (u64, u64) {
        let offset = self.page * self.per_page;
        (offset as u64, self.per_page as u64)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageCreateArgs {
    pub original_name: String,
    pub current_name: String,
    pub extension: String,
    pub xmp_data: Vec<XmpCreateArgs>,
    pub meta_data: Vec<MetaCreateArgs>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XmpCreateArgs {
    pub key: String,
    pub val: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetaCreateArgs {
    pub key: String,
    pub val: String,
}
