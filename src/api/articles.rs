use axum::{
    Json, Router,
    extract::{Path, State},
    routing,
};

use crate::{
    error::{DomainErrors, Error},
    models::{Article, ArticleCreate, AuthorCredentials},
    repo::ArticlesRepo,
    state::AppState,
    utils::{auth::validate_user, discord},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", routing::post(post))
        .route("/{id}/publish", routing::post(publish)) // Temporalmente comentado
}

async fn post(
    State(state): State<AppState>,
    Json(info): Json<ArticleCreate>,
) -> Result<Json<Article>, Error> {
    let author = validate_user(&state.db, &info.author.email, &info.author.password).await?;

    let article = ArticlesRepo::get_by_slug(&state.db, &info.slug).await;

    let Ok(article) = article else {
        return Err(DomainErrors::ArticleNotFound)?;
    };

    let section = if info.section < 0 {
        None
    } else {
        Some(info.section)
    };

    if let Some(article) = article {
        if article.author_id != author.id {
            return Err(DomainErrors::NotOwner)?;
        }

        if article.published {
            return Err(DomainErrors::CannotUpdatePublishedArticle)?;
        }

        let article = ArticlesRepo::update(
            &state.db,
            &info.slug, // assuming slug is used as id for this example
            &info.title,
            &info.content,
            &info.tags,
            &info.excerpt,
            section,
        )
        .await
        .map_err(DomainErrors::UpdateError)?;

        return Ok(Json(article));
    }

    let article = ArticlesRepo::create(&state.db, author.id, &info).await;

    if let Err(sqlx::Error::Database(db_err)) = article {
        return match db_err.code() {
            Some(code) if code == "23505" => {
                Err(DomainErrors::ArticleAlreadyExists { slug: info.slug })?
            }
            _ => Err(Error::Database(sqlx::Error::Database(db_err))),
        };
    }

    let article = article.unwrap();

    if let Some(discord_bot) = &state.discord_bot {
        tokio::spawn(discord::notify_discord_bot(
            discord_bot.clone(),
            article.clone(),
        ));
    }

    Ok(Json(article))
}

#[derive(serde::Serialize)]
struct SuccessResponse {
    message: String,
}

async fn publish(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(credentials): Json<AuthorCredentials>,
) -> Result<Json<SuccessResponse>, Error> {
    let author = validate_user(&state.db, &credentials.email, &credentials.password).await?;

    if !author.can_publish {
        return Err(DomainErrors::NotAllowedToPublish)?;
    }

    let article = ArticlesRepo::publish(&state.db, id)
        .await
        .map_err(DomainErrors::ErrorPublishingArticle)?;

    if let Some(discord_bot) = &state.discord_bot {
        tokio::spawn(discord::notify_discord_bot(discord_bot.clone(), article));
    }

    Ok(Json(SuccessResponse {
        message: "Article published successfully".to_string(),
    }))
}
