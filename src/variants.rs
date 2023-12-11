use super::constants::{DEFAULT_ACCOUNT_HASH, DEFAULT_HOSTNAME, DEFAULT_PORT, DEFAULT_VARIANTS};
use crate::util::parse_variants::parse_variants;

pub fn variants(image_id: &str) -> Vec<String> {
    let account_hash =
        std::env::var("ACCOUNT_HASH").unwrap_or_else(|_| DEFAULT_ACCOUNT_HASH.to_string());
    let hostname = std::env::var("HOSTNAME").unwrap_or_else(|_| DEFAULT_HOSTNAME.to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string());

    let base_url = if port == "80" {
        format!("http://{}/{}", hostname, account_hash)
    } else {
        format!("http://{}:{}/{}", hostname, port, account_hash)
    };

    parse_variants(DEFAULT_VARIANTS)
        .into_iter()
        .map(|(name, _, _)| format!("{}/{}/{}", base_url, image_id, name))
        .collect()
}
