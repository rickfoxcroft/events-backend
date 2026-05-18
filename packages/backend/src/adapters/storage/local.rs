use crate::ports::storage::ImageStorage;
use async_trait::async_trait;
use worker::Result;

/// A local storage implementation for development and testing.
/// Instead of hitting Cloudflare, it returns a local upload URL that points back to the worker.
#[derive(Clone, Debug)]
pub struct LocalImageStorage {
    pub base_url: String,
}

impl LocalImageStorage {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }
}

#[async_trait(?Send)]
impl ImageStorage for LocalImageStorage {
    async fn generate_upload_url(&self, image_id: &str) -> Result<String> {
        // In a real local setup, we might have a handler at /local-storage/upload/:id
        // For now, we return a dummy URL that looks like a valid upload destination
        Ok(format!(
            "{}/local-storage/upload/{}",
            self.base_url, image_id
        ))
    }

    async fn get_public_url(&self, _venue_id: &str, image_id: &str) -> Result<String> {
        // Returns a local path that doesn't require a real Cloudflare account hash
        Ok(format!(
            "{}/local-storage/public/{}",
            self.base_url, image_id
        ))
    }
}
