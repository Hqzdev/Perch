use perch_storage::Database;
use perch_types::api::{
    DashboardConversationSummary, DashboardPageSummary, DashboardSiteDetail, DashboardSiteSummary,
};
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

    pub async fn list_dashboard_sites(
        &self,
    ) -> Result<Vec<DashboardSiteSummary>, SiteRepositoryError> {
        let rows = sqlx::query(
            "
            select
                sites.id,
                sites.organization_id,
                sites.name,
                sites.origin,
                sites.script_key,
                sites.created_at::text as created_at,
                count(distinct site_pages.id)::bigint as pages_indexed,
                count(distinct conversations.id)::bigint as conversations_count,
                max(site_pages.last_indexed_at)::text as last_indexed_at
            from sites
            left join site_pages on site_pages.site_id = sites.id
            left join conversations on conversations.site_id = sites.id
            group by sites.id
            order by sites.created_at desc
            ",
        )
        .fetch_all(self.database.pool())
        .await?;

        rows.into_iter()
            .map(Self::dashboard_site_from_row)
            .collect()
    }

    pub async fn dashboard_site(
        &self,
        site_id: Uuid,
    ) -> Result<Option<DashboardSiteDetail>, SiteRepositoryError> {
        let row = sqlx::query(
            "
            select
                sites.id,
                sites.organization_id,
                sites.name,
                sites.origin,
                sites.script_key,
                sites.created_at::text as created_at,
                count(distinct site_pages.id)::bigint as pages_indexed,
                count(distinct conversations.id)::bigint as conversations_count,
                max(site_pages.last_indexed_at)::text as last_indexed_at
            from sites
            left join site_pages on site_pages.site_id = sites.id
            left join conversations on conversations.site_id = sites.id
            where sites.id = $1
            group by sites.id
            ",
        )
        .bind(site_id)
        .fetch_optional(self.database.pool())
        .await?;

        row.map(Self::dashboard_site_from_row)
            .transpose()
            .map(|summary| {
                summary.map(|site| DashboardSiteDetail {
                    install_snippet: install_snippet(&site.script_key),
                    site,
                })
            })
    }

    pub async fn list_dashboard_pages(
        &self,
        site_id: Uuid,
    ) -> Result<Vec<DashboardPageSummary>, SiteRepositoryError> {
        let rows = sqlx::query(
            "
            select
                site_pages.id,
                site_pages.url,
                site_pages.title,
                site_pages.status,
                site_pages.last_indexed_at::text as last_indexed_at,
                count(page_chunks.id)::bigint as chunks_indexed
            from site_pages
            left join page_chunks on page_chunks.page_id = site_pages.id
            where site_pages.site_id = $1
            group by site_pages.id
            order by site_pages.last_indexed_at desc nulls last, site_pages.created_at desc
            ",
        )
        .bind(site_id)
        .fetch_all(self.database.pool())
        .await?;

        rows.into_iter()
            .map(Self::dashboard_page_from_row)
            .collect()
    }

    pub async fn list_dashboard_conversations(
        &self,
        site_id: Uuid,
    ) -> Result<Vec<DashboardConversationSummary>, SiteRepositoryError> {
        let rows = sqlx::query(
            "
            select
                conversations.id,
                conversations.visitor_id,
                conversations.created_at::text as created_at,
                count(messages.id)::bigint as messages_count,
                max(messages.created_at)::text as last_message_at
            from conversations
            left join messages on messages.conversation_id = conversations.id
            where conversations.site_id = $1
            group by conversations.id
            order by max(messages.created_at) desc nulls last, conversations.created_at desc
            limit 20
            ",
        )
        .bind(site_id)
        .fetch_all(self.database.pool())
        .await?;

        rows.into_iter()
            .map(Self::dashboard_conversation_from_row)
            .collect()
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

    fn dashboard_site_from_row(
        row: sqlx::postgres::PgRow,
    ) -> Result<DashboardSiteSummary, SiteRepositoryError> {
        Ok(DashboardSiteSummary {
            id: row.get("id"),
            organization_id: row.get("organization_id"),
            name: row.get("name"),
            origin: row.get("origin"),
            script_key: row.get("script_key"),
            pages_indexed: integer_from_row(&row, "pages_indexed")?,
            conversations_count: integer_from_row(&row, "conversations_count")?,
            last_indexed_at: row.get("last_indexed_at"),
            created_at: row.get("created_at"),
        })
    }

    fn dashboard_page_from_row(
        row: sqlx::postgres::PgRow,
    ) -> Result<DashboardPageSummary, SiteRepositoryError> {
        Ok(DashboardPageSummary {
            id: row.get("id"),
            url: row.get("url"),
            title: row.get("title"),
            status: row.get("status"),
            chunks_indexed: integer_from_row(&row, "chunks_indexed")?,
            last_indexed_at: row.get("last_indexed_at"),
        })
    }

    fn dashboard_conversation_from_row(
        row: sqlx::postgres::PgRow,
    ) -> Result<DashboardConversationSummary, SiteRepositoryError> {
        Ok(DashboardConversationSummary {
            id: row.get("id"),
            visitor_id: row.get("visitor_id"),
            messages_count: integer_from_row(&row, "messages_count")?,
            last_message_at: row.get("last_message_at"),
            created_at: row.get("created_at"),
        })
    }
}

fn integer_from_row(
    row: &sqlx::postgres::PgRow,
    column: &str,
) -> Result<usize, SiteRepositoryError> {
    let value: i64 = row.get(column);

    usize::try_from(value).map_err(|error| SiteRepositoryError::Serialization(error.to_string()))
}

fn install_snippet(script_key: &str) -> String {
    format!(
        "<script src=\"https://cdn.perch.ai/widget.js\" data-perch-key=\"{}\"></script>",
        script_key
    )
}
