#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

use crate::graphql::schema::{Mutation, Query, Schema};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::env;

mod graphql;
mod schema;
mod util;
mod video;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate juniper;

pub struct Context {
    pool: Pool<ConnectionManager<PgConnection>>,
}

fn main() {
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .expect("failed to build connection pool");
    let context = Context { pool };

    rocket::ignite()
        .manage(context)
        .manage(Schema::new(Query, Mutation))
        .mount(
            "/",
            routes![graphql::handler::graphiql, graphql::handler::graphql],
        )
        .attach(
            rocket_cors::CorsOptions {
                ..Default::default()
            }
            .to_cors()
            .expect("error while building CORS"),
        )
        .launch();
}
