use serde::{Serialize, Deserialize};



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageCreateArgs{
    pub original_name: String,
    pub current_name: String,
    pub extension: String,
    pub xmp_data: Vec<XmpCreateArgs>,
    pub meta_data: Vec<MetaCreateArgs>
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
