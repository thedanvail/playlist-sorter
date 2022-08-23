pub struct Line<'a, T> {
    header: &'a str,
    data: Option<T>
}

impl<T> Line<'_, T> {
    pub fn new(header: Headers) -> Self {
        match header {
            Headers::M3U => Self { header: "#EXTM3U", data: None },
            Headers::IndependentSegments => {
                Self {
                    header: "#EXT-X-INDEPENDENT-SEGMENTS",
                    data: None
                }
            },
            Headers::Media(media_data) => {
                Self {
                    header: "#EXT-X-MEDIA",
                    data: None
                }
            },
            Headers::StreamInfo(_) => todo!(),
            Headers::IFrameStreamInfo(_) => todo!(),
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

struct IFrameStreamInfoData {
    bandwidth: i32,
    codecs: Vec<String>,
    resolution: (i32, i32),
    video_range: String,
    uri: String
}
