#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate juniper;
#[macro_use]
extern crate rocket;
mod model;
mod schema;

use model::Database;
use rocket::{State, http::Header, response::content};
use serde::{Deserialize, Serialize};

use dotenv::dotenv;
use lazy_static::lazy_static;
use mongodb::{
    bson::{doc, oid::ObjectId, serde_helpers},
    sync::Client,
};
use std::env;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};


lazy_static! {
  static ref MONGO_DATABASE: mongodb::sync::Database = {
    dotenv().expect(".env not found");
    let url:String = env::var("MONGO_URL").unwrap();
    let client:mongodb::sync::Client = Client::with_uri_str(url).unwrap();
    let database = client.database("resources");
    database
  };
}


pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
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

#[options("/graphql")]
fn options_graphql_handler() {
	
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
        .attach(CORS)
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler, options_graphql_handler],
        )
}
