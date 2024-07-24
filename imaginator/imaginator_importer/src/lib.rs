pub mod adapters;
pub mod errors;

use adapters::filesystem_adapter::get_media_with_xmp;
use errors::MediaReadErr;
use imaginator_types::media::Media;

pub struct ReadMediaDirectory {
    pub media: Vec<Media>,
    pub errors: Vec<MediaReadErr>,
}

pub fn scan_path(path: String) -> ReadMediaDirectory {
    get_media_with_xmp(&path).unwrap()
}
