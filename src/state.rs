use sqlx::PgPool;

use crate::{db, error::Error};

#[derive(Clone)]
pub struct DiscordBotConfig {
    pub url: String,
    pub token: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub discord_bot: Option<DiscordBotConfig>,
}

impl AppState {
    pub async fn from_env() -> Result<Self, Error> {
        let db = db::create_pool().await.map_err(Error::DatabaseConnection)?;

        Ok(AppState {
            db,
            discord_bot: DiscordBotConfig::from_env(),
        })
    }
}

impl DiscordBotConfig {
    pub fn from_env() -> Option<Self> {
        let url = std::env::var("DISCORD_BOT_URL").ok()?;
        let token = std::env::var("DISCORD_BOT_TOKEN").ok()?;
        Some(Self { url, token })
    }
}
