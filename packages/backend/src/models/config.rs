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

        Ok(Self {
            environment,
            cf_account_id: env.var("CF_ACCOUNT_ID")?.to_string(),
            cf_images_api_token: env.secret("CF_IMAGES_API_TOKEN")?.to_string(),
            cf_images_account_hash: env.var("CF_IMAGES_ACCOUNT_HASH")?.to_string(),
        })
    }
}
