use crate::ports::auth::{ExternalUserInfo, IdentityProvider};
use async_trait::async_trait;
use serde::Deserialize;
use worker::wasm_bindgen::JsValue;
use worker::*;

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_USER_INFO_URL: &str = "https://openidconnect.googleapis.com/v1/userinfo";

#[derive(Clone)]
pub struct GoogleOAuthProvider {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl GoogleOAuthProvider {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
        }
    }
}

#[derive(Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct GoogleUserInfo {
    sub: String,
    email: String,
    name: String,
    picture: Option<String>,
}

#[async_trait(?Send)]
impl IdentityProvider for GoogleOAuthProvider {
    fn login_url(&self, state: &str) -> Result<String> {
        let url = format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope=openid%20email%20profile&state={}",
            GOOGLE_AUTH_URL, self.client_id, self.redirect_uri, state
        );
        Ok(url)
    }

    async fn exchange_code_for_user(&self, code: &str) -> Result<ExternalUserInfo> {
        // 1. Exchange code for token
        let headers = Headers::new();
        headers.set("Content-Type", "application/x-www-form-urlencoded")?;

        let body = format!(
            "code={}&client_id={}&client_secret={}&redirect_uri={}&grant_type=authorization_code",
            code, self.client_id, self.client_secret, self.redirect_uri
        );

        let mut request_init = RequestInit::new();
        request_init.with_method(Method::Post);
        request_init.with_headers(headers);
        request_init.with_body(Some(JsValue::from_str(&body)));

        let mut token_resp =
            Fetch::Request(Request::new_with_init(GOOGLE_TOKEN_URL, &request_init)?)
                .send()
                .await?;

        if token_resp.status_code() != 200 {
            return Err(Error::from(format!(
                "Failed to exchange code: {}",
                token_resp.text().await?
            )));
        }

        let token_data: GoogleTokenResponse = token_resp.json().await?;

        // 2. Fetch User Info
        let user_headers = Headers::new();
        user_headers.set(
            "Authorization",
            &format!("Bearer {}", token_data.access_token),
        )?;

        let mut user_request_init = RequestInit::new();
        user_request_init.with_headers(user_headers);

        let mut user_resp = Fetch::Request(Request::new_with_init(
            GOOGLE_USER_INFO_URL,
            &user_request_init,
        )?)
        .send()
        .await?;

        if user_resp.status_code() != 200 {
            return Err(Error::from(format!(
                "Failed to fetch user info: {}",
                user_resp.text().await?
            )));
        }

        let google_user: GoogleUserInfo = user_resp.json().await?;

        Ok(ExternalUserInfo {
            provider_id: format!("google|{}", google_user.sub),
            email: google_user.email,
            name: google_user.name,
            avatar_url: google_user.picture,
        })
    }
}
