use crate::handler::book::{delete_book, register_book, show_book, show_book_list, update_book};
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use registry::AppRegistry;

pub fn build_book_routers() -> Router<AppRegistry> {
    let book_routers = Router::new()
        .route("/", post(register_book))
        .route("/", get(show_book_list))
        .route("/{id}", get(show_book))
        .route("/{id}", put(update_book))
        .route("/{id}", delete(delete_book));

    Router::new().nest("/books", book_routers)
}
