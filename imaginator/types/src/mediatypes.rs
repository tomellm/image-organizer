use core::panic;
use std::fmt;
use std::fmt::{Display, Formatter};

#[allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MediaType {
    Image(ImageType),
    Video(VideoType),
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ImageType {
    HEIC = 0,
    PNG = 1,
    JPG = 2,
    JPEG = 3,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VideoType {
    MOV = 0,
    MP4 = 1,
}

impl MediaType {
    pub fn from_ext(ext: &str) -> Self {
        match (ImageType::from_ext(&ext), VideoType::from_ext(&ext)) {
            (Some(_), Some(_)) => panic!("There has been a mistake and this extension can be found in both the video and image types!"),
            (Some(image), None) => Self::Image(image),
            (None, Some(video)) => Self::Video(video),
            _ => Self::Unknown
        }
    }

    pub fn from_i8(num: i8) -> Option<Self> {
        let internal_num = num % 10;
        match num / 10 {
            0 => Some(Self::Image(ImageType::from_i8(internal_num)?)),
            10 => Some(Self::Video(VideoType::from_i8(internal_num)?)),
            20 => Some(Self::Unknown),
            _ => None
        }
    }

    pub fn to_db(self) -> i8 {
        match self {
            Self::Image(image) => 0 + image as i8,
            Self::Video(video) => 10 + video as i8,
            Self::Unknown => 20
        }
    }

    pub fn get_image_types_u8() -> Vec<i8> {
        ImageType::get_all().into_iter().map(|t| t as i8).collect()
    }

    pub fn get_video_types_u8() -> Vec<i8> {
        VideoType::get_all().into_iter().map(|t| (t as i8) + 10).collect()
    }

}


impl ImageType {
    pub fn from_ext(ext: &str) -> Option<Self> {
        match ext.to_uppercase().as_str() {
            "HEIC" => Some(Self::HEIC),
            "PNG" => Some(Self::PNG),
            "JPG" => Some(Self::JPG),
            "JPEG" => Some(Self::JPEG),
            _ => None
        }
    }

    pub fn from_i8(num: i8) -> Option<Self> {
        match num {
            0 => Some(Self::HEIC),
            1 => Some(Self::PNG),
            2 => Some(Self::JPG),
            3 => Some(Self::JPEG),
            _ => None
        }
    }

    pub fn get_all() -> Vec<Self> {
        vec![Self::HEIC, Self::PNG, Self::JPG, Self::JPEG]
    }
    
    
}

impl Display for ImageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}

impl VideoType {
    pub fn from_ext(ext: &str) -> Option<Self> {
        match ext.to_uppercase().as_str() {
            "MOV" => Some(Self::MOV),
            "MP4" => Some(Self::MP4),
            _ => None
        }
    }

    pub fn from_i8(num: i8) -> Option<Self> {
        match num {
            0 => Some(Self::MOV),
            1 => Some(Self::MP4),
            _ => None
        }
    }

    pub fn get_all() -> Vec<Self> {
        vec![Self::MOV, Self::MP4]
    }
}

impl Display for VideoType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
