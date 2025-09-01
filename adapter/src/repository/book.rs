use crate::database::model::book::{BookRow, PaginatedBookRow};
use crate::database::ConnectionPool;
use async_trait::async_trait;
use derive_new::new;
use kernel::model::book::event::{DeleteBook, UpdateBook};
use kernel::model::book::{event::CreateBook, Book, BookListOptions};
use kernel::model::id::{BookId, UserId};
use kernel::model::list::PaginatedList;
use kernel::repository::book::BookRepository;
use shared::error::{AppError, AppResult};

#[derive(new)]
pub struct BookRepositoryImpl {
    pool: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn create(&self, event: CreateBook, user_id: UserId) -> AppResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO books (title, author, isbn, description, user_id)
                VALUES ($1, $2, $3, $4, $5)
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description,
            user_id as _
        )
        .execute(self.pool.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(())
    }

    async fn find_all(&self, options: BookListOptions) -> AppResult<PaginatedList<Book>> {
        let BookListOptions { limit, offset } = options;

        let rows: Vec<PaginatedBookRow> = sqlx::query_as!(
            PaginatedBookRow,
            r#"
                SELECT
                    COUNT(*) OVER() AS "total!",
                    b.id AS id
                FROM books AS b
                ORDER BY b.created_at DESC
                LIMIT $1 OFFSET $2
            "#,
            limit,
            offset,
        )
        .fetch_all(self.pool.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        let total = rows.first().map(|row| row.total).unwrap_or(0);

        let book_ids: Vec<BookId> = rows.into_iter().map(|row| row.id).collect::<Vec<BookId>>();

        let rows: Vec<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                 b.id AS id,
                 b.title AS title,
                 b.author AS author,
                 b.isbn AS isbn,
                 b.description AS description,
                 u.id AS owned_by,
                 u.name AS owner_name
                FROM books AS b
                INNER JOIN users as u ON u.id = b.user_id
                WHERE b.id IN (SELECT * FROM UNNEST($1::uuid[]))
                ORDER BY b.created_at DESC
            "#,
            &book_ids as _
        )
            .fetch_all(self.pool.inner_ref())
        .await
            .map_err(AppError::SpecificOperationError)?;

        let items = rows.into_iter().map(Book::from).collect::<Vec<Book>>();

        Ok(PaginatedList {
            total,
            limit,
            offset,
            items,
        })
    }

    async fn find_by_id(&self, id: BookId) -> AppResult<Option<Book>> {
        let row: Option<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                 b.id AS id,
                 b.title AS title,
                 b.author AS author,
                 b.isbn AS isbn,
                 b.description AS description,
                 u.id AS owned_by,
                 u.name AS owner_name
                FROM books AS b
                INNER JOIN users as u ON u.id = b.user_id
                WHERE b.id = $1
            "#,
            id as _
        )
        .fetch_optional(self.pool.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(row.map(Book::from))
    }

    async fn update(&self, event: UpdateBook) -> AppResult<()> {

        let res = sqlx::query!(
            r#"
                UPDATE books
                SET
                    title = $1,
                    author = $2,
                    isbn = $3,
                    description = $4
                WHERE id = $5
                AND user_id = $6
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description,
            event.id as _,
            event.requested_user as _
        )
            .execute(self.pool.inner_ref())
            .await
            .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::EntityNotFound(
                format!("Book with id {} not found", event.id),
            ));

        }
        Ok(())
    }

    async fn delete(&self, event: DeleteBook) -> AppResult<()> {

        let res = sqlx::query!(
            r#"
                DELETE FROM books
                WHERE id = $1
                AND user_id = $2
            "#,
            event.id as _,
            event.requested_user as _
        )
            .execute(self.pool.inner_ref())
            .await
            .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::EntityNotFound(
                format!("Book with id {} not found", event.id),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::user::UserRepositoryImpl;
    use kernel::{
        model::user::event::CreateUser,
        repository::user::UserRepository,
    };

    #[sqlx::test]
    async fn test_register_book(pool: sqlx::PgPool) -> anyhow::Result<()> {

        sqlx::query!(
            r#"
            INSERT INTO roles(name) VALUES ('Admin'), ('User');
            "#
        ).execute(&pool).await?;

        let user_repo = UserRepositoryImpl::new(ConnectionPool::new(pool.clone()));

        let user = user_repo
            .create(CreateUser {
                name: "Test User".into(),
                email: "test@example.com".into(),
                password: "test_password".into(),
            })
            .await?;

        let repo = BookRepositoryImpl::new(ConnectionPool::new(pool));

        let book = CreateBook {
            title: "Test Title".into(),
            author: "Test Author".into(),
            isbn: "Test ISBN".into(),
            description: "Test Description".into(),
        };

        repo.create(book, user.id).await?;

        let options = BookListOptions{
            limit: 10,
            offset: 0,
        };
        let res = repo.find_all(options).await?;
        assert_eq!(res.items.len(), 1);

        let book_id = res.items[0].id;
        let res = repo.find_by_id(book_id).await?;
        assert!(res.is_some());

        let Book {
            id,
            title,
            author,
            isbn,
            description,
            owner
        } = res.unwrap();
        assert_eq!(id, book_id);
        assert_eq!(title, "Test Title");
        assert_eq!(author, "Test Author");
        assert_eq!(isbn, "Test ISBN");
        assert_eq!(description, "Test Description");
        assert_eq!(owner.name, "Test User");

        Ok(())
    }
}
