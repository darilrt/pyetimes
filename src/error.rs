use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error(
        "Failed to create database pool: {0}. Ensure the database is running and the connection string is correct."
    )]
    DatabaseConnection(sqlx::Error),

    #[error("Unauthorized")]
    Unauthorized,

    #[error(transparent)]
    DomainErrors(#[from] DomainErrors),
}

#[derive(Error, Debug)]
pub enum DomainErrors {
    #[error("Error fetching article")]
    FetchingArticle(sqlx::Error),
    
    #[error("Error checking for existing article")]
    ArticleNotFound,

    #[error("You are not allowed to publish articles")]
    NotAllowedToPublish,

    #[error("You can only update your own articles")]
    NotOwner,

    #[error("Cannot update a published article")]
    CannotUpdatePublishedArticle,

    #[error("Error updating article")]
    UpdateError(sqlx::Error),

    #[error("Article with slug '{slug}' already exists")]
    ArticleAlreadyExists { slug: String },

    #[error("Error publishing article")]
    ErrorPublishingArticle(sqlx::Error),

    #[error("Error creating author")]
    CreatingAuthor(sqlx::Error),

    #[error("Author not found")]
    AuthorNotFound,

    #[error("Error fetching author")]
    ErrorFetchingAuthor(sqlx::Error)
}

impl IntoResponse for DomainErrors {
    fn into_response(self) -> Response {
        let message = format!("{self}");
        match self {
            DomainErrors::FetchingArticle(sqlx_error) => {
                tracing::error!("{message} {sqlx_error}");
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            },
            DomainErrors::ArticleNotFound => {
                tracing::error!("Article not found: {}", DomainErrors::ArticleNotFound);
                (StatusCode::NOT_FOUND, message).into_response()
            }
            DomainErrors::NotOwner => (StatusCode::FORBIDDEN, message).into_response(),
            DomainErrors::CannotUpdatePublishedArticle => {
                (StatusCode::CONFLICT, message).into_response()
            }
            DomainErrors::UpdateError(sqlx_error) => {
                tracing::error!("Error updating article {sqlx_error}");
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }
            DomainErrors::ArticleAlreadyExists { .. } => {
                (StatusCode::CONFLICT, message).into_response()
            }
            DomainErrors::NotAllowedToPublish => (StatusCode::FORBIDDEN, message).into_response(),
            DomainErrors::ErrorPublishingArticle(sqlx_error) => {
                tracing::error!("Error publishing article {sqlx_error}");
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            },
            DomainErrors::CreatingAuthor(sqlx_error) => {
                tracing::error!("{message} {sqlx_error}");
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            },
            DomainErrors::AuthorNotFound => {
                (StatusCode::NOT_FOUND, message).into_response()
            },
            DomainErrors::ErrorFetchingAuthor(sqlx_error) => {
                tracing::error!("{message} {sqlx_error}");
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Database(err) => {
                tracing::error!("Database error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
            Error::DatabaseConnection(err) => {
                tracing::error!("Database error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
            Error::Unauthorized => {
                tracing::info!("Unauthorized access");
                (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
            }
            Error::DomainErrors(e) => e.into_response(),
        }
    }
}
