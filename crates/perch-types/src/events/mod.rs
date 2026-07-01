use serde::{Deserialize, Serialize};

use crate::identifiers::{SiteId, TenantId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrawlRequested {
    pub tenant_id: TenantId,
    pub site_id: SiteId,
    pub root_url: String,
}
