use crate::schema::videos;
use crate::video::entity as E;
use crate::Context;
use diesel::prelude::*;

#[derive(Debug)]
pub struct Video {
    pub src: String,
}

#[derive(Insertable)]
#[table_name = "videos"]
pub struct NewVideo {
    pub src: String,
}

impl Video {
    pub fn all(context: &Context) -> Result<Vec<Video>, String> {
        let conn = context.pool.get().map_err(|e| e.to_string())?;
        let e_videos: Vec<E::Video> = videos::dsl::videos.load(&conn).map_err(|e| e.to_string())?;

        Ok(e_videos
            .into_iter()
            .map(|e_video| Video { src: e_video.src })
            .collect())
    }

    pub fn create(context: &Context, src: String) -> Result<Video, String> {
        let conn = context.pool.get().map_err(|e| e.to_string())?;

        let new_video = NewVideo { src };

        let e_video: E::Video = diesel::insert_into(videos::table)
            .values(&new_video)
            .get_result(&conn)
            .map_err(|e| e.to_string())?;

        Ok(Video { src: e_video.src })
    }
}
