use crate::schema::videos::dsl::videos;
use crate::video::entity as E;
use crate::Context;
use diesel::prelude::*;

#[derive(Debug)]
pub struct Video {
    pub src: String,
}

impl Video {
    pub fn all(context: &Context) -> Result<Vec<Video>, String> {
        let conn = context.pool.get().map_err(|e| e.to_string())?;
        let e_videos: Vec<E::Video> = videos.load(&conn).map_err(|e| e.to_string())?;

        Ok(e_videos
            .into_iter()
            .map(|e_video| Video { src: e_video.src })
            .collect())
    }
}
