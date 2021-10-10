// mod types;

// use mongodb::{bson::doc, options::ClientOptions, Client};
// use std::env;
// use dotenv::dotenv;
// use async_once::AsyncOnce;
// use lazy_static::lazy_static;
// #[macro_use] extern crate rocket;

// lazy_static! {
//   static ref CLIENT: AsyncOnce<Client> = AsyncOnce::new(async {
//     dotenv().expect(".env file not found");
//     let uri = env::var("LOCAL_URL").unwrap();
//     let client_options = ClientOptions::parse(&uri).await;
//     let client = Client::with_options(client_options)?;
//     let client = Client::with_uri_str(&uri).await.unwrap();

//     client
//   });
// }

// // #[tokio::main]
// // async fn connect_db() -> mongodb::error::Result<()> {
// //   let mut client_options = ClientOptions::parse(get_url("MONGO_URL")).await?;
// //   // client_options.app_name= Some("AnC Resources".to_string());
// //   println!("Reached");
// //   let client = Client::with_options(client_options)?;
// //   let db = client.database("development");

// //   for collection_name in db.list_collection_names(None).await? {
// //       println!("{}", collection_name);
// //   }
// //   return Ok(());
// // }

// #[get("/resources")]
// async fn resources() {
//   let db = CLIENT.get().await.database("resourcesDB");
//   let collection_name = db.list_collection_names(None).await;
//   println!("{:#?}", collection_name);
// }

// #[launch] 
// async fn rocket() -> _{
  
//   rocket::build().mount("/", routes![resources])
// }

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate juniper;

mod model;
mod schema;
// mod schema_old;

use juniper::{EmptyMutation, RootNode};
// use model::Database;
use rocket::response::content;
use rocket::State;
// use schema::Query;

// mod schema;

use mongodb::{bson::doc,bson::Document , options::ClientOptions, Client, Database};
use std::env;
use dotenv::dotenv;
use async_once::AsyncOnce;
use lazy_static::lazy_static;

#[macro_use] extern crate rocket;

// static LOCAL_DB:model::Database = model::Database::new();

lazy_static! {
  static ref DB: AsyncOnce<Database> = AsyncOnce::new( async {
    dotenv().expect(".env not found");
    let url = env::var("MONGO_URL").unwrap();
    let client_options = ClientOptions::parse(&url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("resources");
    db
  });
  // static ref LOCAL_DB:AsyncOnce<model::Database> = AsyncOnce::new(async {
  //   model::Database::new()
  // });
  static ref LOCAL_DB:model::Database = model::Database::new();
}

#[get("/resources")] 
async fn resources() {
  let collection_name = DB.get().await.list_collection_names(None).await.unwrap();
  for i in collection_name {
    println!("{}", i);
  }
}


#[launch] 
async fn rocket() -> _{
  let _collection_name = DB.get().await.list_collection_names(None).await.unwrap();
  rocket::build().mount("/", routes![resources])
}