use crate::ports::storage::ImageStorage;
use async_trait::async_trait;
use serde::Deserialize;
use worker::wasm_bindgen::JsValue;
use worker::*;

#[derive(Clone)]
pub struct CloudflareImagesConfig {
    pub account_id: String,
    pub api_token: String,
    pub account_hash: String,
    pub id_prefix: Option<String>,
}

impl CloudflareImagesConfig {
    pub fn new(
        account_id: &str,
        api_token: &str,
        account_hash: &str,
        id_prefix: Option<String>,
    ) -> Self {
        Self {
            account_id: account_id.to_string(),
            api_token: api_token.to_string(),
            account_hash: account_hash.to_string(),
            id_prefix,
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

#[async_trait(?Send)]
impl ImageStorage for CloudflareImagesConfig {
    async fn generate_upload_url(&self, image_id: &str) -> Result<String> {
        let url = format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/images/v2/direct_upload",
            self.account_id
        );

        let final_id = if let Some(prefix) = &self.id_prefix {
            format!("{}-{}", prefix, image_id)
        } else {
            image_id.to_string()
        };

        let headers = Headers::new();
        headers.set("Authorization", &format!("Bearer {}", self.api_token))?;

        // We use FormData to specify the custom ID
        let form = FormData::new();
        form.append("id", &final_id)?;

        let mut request_init = RequestInit::new();
        request_init.with_method(Method::Post);
        request_init.with_headers(headers);
        request_init.with_body(Some(JsValue::from(form)));

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
        let final_id = if let Some(prefix) = &self.id_prefix {
            format!("{}-{}", prefix, image_id)
        } else {
            image_id.to_string()
        };

        Ok(format!(
            "https://imagedelivery.net/{}/{}/public",
            self.account_hash, final_id
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_public_url() {
        let storage =
            CloudflareImagesConfig::new("acc-id", "token", "hash", Some("test".to_string()));
        let url = storage.get_public_url("venue-1", "img-1").await.unwrap();
        assert_eq!(url, "https://imagedelivery.net/hash/test-img-1/public");
    }
}
