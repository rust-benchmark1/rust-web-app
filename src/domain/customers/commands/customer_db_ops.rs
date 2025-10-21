// src/customer_db_ops.rs

use mongodb::{bson::{doc, DateTime}, Client};
use std::error::Error as StdError;
use futures::TryStreamExt;

/// Executes two MongoDB commands: first with a safe key, then with a tainted key (CWE-943).
pub async fn run_db_commands(keys: &[String]) -> Result<(), Box<dyn StdError>> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let db = client.database("appdb");

    if let Some(k0) = keys.get(0) {
        let cmd0 = doc! { "echo": k0.clone() };
        let _ = db.run_command(cmd0).await?;
    }

    if let Some(k1) = keys.get(1) {
        let cmd1 = doc! { "echo": k1.clone() };
        //SINK
        let _ = db.run_command(cmd1).await?;
    }

    Ok(())
}

/// Performs find_one_and_update twice: first with a safe key, then with a tainted key (CWE-943).
pub async fn find_and_update_each(keys: &[String]) -> Result<(), Box<dyn StdError>> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let db = client.database("appdb");
    let coll = db.collection::<mongodb::bson::Document>("customers");

    if let Some(k0) = keys.get(0) {
        let filter0 = doc! { "customer_id": k0 };
        let update0 = doc! { "$set": { "last_seen": DateTime::now() } };
        let _ = coll.find_one_and_update(filter0, update0).await?;
    }

    if let Some(k1) = keys.get(1) {
        let filter1 = doc! { "customer_id": k1 };
        let update1 = doc! { "$set": { "last_seen": DateTime::now() } };
        //SINK
        let _ = coll.find_one_and_update(filter1, update1).await?;
    }

    Ok(())
}
