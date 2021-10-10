#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate juniper;

mod model;
mod schema;
// mod schema_old;

use serde::{Deserialize, Serialize};
use juniper::{EmptyMutation, RootNode};
// use model::Database;
use rocket::response::content;
use rocket::State;
// use schema::Query;

// mod schema;

use mongodb::{bson::doc,bson::Document , options::ClientOptions, sync::Client, bson::oid::ObjectId};
use std::env;
use dotenv::dotenv;
use lazy_static::lazy_static;
// use async_once::AsyncOnce;

#[macro_use] extern crate rocket;

// static LOCAL_DB:model::Database = model::Database::new();

lazy_static! {
  static ref MONGO_DATABASE: mongodb::sync::Database = {
    dotenv().expect(".env not found");
    let url:String = env::var("MONGO_URL").unwrap();
    let client:mongodb::sync::Client = Client::with_uri_str(url).unwrap();
    let database = client.database("resources");
    database
  };
  #[derive(Debug)]
  static ref LOCAL_DB:model::Database = model::Database::new();
}

fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>())
}
fn main() {
  // let objects = MONGO_DATABASE.collection("name")
  // println!("{:#?}", LOCAL_DB);
  println!("{:#?}", model::Database::new());
}