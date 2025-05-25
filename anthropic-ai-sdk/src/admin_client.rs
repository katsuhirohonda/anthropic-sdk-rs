//! Admin API
//!
//! This module contains the implementations for the Anthropic Admin API endpoints.
//! It provides functionality for managing API keys and other administrative tasks.

use crate::client::AnthropicClient;
use crate::types::admin::api_keys::{
    AdminClient, AdminError, AdminUpdateApiKeyParams, ApiKey, ListApiKeysParams,
    ListApiKeysResponse,
};
use crate::types::admin::users::{
    AdminUpdateUserParams, DeleteUserResponse, ListUsersParams, ListUsersResponse, OrganizationUser,
};
use crate::types::admin::workspaces::{
    GetWorkspaceResponse, ListWorkspacesParams, ListWorkspacesResponse,
};
use async_trait::async_trait;
use crate::types::admin::workspace_members::{
    AdminAddWorkspaceMemberParams, AdminUpdateWorkspaceMemberParams,
    GetWorkspaceMemberResponse, ListWorkspaceMembersParams, ListWorkspaceMembersResponse,
    WorkspaceMember,
};
use crate::types::admin::invites::{
    DeleteInviteResponse, GetInviteResponse, ListInvitesParams, ListInvitesResponse,
};

#[async_trait]
impl AdminClient for AnthropicClient {
    /// Lists API keys
    ///
    /// Retrieves a list of API keys with optional filtering and pagination.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Returns
    ///
    /// Returns a list of API keys and pagination information on success.
    ///
    /// # Errors
    ///
    /// Returns an `AdminError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anthropic_ai_sdk::client::AnthropicClient;
    /// use anthropic_ai_sdk::types::admin::api_keys::{AdminClient, AdminError};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), AdminError> {
    ///     let client = AnthropicClient::new::<AdminError>(
    ///         "your-admin-api-key",
    ///         "2023-06-01",
    ///     )?;
    ///
    ///     // List all API keys
    ///     let api_keys = client.list_api_keys(None).await?;
    ///     for api_key in api_keys.data {
    ///         println!("API Key: {} ({})", api_key.name, api_key.id);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn list_api_keys<'a>(
        &'a self,
        params: Option<&'a ListApiKeysParams>,
    ) -> Result<ListApiKeysResponse, AdminError> {
        self.get("/organizations/api_keys", params).await
    }

    /// Gets a specific API key
    ///
    /// Retrieves details for a specific API key by its ID.
    ///
    /// # Arguments
    ///
    /// * `api_key_id` - The ID of the API key to retrieve
    ///
    /// # Returns
    ///
    /// Returns the API key details on success.
    ///
    /// # Errors
    ///
    /// Returns an `AdminError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    /// - The API key is not found
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anthropic_ai_sdk::client::AnthropicClient;
    /// use anthropic_ai_sdk::types::admin::api_keys::{AdminClient, AdminError};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), AdminError> {
    ///     let client = AnthropicClient::new::<AdminError>(
    ///         "your-admin-api-key",
    ///         "2023-06-01",
    ///     )?;
    ///
    ///     // Get a specific API key
    ///     let api_key = client.get_api_key_by_id("api_key_xyz").await?;
    ///     println!("API Key: {} ({})", api_key.name, api_key.id);
    ///     println!("Status: {:?}", api_key.status);
    ///     println!("Partial Hint: {}", api_key.partial_key_hint);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn get_api_key_by_id<'a>(&'a self, api_key_id: &'a str) -> Result<ApiKey, AdminError> {
        self.get(
            &format!("/organizations/api_keys/{}", api_key_id),
            Option::<&()>::None,
        )
        .await
    }

    /// Updates an API key
    ///
    /// Updates properties of an API key by its ID.
    ///
    /// # Arguments
    ///
    /// * `api_key_id` - The ID of the API key to update
    /// * `params` - Parameters for updating the API key
    ///
    /// # Returns
    ///
    /// Returns the updated API key details on success.
    ///
    /// # Errors
    ///
    /// Returns an `AdminError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    /// - The API key is not found
    /// - Invalid parameters are provided
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anthropic_ai_sdk::client::AnthropicClient;
    /// use anthropic_ai_sdk::types::admin::api_keys::{AdminClient, AdminError, ApiKeyStatus, AdminUpdateApiKeyParams};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), AdminError> {
    ///     let client = AnthropicClient::new::<AdminError>(
    ///         "your-admin-api-key",
    ///         "2023-06-01",
    ///     )?;
    ///
    ///     // Update an API key
    ///     let params = AdminUpdateApiKeyParams::new()
    ///         .name("Updated API Key")
    ///         .status(ApiKeyStatus::Inactive);
    ///
    ///     let api_key = client.update_api_key("api_key_xyz", &params).await?;
    ///     println!("Updated API Key: {} ({})", api_key.name, api_key.id);
    ///     println!("New Status: {:?}", api_key.status);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn update_api_key<'a>(
        &'a self,
        api_key_id: &'a str,
        params: &'a AdminUpdateApiKeyParams,
    ) -> Result<ApiKey, AdminError> {
        self.post(
            &format!("/organizations/api_keys/{}", api_key_id),
            Some(params),
        )
        .await
    }

    /// Lists organization users
    async fn list_users<'a>(
        &'a self,
        params: Option<&'a ListUsersParams>,
    ) -> Result<ListUsersResponse, AdminError> {
        self.get("/organizations/users", params).await
    }

    /// Retrieves a user in the organization
    async fn get_user<'a>(&'a self, user_id: &'a str) -> Result<OrganizationUser, AdminError> {
        self.get(&format!("/organizations/users/{}", user_id), Option::<&()>::None)
            .await
    }

    async fn update_user<'a>(
        &'a self,
        user_id: &'a str,
        params: &'a AdminUpdateUserParams,
    ) -> Result<OrganizationUser, AdminError> {
        self.post(
            &format!("/organizations/users/{}", user_id),
            Some(params),
        )
        .await
    }

    async fn delete_user<'a>(&'a self, user_id: &'a str) -> Result<DeleteUserResponse, AdminError> {
        self.delete::<DeleteUserResponse, (), AdminError>(
            &format!("/organizations/users/{}", user_id),
            Option::<&()>::None,
        )
        .await
    }

    async fn list_workspaces<'a>(
        &'a self,
        params: Option<&'a ListWorkspacesParams>,
    ) -> Result<ListWorkspacesResponse, AdminError> {
        self.get("/organizations/workspaces", params).await
    }

    async fn get_workspace<'a>(
        &'a self,
        workspace_id: &'a str,
    ) -> Result<GetWorkspaceResponse, AdminError> {
        self.get(
            &format!("/organizations/workspaces/{}", workspace_id),
            Option::<&()>::None,
        )
        .await
    }

    async fn update_workspace<'a>(
        &'a self,
        workspace_id: &'a str,
        params: &'a crate::types::admin::workspaces::AdminUpdateWorkspaceParams,
    ) -> Result<crate::types::admin::workspaces::Workspace, AdminError> {
        self.post(
            &format!("/organizations/workspaces/{}", workspace_id),
            Some(params),
        )
        .await
    }

    async fn list_workspace_members<'a>(
        &'a self,
        workspace_id: &'a str,
        params: Option<&'a ListWorkspaceMembersParams>,
    ) -> Result<ListWorkspaceMembersResponse, AdminError> {
        self.get(
            &format!("/organizations/workspaces/{}/members", workspace_id),
            params,
        )
        .await
    }

    async fn get_workspace_member<'a>(
        &'a self,
        workspace_id: &'a str,
        user_id: &'a str,
    ) -> Result<GetWorkspaceMemberResponse, AdminError> {
        self.get(
            &format!("/organizations/workspaces/{}/members/{}", workspace_id, user_id),
            Option::<&()>::None,
        )
        .await
    }

    async fn add_workspace_member<'a>(
        &'a self,
        workspace_id: &'a str,
        params: &'a AdminAddWorkspaceMemberParams,
    ) -> Result<WorkspaceMember, AdminError> {
        self.post(
            &format!("/organizations/workspaces/{}/members", workspace_id),
            Some(params),
        )
        .await
    }

    async fn update_workspace_member<'a>(
        &'a self,
        workspace_id: &'a str,
        user_id: &'a str,
        params: &'a AdminUpdateWorkspaceMemberParams,
    ) -> Result<WorkspaceMember, AdminError> {
        self.post(
            &format!(
                "/organizations/workspaces/{}/members/{}",
                workspace_id, user_id
            ),
            Some(params),
        )
        .await
    }

    async fn delete_workspace_member<'a>(
        &'a self,
        workspace_id: &'a str,
        user_id: &'a str,
    ) -> Result<crate::types::admin::workspace_members::DeleteWorkspaceMemberResponse, AdminError> {
        self.delete::<crate::types::admin::workspace_members::DeleteWorkspaceMemberResponse, (), AdminError>(
            &format!("/organizations/workspaces/{}/members/{}", workspace_id, user_id),
            Option::<&()>::None,
        )
        .await
    }

    async fn list_invites<'a>(
        &'a self,
        params: Option<&'a ListInvitesParams>,
    ) -> Result<ListInvitesResponse, AdminError> {
        self.get("/organizations/invites", params).await
    }

    async fn create_invite<'a>(
        &'a self,
        params: &'a crate::types::admin::invites::CreateInviteParams,
    ) -> Result<crate::types::admin::invites::Invite, AdminError> {
        self.post("/organizations/invites", Some(params)).await
    }

    async fn get_invite<'a>(&'a self, invite_id: &'a str) -> Result<GetInviteResponse, AdminError> {
        self.get(&format!("/organizations/invites/{}", invite_id), Option::<&()>::None).await
    }

    async fn delete_invite<'a>(
        &'a self,
        invite_id: &'a str,
    ) -> Result<DeleteInviteResponse, AdminError> {
        self.delete::<DeleteInviteResponse, (), AdminError>(
            &format!("/organizations/invites/{}", invite_id),
            Option::<&()>::None,
        )
        .await
    }
}
