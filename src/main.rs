use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::env;

mod schema;
mod video;

#[macro_use]
extern crate diesel;

pub struct Context {
    pool: Pool<ConnectionManager<PgConnection>>,
}

fn main() {
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .expect("failed to build connection pool");
    let context = Context { pool };

    println!("{:?}", video::model::Video::all(&context));
}
