use crate::model::id::{BookId, UserId};

pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

#[derive(Debug)]
pub struct UpdateBook {
    pub id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,   
    pub requested_user: UserId
}

#[derive(Debug)]
pub struct DeleteBook {
    pub id: BookId,
    pub requested_user: UserId
}