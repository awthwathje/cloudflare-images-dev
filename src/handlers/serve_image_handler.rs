use crate::constants::FILE_STORAGE_DIR;
use crate::variants::variants;
use mime_guess::from_path;
use std::path::PathBuf;
use tokio::fs;
use warp::http::Response;
use warp::http::StatusCode;
use warp::hyper::Body;
use warp::Reply;

pub async fn serve_image_handler(
    image_id: String,
    variant: warp::filters::path::Tail,
) -> Result<impl warp::Reply, warp::Rejection> {
    let variant_str = variant.as_str();

    let known_variants = variants(&image_id);
    if !known_variants.iter().any(|v| v.ends_with(variant_str)) {
        eprintln!("Unknown variant requested: {}", variant_str);
        let error_message =
            "ERROR 9425: Image access denied: This account doesn't have variant with this name";
        return Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .header("Content-Type", "text/plain")
            .body(Body::from(error_message))
            .unwrap());
    }

    let directory_path = PathBuf::from(FILE_STORAGE_DIR);

    let mut image_path = None;

    let mut dir_entries = fs::read_dir(directory_path).await.map_err(|e| {
        eprintln!("Failed to read directory: {}", e);
        warp::reject::reject()
    })?;

    while let Some(entry) = dir_entries.next_entry().await.map_err(|e| {
        eprintln!("Failed to read directory entry: {}", e);
        warp::reject::reject()
    })? {
        let path = entry.path();
        if path.is_file()
            && path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map_or(false, |stem| {
                    stem.starts_with(&format!("{}_{}", image_id, variant_str))
                })
        {
            image_path = Some(path);
            break;
        }
    }

    match image_path {
        Some(path) => {
            let image_data = tokio::fs::read(&path).await.map_err(|e| {
                eprintln!("Failed to read image file: {}", e);
                warp::reject::reject()
            })?;

            let mime_type = from_path(&path).first_or_octet_stream();

            Ok(Response::builder()
                .header("Content-Type", mime_type.as_ref())
                .body(Body::from(image_data))
                .unwrap())
        }
        None => {
            eprintln!("Image not found: {}", image_id);
            let json_reply = warp::reply::json(&"Image not found");
            let response =
                warp::reply::with_status(json_reply, StatusCode::NOT_FOUND).into_response();
            Ok(response)
        }
    }
}
