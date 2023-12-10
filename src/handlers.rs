use super::responses::{ApiResponse, ImageResult};
use super::variants::variants;
use bytes::Buf;
use chrono::Utc;
use futures::StreamExt;
use mime_guess::from_path;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs;
use uuid::Uuid;
use warp::http::Response;
use warp::http::StatusCode;
use warp::hyper::Body;
use warp::multipart::FormData;
use warp::Reply;

pub async fn upload_image_handler(
    mut form_data: FormData,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut original_filename = String::new();
    let image_id = Uuid::new_v4().to_string();

    while let Some(Ok(part)) = form_data.next().await {
        if part.name() == "file" {
            if let Some(filename) = part.filename() {
                original_filename = filename.to_string();

                let directory_path = "./.files/";
                let new_filename = format!(
                    "{}.{}",
                    image_id,
                    original_filename.split('.').last().unwrap_or("")
                );
                let path = format!("{}{}", directory_path, new_filename);

                if !Path::new(directory_path).exists() {
                    fs::create_dir_all(directory_path).await.map_err(|e| {
                        eprintln!("Failed to create directory: {}", e);
                        warp::reject::reject()
                    })?;
                }

                let mut file = File::create(&path).map_err(|e| {
                    eprintln!("Failed to create file: {}", e);
                    warp::reject::reject()
                })?;

                let mut stream = part.stream();

                while let Some(Ok(data)) = stream.next().await {
                    file.write_all(&data.chunk()).map_err(|e| {
                        eprintln!("Failed to write to file: {}", e);
                        warp::reject::reject()
                    })?;
                }
            }
        }
    }

    let variants = variants(&image_id);

    let response = ApiResponse {
        result: ImageResult {
            id: image_id,
            filename: original_filename,
            uploaded: Utc::now(),
            require_signed_urls: false,
            variants,
        },
        success: true,
        errors: vec![],
        messages: vec![],
    };

    Ok(warp::reply::json(&response))
}

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

    let directory_path = PathBuf::from("./.files/");

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
                .map_or(false, |stem| stem == image_id.as_str())
        {
            image_path = Some(path);
            break;
        }
    }

    match image_path {
        Some(path) => {
            let image_data = fs::read(&path).await.map_err(|e| {
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
            let response = warp::reply::with_status(json_reply, warp::http::StatusCode::NOT_FOUND)
                .into_response();
            Ok(response)
        }
    }
}
