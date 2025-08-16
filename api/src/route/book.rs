use axum::{routing::{get, post}, Router};
use registry::AppRegistry;
use crate::handler::book::{register_book, show_book_list, show_book};

pub fn build_book_routers() -> Router<AppRegistry> {
    let book_routers = Router::new()
        .route("/", post(register_book))
        .route("/", get(show_book_list))
        .route("/{id}", get(show_book));

    Router::new().nest("/books", book_routers)
}

