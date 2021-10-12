#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate juniper;
mod model;
mod schema;

use model::Database;
use rocket::response::content;
use rocket::State;
use serde::{Deserialize, Serialize};

use dotenv::dotenv;
use lazy_static::lazy_static;
use mongodb::{
    bson::{doc, oid::ObjectId, serde_helpers},
    sync::Client,
};
use std::env;

#[macro_use]
extern crate rocket;

lazy_static! {
  static ref MONGO_DATABASE: mongodb::sync::Database = {
    dotenv().expect(".env not found");
    let url:String = env::var("MONGO_URL").unwrap();
    let client:mongodb::sync::Client = Client::with_uri_str(url).unwrap();
    let database = client.database("resources");
    database
  };
}

#[get("/graphql")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql", Some("/graphql"))
}

#[get("/graphql?<request>")]
async fn get_graphql_handler(
    context: &State<model::Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<schema::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context).await
}

#[post("/graphql", data = "<request>")]
async fn post_graphql_handler(
    context: &State<model::Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<schema::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context).await
}

#[launch]
fn rocket() -> _ {
    let _b = &MONGO_DATABASE;
    rocket::build()
        .manage(model::Database::new())
        .manage(schema::Schema::new(
            schema::Query,
            juniper::EmptyMutation::<model::Database>::new(),
            juniper::EmptySubscription::<Database>::new(),
        ))
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
}
