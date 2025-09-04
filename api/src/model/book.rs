use chrono::DateTime;
use derive_new::new;
use garde::Validate;
use kernel::model::book::{Book, event::CreateBook, BookListOptions, Checkout};
use kernel::model::id::{BookId, CheckoutId, UserId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use kernel::model::book::event::UpdateBook;
use kernel::model::list::PaginatedList;
use crate::model::user::{BookOwner, CheckoutUser};

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookRequest {
    #[garde(length(min = 1))]
    pub title: String,

    #[garde(length(min = 1))]
    pub author: String,

    #[garde(length(min = 1))]
    pub isbn: String,

    #[garde(skip)]
    pub description: String,
}

impl From<CreateBookRequest> for CreateBook {
    fn from(value: CreateBookRequest) -> Self {
        let CreateBookRequest {
            title,
            author,
            isbn,
            description,
        } = value;
        Self {
            title,
            author,
            isbn,
            description,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBookRequest {
    #[garde(length(min=1))]
    pub title: String,

    #[garde(length(min=1))]
    pub author: String,

    #[garde(length(min=1))]
    pub isbn: String,

    #[garde(skip)]
    pub description: String,
}

#[derive(new)]
pub struct UpdateBookRequestWithIds(BookId, UserId, UpdateBookRequest);

impl From<UpdateBookRequestWithIds> for UpdateBook {

    fn from(value: UpdateBookRequestWithIds) -> Self {

        let UpdateBookRequestWithIds(
            book_id,
            user_id,
            UpdateBookRequest {
                title,
                author,
                isbn,
                description,
            },
        ) = value;

        UpdateBook {
            book_id,
            title,
            author,
            isbn,
            description,
            requested_user: user_id,
        }
    }

}

#[derive(Debug, Deserialize, Validate)]
pub struct BookListQuery {
    #[garde(range(min=0))]
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[garde(range(min=0))]
    #[serde(default)]
    pub offset: i64,
}

const DEFAULT_LIMIT: i64 = 20;
const fn default_limit() -> i64 {
    DEFAULT_LIMIT
}

impl From<BookListQuery> for BookListOptions {
    fn from(value: BookListQuery) -> Self {
        let BookListQuery { limit, offset } = value;
        Self {
            limit,
            offset,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BookResponse {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub owner: BookOwner,
    pub checkout: Option<BookCheckoutResponse>,
}

impl From<Book> for BookResponse {
    fn from(value: Book) -> Self {
        let Book {
            book_id,
            title,
            author,
            isbn,
            description,
            owner,
            checkout,
        } = value;
        Self {
            book_id,
            title,
            author,
            isbn,
            description,
            owner: owner.into(),
            checkout: checkout.map(BookCheckoutResponse::from),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedBookResponse {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub books: Vec<BookResponse>,
}

impl From<PaginatedList<Book>> for PaginatedBookResponse {

    fn from(value: PaginatedList<Book>) -> Self {
        Self {
            total: value.total,
            limit: value.limit,
            offset: value.offset,
            books: value.items.into_iter().map(BookResponse::from).collect(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BookCheckoutResponse {
    pub checkout_id: CheckoutId,
    pub checked_out_by: CheckoutUser,
    pub checked_out_at: DateTime<chrono::Utc>,
}

impl From<Checkout> for BookCheckoutResponse {

    fn from(value: Checkout) -> Self {
        Self {
            checkout_id: value.checkout_id,
            checked_out_by: value.checked_out_by.into(),
            checked_out_at: value.checked_out_at,
        }
    }
}