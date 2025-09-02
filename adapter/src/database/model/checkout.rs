use kernel::model::{
    checkout::{Checkout, CheckoutBook},
    id::{BookId, CheckoutId, UserId},
};
use sqlx::types::chrono::{DateTime, Utc};

pub struct CheckoutStateRow {
    pub book_id: BookId,
    pub checkout_id: Option<CheckoutId>,
    pub user_id: Option<UserId>,
}

pub struct CheckoutRow {
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub user_id: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl From<CheckoutRow> for Checkout {
    fn from(value: CheckoutRow) -> Self {
        Checkout {
            checkout_id: value.checkout_id,
            checked_out_by: value.user_id,
            checked_out_at: value.checked_out_at,
            returned_at: None,
            book: CheckoutBook {
                book_id: value.book_id,
                title: value.title,
                author: value.author,
                isbn: value.isbn,
            }       
        }
    }
}

pub struct ReturnedCheckoutRow {
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub user_id: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub returned_at: DateTime<Utc>,
    pub title: String,
    pub author: String,
    pub isbn: String,   
}

impl From<ReturnedCheckoutRow> for Checkout {
    fn from(value: ReturnedCheckoutRow) -> Self {
        Checkout {
            checkout_id: value.checkout_id,
            checked_out_by: value.user_id,
            checked_out_at: value.checked_out_at,
            returned_at: Some(value.returned_at),
            book: CheckoutBook {
                book_id: value.book_id,
                title: value.title,
                author: value.author,
                isbn: value.isbn,
            }       
        }
    }
}