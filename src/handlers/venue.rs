use crate::adapters::storage::r2::R2ImageStorage;
use crate::ports::VenueRepository;
use crate::services::VenueService;
use worker::*;

pub struct R2Config {
    pub bucket_name: String,
    pub account_id: String,
    pub access_key: String,
    pub secret_key: String,
    pub public_domain: String,
}

pub async fn list_venues<R: VenueRepository>(repo: R, config: R2Config) -> Result<Response> {
    let storage = R2ImageStorage::new(
        &config.bucket_name,
        &config.account_id,
        &config.access_key,
        &config.secret_key,
        &config.public_domain,
    )?;
    let service = VenueService::new(repo, storage);
    let dtos = service.list_venues().await?;
    Response::from_json(&dtos)
}

pub async fn create_venue<R: VenueRepository>(
    mut req: Request,
    repo: R,
    config: R2Config,
) -> Result<Response> {
    let input = req.json().await?;
    let storage = R2ImageStorage::new(
        &config.bucket_name,
        &config.account_id,
        &config.access_key,
        &config.secret_key,
        &config.public_domain,
    )?;
    let service = VenueService::new(repo, storage);
    service.create_venue(input).await?;
    Response::ok("Venue created")
}

pub async fn get_upload_url<R: VenueRepository>(
    ctx: RouteContext<()>,
    repo: R,
    config: R2Config,
) -> Result<Response> {
    let id = ctx.param("id").ok_or_else(|| Error::from("Missing id"))?;
    let storage = R2ImageStorage::new(
        &config.bucket_name,
        &config.account_id,
        &config.access_key,
        &config.secret_key,
        &config.public_domain,
    )?;
    let service = VenueService::new(repo, storage);
    let result = service.get_upload_url(id).await?;
    Response::from_json(&result)
}

pub async fn complete_upload<R: VenueRepository>(
    mut req: Request,
    ctx: RouteContext<()>,
    repo: R,
    config: R2Config,
) -> Result<Response> {
    let id = ctx.param("id").ok_or_else(|| Error::from("Missing id"))?;
    let input = req.json().await?;
    let storage = R2ImageStorage::new(
        &config.bucket_name,
        &config.account_id,
        &config.access_key,
        &config.secret_key,
        &config.public_domain,
    )?;
    let service = VenueService::new(repo, storage);
    service.complete_upload(id, input).await?;
    Response::ok("Upload completed")
}
