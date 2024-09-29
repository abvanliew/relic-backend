use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
pub enum Tier {
  Initiate,
  Journeyman,
  Master,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EditingState {
  Ready,
  Editing,
  #[serde(rename = "In Progress")]
  InProgress,
  Concept,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
  #[serde(rename = "_id")]
  oid: ObjectId,
  tier: Tier,
  title: String,
  description: String,
  training_cost_id: ObjectId,
  training_cost: Option<String>,
  action: Action,
  order: Ordering,
  paths: Option<Vec<PathRef>>,
  editing_state: EditingState,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Action {
  type_id: ObjectId,
  #[serde(rename = "type")]
  act_type: Option<String>,
  mana_cost: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ordering {
  category: i32,
  training_cost: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PathRef {
  id: ObjectId,
  title: String,
}