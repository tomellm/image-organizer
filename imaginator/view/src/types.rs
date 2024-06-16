use serde::{Serialize, Deserialize};
use types::image::Media;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MediaPage {
    Page(Vec<Media>),
    Error(String),
    Final
}

impl MediaPage {
    pub fn from_res_opt(val: Result<Option<Vec<Media>>, String>) -> Self {
        if val.as_ref().is_err() {
            Self::Error(val.err().unwrap())
        } else if val.as_ref().unwrap().is_none() {
            Self::Final
        } else {
            Self::Page(val.unwrap().unwrap())
        }
    }
}
