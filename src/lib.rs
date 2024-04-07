use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use serde_json::json;
use surrealdb::engine::local::Mem;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Error;
use surrealdb::Surreal;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsError;
use web_sys::console;

#[derive(Debug, Serialize, Deserialize)]
struct Name {
    first: String,
    last: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    #[serde(skip_serializing)]
    id: Option<Thing>,
    title: String,
    name: Name,
    marketing: bool,
}

#[wasm_bindgen]
pub fn log_thing() -> Result<(), JsError> {
    console::log_1(&"Hello using web-sys".into());
    Ok(())
}

#[wasm_bindgen]
pub async fn run() -> Result<String, JsError> {
    let db = Surreal::new::<surrealdb::engine::remote::ws::Ws>("localhost:8000").await?;

    console::log_1(&"Hello using web-sys in an async context".into());
    //let db = Surreal::new::<Mem>(()).await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace and database
    db.use_ns("namespace").use_db("database").await?;

    // Create a new person with a random ID
    let tobie: Vec<Person> = db
        .create("person")
        .content(Person {
            id: None,
            title: "Founder & CEO".into(),
            name: Name {
                first: "Tobie".into(),
                last: "Morgan Hitchcock".into(),
            },
            marketing: true,
        })
        .await?;

    // Create a new person with a specific ID
    let mut jaime: Option<Person> = db
        .create(("person", "jaime"))
        .content(Person {
            id: None,
            title: "Founder & COO".into(),
            name: Name {
                first: "Jaime".into(),
                last: "Morgan Hitchcock".into(),
            },
            marketing: false,
        })
        .await?;

    // Update a person record with a specific ID
    jaime = db
        .update(("person", "jaime"))
        .merge(json!({ "marketing": true }))
        .await?;

    // Select all people records
    let people: Vec<Person> = db.select("person").await?;

    console::log_1(&format!("{:?}", jaime).into());
    console::log_1(&format!("{:?}", people).into());

    Ok("Success".to_string())
}
