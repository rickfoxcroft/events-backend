pub mod google;
pub mod jwt;
pub mod mock;

use crate::models::config::AppConfig;
use crate::ports::auth::{ExternalUserInfo, IdentityProvider, TokenProvider};
use async_trait::async_trait;
use google::GoogleOAuthProvider;
use jwt::JwtProvider;
use mock::{MockIdentityProvider, MockTokenProvider};
use worker::Result;

#[derive(Clone)]
pub enum IdentityProviderProvider {
    Google(GoogleOAuthProvider),
    Mock(MockIdentityProvider),
}

impl IdentityProviderProvider {
    pub fn from_config(config: &AppConfig) -> Self {
        if config.google_client_id == "local-client-id"
            || config.google_client_secret == "local-client-secret"
        {
            Self::Mock(MockIdentityProvider::new())
        } else {
            Self::Google(GoogleOAuthProvider::new(
                config.google_client_id.clone(),
                config.google_client_secret.clone(),
                config.google_redirect_uri.clone(),
            ))
        }
    }
}

#[async_trait(?Send)]
impl IdentityProvider for IdentityProviderProvider {
    fn login_url(&self, state: &str) -> Result<String> {
        match self {
            Self::Google(p) => p.login_url(state),
            Self::Mock(p) => p.login_url(state),
        }
    }

    async fn exchange_code_for_user(&self, code: &str) -> Result<ExternalUserInfo> {
        match self {
            Self::Google(p) => p.exchange_code_for_user(code).await,
            Self::Mock(p) => p.exchange_code_for_user(code).await,
        }
    }
}

#[derive(Clone)]
pub enum TokenProviderProvider {
    Jwt(JwtProvider),
    Mock(MockTokenProvider),
}

impl TokenProviderProvider {
    pub fn from_config(config: &AppConfig) -> Self {
        if config.jwt_secret == "local-jwt-secret" {
            Self::Mock(MockTokenProvider::new())
        } else {
            Self::Jwt(JwtProvider::new(config.jwt_secret.clone()))
        }
    }
}

impl TokenProvider for TokenProviderProvider {
    fn generate_token(&self, user_id: &crate::models::entities::UserId) -> Result<String> {
        match self {
            Self::Jwt(p) => p.generate_token(user_id),
            Self::Mock(p) => p.generate_token(user_id),
        }
    }

    fn verify_token(&self, token: &str) -> Result<crate::models::entities::UserId> {
        match self {
            Self::Jwt(p) => p.verify_token(token),
            Self::Mock(p) => p.verify_token(token),
        }
    }
}
