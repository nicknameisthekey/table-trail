use crate::front_models;
use crate::App;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionParams {
    pub id: i64,
    pub name: String,
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub database_type: String,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn connection_profiles(
    app: State<'_, App>,
) -> Result<Vec<front_models::ConnectionProfile>, ()> {
    let configs = app.connection_profiles().await;
    Ok(configs)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn add_profile(
    app: State<'_, App>,
    profile: front_models::ConnectionProfile,
) -> Result<(), ()> {
    app.add_profile(profile).await;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_profile(app: State<'_, App>, profile_id: i64) -> Result<(), ()> {
    app.delete_profile(profile_id).await;
    Ok(())
}
