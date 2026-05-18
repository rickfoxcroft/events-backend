use crate::ports::storage::ImageStorage;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use worker::Result;

#[derive(Debug, Clone)]
pub struct MockImageStorage {
    pub upload_urls: Arc<RwLock<HashMap<String, String>>>,
    pub public_url_prefix: String,
}

impl MockImageStorage {
    pub fn new() -> Self {
        Self {
            upload_urls: Arc::new(RwLock::new(HashMap::new())),
            public_url_prefix: "https://mock-storage.com".to_string(),
        }
    }
}

impl Default for MockImageStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl ImageStorage for MockImageStorage {
    async fn generate_upload_url(&self, image_id: &str) -> Result<String> {
        let url = format!("{}/upload/{}", self.public_url_prefix, image_id);
        let mut urls = self
            .upload_urls
            .write()
            .map_err(|_| worker::Error::from("Lock poisoned"))?;
        urls.insert(image_id.to_string(), url.clone());
        Ok(url)
    }

    async fn get_public_url(&self, _venue_id: &str, image_id: &str) -> Result<String> {
        Ok(format!("{}/public/{}", self.public_url_prefix, image_id))
    }
}
