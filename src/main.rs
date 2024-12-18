#![allow(non_snake_case)]

mod skills;

use skills::Skill;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use std::error::Error;
use tokio;

use mongodb::{Client, Collection};
use mongodb::bson::doc;
use futures::stream::StreamExt;

use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;

fn main() {
  dioxus_logger::init( Level::INFO ).expect( "failed to init logger" );
  info!( "starting app" );
  launch( App );
}

fn App() -> Element {
  rsx! {
    Router::<Route> {}
  }
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
  #[route("/")]
  Home {},
  #[route("/skill/:id")]
  Skill { id: String },
}

#[component]
fn Skill( id: String ) -> Element {
  rsx! {
    Link { to: Route::Home {}, "Go to counter" }
    "Skill {id}"
  }
}

#[component]
fn Home() -> Element {
  let mut count = use_signal(|| 0);

  rsx! {
    Link {
      to: Route::Skill { id: "id".into() },
      "Go to skill"
    }
    div {
      h1 { "Count: {count}" }
      button { onclick: move |_| count += 1, "Up" }
      button { onclick: move |_| count -= 1, "Down" }
    }
  }
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//   let client = Client::with_uri_str("mongodb://localhost:27017").await?;

//   let skills_collection: Collection<Skill> = client.database("relic").collection("skills_paths");

//   let mut results = skills_collection
//   .find(
//     doc! {},
//   ).await?;
//   while let Some(doc) = results.next().await {
//     println!("{:?}", doc?)
//   }

//   Ok(())
// }
