use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

/// Workspace information returned by the Admin API.
#[derive(Debug, Deserialize)]
pub struct Workspace {
    /// When the workspace was archived, if at all.
    #[serde(with = "rfc3339::option")]
    pub archived_at: Option<OffsetDateTime>,
    /// When the workspace was created.
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
    /// Hex color code representing the workspace.
    pub display_color: String,
    /// ID of the workspace.
    pub id: String,
    /// Name of the workspace.
    pub name: String,
    /// Object type (always "workspace").
    #[serde(rename = "type")]
    pub type_: String,
}

/// Parameters for listing workspaces.
#[derive(Debug, Serialize, Default)]
pub struct ListWorkspacesParams {
    /// Whether to include archived workspaces in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_archived: Option<bool>,
    /// Cursor for pagination (before).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_id: Option<String>,
    /// Cursor for pagination (after).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_id: Option<String>,
    /// Number of items per page (1-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

impl ListWorkspacesParams {
    /// Create a new `ListWorkspacesParams` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Include archived workspaces in the response.
    pub fn include_archived(mut self, include: bool) -> Self {
        self.include_archived = Some(include);
        self
    }

    /// Set the `before_id` parameter.
    pub fn before_id(mut self, before_id: impl Into<String>) -> Self {
        self.before_id = Some(before_id.into());
        self
    }

    /// Set the `after_id` parameter.
    pub fn after_id(mut self, after_id: impl Into<String>) -> Self {
        self.after_id = Some(after_id.into());
        self
    }

    /// Set the `limit` parameter (1-1000).
    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit.clamp(1, 1000));
        self
    }
}

/// Response structure for listing workspaces.
#[derive(Debug, Deserialize)]
pub struct ListWorkspacesResponse {
    /// List of workspaces returned.
    pub data: Vec<Workspace>,
    /// First ID in the list.
    pub first_id: Option<String>,
    /// Indicates if there are more results.
    pub has_more: bool,
    /// Last ID in the list.
    pub last_id: Option<String>,
}

/// Response type for retrieving a workspace.
pub type GetWorkspaceResponse = Workspace;

#[cfg(test)]
mod tests {
    use super::ListWorkspacesParams;

    #[test]
    fn limit_clamps_upper_bound() {
        let params = ListWorkspacesParams::new().limit(2000);
        assert_eq!(params.limit, Some(1000));
    }

    #[test]
    fn limit_clamps_lower_bound() {
        let params = ListWorkspacesParams::new().limit(0);
        assert_eq!(params.limit, Some(1));
    }
}

