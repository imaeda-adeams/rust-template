use derive_new::new;
use garde::Validate;
use kernel::model::{
    id::UserId,
    role::Role,
    user::{
        event::{CreateUser, UpdateUserPassword, UpdateUserRole},
        User,
    }
};
use serde::{Deserialize, Serialize};
use strum::VariantNames;

#[derive(Serialize, Deserialize, VariantNames)]
#[strum(serialize_all = "kebab-case")]
pub enum RoleName {
    Admin,
    User,
}

impl From<Role> for RoleName {
    fn from(role: Role) -> Self {
        match role {
            Role::Admin => RoleName::Admin,
            Role::User => RoleName::User,
        }
    }
}

impl From<RoleName> for Role {
    fn from(role: RoleName) -> Self {
        match role {
            RoleName::Admin => Role::Admin,
            RoleName::User => Role::User,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersResponse {
    pub items: Vec<UserResponse>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub role: RoleName,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            role: RoleName::from(user.role),
        }
    }
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    #[garde(length(min = 1))]
    name: String,
    #[garde(email)]
    email: String,
    #[garde(length(min = 1))]
    password: String,
}

impl From<CreateUserRequest> for CreateUser {
    
    fn from(request: CreateUserRequest) -> Self {
        Self {
            name: request.name,
            email: request.email,
            password: request.password,
        }
    }
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserPasswordRequest {

    #[garde(length(min = 1))]
    current_password: String,

    #[garde(length(min = 1))]
    new_password: String,
}

#[derive(new)]
pub struct UpdateUserPasswordRequestWithUserId (
    UserId,
    UpdateUserPasswordRequest
);

impl From<UpdateUserPasswordRequestWithUserId> for UpdateUserPassword {

    fn from(request: UpdateUserPasswordRequestWithUserId) -> Self {

        let UpdateUserPasswordRequestWithUserId(
            user_id,
            UpdateUserPasswordRequest {
                current_password,
                new_password,
            },
        ) = request;

        UpdateUserPassword {
            user_id,
            current_password,
            new_password,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRoleRequest {
    role: RoleName,   
}

#[derive(new)]
pub struct UpdateUserRoleRequestWithUserId (
    UserId,
    UpdateUserRoleRequest
);

impl From<UpdateUserRoleRequestWithUserId> for UpdateUserRole {
    
    fn from(request: UpdateUserRoleRequestWithUserId) -> Self {
        let UpdateUserRoleRequestWithUserId(
            user_id,
            UpdateUserRoleRequest {
                role,
            },
        ) = request;
        
        Self {
            user_id,
            role: Role::from(role),
        }
    }
}