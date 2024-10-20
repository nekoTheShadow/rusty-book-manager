use axum::Router;
use registry::AppRegistry;
use crate::handler::user::{change_password, get_current_user, register_user, delete_user, change_role, list_users};
use axum::routing::{delete, get, put};


pub fn build_user_router() -> Router<AppRegistry> {
    Router::new()
        .route("/users/me", get(get_current_user))
        .route("/users/me/password", put(change_password))
        .route("/users", get(list_users).post(register_user))
        .route("/users/:user_id", delete(delete_user))
        .route("/users/:user_id/role", put(change_role))
}