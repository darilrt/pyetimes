use bcrypt::hash;
use sqlx::PgPool;

use crate::models::{Author, AuthorCreate};

pub struct AuthorsRepo;

impl AuthorsRepo {
    pub async fn create(db: &PgPool, author: &AuthorCreate) -> Result<Author, sqlx::Error> {
        let query = r"
            INSERT INTO authors (name, email, password_hash, bio)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, email, bio, password_hash, can_publish, profile_image, created_at
        ";

        let password_hash = hash(&author.password, 10).unwrap();

        let author = sqlx::query_as::<_, Author>(query)
            .bind(&author.name)
            .bind(&author.email)
            .bind(&password_hash)
            .bind(&author.bio)
            .fetch_one(db)
            .await?;

        Ok(author)
    }

    pub async fn get_all(db: &PgPool) -> Result<Vec<Author>, sqlx::Error> {
        let query = "SELECT id, name, email, bio, '' as password_hash, can_publish, profile_image, created_at FROM authors";
        let author = sqlx::query_as::<_, Author>(query).fetch_all(db).await?;

        Ok(author)
    }

    pub async fn get_by_id(db: &PgPool, id: i32) -> Result<Option<Author>, sqlx::Error> {
        let query = "SELECT id, name, email, bio, '' as password_hash, can_publish, can_publish, profile_image, created_at FROM authors WHERE id = $1";
        let author = sqlx::query_as::<_, Author>(query)
            .bind(id)
            .fetch_optional(db)
            .await?;

        Ok(author)
    }

    pub async fn get_by_email(db: &PgPool, email: &str) -> Result<Author, sqlx::Error> {
        let query = "SELECT id, name, email, bio, password_hash, can_publish, profile_image, created_at FROM authors WHERE email = $1";

        sqlx::query_as::<_, Author>(query)
            .bind(email)
            .fetch_one(db)
            .await
    }
}
