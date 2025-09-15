use sqlx::PgPool;

use crate::{error::Error, models::Section};

pub struct SectionsRepo;

impl SectionsRepo {
    pub async fn get_sections_flat(db: &PgPool) -> Result<Vec<Section>, Error> {
        let sections: Vec<Section> =
            sqlx::query_as::<_, Section>("SELECT * FROM sections ORDER BY priority")
                .fetch_all(db)
                .await?;

        Ok(sections)
    }
}
