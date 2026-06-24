use crate::models::config::AppConfig;
use crate::models::entities::{UserEntity, UserId};
use crate::ports::auth::{IdentityProvider, TokenProvider};
use crate::ports::UserRepository;
use uuid::Uuid;
use worker::*;

pub struct AuthenticatedUser {
    pub user_id: UserId,
}

pub fn get_authenticated_user(
    req: &Request,
    token_provider: &impl TokenProvider,
) -> Result<AuthenticatedUser> {
    let auth_header = req
        .headers()
        .get("Authorization")?
        .ok_or_else(|| Error::from("Missing Authorization header"))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(Error::from("Invalid Authorization header format"));
    }

    let token = &auth_header[7..];
    let user_id = token_provider.verify_token(token)?;

    Ok(AuthenticatedUser { user_id })
}

pub fn login(identity_provider: &impl IdentityProvider) -> Result<Response> {
    let state = Uuid::new_v4().to_string();
    let url = identity_provider.login_url(&state)?;

    let mut response = Response::redirect(Url::parse(&url)?)?;

    // Store state in a cookie for validation in callback
    let cookie = format!(
        "oauth_state={}; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age=600",
        state
    );
    response.headers_mut().append("Set-Cookie", &cookie)?;

    Ok(response)
}

pub async fn callback<R: UserRepository>(
    req: Request,
    identity_provider: &impl IdentityProvider,
    token_provider: &impl TokenProvider,
    config: &AppConfig,
    repo: R,
) -> Result<Response> {
    let url = req.url()?;
    let query: std::collections::HashMap<String, String> = url.query_pairs().into_owned().collect();

    let code = query
        .get("code")
        .ok_or_else(|| Error::from("Missing code"))?;
    let state = query
        .get("state")
        .ok_or_else(|| Error::from("Missing state"))?;

    // Validate state from cookie
    let cookie_header = req.headers().get("Cookie")?.unwrap_or_default();
    if !cookie_header.contains(&format!("oauth_state={}", state)) {
        return Response::error("Invalid OAuth state", 400);
    }

    // 1. Exchange code for user details
    let ext_user = identity_provider.exchange_code_for_user(code).await?;

    // 2. Sync User
    let provider_id = ext_user.provider_id;
    let existing_user = repo.get_user_by_provider_id(&provider_id).await?;

    let user_id = if let Some(user) = existing_user {
        user.id
    } else {
        let new_id = UserId::new_v7();
        let new_user = UserEntity {
            id: new_id.clone(),
            provider_id,
            email: ext_user.email,
            name: ext_user.name,
            avatar_url: ext_user.avatar_url,
        };
        repo.save_user(new_user).await?;
        new_id
    };

    // 3. Issue Token
    let token = token_provider.generate_token(&user_id)?;

    // 4. Set Cookie and Redirect
    let mut response = Response::redirect(Url::parse(&config.frontend_url)?)?;
    let auth_cookie = format!(
        "auth_token={}; Path=/; Secure; SameSite=Lax; Max-Age=7200",
        token
    );
    response.headers_mut().append("Set-Cookie", &auth_cookie)?;

    // Clear state cookie
    response
        .headers_mut()
        .append("Set-Cookie", "oauth_state=; Path=/; Max-Age=0")?;

    Ok(response)
}

pub fn logout(config: &AppConfig) -> Result<Response> {
    let mut response = Response::redirect(Url::parse(&config.frontend_url)?)?;
    response
        .headers_mut()
        .append("Set-Cookie", "auth_token=; Path=/; Max-Age=0")?;
    Ok(response)
}
