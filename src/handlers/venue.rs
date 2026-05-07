use crate::ports::VenueRepository;
use crate::services::VenueService;
use worker::*;

pub async fn list_venues<R: VenueRepository>(repo: R) -> Result<Response> {
    let service = VenueService::new(repo);
    let dtos = service.list_venues().await?;
    Response::from_json(&dtos)
}

pub async fn create_venue<R: VenueRepository>(mut req: Request, repo: R) -> Result<Response> {
    let input = req.json().await?;
    let service = VenueService::new(repo);
    service.create_venue(input).await?;
    Response::ok("Venue created")
}
