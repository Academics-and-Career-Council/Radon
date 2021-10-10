use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pdfs {
  name: String,
  cat: String,
  link: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
  name: String,
  link: String
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Resources {
  
}
