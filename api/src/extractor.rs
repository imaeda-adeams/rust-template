use axum::http::header;
use axum::{extract::FromRequestParts, http::request::Parts};

use kernel::model::{auth::AccessToken, id::UserId, role::Role, user::User};
use shared::error::AppError;

use registry::AppRegistry;
pub struct AuthorizedUser {
    pub access_token: AccessToken,
    pub user: User,
}

impl AuthorizedUser {
    pub fn id(&self) -> UserId {
        self.user.id
    }

    pub fn is_admin(&self) -> bool {
        self.user.role == Role::Admin
    }
}

impl FromRequestParts<AppRegistry> for AuthorizedUser {
    type Rejection = AppError;
    async fn from_request_parts(
        parts: &mut Parts,
        registry: &AppRegistry,
    ) -> Result<Self, Self::Rejection> {
        // Authorization: Bearer <token> を手動で取り出す（ボディには触らない）
        let auth = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or(AppError::UnauthorizedError)?;

        // "Bearer " フレックスを確認
        let token_str = auth
            .strip_prefix("Bearer ")
            .ok_or(AppError::UnauthorizedError)?;

        let access_token = AccessToken(token_str.to_string());

        let user_id = registry
            .auth_repository()
            .fetch_user_id_from_token(&access_token)
            .await?
            .ok_or(AppError::UnauthenticatedError)?;

        let user = match registry
            .user_repository()
            .find_current_user(user_id)
            .await?
        {
            Some(user) => user,
            None => return Err(AppError::UnauthenticatedError),
        };
         //   .ok_or(AppError::UnauthenticatedError)?;

        Ok(Self { access_token, user })
    }
}
