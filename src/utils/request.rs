use crate::models::inscription::{Inscription, NewInscriptions};
use lazy_static::lazy_static;
use std::env;
use tracing::error;

use super::last_update::set_last_update_to_now;

fn env_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or(default.to_string())
}

fn env_or_panic(key: &str) -> String {
    env::var(key).expect(&format!("{} must be set", key))
}

lazy_static! {
    static ref API_BASE_URL: String =
        env_or_default("API_BASE_URL", "https://turbo.ordinalswallet.com/");
    static ref CREATOR_ADDRESS: String = env_or_panic("CREATOR_ADDRESS");
    static ref CREATOR_SIGNATURE: String = env_or_panic("CREATOR_SIGNATURE");
    static ref SLUG: String = env_or_default("SLUG", "xiler-dns");
}

pub fn publish_inscriptions(inscriptions: Vec<Inscription>) -> reqwest::Result<Result<(), ()>> {
    let new_inscriptions = NewInscriptions::new(
        inscriptions,
        CREATOR_ADDRESS.to_string(),
        CREATOR_SIGNATURE.to_string(),
        SLUG.to_string(),
    );

    let client = reqwest::blocking::Client::new();
    let url = format!("{}/collection/update", *API_BASE_URL);

    let response = client.post(&url).json(&new_inscriptions).send()?;

    if !response.status().is_success() {
        error!("Error publishing inscriptions: {:?}", response);
        return Ok(Err(()));
    }

    set_last_update_to_now();
    Ok(Ok(()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ctor::ctor;
    use mockito;

    #[cfg(test)]
    #[ctor]
    fn setup_env() {
        env::set_var("CREATOR_ADDRESS", "test");
        env::set_var("CREATOR_SIGNATURE", "test");
        env::set_var("SLUG", "test");
    }

    #[test]
    fn test_publish_inscriptions_success() {
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

        let result = publish_inscriptions(inscriptions);

        assert!(result.is_ok());
        assert!(result.unwrap().is_ok());

        mock.assert();
    }
}
