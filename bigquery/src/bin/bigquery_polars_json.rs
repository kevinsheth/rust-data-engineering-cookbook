// Authenticates with Google Cloud and queries BigQuery.
// Results are converted to JSON and read with Polars.

use google_cloud_bigquery::client::{Client, ClientConfig};
use google_cloud_bigquery::http::job::query::QueryRequest;
use std::io::Cursor;
use polars::prelude::*;
use google_cloud_bigquery::query::row::Row;
use serde::Serialize;

// Simplifies explicit type required by compiler
type BigQueryResult = google_cloud_bigquery::query::Iterator<Row>;

// Derive serde::Serialize for struct
#[derive(Debug, Serialize)]
struct Balance {
    address: String,
    eth_balance: String, // in real world we should make this u128
}

// Implement conversion from bigquery row to struct
impl Balance {

    fn from_row(row: &Row) -> Self {
        let address = row.column::<String>(0)
            .expect("Unable to extract address");
        let eth_balance = row.column::<String>(1)
            .expect("Unable to extract eth_balance");
        Self { address, eth_balance }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Auth with application default
    let (config, project_id) = ClientConfig::new_with_auth().await?;

    let project_id = project_id.ok_or("No project id!")?;


    // Execute query and read into vec of structs
    // Current "next" implementation necessitates while loop
    let query = "SELECT * \
        FROM `bigquery-public-data.crypto_ethereum.balances` \
        WHERE eth_balance != 0 \
        LIMIT 10".to_string();

    let request = QueryRequest {
        query,
        ..Default::default()
    };

    let mut res: BigQueryResult = Client::new(config)
        .await?
        .query(&project_id, request)
        .await?;

    let mut balances: Vec<Balance> = vec![];

    while let Some(row) = res.next().await? {

        balances.push(Balance::from_row(&row));

    }

    // Serialize as json and read with JsonReader

    let json = serde_json::to_string(&balances)?;

    let cursor = Cursor::new(json);

    let df = JsonReader::new(cursor)
        .finish()?;

    println!("{}", df.head(Some(10)));

    Ok(())

}
