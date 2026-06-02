use worker::*;

#[derive(Clone)]
pub struct AppConfig {
    pub environment: String,
    pub cf_account_id: String,
    pub cf_images_api_token: String,
    pub cf_images_account_hash: String,
    pub jwt_secret: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
    pub frontend_url: String,
}

impl AppConfig {
    pub fn from_env(env: &Env) -> Result<Self> {
        let environment = env.var("ENVIRONMENT")?.to_string();

        Ok(Self {
            environment,
            cf_account_id: env.var("CF_ACCOUNT_ID")?.to_string(),
            cf_images_api_token: env.secret("CF_IMAGES_API_TOKEN")?.to_string(),
            cf_images_account_hash: env.var("CF_IMAGES_ACCOUNT_HASH")?.to_string(),
            jwt_secret: env.var("JWT_SECRET")?.to_string(),
            google_client_id: env.var("GOOGLE_CLIENT_ID")?.to_string(),
            google_client_secret: env.var("GOOGLE_CLIENT_SECRET")?.to_string(),
            google_redirect_uri: env.var("GOOGLE_REDIRECT_URI")?.to_string(),
            frontend_url: env.var("FRONTEND_URL")?.to_string(),
        })
    }
}
