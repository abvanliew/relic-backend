mod skills;

use skills::Skill;
use std::error::Error;
use std::fmt::{Display, Formatter};

use mongodb::{Client, Collection};
use mongodb::bson::doc;
use bson::oid::ObjectId;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() {
  let _x = get_skills().await;
}

async fn get_skills() -> Result<(), Box<dyn Error>> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  let skills_collection: Collection<Skill> = client.database("relic").collection("skills_paths");
  let Ok( id ) = ObjectId::parse_str( "66f45f6b7e098103fc4220a1" ) else { return Err( Box::new(SkillError) ); };
  let mut results = skills_collection
  .find( doc! { "_id": id }, ).await?;
  while let Some(doc) = results.next().await {
    println!("{:?}", doc?)
  }
  Ok(())
}

#[derive(Debug, Clone)]
struct SkillError;

impl Display for SkillError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!( f, "unable to find skill" )
  }
}

impl Error for SkillError {}