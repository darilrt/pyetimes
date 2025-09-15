use sqlx::PgPool;

use crate::models::{Article, ArticleCreate};

pub struct ArticlesRepo;

impl ArticlesRepo {
    // function to lowercase the tags
    pub fn lowercase_tags(tags: &[String]) -> Vec<String> {
        tags.iter().map(|tag| tag.trim().to_lowercase()).collect()
    }

    pub async fn create(
        db: &PgPool,
        author_id: i32,
        article: &ArticleCreate,
    ) -> Result<Article, sqlx::Error> {
        let query = r"
            INSERT INTO articles (title, slug, content, author_id, tags, excerpt, section_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
        ";

        let article = sqlx::query_as::<_, Article>(query)
            .bind(&article.title)
            .bind(&article.slug)
            .bind(&article.content)
            .bind(author_id)
            .bind(ArticlesRepo::lowercase_tags(&article.tags))
            .bind(&article.excerpt)
            .bind(article.section) // Assuming section is an Option<i32>
            .fetch_one(db)
            .await?;

        Ok(article)
    }

    pub async fn get_by_slug(db: &PgPool, slug: &str) -> Result<Option<Article>, sqlx::Error> {
        let query = "SELECT * FROM articles WHERE slug = $1";
        let article = sqlx::query_as::<_, Article>(query)
            .bind(slug)
            .fetch_optional(db)
            .await?;

        Ok(article)
    }

    pub async fn get_by_id(db: &PgPool, id: i32) -> Result<Option<Article>, sqlx::Error> {
        let query = "SELECT * FROM articles WHERE id = $1";
        let article = sqlx::query_as::<_, Article>(query)
            .bind(id)
            .fetch_optional(db)
            .await?;

        Ok(article)
    }

    pub async fn update(
        db: &PgPool,
        slug: &str, // assuming slug is used as id for this example
        title: &str,
        content: &str,
        tags: &[String],
        excerpt: &str,
        section: Option<i32>,
    ) -> Result<Article, sqlx::Error> {
        let query = r"
            UPDATE articles
            SET content = $1, tags = $2, excerpt = $3, title = $4, section_id = $5
            WHERE slug = $6
            RETURNING *
        ";

        let article = sqlx::query_as::<_, Article>(query)
            .bind(content)
            .bind(ArticlesRepo::lowercase_tags(tags))
            .bind(excerpt)
            .bind(title)
            .bind(slug)
            .bind(section)
            .fetch_one(db)
            .await?;

        Ok(article)
    }

    pub async fn publish(db: &PgPool, id: i32) -> Result<Article, sqlx::Error> {
        let query = r"
            UPDATE articles
            SET published = TRUE, published_at = NOW()
            WHERE id = $1
            RETURNING *
        ";

        let article = sqlx::query_as::<_, Article>(query)
            .bind(id)
            .fetch_one(db)
            .await?;

        Ok(article)
    }
}
