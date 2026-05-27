use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse, post, web};

use crate::AppState;

#[post("/file")]
async fn upload_file(
    service: web::Data<AppState>,
    payload: Multipart,
) -> Result<HttpResponse, Error> {
    service.upload_service.upload(payload).await
}
