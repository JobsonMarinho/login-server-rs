use std::sync::Arc;
use crate::config::ServerConfig;
use crate::models::{Character, World};

use tracing::info;

#[cfg(feature = "mysql")]
use sqlx::mysql::MySqlPoolOptions;

#[cfg(feature = "redis")]
use redis::AsyncCommands;

#[derive(Clone)]
pub struct AppState {
    pub cfg: ServerConfig,
    #[cfg(feature = "mysql")]
    pub mysql: Option<sqlx::MySqlPool>,
    #[cfg(feature = "redis")]
    pub redis: Option<redis::Client>,
}

impl AppState {
    pub async fn new(cfg: ServerConfig) -> anyhow::Result<Self> {
        #[cfg(feature = "mysql")]
        let mysql = if let Some(url) = &cfg.mysql_url {
            let pool = MySqlPoolOptions::new()
                .max_connections(16)
                .connect(url)
                .await?;
            info!("Connected to MySQL");
            Some(pool)
        } else {
            None
        };
        #[cfg(not(feature = "mysql"))]
        let mysql = None;

        #[cfg(feature = "redis")]
        let redis = if let Some(url) = &cfg.redis_url {
            let client = redis::Client::open(url.as_ref())?;
            let mut conn = client.get_multiplexed_async_connection().await?;
            let _:() = redis::cmd("PING").query_async(&mut conn).await?;
            info!("Connected to Redis");
            Some(client)
        } else {
            None
        };
        #[cfg(not(feature = "redis"))]
        let redis = None;

        Ok(Self {
            cfg,
            #[cfg(feature = "mysql")]
            mysql,
            #[cfg(feature = "redis")]
            redis,
        })
    }

    pub async fn login(&self, account: &str, password: &str) -> anyhow::Result<(Vec<Character>, World)> {
        #[cfg(feature = "redis")]
        {
            if let Some(redis_client) = &self.redis {
                if let Ok(mut conn) = redis_client.get_multiplexed_async_connection().await {
                    let key = format!("login:{}", account);
                    if let Ok(cached) = conn.get::<_, Option<String>>(&key).await {
                        if let Some(cached) = cached {
                            if let Ok(result) = serde_json::from_str::<(Vec<Character>, World)>(&cached) {
                                info!("cache hit for account: {}", account);
                                return Ok(result);
                            }
                        }
                    }
                }
            }
        }

        info!("cache miss for account: {}", account);
        let result = self.login_db(account, password).await;

        #[cfg(feature = "redis")]
        {
            if let (Some(redis_client), Ok(result_data)) = (&self.redis, &result) {
                info!("entering cache block");
                 if let Ok(mut conn) = redis_client.get_multiplexed_async_connection().await {
                    let key = format!("login:{}", account);
                    if let Ok(value) = serde_json::to_string(result_data) {
                        info!("caching result for account: {}", account);
                        let _: redis::RedisResult<()> = conn.set_ex(key, value, 300).await;
                    }
                }
            }
        }

        result
    }

    async fn login_db(&self, account: &str, password: &str) -> anyhow::Result<(Vec<Character>, World)> {
        if self.cfg.use_mock {
            if account != password {
                anyhow::bail!("invalid credentials");
            }
            let chars = vec![
                Character { name: format!("{}-Knight", account), level: 220, vocation: "Knight".into() },
                Character { name: format!("{}-Druid", account), level: 180, vocation: "Druid".into() },
                Character { name: format!("{}-Sorcerer", account), level: 150, vocation: "Sorcerer".into() },
            ];
            let world = World {
                name: self.cfg.server_name.clone(),
                ip: self.cfg.server_ip.clone(),
                port: self.cfg.server_port,
                location: self.cfg.server_location.clone(),
            };
            return Ok((chars, world));
        }

        #[cfg(feature = "mysql")]
        {
            use sqlx::Row;
            let pool = self.mysql.as_ref().ok_or_else(|| anyhow::anyhow!("mysql not configured"))?;

            let row = sqlx::query("SELECT id, password FROM accounts WHERE account = ?")
                .bind(account)
                .fetch_optional(pool)
                .await?;
            let Some(row) = row else { anyhow::bail!("account not found"); };
            let db_pass: String = row.try_get("password")?;
            if db_pass != password {
                anyhow::bail!("invalid credentials");
            }

            let mut chars = Vec::new();
            let mut rows = sqlx::query("SELECT name, level, vocation FROM players WHERE account = ?")
                .bind(account)
                .fetch(pool);
            use futures_util::TryStreamExt;
            while let Some(row) = rows.try_next().await? {
                let name: String = row.try_get("name")?;
                let level: i32 = row.try_get("level")?;
                let vocation: String = row.try_get("vocation")?;
                chars.push(crate::models::Character { name, level, vocation });
            }

            let world = World {
                name: self.cfg.server_name.clone(),
                ip: self.cfg.server_ip.clone(), port: self.cfg.server_port,
                location: self.cfg.server_location.clone(),
            };
            return Ok((chars, world));
        }

        anyhow::bail!("no backend configured")
    }
}

pub type SharedState = Arc<AppState>;