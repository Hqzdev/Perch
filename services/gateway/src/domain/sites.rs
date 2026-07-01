use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Site {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub origin: String,
    pub script_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewSite {
    pub organization_name: String,
    pub site_name: String,
    pub origin: String,
}

impl NewSite {
    pub fn new(organization_name: String, site_name: String, origin: String) -> Self {
        Self {
            organization_name: organization_name.trim().to_string(),
            site_name: site_name.trim().to_string(),
            origin: origin.trim().trim_end_matches('/').to_string(),
        }
    }

    pub fn valid(&self) -> bool {
        !self.organization_name.is_empty()
            && !self.site_name.is_empty()
            && (self.origin.starts_with("https://") || self.origin.starts_with("http://localhost"))
    }
}
