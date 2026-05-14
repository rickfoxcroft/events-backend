pub mod cloudflare_images;
pub mod local;
pub mod mock;

use crate::ports::storage::ImageStorage;
use async_trait::async_trait;
use cloudflare_images::CloudflareImagesConfig;
use local::LocalImageStorage;
use worker::Result;

#[derive(Clone)]
pub enum StorageProvider {
    Local(LocalImageStorage),
    Cloudflare(CloudflareImagesConfig),
}

#[async_trait(?Send)]
impl ImageStorage for StorageProvider {
    async fn generate_upload_url(&self, image_id: &str) -> Result<String> {
        match self {
            Self::Local(s) => s.generate_upload_url(image_id).await,
            Self::Cloudflare(s) => s.generate_upload_url(image_id).await,
        }
    }

    async fn get_public_url(&self, venue_id: &str, image_id: &str) -> Result<String> {
        match self {
            Self::Local(s) => s.get_public_url(venue_id, image_id).await,
            Self::Cloudflare(s) => s.get_public_url(venue_id, image_id).await,
        }
    }
}
