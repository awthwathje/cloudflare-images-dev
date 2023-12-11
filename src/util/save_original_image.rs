use crate::constants::FILE_STORAGE_DIR;
use futures::StreamExt;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use warp::multipart::Part;
use warp::Buf;
use warp::Rejection;

pub async fn save_original_image(
    image_id: &str,
    file_extension: &str,
    part: Part,
) -> Result<PathBuf, Rejection> {
    let new_filename = format!("{}.{}", image_id, file_extension);
    let path = Path::new(FILE_STORAGE_DIR).join(&new_filename);

    if let Some(dir) = path.parent() {
        tokio::fs::create_dir_all(dir).await.map_err(|e| {
            eprintln!("Failed to create directory: {}", e);
            warp::reject::reject()
        })?;
    }

    let mut file = File::create(&path).await.map_err(|e| {
        eprintln!("Failed to create file: {}", e);
        warp::reject::reject()
    })?;

    let mut stream = part.stream();
    while let Some(Ok(data)) = stream.next().await {
        file.write_all(data.chunk()).await.map_err(|e| {
            eprintln!("Failed to write to file: {}", e);
            warp::reject::reject()
        })?;
    }

    Ok(path)
}
