use async_trait::async_trait;
use worker::Result;

#[async_trait(?Send)]
pub trait ImageStorage: Send + Sync {
    async fn generate_upload_url(&self, image_id: &str) -> Result<String>;
    async fn get_public_url(&self, venue_id: &str, image_id: &str) -> Result<String>;
}
