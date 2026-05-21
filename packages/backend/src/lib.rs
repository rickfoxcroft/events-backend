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
use adapters::storage::ImageStorageProvider;
use models::config::AppConfig;

struct AppState {
    storage: ImageStorageProvider,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let config = AppConfig::from_env(&env)?;
    let storage = ImageStorageProvider::from_config(&config);

    build_app(req, env, storage).await
}

pub async fn build_app(req: Request, env: Env, storage: ImageStorageProvider) -> Result<Response> {
    let state = AppState { storage };
    let router = Router::with_data(state);

    router
        .get("/", |_, _| Response::ok("Event Venue API"))
        .get_async("/venues", |_, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            let storage = ctx.data.storage.clone();
            handlers::venue::list_venues(repo, storage).await
        })
        .post_async("/venues", |req, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            let storage = ctx.data.storage.clone();
            handlers::venue::create_venue(req, repo, storage).await
        })
        .post_async("/images/upload-url", |req, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            let storage = ctx.data.storage.clone();
            handlers::image::get_upload_url(req, repo, storage).await
        })
        .run(req, env)
        .await
}
