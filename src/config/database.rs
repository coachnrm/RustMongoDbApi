use mongodb::{Client, options::ClientOptions, Database};
use tracing::log::info;

pub async fn dbconnect() -> mongodb::error::Result<Database> {
    // Parse a conneciton string into an options struct.
    let mut client_options: ClientOptions = ClientOptions::parse("mongodb://root:123456@localhost:27020").await?;

    // Manually set an option
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client: Client = Client::with_options(client_options)?;
    let db: Database = client.database("crab_db");

    info!("Database has been connected");
    
    Ok(db)
}