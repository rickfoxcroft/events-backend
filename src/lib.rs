use worker::*;

/*
TODO: Implement a custom error catalog using the `thiserror` crate.
This will allow for mapping internal errors (database, storage, etc.)
to meaningful API errors (404 Not Found, 400 Bad Request, etc.)
instead of returning generic 500 Internal Server Errors via `worker::Error`.
*/

pub mod adapters;
pub mod handlers;
pub mod models;
pub mod ports;
pub mod services;
use adapters::database::D1VenueRepository;
use adapters::storage::cloudflare_images::CloudflareImagesConfig;
use adapters::storage::local::LocalImageStorage;
use ports::storage::ImageStorage;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("Event Venue API"))
        .get_async("/venues", |_, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            let storage = get_image_storage(&ctx.env)?;
            handlers::venue::list_venues(repo, storage).await
        })
        .post_async("/venues", |req, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            let storage = get_image_storage(&ctx.env)?;
            handlers::venue::create_venue(req, repo, storage).await
        })
        .post_async("/images/upload-url", |req, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            let storage = get_image_storage(&ctx.env)?;
            handlers::image::get_upload_url(req, repo, storage).await
        })
        .run(req, env)
        .await
}

fn get_image_storage(env: &Env) -> Result<Box<dyn ImageStorage>> {
    if env
        .var("ENVIRONMENT")
        .map(|v| v.to_string())
        .unwrap_or_default()
        == "local"
    {
        return Ok(Box::new(LocalImageStorage::new("http://localhost:8787")));
    }

    let config = CloudflareImagesConfig::new(
        &env.var("CLOUDFLARE_ACCOUNT_ID")?.to_string(),
        &env.secret("CLOUDFLARE_API_TOKEN")?.to_string(),
        &env.var("CLOUDFLARE_ACCOUNT_HASH")?.to_string(),
    );
    Ok(Box::new(config))
}
