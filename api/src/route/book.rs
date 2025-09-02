use crate::handler::book::{delete_book, register_book, show_book, show_book_list, update_book};
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use registry::AppRegistry;
use crate::handler::checkout::
    {checkout_book, checkout_history, return_book, show_checked_out_list, get_checkouts};

pub fn build_book_routers() -> Router<AppRegistry> {

    let book_routers = Router::new()
        .route("/", post(register_book))
        .route("/", get(show_book_list))
        .route("/{id}", get(show_book))
        .route("/{id}", put(update_book))
        .route("/{id}", delete(delete_book));

    let checkout_routers = Router::new()
        .route("/checkouts", get(show_checked_out_list))
        .route("/checkouts/me", get(get_checkouts))
        .route("/{book_id}/checkouts", post(checkout_book))
        .route("/{book_id}/checkouts/{checkout_id}/returned", put(return_book))
        .route("/{book_id}/checkout-history", get(checkout_history));

    Router::new().nest("/books", book_routers.merge(checkout_routers))
}
