use kernel::model::book::Book;
use kernel::model::id::{BookId, UserId};
use kernel::model::user::BookOwner;

pub struct BookRow {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub owned_by: UserId,
    pub owner_name: String,
}

impl From<BookRow> for Book {
    fn from(row: BookRow) -> Self {
        Self {
            book_id: row.book_id,
            title: row.title,
            author: row.author,
            isbn: row.isbn,
            description: row.description,
            owner: BookOwner {
                user_id: row.owned_by,
                name: row.owner_name,
            },
        }
    }
}

pub struct PaginatedBookRow {
    pub total: i64,
    pub book_id: BookId,
}