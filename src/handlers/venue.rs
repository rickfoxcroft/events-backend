use crate::adapters::storage::cloudflare_images::CloudflareImagesConfig;
use crate::ports::VenueRepository;
use crate::services::VenueService;
use worker::*;

pub async fn list_venues<R: VenueRepository>(
    repo: R,
    config: CloudflareImagesConfig,
) -> Result<Response> {
    let service = VenueService::new(repo, config);
    let dtos = service.list_venues().await?;
    Response::from_json(&dtos)
}

pub async fn create_venue<R: VenueRepository>(
    mut req: Request,
    repo: R,
    config: CloudflareImagesConfig,
) -> Result<Response> {
    let input = req.json().await?;
    let service = VenueService::new(repo, config);
    service.create_venue(input).await?;
    Response::ok("Venue created")
}

pub async fn get_upload_url<R: VenueRepository>(
    ctx: RouteContext<()>,
    repo: R,
    config: CloudflareImagesConfig,
) -> Result<Response> {
    let id = ctx.param("id").ok_or_else(|| Error::from("Missing id"))?;
    let service = VenueService::new(repo, config);
    let result = service.get_upload_url(id).await?;
    Response::from_json(&result)
}

pub async fn complete_upload<R: VenueRepository>(
    mut req: Request,
    ctx: RouteContext<()>,
    repo: R,
    config: CloudflareImagesConfig,
) -> Result<Response> {
    let id = ctx.param("id").ok_or_else(|| Error::from("Missing id"))?;
    let input = req.json().await?;
    let service = VenueService::new(repo, config);
    service.complete_upload(id, input).await?;
    Response::ok("Upload completed")
}
