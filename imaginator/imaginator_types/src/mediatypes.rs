use core::panic;
use std::fmt;
use std::fmt::{Display, Formatter};

#[allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MediaType {
    Image(ImageType),
    Video(VideoType),
    Unknown,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ImageType {
    HEIC = 0,
    PNG = 1,
    JPG = 2,
    JPEG = 3,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

    pub fn is_video(&self) -> bool {
        match self {
            Self::Video(_) => true,
            _ => false,
        }
    }
    pub fn video(&self) -> Option<VideoType> {
        match self {
            Self::Video(video_type) => Some(*video_type),
            _ => None,
        }
    }
    pub fn is_image(&self) -> bool {
        match self {
            Self::Image(_) => true,
            _ => false,
        }
    }
    pub fn image(&self) -> Option<ImageType> {
        match self {
            Self::Image(image_type) => Some(*image_type),
            _ => None,
        }
    }
}

impl ImageType {
    pub fn from_ext(ext: &str) -> Option<Self> {
        match ext.to_uppercase().as_str() {
            "HEIC" => Some(Self::HEIC),
            "PNG" => Some(Self::PNG),
            "JPG" => Some(Self::JPG),
            "JPEG" => Some(Self::JPEG),
            _ => None,
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
            _ => None,
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
