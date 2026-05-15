use worker::*;

pub struct AppConfig {
    pub environment: String,
    pub cf_account_id: String,
    pub cf_images_api_token: String,
    pub cf_images_account_hash: String,
}

impl AppConfig {
    pub fn from_env(env: &Env) -> Result<Self> {
        let environment = env.var("ENVIRONMENT")?.to_string();

        let cf_images_api_token = if environment == "local" {
            env.secret("CF_IMAGES_API_TOKEN")
                .map(|s| s.to_string())
                .unwrap_or_else(|_| "local-token".to_string())
        } else {
            env.secret("CF_IMAGES_API_TOKEN")?.to_string()
        };

        Ok(Self {
            environment,
            cf_account_id: env.var("CF_ACCOUNT_ID")?.to_string(),
            cf_images_api_token,
            cf_images_account_hash: env.var("CF_IMAGES_ACCOUNT_HASH")?.to_string(),
        })
    }
}
