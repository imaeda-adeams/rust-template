use crate::model::id::UserId;
use uuid::Uuid;

pub struct CreateToken {
    pub id: UserId,
    pub access_token: String,
}

impl CreateToken {
    pub fn new(id: UserId) -> Self {
        let access_token = Uuid::new_v4().simple().to_string();
        Self {id, access_token }
    }   
}
