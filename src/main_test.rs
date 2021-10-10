mod types;

use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;
use dotenv::dotenv;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
#[macro_use] extern crate rocket;

lazy_static! {
  static ref CLIENT: AsyncOnce<Client> = AsyncOnce::new(async {
    dotenv().expect(".env file not found");
    let uri = env::var("LOCAL_URL").unwrap();
    let client_options = ClientOptions::parse(&uri).await;
    let client = Client::with_options(client_options)?;
    let client = Client::with_uri_str(&uri).await.unwrap();

    client
  });
}

// #[tokio::main]
// async fn connect_db() -> mongodb::error::Result<()> {
//   let mut client_options = ClientOptions::parse(get_url("MONGO_URL")).await?;
//   // client_options.app_name= Some("AnC Resources".to_string());
//   println!("Reached");
//   let client = Client::with_options(client_options)?;
//   let db = client.database("development");

//   for collection_name in db.list_collection_names(None).await? {
//       println!("{}", collection_name);
//   }
//   return Ok(());
// }

#[get("/resources")]
async fn resources() {
  let db = CLIENT.get().await.database("resourcesDB");
  let collection_name = db.list_collection_names(None).await;
  println!("{:#?}", collection_name);
}

#[launch] 
async fn rocket() -> _{
  
  rocket::build().mount("/", routes![resources])
}

