use crate::parser::{
    media_data::MediaData,
    stream_info::StreamInfoData,
    i_frame_stream_info_data::IFrameStreamInfoData,
    parse::Parse
};

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone)]
pub struct Line<'a, T> {
    header: &'a str,
    data: Option<Box<T>>
}

impl Line<'_, MediaData> {
    pub fn media_data(data: String) -> Self {
        Self {
            header: "#EXT-X-MEDIA",
            data: Some(MediaData::from_string(data).unwrap())
        }
    }
}

impl Line<'_, StreamInfoData> {
    pub fn stream_info_data(data: String) -> Self {
        Self {
            header: "#EXT-X-STREAM-INF:",
            data: Some(StreamInfoData::from_string(data).unwrap())
        }
    }
}

impl Line<'_, IFrameStreamInfoData> {
    pub fn i_frame_stream_info_data(data: String) -> Self {
        Self {
            header: "#EXT-X-I-FRAME-STREAM-INF:",
            data: Some(IFrameStreamInfoData::from_string(data).unwrap())
        }
    }
}

impl<T> Line<'_, T> {
    pub fn new(header: &str) -> Self {
        Self {
            header,
            data: None
        }
    }
}

pub enum Headers {
    M3U,
    IndependentSegments,
    Media(String),
    StreamInfo(String),
    IFrameStreamInfo(String)
}
