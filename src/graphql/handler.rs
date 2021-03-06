use crate::graphql::schema::Schema;
use crate::Context;
use juniper_rocket::{GraphQLRequest, GraphQLResponse};
use rocket::State;

#[post("/graphql", data = "<request>")]
pub fn graphql(
    context: State<Context>,
    request: GraphQLRequest,
    schema: State<Schema>,
) -> GraphQLResponse {
    request.execute(&schema, &context)
}

#[get("/")]
pub fn graphiql() -> rocket::response::content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}
