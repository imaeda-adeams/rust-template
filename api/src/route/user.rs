use crate::handler::user::{
    change_password, change_role, list_users, delete_user, get_current_user, register_user,
};
use axum::{
    Router,
    routing::{delete, get, put},
};
use registry::AppRegistry;

pub fn build_user_routers() -> Router<AppRegistry> {
    Router::new()
        .route("/users/me", get(get_current_user))
        .route("/users/me/password", put(change_password))
        .route("/users", get(list_users).post(register_user))
        .route("/users/{user_id}", delete(delete_user))
        .route("/users/{user_id}/role", put(change_role))
}
