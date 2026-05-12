#![allow(async_fn_in_trait)]
use worker::Result;

pub trait ImageStorage {
    async fn generate_upload_url(&self, image_id: &str) -> Result<String>;
    async fn get_public_url(&self, venue_id: &str, image_id: &str) -> Result<String>;
}
