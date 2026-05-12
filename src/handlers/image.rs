use crate::ports::storage::ImageStorage;
use crate::ports::VenueRepository;
use crate::services::VenueService;
use worker::*;

pub async fn get_upload_url<R: VenueRepository>(
    _req: Request,
    repo: R,
    storage: Box<dyn ImageStorage>,
) -> Result<Response> {
    let service = VenueService::new(repo, storage);
    let result = service.get_upload_url().await?;
    Response::from_json(&result)
}
