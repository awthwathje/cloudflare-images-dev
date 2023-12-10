use super::constants::{DEFAULT_ACCOUNT_HASH, DEFAULT_HOSTNAME, DEFAULT_PORT, DEFAULT_VARIANTS};

pub fn variants(image_id: &str) -> Vec<String> {
    let account_hash =
        std::env::var("ACCOUNT_HASH").unwrap_or_else(|_| DEFAULT_ACCOUNT_HASH.to_string());
    let hostname = std::env::var("HOSTNAME").unwrap_or_else(|_| DEFAULT_HOSTNAME.to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string());
    let variants_env = std::env::var("VARIANTS").unwrap_or_else(|_| DEFAULT_VARIANTS.to_string());

    let base_url = if port == "80" {
        format!("http://{}/{}", hostname, account_hash)
    } else {
        format!("http://{}:{}/{}", hostname, port, account_hash)
    };

    variants_env
        .split(',')
        .map(|name| format!("{}/{}/{}", base_url, image_id, name.trim()))
        .collect()
}
