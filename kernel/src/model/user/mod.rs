use crate::model::{id::UserId, role::Role};

pub mod event;

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub user_id: UserId,
    pub name: String,
    pub email: String,
    pub role: Role,
}

#[derive(Debug)]
pub struct BookOwner {
    pub user_id: UserId,
    pub name: String,
}

#[derive(Debug)]
pub struct CheckoutUser {
    pub user_id: UserId,
    pub name: String,
}