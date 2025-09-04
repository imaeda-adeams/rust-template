use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use garde::Validate;
use kernel::model::{id::UserId, user::event::DeleteUser};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

use crate::{
    extractor::AuthorizedUser,
    model::user::{
        CreateUserRequest, UpdateUserPasswordRequest, UpdateUserPasswordRequestWithUserId,
        UpdateUserRoleRequest, UpdateUserRoleRequestWithUserId, UserResponse, UsersResponse,
    },
};

#[utoipa::path(post, path = "/users")]
pub async fn register_user(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateUserRequest>,
) -> AppResult<Json<UserResponse>> {

    if !user.is_admin() {
        return Err(AppError::ForbiddenOperationError);
    }
    
    req.validate()?;
    
    let registered_user = registry.user_repository().create(req.into()).await?;
    
    Ok(Json(registered_user.into()))
}


#[utoipa::path(get, path = "/users")]
pub async fn list_users(
    _user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<UsersResponse>> {
    
    let items = registry
        .user_repository()
        .find_all()
        .await?
        .into_iter()
        .map(UserResponse::from)
        .collect();
    
    Ok(Json(UsersResponse{ items}))
}

#[utoipa::path(delete, path = "/users/{user_id}")]
pub async fn delete_user(
    user: AuthorizedUser,
    Path(user_id): Path<UserId>,
    State(registry): State<AppRegistry>,    
) -> AppResult<StatusCode> {
    
    if !user.is_admin() {
        return Err(AppError::ForbiddenOperationError);
    }
    
    registry
        .user_repository()
        .delete(DeleteUser { user_id })
        .await?;
    
    Ok(StatusCode::OK)
}

#[utoipa::path(put, path = "/users/{user_id}/role")]
pub async fn change_role (
    user: AuthorizedUser,
    Path(user_id): Path<UserId>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateUserRoleRequest>,
) -> AppResult<StatusCode> {
    
    if !user.is_admin() {
        return Err(AppError::ForbiddenOperationError);
    }
    
    registry
        .user_repository()
        .update_role(UpdateUserRoleRequestWithUserId::new(user_id, req).into())
        .await?;
    
    Ok(StatusCode::OK)
}

#[utoipa::path(get, path = "/users/me")]
pub async fn get_current_user(user: AuthorizedUser) -> Json<UserResponse> {
    Json(UserResponse::from(user.user))
}

#[utoipa::path(put, path = "/users/me/password")]
pub async fn change_password (
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateUserPasswordRequest>,
) -> AppResult<StatusCode> {
    
    req.validate()?;
    
    registry
        .user_repository()
        .update_password(UpdateUserPasswordRequestWithUserId::new(user.user.user_id, req).into())
        .await?;
    
    Ok(StatusCode::OK)      
}