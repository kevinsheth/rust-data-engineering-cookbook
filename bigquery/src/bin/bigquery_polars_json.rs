// Authenticates with Google Cloud and queries BigQuery.
// Results are converted to JSON and read with Polars.

use google_cloud_bigquery::client::{Client, ClientConfig};
use google_cloud_bigquery::http::job::query::QueryRequest;
use google_cloud_bigquery::query::{row::Row, Iterator};
use polars::prelude::*;
use serde_json::{json, Value};
use std::io::Cursor;

const QUERY: &str = r#"
    SELECT address, eth_balance
    FROM `bigquery-public-data.crypto_ethereum.balances`
    WHERE eth_balance != 0
    LIMIT 10
"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Auth with application default
    let (config, project_id) = ClientConfig::new_with_auth().await?;

    let project_id = project_id.ok_or("No project id!")?;

    // Execute query and read into vec of structs
    // Current "next" implementation necessitates while loop
    let request = QueryRequest {
        query: QUERY.to_string(),
        ..Default::default()
    };

    let mut res: Iterator<Row> = Client::new(config)
        .await?
        .query(&project_id, request)
        .await?;

    let mut json_rows: Vec<Value> = Vec::new();

    while let Some(row) = res.next().await? {
        json_rows.push(json!({
            "address": row.column::<String>(0).unwrap(),
            "eth_balance": row.column::<String>(1).unwrap()
        }));
    }

    let json = serde_json::to_string(&json_rows)?;

    let cursor = Cursor::new(json);

    let df = JsonReader::new(cursor).finish()?;

    println!("{}", df.head(Some(10)));

    Ok(())
}
