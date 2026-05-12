use crate::ports::storage::ImageStorage;
use serde::Deserialize;
use worker::*;

#[derive(Clone)]
pub struct CloudflareImagesConfig {
    pub account_id: String,
    pub api_token: String,
    pub account_hash: String,
}

impl CloudflareImagesConfig {
    pub fn new(account_id: &str, api_token: &str, account_hash: &str) -> Self {
        Self {
            account_id: account_id.to_string(),
            api_token: api_token.to_string(),
            account_hash: account_hash.to_string(),
        }
    }
}

#[derive(Deserialize)]
struct CFDirectUploadResponse {
    result: CFDirectUploadResult,
    success: bool,
}

#[derive(Deserialize)]
struct CFDirectUploadResult {
    #[allow(dead_code)]
    id: String,
    #[serde(rename = "uploadURL")]
    upload_url: String,
}

impl ImageStorage for CloudflareImagesConfig {
    async fn generate_upload_url(&self, _image_id: &str) -> Result<String> {
        let url = format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/images/v2/direct_upload",
            self.account_id
        );

        let headers = Headers::new();
        headers.set("Authorization", &format!("Bearer {}", self.api_token))?;

        let mut request_init = RequestInit::new();
        request_init.with_method(Method::Post);
        request_init.with_headers(headers);

        let mut response = Fetch::Request(Request::new_with_init(&url, &request_init)?)
            .send()
            .await?;

        if response.status_code() != 200 {
            return Err(Error::from(format!(
                "Failed to get upload URL from Cloudflare: {}",
                response.text().await?
            )));
        }

        let data: CFDirectUploadResponse = response.json().await?;

        if !data.success {
            return Err(Error::from("Cloudflare API returned success: false"));
        }

        Ok(data.result.upload_url)
    }

    async fn get_public_url(&self, _venue_id: &str, image_id: &str) -> Result<String> {
        Ok(format!(
            "https://imagedelivery.net/{}/{}/public",
            self.account_hash, image_id
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_public_url() {
        let storage = CloudflareImagesConfig::new("acc-id", "token", "hash");
        let url = storage.get_public_url("venue-1", "img-1").await.unwrap();
        assert_eq!(url, "https://imagedelivery.net/hash/img-1/public");
    }
}
