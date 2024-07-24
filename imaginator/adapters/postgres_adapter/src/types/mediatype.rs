use std::ops::Deref;

use imaginator_types::mediatypes::{ImageType, MediaType, VideoType};

pub struct DBEnum(i8);

impl DBEnum {
    pub fn get_image_types_u8() -> Vec<i8> {
        ImageType::get_all().into_iter().map(|t| t as i8).collect()
    }

    pub fn get_video_types_u8() -> Vec<i8> {
        VideoType::get_all()
            .into_iter()
            .map(|t| (t as i8) + 10)
            .collect()
    }
}

impl From<i8> for DBEnum {
    fn from(value: i8) -> Self {
        DBEnum(value)
    }
}

impl Deref for DBEnum {
    type Target = i8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<MediaType> for DBEnum {
    fn from(value: MediaType) -> Self {
        match value {
            MediaType::Image(image) => 0i8 + image as i8,
            MediaType::Video(video) => 10i8 + video as i8,
            MediaType::Unknown => 20i8,
        }
        .into()
    }
}

impl TryFrom<DBEnum> for MediaType {
    type Error = ();
    fn try_from(value: DBEnum) -> Result<Self, Self::Error> {
        let internal_num = *value % 10;
        match *value / 10 {
            0 => Ok(Self::Image(ImageType::try_from(DBEnum::from(
                internal_num,
            ))?)),
            1 => Ok(Self::Video(VideoType::try_from(DBEnum::from(
                internal_num,
            ))?)),
            2 => Ok(Self::Unknown),
            _ => Err(()),
        }
    }
}

impl TryFrom<DBEnum> for ImageType {
    type Error = ();
    fn try_from(value: DBEnum) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(Self::HEIC),
            1 => Ok(Self::PNG),
            2 => Ok(Self::JPG),
            3 => Ok(Self::JPEG),
            _ => Err(()),
        }
    }
}

impl TryFrom<DBEnum> for VideoType {
    type Error = ();
    fn try_from(value: DBEnum) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(Self::MOV),
            1 => Ok(Self::MP4),
            _ => Err(()),
        }
    }
}
