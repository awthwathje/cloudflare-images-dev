use super::constants::DEFAULT_ACCOUNT_ID;
use super::handlers::{serve_image_handler, upload_image_handler};
use warp::Filter;

pub fn upload_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let account_id = std::env::var("ACCOUNT_ID").unwrap_or_else(|_| DEFAULT_ACCOUNT_ID.to_string());

    warp::post()
        .and(warp::path("client"))
        .and(warp::path("v4"))
        .and(warp::path("accounts"))
        .and(warp::path(account_id))
        .and(warp::path("images"))
        .and(warp::path("v1"))
        .and(warp::multipart::form())
        .and_then(upload_image_handler)
}

pub fn image_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path::param::<String>()) // account_hash
        .and(warp::path::param::<String>()) // image_id
        .and(warp::path::tail()) // variant
        .and_then(
            |_account_hash: String, image_id: String, variant: warp::filters::path::Tail| {
                serve_image_handler(image_id, variant)
            },
        )
}
