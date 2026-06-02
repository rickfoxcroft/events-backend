use crate::models::entities::UserId;
use crate::ports::storage::ImageStorage;
use crate::ports::VenueRepository;
use crate::services::VenueService;
use worker::*;

pub async fn list_venues<R: VenueRepository, S: ImageStorage>(
    repo: R,
    storage: S,
) -> Result<Response> {
    let service = VenueService::new(repo, storage);
    let dtos = service.list_venues().await?;
    Response::from_json(&dtos)
}

pub async fn get_venue<R: VenueRepository, S: ImageStorage>(
    id: String,
    repo: R,
    storage: S,
) -> Result<Response> {
    let service = VenueService::new(repo, storage);
    match service.get_venue(id).await? {
        Some(dto) => Response::from_json(&dto),
        None => Response::error("Venue not found", 404),
    }
}

pub async fn create_venue<R: VenueRepository, S: ImageStorage>(
    mut req: Request,
    repo: R,
    storage: S,
    owner_id: UserId,
) -> Result<Response> {
    let input = req.json().await?;
    let service = VenueService::new(repo, storage);
    service.create_venue(input, owner_id).await?;
    Response::ok("Venue created").map(|r| r.with_status(201))
}
