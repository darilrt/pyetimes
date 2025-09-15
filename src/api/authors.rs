use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};

use crate::{
    error::{DomainErrors, Error}, models::{Author, AuthorCreate}, repo::AuthorsRepo, state::AppState
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_authors))
        .route("/", post(post_author))
        .route("/{id}", get(get_author_by_id))
}

async fn get_authors(State(state): State<AppState>) -> Json<Vec<Author>> {
    let list = AuthorsRepo::get_all(&state.db).await.unwrap_or_default();

    Json(list)
}

async fn post_author(
    State(state): State<AppState>,
    Json(author): Json<AuthorCreate>,
) -> Result<Json<Author>, Error> {
    let author = AuthorsRepo::create(&state.db, &author).await.map_err(DomainErrors::CreatingAuthor)?;

    Ok(Json(author))
}

async fn get_author_by_id(
    State(state): State<AppState>,
    Path(id): axum::extract::Path<i32>,
) -> Result<Json<Author>, Error> {
    let author = AuthorsRepo::get_by_id(&state.db, id).await.map_err(DomainErrors::ErrorFetchingAuthor)?;

    match author {
        Some(author) => Ok(Json(author)),
        None => Err(DomainErrors::AuthorNotFound)?
    }
}
