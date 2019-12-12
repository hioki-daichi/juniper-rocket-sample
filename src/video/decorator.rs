use crate::util::s3::get_presigned_url;
use crate::video::model::Video;

#[derive(Debug, GraphQLObject)]
pub struct VideoDecorator {
    pub src: String,
}

impl VideoDecorator {
    pub fn from(video: &Video) -> Self {
        Self {
            src: get_presigned_url("videos".to_owned(), video.src.clone()),
        }
    }
}
