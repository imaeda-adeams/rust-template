use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use garde::Validate;
use kernel::model::{book::event::DeleteBook, id::BookId};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};
use crate::extractor::AuthorizedUser;
use crate::model::book::{BookListQuery, BookResponse, CreateBookRequest, PaginatedBookResponse,
                         UpdateBookRequest, UpdateBookRequestWithIds,
};

#[utoipa::path(post, path = "/books")]
pub async fn register_book(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateBookRequest>,
) -> Result<StatusCode, AppError> {
    req.validate()?;
    registry
        .book_repository()
        .create(req.into(), user.user_id())
        .await
        .map(|_| StatusCode::CREATED)
}

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/v1/books",
        responses(
            (status = 200, description = "蔵書一覧の取得に成功した場合", body = PaginatedBookResponse),
            (status = 400, description = "指定されたクエリの値に不備があった場合"),
            (status = 401, description = "認証されていないユーザがアクセスした場合")
        ),
        params(
            ("limit" = i64, Query, description = "一度に取得する蔵書数の上限値"),
            ("offset" = i64, Query, description = "取得対象とする蔵書一覧の開始位置")
        )
    )
)]
#[tracing::instrument(
    skip(_user, registry),
    fields(
        user_id = %_user.user_id().to_string()
    )
)]
pub async fn show_book_list(
    _user: AuthorizedUser,
    Query(query): Query<BookListQuery>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<PaginatedBookResponse>> {
    query.validate()?;

    registry
        .book_repository()
        .find_all(query.into())
        .await
        .map(PaginatedBookResponse::from)
        .map(Json)
}

#[utoipa::path(get, path = "/books/{book_id}")]
pub async fn show_book(
    _user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<BookResponse>> {
    registry
        .book_repository()
        .find_by_id(book_id)
        .await
        .and_then(|bc| match bc {
            Some(bc) => Ok(Json(bc.into())),
            None => Err(AppError::EntityNotFound("Book not found".into())),
        })
}

#[utoipa::path(put, path = "/books/{book_id}")]
pub async fn update_book(
    user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateBookRequest>,
) -> AppResult<StatusCode> {
    req.validate()?;

    let update_book = UpdateBookRequestWithIds::new(book_id, user.user_id(), req);

    registry
        .book_repository()
        .update(update_book.into())
        .await
        .map(|_| StatusCode::OK)
}

#[utoipa::path(delete, path = "/books/{book_id}")]
pub async fn delete_book(
    user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    let delete_book = DeleteBook {
        book_id,
        requested_user: user.user_id(),
    };

    registry
        .book_repository()
        .delete(delete_book)
        .await
        .map(|_| StatusCode::OK)
}