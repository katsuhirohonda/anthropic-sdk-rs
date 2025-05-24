use serde::{Deserialize, Serialize};

/// Role of the Workspace Member.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceRole {
    WorkspaceUser,
    WorkspaceDeveloper,
    WorkspaceAdmin,
    WorkspaceBilling,
}

/// Information about a workspace member returned by the Admin API.
#[derive(Debug, Deserialize)]
pub struct WorkspaceMember {
    /// Object type. Always `"workspace_member"`.
    #[serde(rename = "type")]
    pub type_: String,
    /// ID of the user.
    pub user_id: String,
    /// ID of the workspace.
    pub workspace_id: String,
    /// Role of the member within the workspace.
    pub workspace_role: WorkspaceRole,
}

/// Parameters for adding a workspace member.
#[derive(Debug, Serialize)]
pub struct AdminAddWorkspaceMemberParams {
    /// ID of the user to add to the workspace.
    pub user_id: String,
    /// Role for the new workspace member.
    pub workspace_role: WorkspaceRole,
}

impl AdminAddWorkspaceMemberParams {
    /// Create new parameters with the required fields.
    pub fn new(user_id: impl Into<String>, workspace_role: WorkspaceRole) -> Self {
        Self {
            user_id: user_id.into(),
            workspace_role,
        }
    }
}

/// Parameters for updating a workspace member.
#[derive(Debug, Serialize)]
pub struct AdminUpdateWorkspaceMemberParams {
    /// New workspace role for the User.
    pub workspace_role: WorkspaceRole,
}

impl AdminUpdateWorkspaceMemberParams {
    /// Create new parameters with the required workspace role.
    pub fn new(workspace_role: WorkspaceRole) -> Self {
        Self { workspace_role }
    }
}

/// Parameters for listing workspace members.
#[derive(Debug, Serialize, Default)]
pub struct ListWorkspaceMembersParams {
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

impl ListWorkspaceMembersParams {
    /// Create a new `ListWorkspaceMembersParams` with default values.
    pub fn new() -> Self {
        Self::default()
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

/// Response structure for listing workspace members.
#[derive(Debug, Deserialize)]
pub struct ListWorkspaceMembersResponse {
    /// List of workspace members returned.
    pub data: Vec<WorkspaceMember>,
    /// First ID in the data list.
    pub first_id: Option<String>,
    /// Indicates if there are more results.
    pub has_more: bool,
    /// Last ID in the data list.
    pub last_id: Option<String>,
}

/// Response type for retrieving a workspace member.
pub type GetWorkspaceMemberResponse = WorkspaceMember;

#[cfg(test)]
mod tests {
    use super::ListWorkspaceMembersParams;

    #[test]
    fn limit_clamps_upper_bound() {
        let params = ListWorkspaceMembersParams::new().limit(2000);
        assert_eq!(params.limit, Some(1000));
    }

    #[test]
    fn limit_clamps_lower_bound() {
        let params = ListWorkspaceMembersParams::new().limit(0);
        assert_eq!(params.limit, Some(1));
    }
}
