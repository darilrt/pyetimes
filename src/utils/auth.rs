use crate::error::Error;
use crate::models::Author;
use crate::repo::AuthorsRepo;
use sqlx::PgPool;

/// Validates user credentials and returns the authenticated author if successful
///
/// # Arguments
/// * `db` - Database connection pool
/// * `email` - User's email address
/// * `password` - Plain text password to verify
///
/// # Returns
/// * `Ok(Author)` - Authentication successful
/// * `Err(Error)` - Database error or invalid credentials
pub async fn validate_user(db: &PgPool, email: &str, password: &str) -> Result<Author, Error> {
    let author = AuthorsRepo::get_by_email(db, email)
        .await
        .map_err(|_| Error::Unauthorized)?;

    let password_valid = bcrypt::verify(password, &author.password_hash).unwrap_or(false);

    if password_valid {
        Ok(author)
    } else {
        Err(Error::Unauthorized)
    }
}
