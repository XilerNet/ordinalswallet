use chrono::NaiveDateTime;
use sqlx::PgPool;
use tracing::{debug, info};

use crate::db::traits::DomainsRepository;

pub struct SqlxPostgresqlDomainsRepository {
    pool: PgPool,
}

impl SqlxPostgresqlDomainsRepository {
    pub async fn new() -> Self {
        debug!("[DB] Connecting to Postgresql database");

        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&url)
            .await
            .expect("Failed to connect to Postgres");

        info!("[DB] Successfully connected to Postgresql database");

        Self { pool }
    }
}

impl DomainsRepository for SqlxPostgresqlDomainsRepository {
    /// Get all domains since a given date.
    ///
    /// # Arguments
    ///
    /// * `since` - A NaiveDateTime object.
    ///
    /// # Returns
    ///
    /// A vector of tuples containing the domain and reveal transaction.
    /// Each tuple follows this format: (domain, reveal_tx).
    async fn get_domains_since(&self, since: NaiveDateTime) -> sqlx::Result<Vec<(String, String)>> {
        debug!("[DB] Getting domains since {}", since);

        sqlx::query!(
            r#"SELECT private_keys.domain, payment_inscriptions.reveal_tx FROM private_keys 
               INNER JOIN payment_inscription_contents ON payment_inscription_contents.id = private_keys.payment_inscription_content_id 
               INNER JOIN payment_inscriptions ON payment_inscriptions.content = payment_inscription_contents.id 
               INNER JOIN payments ON payments.id = payment_inscription_contents.payment_id 
               WHERE payments.updated_at > $1;"#,
            since
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|d| Ok((d.domain, d.reveal_tx)))
        .collect()
    }
}
