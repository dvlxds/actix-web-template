use actix_web::{HttpResponse, get, web};

use crate::AppState;

#[get("/{id}")]
async fn get_user(service: web::Data<AppState>, path: web::Path<u32>) -> HttpResponse {
    let user_id = path.into_inner();
    let profile = service.user_uservice.get_profile(user_id);
    HttpResponse::Ok().body(profile)
}
