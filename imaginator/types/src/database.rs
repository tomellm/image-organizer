#[allow(dead_code)]
use sqlx::types::chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageData{
    pub id: i32,
    pub group_id: Option<i32>,
    pub file_name: String,
    pub org_name: String,
    pub datetime: Option<NaiveDateTime>,
    pub file_size: i32 
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateImageData{
    pub group_id: Option<i32>,
    pub file_name: String,
    pub org_name: String,
    pub datetime: Option<NaiveDateTime>,
    pub file_size: i32
}
