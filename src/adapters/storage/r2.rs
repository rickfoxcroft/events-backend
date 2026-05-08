use crate::ports::storage::ImageStorage;
use aws_signer::aws_v4_signer::AwsV4Signer;
use worker::Result;

pub struct R2ImageStorage {
    bucket_name: String,
    account_id: String,
    access_key: String,
    secret_key: String,
    public_domain: String,
}

impl R2ImageStorage {
    pub fn new(
        bucket_name: &str,
        account_id: &str,
        access_key: &str,
        secret_key: &str,
        public_domain: &str,
    ) -> Result<Self> {
        Ok(Self {
            bucket_name: bucket_name.to_string(),
            account_id: account_id.to_string(),
            access_key: access_key.to_string(),
            secret_key: secret_key.to_string(),
            public_domain: public_domain.to_string(),
        })
    }
}

impl ImageStorage for R2ImageStorage {
    async fn generate_upload_url(&self, venue_id: &str, image_id: &str) -> Result<String> {
        let object_url = format!(
            "https://{}.{}.r2.cloudflarestorage.com/{}/{}.jpg",
            self.bucket_name, self.account_id, venue_id, image_id
        );

        let mut signer = AwsV4Signer::new(
            Some("PUT".to_string()),
            &object_url,
            None,
            None,
            self.access_key.clone(),
            self.secret_key.clone(),
            None,
            Some("s3".to_string()),
            Some("auto".to_string()),
            None,
            None,
            Some(true),
            None,
            None,
            None,
        ).map_err(|e| worker::Error::from(e.to_string()))?;

        let signed_request = signer.sign().await.map_err(|e| worker::Error::from(e.to_string()))?;
        
        Ok(signed_request.url().to_string())
    }

    async fn get_public_url(&self, venue_id: &str, image_id: &str) -> Result<String> {
        Ok(format!("{}/{}/{}.jpg", self.public_domain, venue_id, image_id))
    }
}
