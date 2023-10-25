use chrono::NaiveDateTime;

pub trait DomainsRepository {
    async fn get_domains_since(&self, since: NaiveDateTime) -> sqlx::Result<Vec<(String, String)>>;
}
