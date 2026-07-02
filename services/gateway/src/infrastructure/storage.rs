use perch_storage::Database;
use sqlx::Row;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::messages::{AssistantAnswer, VisitorMessage};
use crate::domain::sites::{NewSite, Site};

#[derive(Debug, Clone)]
pub struct SiteRepository {
    database: Database,
}

#[derive(Debug, Error)]
pub enum SiteRepositoryError {
    #[error("site already exists")]
    Conflict,
    #[error("payload serialization failed: {0}")]
    Serialization(String),
    #[error("database query failed: {0}")]
    Query(#[from] sqlx::Error),
}

impl SiteRepository {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub async fn create_site(&self, new_site: NewSite) -> Result<Site, SiteRepositoryError> {
        let organization_id = Uuid::new_v4();
        let site_id = Uuid::new_v4();
        let script_key = format!("pk_dev_{}", Uuid::new_v4().simple());

        let organization_exists = sqlx::query(
            "insert into organizations (id, name) values ($1, $2) on conflict do nothing",
        )
        .bind(organization_id)
        .bind(&new_site.organization_name)
        .execute(self.database.pool())
        .await?;

        if organization_exists.rows_affected() == 0 {
            return Err(SiteRepositoryError::Conflict);
        }

        let row = sqlx::query(
            "insert into sites (id, organization_id, name, origin, script_key) values ($1, $2, $3, $4, $5) returning id, organization_id, name, origin, script_key",
        )
        .bind(site_id)
        .bind(organization_id)
        .bind(&new_site.site_name)
        .bind(&new_site.origin)
        .bind(&script_key)
        .fetch_one(self.database.pool())
        .await
        .map_err(|error| match error {
            sqlx::Error::Database(database_error) if database_error.is_unique_violation() => {
                SiteRepositoryError::Conflict
            }
            other => SiteRepositoryError::Query(other),
        })?;

        Ok(Self::site_from_row(row))
    }

    pub async fn find_by_script_key(
        &self,
        script_key: &str,
    ) -> Result<Option<Site>, SiteRepositoryError> {
        let row = sqlx::query(
            "select id, organization_id, name, origin, script_key from sites where script_key = $1",
        )
        .bind(script_key)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(row.map(Self::site_from_row))
    }

    pub async fn find_by_id(&self, site_id: Uuid) -> Result<Option<Site>, SiteRepositoryError> {
        let row = sqlx::query(
            "select id, organization_id, name, origin, script_key from sites where id = $1",
        )
        .bind(site_id)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(row.map(Self::site_from_row))
    }

    pub async fn record_widget_exchange(
        &self,
        site_id: Uuid,
        message: VisitorMessage,
        answer: AssistantAnswer,
    ) -> Result<AssistantAnswer, SiteRepositoryError> {
        let mut transaction = self.database.pool().begin().await?;
        let conversation_id = Uuid::new_v4();
        let visitor_message_id = Uuid::new_v4();
        let assistant_message_id = Uuid::new_v4();
        let citations = serde_json::to_value(&answer.citations)
            .map_err(|error| SiteRepositoryError::Serialization(error.to_string()))?;

        sqlx::query("insert into conversations (id, site_id, visitor_id) values ($1, $2, $3)")
            .bind(conversation_id)
            .bind(site_id)
            .bind(&message.session_id)
            .execute(&mut *transaction)
            .await?;

        sqlx::query(
            "insert into messages (id, conversation_id, role, content) values ($1, $2, 'visitor', $3)",
        )
        .bind(visitor_message_id)
        .bind(conversation_id)
        .bind(&message.content)
        .execute(&mut *transaction)
        .await?;

        sqlx::query(
            "insert into messages (id, conversation_id, role, content, citations) values ($1, $2, 'assistant', $3, $4)",
        )
        .bind(assistant_message_id)
        .bind(conversation_id)
        .bind(&answer.content)
        .bind(citations)
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(AssistantAnswer {
            conversation_id,
            message_id: assistant_message_id,
            content: answer.content,
            citations: answer.citations,
        })
    }

    fn site_from_row(row: sqlx::postgres::PgRow) -> Site {
        Site {
            id: row.get("id"),
            organization_id: row.get("organization_id"),
            name: row.get("name"),
            origin: row.get("origin"),
            script_key: row.get("script_key"),
        }
    }
}
