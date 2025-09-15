use axum::extract::{Path, Query};
use axum::response::Html;
use axum::{Router, extract::State, routing::get};
use magik::Renderable;
use tokio::join;
use tracing::info;

use crate::error::{DomainErrors, Error};
use crate::middleware::CacheControlLayer;
use crate::repo::{ArticlesRepo, AuthorsRepo, FeedRepo, SectionsRepo};
use crate::state::AppState;
use crate::web::pages;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(index).layer(CacheControlLayer::with_lifespan(3600)),
        )
        .route(
            "/editor",
            get(editor).layer(CacheControlLayer::with_lifespan(3600)),
        )
        .route(
            "/articles/{slug}",
            get(published_article_page).layer(CacheControlLayer::with_lifespan(3600)),
        )
        .route("/drafts/{slug}", get(draft_article_page))
        .route("/register", get(register))
}

async fn index(State(state): State<AppState>) -> Html<String> {
    match FeedRepo::get(&state.db).await {
        Ok((main_story, sections)) => Html(
            pages::Index {
                main_story,
                sections,
            }
            .render(),
        ),
        Err(e) => {
            info!("Error fetching feed: {}", e);
            Html(
                pages::Index {
                    main_story: None,
                    sections: Vec::new(),
                }
                .render(),
            )
        }
    }
}

#[derive(serde::Deserialize)]
pub struct GetParams {
    pub article: Option<i32>,
}

async fn editor(State(state): State<AppState>, id: Query<GetParams>) -> Html<String> {
    let (article, sections) = if let Some(article_id) = id.article {
        let (article, sections) = join!(
            ArticlesRepo::get_by_id(&state.db, article_id),
            SectionsRepo::get_sections_flat(&state.db)
        );

        (article.unwrap_or(None), sections.unwrap_or(Vec::new()))
    } else {
        (
            None,
            SectionsRepo::get_sections_flat(&state.db)
                .await
                .unwrap_or(Vec::new()),
        )
    };

    Html(
        pages::Editor {
            article: article.as_ref(),
            sections: &sections,
        }
        .render(),
    )
}

async fn published_article_page(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Html<String>, Error> {
    article_page_impl(state, slug, false).await
}

async fn draft_article_page(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Html<String>, Error> {
    article_page_impl(state, slug, true).await
}

async fn article_page_impl(
    state: AppState,
    slug: String,
    is_draft: bool,
) -> Result<Html<String>, Error> {
    let article = ArticlesRepo::get_by_slug(&state.db, &slug).await.map_err(DomainErrors::FetchingArticle)?;

    let Some(article) = article else {
        return Ok(Html(pages::NotFound {}.render()));
    };

    if is_draft {
        if article.published {
            return Ok(Html(pages::NotFound {}.render()));
        }
    } else if !article.published {
        return Ok(Html(pages::NotFound {}.render()));
    }

    let author = AuthorsRepo::get_by_id(&state.db, article.author_id).await.map_err(DomainErrors::ErrorFetchingAuthor)?;

    let Some(author) = author else {
        return Err(DomainErrors::AuthorNotFound)?;
    };

    Ok(Html(
        pages::Article {
            article: &article,
            author: &author,
        }
        .render(),
    ))
}

pub async fn register() -> Html<String> {
    Html(pages::Register {}.render())
}
