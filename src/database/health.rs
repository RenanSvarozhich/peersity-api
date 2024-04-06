use deadpool_postgres::Client;
use crate::models::error::DBError;

pub async fn get_database_health(client: &Client) -> Result<bool, DBError> {
    let query = "SELECT 1;";

    let statement = match client.prepare_cached(&query).await {
        Ok(stmt) => stmt,
        Err(e) => return Err(DBError::from(e)),
    };

    match client.query_one(&statement, &[]).await {
        Ok(_) => Ok(true),
        Err(e) => Err(DBError::from(e)),
    }
}