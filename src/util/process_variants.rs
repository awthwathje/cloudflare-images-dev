use crate::constants::{DEFAULT_VARIANTS, FILE_STORAGE_DIR};
use crate::util::parse_variants::parse_variants;
use crate::util::resize_image::resize_image;
use futures::future;
use std::path::Path;

pub async fn process_variants(
    image_id: &str,
    file_extension: &str,
    original_path: &Path,
) -> Result<(), warp::Rejection> {
    let variants = parse_variants(DEFAULT_VARIANTS);

    let futures: Vec<_> = variants
        .into_iter()
        .map(|(variant_name, width, height)| {
            let variant_filename = format!("{}_{}.{}", image_id, variant_name, file_extension);
            let variant_path = Path::new(FILE_STORAGE_DIR).join(&variant_filename);

            let original_path = original_path.to_path_buf();

            tokio::spawn(
                async move { resize_image(original_path, variant_path, width, height).await },
            )
        })
        .collect();

    let results: Vec<_> = future::join_all(futures).await;

    let results: Result<Vec<_>, _> = results.into_iter().collect();

    match results {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed to process variants: {}", e);
            Err(warp::reject::reject())
        }
    }
}
