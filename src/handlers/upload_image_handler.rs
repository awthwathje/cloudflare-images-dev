use crate::responses::{ApiResponse, ImageResult};
use crate::util::process_variants::process_variants;
use crate::util::save_original_image::save_original_image;
use crate::variants::variants;
use futures::StreamExt;
use uuid::Uuid;
use warp::multipart::FormData;

pub async fn upload_image_handler(
    mut form_data: FormData,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut original_filename = String::new();
    let image_id = Uuid::new_v4().to_string();

    while let Some(Ok(part)) = form_data.next().await {
        if part.name() == "file" {
            if let Some(filename) = part.filename() {
                original_filename = filename.to_string();
                let file_extension = original_filename.split('.').last().unwrap_or("");

                let original_path = save_original_image(&image_id, file_extension, part).await?;
                let process_result =
                    process_variants(&image_id, file_extension, &original_path).await;

                match process_result {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }
        }
    }

    let variants = variants(&image_id);

    let response = ApiResponse {
        result: ImageResult {
            id: image_id,
            filename: original_filename,
            uploaded: chrono::Utc::now(),
            require_signed_urls: false,
            variants,
        },
        success: true,
        errors: vec![],
        messages: vec![],
    };

    Ok(warp::reply::json(&response))
}
