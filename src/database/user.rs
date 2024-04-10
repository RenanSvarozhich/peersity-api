use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::models::{error::DBError, user::User};

pub async fn get_users(client: &Client) -> Result<Vec<User>, DBError> {
    let query = "SELECT $table_fields FROM users;"
        .replace("$table_fields", &User::sql_table_fields());

    let statement = client.prepare_cached(&query).await.unwrap();

    let users = client
        .query(&statement, &[]).await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>();

    Ok(users)
}