use crate::ports::storage::ImageStorage;
use crate::ports::VenueRepository;
use crate::services::VenueService;
use worker::*;

pub async fn list_venues<R: VenueRepository>(
    repo: R,
    storage: Box<dyn ImageStorage>,
) -> Result<Response> {
    let service = VenueService::new(repo, storage);
    let dtos = service.list_venues().await?;
    Response::from_json(&dtos)
}

pub async fn create_venue<R: VenueRepository>(
    mut req: Request,
    repo: R,
    storage: Box<dyn ImageStorage>,
) -> Result<Response> {
    let input = req.json().await?;
    let service = VenueService::new(repo, storage);
    service.create_venue(input).await?;
    Response::ok("Venue created")
}
