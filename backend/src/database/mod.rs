use std::str::FromStr;

use log::debug;
use sqlx::postgres::PgConnectOptions as ConnOpt;
use sqlx::postgres::PgPool as Pool;

pub mod error;
pub mod model;
pub mod table;

pub struct Database {
    pub pool: Pool,
    // pub feed_table: FeedTable,
}

impl Database {
    pub async fn new(db_url: &str) -> anyhow::Result<Self> {
        debug!("Connecting to db...");
        let opts = ConnOpt::from_str(db_url)?;
        let pool = Pool::connect_with(opts).await?;
        log::info!("Connected to db.");

        // let feed_table = FeedTable::new(pool.clone());

        Ok(Self {
            pool,
            // feed_table,
        })
    }

    pub async fn run_migrations(&self) -> anyhow::Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    pub async fn drop_all_tables(&self) -> anyhow::Result<()> {
        // self.feed_table.drop_table().await?;
        Ok(())
    }

    pub async fn delete_all_tables(&self) -> anyhow::Result<()> {
        // self.feed_table.delete_all().await?;
        Ok(())
    }
}
