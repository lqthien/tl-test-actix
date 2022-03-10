use jelly::chrono::{DateTime, Utc};
use jelly::error::Error;
use jelly::serde::{Deserialize, Serialize};
use jelly::sqlx::{self, postgres::PgPool};

/// A Package Object.
#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub downloads_count: i32,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Package {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as_unchecked!(
            Package,
            "
            SELECT
                id, name, description, downloads_count,
                created, updated
            FROM packages
        "
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn create_sample(pool: &PgPool) -> Result<i32, Error> {
        Ok(sqlx::query!(
            "
            INSERT INTO packages (name, description)
            VALUES ($1, $2)
            RETURNING id
        ",
            "Sample Package 1",
            "Sample Description 1"
        )
        .fetch_one(pool)
        .await?
        .id)
    }
}
