use crate::models::entities::UserId;
use crate::ports::auth::{ExternalUserInfo, IdentityProvider, TokenProvider};
use async_trait::async_trait;
use worker::Result;

#[derive(Clone)]
pub struct MockIdentityProvider {
    pub default_user: ExternalUserInfo,
}

impl MockIdentityProvider {
    pub fn new() -> Self {
        Self {
            default_user: ExternalUserInfo {
                provider_id: "google|mock-user-123".to_string(),
                email: "mock@example.com".to_string(),
                name: "Mock User".to_string(),
                avatar_url: Some("http://example.com/avatar.png".to_string()),
            },
        }
    }
}

impl Default for MockIdentityProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl IdentityProvider for MockIdentityProvider {
    fn login_url(&self, state: &str) -> Result<String> {
        Ok(format!("http://localhost:8787/auth/login?state={}", state))
    }

    async fn exchange_code_for_user(&self, _code: &str) -> Result<ExternalUserInfo> {
        Ok(ExternalUserInfo {
            provider_id: self.default_user.provider_id.clone(),
            email: self.default_user.email.clone(),
            name: self.default_user.name.clone(),
            avatar_url: self.default_user.avatar_url.clone(),
        })
    }
}

#[derive(Clone)]
pub struct MockTokenProvider {
    pub mock_token: String,
}

impl MockTokenProvider {
    pub fn new() -> Self {
        Self {
            mock_token: "mock-jwt-token-xyz".to_string(),
        }
    }
}

impl Default for MockTokenProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenProvider for MockTokenProvider {
    fn generate_token(&self, _user_id: &UserId) -> Result<String> {
        Ok(self.mock_token.clone())
    }

    fn verify_token(&self, token: &str) -> Result<UserId> {
        if token == self.mock_token {
            Ok(UserId("mock-user-id".to_string()))
        } else {
            Err(worker::Error::from("Invalid mock token"))
        }
    }
}
