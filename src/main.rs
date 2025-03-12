use std::{error::Error, sync::Arc};
use time::format_description;
use sqlx::types::time::OffsetDateTime;
mod api;
use api::configure;
mod server_serve;
mod settings;
mod application_state;
mod services;
use crate::server_serve::server_serve;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let current_dateoffset : OffsetDateTime = OffsetDateTime::now_utc();
    println!("Current date and time: {}", current_dateoffset);
    // Print the string representation of the current date and time
    println!("Current date and time string : {}", current_dateoffset.to_string());
    // Print the string representation of the current date and time in RFC3339 format
    let format = format_description::parse(
        "[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour \
             sign:mandatory]:[offset_minute]",
    )?;
    println!("Current date and time format: {}", current_dateoffset.format(&format)?);
    let current_datetime : chrono::DateTime<chrono::Utc> = chrono::DateTime::parse_from_rfc3339(current_dateoffset.format(&format)?.to_string().as_str()).unwrap().into();
    println!("Current date and time endall: {}", current_datetime);
    let empty : Option<i32> = Some(10);
    let value: Option<i32> = empty.map(|v| v + 1).or(None);
    println!("Value: {}", value.unwrap());

    let settings = settings::Settings::new("config.json", "APP")?;
    let db_url = settings
    .database
    .url
    .clone()
    .expect("Database URL is not set");
    let pool = sqlx::MySqlPool::connect(&db_url).await?;
    let application_state = Arc::new(application_state::ApplicationState::new(&settings, pool)?);
    
    let app = configure(application_state.clone());
    server_serve(app, application_state).await?;
    Ok(())
}