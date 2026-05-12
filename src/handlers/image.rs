use crate::adapters::storage::cloudflare_images::CloudflareImagesConfig;
use crate::ports::VenueRepository;
use crate::services::VenueService;
use worker::*;

pub async fn get_upload_url<R: VenueRepository>(
    _req: Request,
    repo: R,
    config: CloudflareImagesConfig,
) -> Result<Response> {
    let service = VenueService::new(repo, config);
    let result = service.get_upload_url().await?;
    Response::from_json(&result)
}
