use crate::models::entities::UserId;
use async_trait::async_trait;
use worker::Result;

#[derive(Clone)]
pub struct ExternalUserInfo {
    pub provider_id: String,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
}

#[async_trait(?Send)]
pub trait IdentityProvider {
    fn login_url(&self, state: &str) -> Result<String>;
    async fn exchange_code_for_user(&self, code: &str) -> Result<ExternalUserInfo>;
}

pub trait TokenProvider {
    fn generate_token(&self, user_id: &UserId) -> Result<String>;
    fn verify_token(&self, token: &str) -> Result<UserId>;
}
