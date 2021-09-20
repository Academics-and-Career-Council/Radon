use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;
use dotenv::dotenv;


fn get_url(key: &str) -> String {
      // Accessing an env var
      dotenv().expect(".env file not found");
      match env::var(key) {
        Ok(val) => return val,
        Err(e) => return e.to_string(),
      }
    }


#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let mut client_options = ClientOptions::parse(get_url("LOCALURL")).await?;
    // client_options.app_name= Some("AnC Resources".to_string());

    let client = Client::with_options(client_options)?;
    let db = client.database("resourcesDB");

    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }

    Ok(())
}
