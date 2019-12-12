use crate::video::decorator::VideoDecorator;
use crate::video::model::Video;
use crate::Context;
use juniper::{FieldError, FieldResult, RootNode, Value};

pub struct Query;

graphql_object!(Query: Context |&self| {
    field videos(&executor) -> FieldResult<Vec<VideoDecorator>> {
        Video::all(executor.context()).map(|videos| videos.iter().map(|video|VideoDecorator::from(&video)).collect()).map_err(|e| FieldError::new(e, Value::null()))
    }
});

pub struct Mutation;

graphql_object!(Mutation: Context | &self | {});

pub type Schema = RootNode<'static, Query, Mutation>;
