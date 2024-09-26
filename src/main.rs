use mongodb::Client;
use std::error::Error;
use tokio;
use mongodb::bson::doc;
use bson::Document;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  for name in client.list_database_names().await? {
    println!("- {}", name);
  }
  let skills = client.database("relic").collection("skills");

  // let new_doc = doc! {
  //   "title": "Parasite",
  //   "year": 2020,
  //   "plot": "Blarg",
  // };
  // let insert_result = movies.insert_one( new_doc.clone() ).await?;
  // println!("New document ID: {}", insert_result.inserted_id);

  let skill: Document = skills
    .find_one(
      doc! {},
    ).await?
    .expect( "No skills found" );
  println!("Skill: {}", skill);

  // let ready_skills = skills.aggregate(pipeline)

  Ok(())
}