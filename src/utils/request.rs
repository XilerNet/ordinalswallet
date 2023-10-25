use crate::{
    models::inscription::{Inscription, NewInscriptions},
    utils::environment::{env_or_default, env_or_panic},
};
use lazy_static::lazy_static;
use tracing::error;

use super::last_update::set_last_update_to_now;

lazy_static! {
    static ref API_BASE_URL: String =
        env_or_default("API_BASE_URL", "https://turbo.ordinalswallet.com/");
    static ref CREATOR_ADDRESS: String = env_or_panic("CREATOR_ADDRESS");
    static ref CREATOR_SIGNATURE: String = env_or_panic("CREATOR_SIGNATURE");
    static ref SLUG: String = env_or_default("SLUG", "xiler-dns");
}

/// Publishes new inscriptions to the API
///
/// # Environment variables
///
/// * `API_BASE_URL` - The base URL of the API
/// * `CREATOR_ADDRESS` - The address of the creator
/// * `CREATOR_SIGNATURE` - The signature of the creator
/// * `SLUG` - The slug of the collection
///
/// # Arguments
///
/// * `inscriptions` - A vector of new inscriptions to publish
///
/// # Example
///
/// ```
/// use xiler::utils::request::publish_inscriptions;
/// use xiler::models::inscription::Inscription;
///
/// let inscriptions = vec![
///     Inscription::new("test".to_string(), "test".to_string(), vec![]),
///     Inscription::new("test2".to_string(), "test2".to_string(), vec![]),
/// ];
///
/// let result = publish_inscriptions(inscriptions).await;
///
/// assert!(result.is_ok());
/// ```
pub async fn publish_inscriptions(
    inscriptions: Vec<Inscription>,
) -> reqwest::Result<Result<(), String>> {
    let new_inscriptions = NewInscriptions::new(
        inscriptions,
        CREATOR_ADDRESS.to_string(),
        CREATOR_SIGNATURE.to_string(),
        SLUG.to_string(),
    );

    let client = reqwest::Client::new();
    let url = format!("{}/collection/update", *API_BASE_URL);

    let response = client.post(&url).json(&new_inscriptions).send().await?;

    if !response.status().is_success() {
        error!("Error publishing inscriptions: {:?}", response);
        let status = response.status();
        let error = response.text().await?;
        let error = format!("{}: {}", status, error);
        return Ok(Err(error));
    }

    set_last_update_to_now();
    Ok(Ok(()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;
    use std::env;

    static LAST_UPDATE_FILE: &'static str = "./last_update.timestamp.test";

    #[ctor::ctor]
    fn setup_env() {
        env::set_var("CREATOR_ADDRESS", "test");
        env::set_var("CREATOR_SIGNATURE", "test");
        env::set_var("SLUG", "test");
        env::set_var("LAST_UPDATE_FILE", LAST_UPDATE_FILE);
    }

    #[tokio::test]
    async fn test_publish_inscriptions_success() {
        let mut server = mockito::Server::new();

        let inscriptions = vec![
            Inscription::new("test".to_string(), "test".to_string(), vec![]),
            Inscription::new("test2".to_string(), "test2".to_string(), vec![]),
        ];

        let new_inscriptions = NewInscriptions::new(
            inscriptions.clone(),
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
        );

        let json = serde_json::to_string(&new_inscriptions).unwrap();

        let mock = server
            .mock("POST", "/collection/update")
            .match_header("content-type", "application/json")
            .match_body(json.as_str())
            .with_status(200)
            .create();

        env::set_var("API_BASE_URL", server.url());

        let result = publish_inscriptions(inscriptions).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_ok());

        mock.assert();

        std::fs::remove_file(LAST_UPDATE_FILE).unwrap();
    }
}
