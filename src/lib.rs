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

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("Event Venue API"))
        .get_async("/venues", |_, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            let config = get_cloudflare_images_config(&ctx.env)?;
            handlers::venue::list_venues(repo, config).await
        })
        .post_async("/venues", |req, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            let config = get_cloudflare_images_config(&ctx.env)?;
            handlers::venue::create_venue(req, repo, config).await
        })
        .post_async("/venues/:id/images/upload-url", |_, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            let config = get_cloudflare_images_config(&ctx.env)?;
            handlers::venue::get_upload_url(ctx, repo, config).await
        })
        .post_async("/venues/:id/images/complete", |req, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            let config = get_cloudflare_images_config(&ctx.env)?;
            handlers::venue::complete_upload(req, ctx, repo, config).await
        })
        .run(req, env)
        .await
}

fn get_cloudflare_images_config(env: &Env) -> Result<CloudflareImagesConfig> {
    Ok(CloudflareImagesConfig::new(
        &env.var("CLOUDFLARE_ACCOUNT_ID")?.to_string(),
        &env.secret("CLOUDFLARE_API_TOKEN")?.to_string(),
        &env.var("CLOUDFLARE_ACCOUNT_HASH")?.to_string(),
    ))
}
