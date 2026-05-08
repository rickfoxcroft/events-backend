use worker::*;

pub mod adapters;
pub mod handlers;
pub mod models;
pub mod ports;
pub mod services;
use adapters::database::D1VenueRepository;
use handlers::venue::R2Config;
use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("Event Venue API"))
        .get_async("/venues", |_, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            let config = get_r2_config(&ctx.env)?;
            handlers::venue::list_venues(repo, config).await
        })
        .post_async("/venues", |req, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            let config = get_r2_config(&ctx.env)?;
            handlers::venue::create_venue(req, repo, config).await
        })
        .post_async("/venues/:id/images/upload-url", |_, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            let config = get_r2_config(&ctx.env)?;
            handlers::venue::get_upload_url(ctx, repo, config).await
        })
        .post_async("/venues/:id/images/complete", |req, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            let config = get_r2_config(&ctx.env)?;
            handlers::venue::complete_upload(req, ctx, repo, config).await
        })
        .run(req, env)
        .await
}

fn get_r2_config(env: &Env) -> Result<R2Config> {
    Ok(R2Config {
        bucket_name: env.var("R2_BUCKET_NAME")?.to_string(),
        account_id: env.var("R2_ACCOUNT_ID")?.to_string(),
        access_key: env.secret("R2_ACCESS_KEY_ID")?.to_string(),
        secret_key: env.secret("R2_SECRET_ACCESS_KEY")?.to_string(),
        public_domain: env.var("R2_PUBLIC_DOMAIN")?.to_string(),
    })
}

