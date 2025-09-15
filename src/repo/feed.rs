use std::collections::HashMap;

use sqlx::{FromRow, PgPool, Row};
use tokio::join;

use crate::models::{Article, Author, Section};

pub struct FeedRepo;

impl FeedRepo {
    pub async fn get(
        db: &PgPool,
    ) -> Result<(Option<(Article, Author)>, Vec<Section>), sqlx::Error> {
        let (story, sections) = join!(Self::get_main_story(db), Self::get_sections(db),);
        Ok((story?, sections?))
    }

    async fn get_main_story(db: &PgPool) -> Result<Option<(Article, Author)>, sqlx::Error> {
        let row = sqlx::query( 
            r#"SELECT a.*, au.* FROM articles AS a 
                    JOIN authors AS au ON a.author_id = au.id 
                    WHERE a.id = (SELECT article_id FROM main_story WHERE disabled = FALSE LIMIT 1)"#,
        )
        .fetch_optional(db)
        .await?;

        if let Some(row) = row {
            let article = Article::from_row(&row)?;
            let author = Author::from_row(&row)?;

            return Ok(Some((article, author)));
        }

        Ok(None)
    }

    async fn get_sections(db: &PgPool) -> Result<Vec<Section>, sqlx::Error> {
        let row = sqlx::query(
            r"
                SELECT 
                    -- Article fields
                    a.id as article_id,
                    a.title as article_title,
                    a.slug as article_slug,
                    a.content as article_content,
                    a.excerpt as article_excerpt,
                    a.author_id as article_author_id,
                    a.section_id as article_section_id,
                    a.published as article_published,
                    a.published_at as article_published_at,
                    a.tags as article_tags,
                    a.created_at as article_created_at,
                    a.updated_at as article_updated_at,
                    
                    -- Section fields
                    s.id as section_id,
                    s.priority as section_priority,
                    s.title as section_title
                FROM articles a 
                JOIN sections s ON a.section_id = s.id 
                WHERE 
                    a.published = TRUE AND
                    a.id IN (
                        SELECT id FROM articles a2 
                        WHERE 
                            a2.section_id = a.section_id AND
                            a2.published = TRUE 
                        ORDER BY a2.published_at ASC 
                        LIMIT 4
                    )
                ORDER BY a.published_at DESC
            ",
        )
        .fetch_all(db)
        .await?;

        let mut sections: HashMap<i32, Section> = HashMap::with_capacity(row.len());
        let mut articles: HashMap<i32, Vec<Article>> = HashMap::with_capacity(row.len());

        row.iter()
            .map(|row| {
                let article = Article {
                    id: row.try_get("article_id")?,
                    title: row.try_get("article_title")?,
                    slug: row.try_get("article_slug")?,
                    content: row.try_get("article_content")?,
                    excerpt: row.try_get("article_excerpt")?,
                    author_id: row.try_get("article_author_id")?,
                    section_id: row.try_get("article_section_id")?,
                    published: row.try_get("article_published")?,
                    published_at: row.try_get("article_published_at")?,
                    tags: row.try_get("article_tags")?,
                    created_at: row.try_get("article_created_at")?,
                    updated_at: row.try_get("article_updated_at")?,
                };

                let section = Section {
                    id: row.try_get("section_id")?,
                    priority: row.try_get("section_priority")?,
                    title: row.try_get("section_title")?,
                    articles: Vec::new(),
                };

                sections.insert(section.id, section);

                if let Some(article_section_id) = article.section_id {
                    if let Some(articles) = articles.get_mut(&article_section_id) {
                        articles.push(article);
                    } else {
                        articles.insert(article_section_id, vec![article]);
                    }
                }

                Ok::<_, sqlx::Error>(())
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()?;

        let mut sections = sections
            .into_values()
            .map(|mut section| {
                section.articles = articles.remove(&section.id).unwrap_or_default();
                section
            })
            .collect::<Vec<_>>();

        sections.sort_by_key(|s| s.priority);

        Ok(sections)
    }
}
