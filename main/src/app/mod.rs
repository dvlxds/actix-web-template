use actix_web::web;

use crate::{upload, users};

pub struct AppState {
    pub user_uservice: users::UserService,
    pub upload_service: upload::UploadService,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            user_uservice: users::UserService::new(),
            upload_service: upload::UploadService::new(),
        }
    }
}

pub type AppStateData = web::Data<AppState>;
