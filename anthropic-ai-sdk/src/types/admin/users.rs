use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

/// Organization role of the user.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    User,
    Developer,
    Billing,
    Admin,
}

/// Detailed information about an organization user.
#[derive(Debug, Deserialize)]
pub struct OrganizationUser {
    /// When the user was added to the organization.
    #[serde(with = "rfc3339")]
    pub added_at: OffsetDateTime,
    /// Email of the user.
    pub email: String,
    /// ID of the user.
    pub id: String,
    /// Name of the user.
    pub name: String,
    /// Role of the user within the organization.
    pub role: UserRole,
    /// Object type. Always `"user"`.
    #[serde(rename = "type")]
    pub type_: String,
}

/// Parameters for listing organization users
#[derive(Debug, Serialize, Default)]
pub struct ListUsersParams {
    /// Cursor for pagination (before)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_id: Option<String>,
    /// Cursor for pagination (after)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_id: Option<String>,
    /// Number of items per page (1-1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    /// Filter by user email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl ListUsersParams {
    /// Create a new `ListUsersParams` with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `before_id` parameter
    pub fn before_id(mut self, before_id: impl Into<String>) -> Self {
        self.before_id = Some(before_id.into());
        self
    }

    /// Set the `after_id` parameter
    pub fn after_id(mut self, after_id: impl Into<String>) -> Self {
        self.after_id = Some(after_id.into());
        self
    }

    /// Set the `limit` parameter (1-1000)
    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit.clamp(1, 1000));
        self
    }

    /// Set the `email` filter
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }
}

/// Response structure for listing organization users
#[derive(Debug, Deserialize)]
pub struct ListUsersResponse {
    /// List of users
    pub data: Vec<OrganizationUser>,
    /// First ID in the data list
    pub first_id: Option<String>,
    /// Indicates if there are more results
    pub has_more: bool,
    /// Last ID in the data list
    pub last_id: Option<String>,
}

/// Parameters for updating a user.
#[derive(Debug, Serialize)]
pub struct AdminUpdateUserParams {
    /// New role for the User.
    pub role: UserRole,
}

impl AdminUpdateUserParams {
    /// Create a new `AdminUpdateUserParams` with the required role.
    pub fn new(role: UserRole) -> Self {
        Self { role }
    }
}

/// Response type for deleting a user.
#[derive(Debug, Deserialize)]
pub struct DeleteUserResponse {
    /// ID of the User.
    pub id: String,
    /// Deleted object type. Always `"user_deleted"`.
    #[serde(rename = "type")]
    pub obj_type: String,
}

#[cfg(test)]
mod tests {
    use super::ListUsersParams;

    #[test]
    fn limit_clamps_upper_bound() {
        let params = ListUsersParams::new().limit(2000);
        assert_eq!(params.limit, Some(1000));
    }

    #[test]
    fn limit_clamps_lower_bound() {
        let params = ListUsersParams::new().limit(0);
        assert_eq!(params.limit, Some(1));
    }
}
