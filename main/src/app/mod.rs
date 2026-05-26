use actix_web::web;

use crate::users;

pub struct AppState {
    pub user_uservice: users::UserService,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            user_uservice: users::UserService::new(),
        }
    }
}

pub type AppStateData = web::Data<AppState>;