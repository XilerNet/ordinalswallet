use async_recursion::async_recursion;
use lazy_static::lazy_static;
use serde_json::json;
use tracing::{debug, error};

use crate::utils::environment::env_or_panic;

lazy_static! {
    static ref WEBHOOK_URL: String = env_or_panic("WEBHOOK_URL");
}

async fn new_domain_registered(domains: &[(String, String)]) -> reqwest::Result<()> {
    let client = reqwest::Client::new();

    let embeds = domains
        .iter()
        .map(|(domain, inscription_id)| {
            json!({
                "description": format!(
                    "`{}` has just been registered!\n[{}](https://ordinalswallet.com/inscription/{})",
                    domain, inscription_id, inscription_id
                ),
                "color": 3512539
            })
        })
        .collect::<Vec<serde_json::Value>>();

    let json = json!({
        "content": null,
        "embeds": embeds,
        "attachments": []
    });

    client.post(&*WEBHOOK_URL).json(&json).send().await?;

    Ok(())
}

#[async_recursion]
pub async fn new_domains_registered(domains: Vec<(String, String)>) -> reqwest::Result<()> {
    let mut domains = domains.into_iter().peekable();
    let mut domains_to_retry = vec![];

    while domains.peek().is_some() {
        let domains_to_send = domains.by_ref().take(10).collect::<Vec<_>>();

        if let Err(e) = new_domain_registered(&domains_to_send).await {
            error!("Error sending webhook: {:?}", e);
            debug!("Retrying in 5 seconds...");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            domains_to_retry.extend(domains_to_send);
        }
    }

    if !domains_to_retry.is_empty() {
        return new_domains_registered(domains_to_retry).await;
    }

    Ok(())
}
