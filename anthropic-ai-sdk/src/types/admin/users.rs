use serde::Deserialize;
use time::serde::rfc3339;
use time::OffsetDateTime;

/// Organization role of the user.
#[derive(Debug, Deserialize)]
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
