mod types;

use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use std::env;
use dotenv::dotenv;
use async_once::AsyncOnce;
use lazy_static::lazy_static;

#[macro_use] extern crate rocket;

lazy_static! {
  static ref DB: AsyncOnce<Database> = AsyncOnce::new( async {
    dotenv().expect(".env not found");
    let url = env::var("MONGO_URL").unwrap();
    let client_options = ClientOptions::parse(&url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("resources");
    db
  });
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