use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio;
use mongodb::bson::doc;
use bson::Document;
use bson::oid::ObjectId;
use futures::stream::{StreamExt, TryStreamExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  deserialize();


  // let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  // for name in client.list_database_names().await? {
  //   println!("- {}", name);
  // }
  // let skills_collection: Collection<Skill> = client.database("relic").collection("skills_paths");

  // let mut results = skills_collection
  // .find(
  //   doc! {},
  // ).await?;
  // while let Some(doc) = results.next().await {
  //   println!("{:?}", doc?)
  // }

  Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Skill {
  // #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
  oid: ObjectId,
  title: String,
  description: String,
  tier: Tier,
  action: Action,
  editing_state: EditingState,
  training_cost_id: String,

  grade: u32,
  test_scores: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Action {
  #[serde( alias = "typeId" )]
  act_type_id: String,
  act_type: String
}

#[derive(Serialize, Deserialize, Debug)]
enum Tier {
  Initiate,
  Journeyman,
  Master,
}

#[derive(Serialize, Deserialize, Debug)]
enum EditingState {
  Ready,
  Editing,
  #[serde(rename = "In Progress")]
  InProgress,
  Concept,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Example {
  #[serde(rename = "_id")]
  oid: ObjectId,
  tier: Tier,
  editing_state: EditingState,
  sub: SubExample,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SubExample {
  #[serde(rename = "type")]
  act_type: String,
  type_id: String,
}

fn deserialize() {
  let example: String = 
    r#"{
      "_id": { "$oid": "66f45f6b7e098103fc4221d0" },
      "tier": "Initiate",
      "editingState": "In Progress",
      "sub": {
        "type": "BOX",
        "typeId": { "$oid": "66f45f6b7e098103fc4221d1" }
      }
    }"#
  .into();

  let deserialized:Result<Example, serde_json::Error> = serde_json::from_str( &example );
  match deserialized {
    Ok( obj ) => println!( "deserialized: {:?}", obj ),
    Err( err ) => println!( "failed: {:?}", err ),
  }
}
