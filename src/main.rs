#![feature(exclusive_range_pattern)]
#![allow(async_fn_in_trait)]

use db::{DomainsRepository, Repository};
use tracing::{debug, error, info};
use utils::last_update::get_last_update;

use crate::{
    models::{
        domain_length::DomainLength,
        inscription::{Inscription, InscriptionMetaAttribute},
    },
    utils::{last_update::set_last_update_to_now, request::publish_inscriptions},
};

pub mod db;
pub mod models;
pub mod utils;

async fn sleep(n: u64) {
    tokio::time::sleep(tokio::time::Duration::from_secs(n)).await;
}

#[tokio::main]
async fn main() {
    color_eyre::install().ok();
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db = Repository::new().await;

    loop {
        let last_update = get_last_update();
        let domains = db.get_domains_since(last_update.naive_utc()).await;

        if let Err(e) = domains {
            error!("Error getting domains: {:?}", e);
            debug!("Retrying in 5 seconds...");
            sleep(5).await;
            continue;
        }

        let domains = domains.unwrap();

        if domains.is_empty() {
            debug!("No new domains to publish");
            debug!("Retrying in 60 seconds...");
            sleep(60).await;
            continue;
        }

        info!("Publishing {} new domains", domains.len());
        debug!("Converting domains to Inscriptions objects...");
        let inscriptions: Vec<Inscription> = domains
            .into_iter()
            .map(|(domain, reveal_tx)| {
                let id = format!("{}i0", reveal_tx);

                let attributes = vec![InscriptionMetaAttribute::new(
                    "length".to_string(),
                    DomainLength::from(domain.as_str()).to_string(),
                )];

                Inscription::new(id, domain, attributes)
            })
            .collect();

        debug!("Publishing inscriptions...");
        match publish_inscriptions(inscriptions).await {
            Ok(Ok(_)) => {
                set_last_update_to_now();
                info!("Successfully published inscriptions");
                debug!("Sleeping for 60 seconds...");
                sleep(60).await;
            }
            Ok(Err(e)) => {
                error!("Error publishing inscriptions: {:?}", e);
                debug!("Retrying in 5 seconds...");
                sleep(5).await;
                continue;
            }
            Err(e) => {
                error!("HTTP Error publishing inscriptions: {:?}", e);
                debug!("Retrying in 5 seconds...");
                sleep(5).await;
                continue;
            }
        }
    }
}
