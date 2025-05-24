use serde::{Deserialize, Serialize};
use time::serde::rfc3339;
use time::OffsetDateTime;

use super::users::UserRole;

/// Status of the Invite.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InviteStatus {
    Accepted,
    Expired,
    Deleted,
    Pending,
}

/// Information about an organization invite.
#[derive(Debug, Deserialize)]
pub struct Invite {
    /// Email of the User being invited.
    pub email: String,
    /// RFC 3339 datetime string indicating when the Invite expires.
    #[serde(with = "rfc3339")]
    pub expires_at: OffsetDateTime,
    /// ID of the Invite.
    pub id: String,
    /// RFC 3339 datetime string indicating when the Invite was created.
    #[serde(with = "rfc3339")]
    pub invited_at: OffsetDateTime,
    /// Organization role of the User.
    pub role: UserRole,
    /// Status of the Invite.
    pub status: InviteStatus,
    /// Object type. Always `"invite"`.
    #[serde(rename = "type")]
    pub type_: String,
}

/// Parameters for creating an invite.
#[derive(Debug, Serialize)]
pub struct CreateInviteParams {
    /// Email of the User.
    pub email: String,
    /// Role for the invited User. Cannot be `Admin`.
    pub role: UserRole,
}

impl CreateInviteParams {
    /// Create a new [`CreateInviteParams`].
    pub fn new(email: impl Into<String>, role: UserRole) -> Self {
        Self {
            email: email.into(),
            role,
        }
    }
}

/// Parameters for listing invites.
#[derive(Debug, Serialize, Default)]
pub struct ListInvitesParams {
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

impl ListInvitesParams {
    /// Create a new `ListInvitesParams` with default values.
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

/// Response structure for listing invites.
#[derive(Debug, Deserialize)]
pub struct ListInvitesResponse {
    /// List of invites returned.
    pub data: Vec<Invite>,
    /// First ID in the data list. Can be used as the before_id for the previous page.
    pub first_id: Option<String>,
    /// Indicates if there are more results in the requested page direction.
    pub has_more: bool,
    /// Last ID in the data list. Can be used as the after_id for the next page.
    pub last_id: Option<String>,
}

/// Response type for retrieving an invite.
pub type GetInviteResponse = Invite;

#[cfg(test)]
mod tests {
    use super::ListInvitesParams;

    #[test]
    fn limit_clamps_upper_bound() {
        let params = ListInvitesParams::new().limit(2000);
        assert_eq!(params.limit, Some(1000));
    }

    #[test]
    fn limit_clamps_lower_bound() {
        let params = ListInvitesParams::new().limit(0);
        assert_eq!(params.limit, Some(1));
    }
}

