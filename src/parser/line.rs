use std::fmt;
use std::fmt::Formatter;

use crate::parser::{
    media_data::MediaData,
    stream_info::StreamInfoData,
    i_frame_stream_info_data::IFrameStreamInfoData,
    parse::Parse,
    headers
};

/// The following allows us to make the type of line generic so long as we enforce them all
/// implementing the same trait - in this case, Display, for the sake of potentially using
/// trait objects where necessary (which it is not at this time).
#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
pub struct Line<T>
where
    T: fmt::Display
{
    header: String,
    pub data: Box<T>
}

impl Line<MediaData> {
    pub fn media_data(data: String) -> Self {
        Self {
            header: String::from(headers::MEDIA_DATA),
            data: MediaData::from_string(data).unwrap()
        }
    }
}

impl Line<StreamInfoData> {
    pub fn stream_info_data(data: String) -> Self {
        Self {
            header: String::from(headers::STREAM_INFO_DATA),
            data: StreamInfoData::from_string(data).unwrap()
        }
    }
}

impl Line<IFrameStreamInfoData> {
    pub fn i_frame_stream_info_data(data: String) -> Self {
        Self {
            header: String::from(headers::I_FRAME_STREAM),
            data: IFrameStreamInfoData::from_string(data).unwrap()
        }
    }
}

impl<T: fmt::Display> fmt::Display for Line<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.header, self.data)
    }
}
