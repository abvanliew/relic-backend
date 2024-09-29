mod skills;

use skills::Skill;

use std::error::Error;
use tokio;

use mongodb::{Client, Collection};
use mongodb::bson::doc;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;

  let skills_collection: Collection<Skill> = client.database("relic").collection("skills_paths");

  let mut results = skills_collection
  .find(
    doc! {},
  ).await?;
  while let Some(doc) = results.next().await {
    println!("{:?}", doc?)
  }

  Ok(())
}
