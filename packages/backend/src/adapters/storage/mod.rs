pub mod cloudflare_images;
pub mod mock;

use crate::models::config::AppConfig;
use crate::ports::storage::ImageStorage;
use async_trait::async_trait;
use cloudflare_images::CloudflareImagesConfig;
use mock::MockImageStorage;
use worker::Result;

#[derive(Clone)]
pub enum ImageStorageProvider {
    Cloudflare(CloudflareImagesConfig),
    Mock(MockImageStorage),
}

impl ImageStorageProvider {
    pub fn from_config(config: &AppConfig) -> Self {
        // In local development, we use Mock storage if real credentials are missing.
        // In CI or production-like local testing, real credentials should be used.
        if config.cf_account_id == "local-account-id" || config.cf_images_api_token == "local-token"
        {
            Self::Mock(MockImageStorage::new())
        } else {
            let prefix = if config.environment == "local" {
                Some("dev".to_string())
            } else {
                None
            };

            Self::Cloudflare(CloudflareImagesConfig::new(
                &config.cf_account_id,
                &config.cf_images_api_token,
                &config.cf_images_account_hash,
                prefix,
            ))
        }
    }
}

#[async_trait(?Send)]
impl ImageStorage for ImageStorageProvider {
    async fn generate_upload_url(&self, image_id: &str) -> Result<String> {
        match self {
            Self::Cloudflare(s) => s.generate_upload_url(image_id).await,
            Self::Mock(s) => s.generate_upload_url(image_id).await,
        }
    }

    async fn get_public_url(&self, venue_id: &str, image_id: &str) -> Result<String> {
        match self {
            Self::Cloudflare(s) => s.get_public_url(venue_id, image_id).await,
            Self::Mock(s) => s.get_public_url(venue_id, image_id).await,
        }
    }
}
