#![deny(unused_qualifications)]

use adapters::auth::{IdentityProviderProvider, TokenProviderProvider};
use adapters::database::D1VenueRepository;
use adapters::storage::ImageStorageProvider;
use models::config::AppConfig;
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

struct AppState {
    config: AppConfig,
    storage: ImageStorageProvider,
    identity_provider: IdentityProviderProvider,
    token_provider: TokenProviderProvider,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let config = AppConfig::from_env(&env)?;
    let storage = ImageStorageProvider::from_config(&config);
    let identity_provider = IdentityProviderProvider::from_config(&config);
    let token_provider = TokenProviderProvider::from_config(&config);

    let state = AppState {
        config,
        storage,
        identity_provider,
        token_provider,
    };

    build_app(req, env, state).await
}

async fn build_app(req: Request, env: Env, state: AppState) -> Result<Response> {
    let router = Router::with_data(state);

    router
        .get("/", |_, _| Response::ok("Event Venue API"))
        .get("/auth/login", |_, ctx| {
            handlers::auth::login(&ctx.data.identity_provider)
        })
        .get_async("/auth/callback", |req, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            handlers::auth::callback(
                req,
                &ctx.data.identity_provider,
                &ctx.data.token_provider,
                &ctx.data.config,
                repo,
            )
            .await
        })
        .get("/auth/logout", |_, ctx| {
            handlers::auth::logout(&ctx.data.config)
        })
        .get_async("/venues", |_, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            let storage = ctx.data.storage.clone();
            handlers::venue::list_venues(repo, storage).await
        })
        .get_async("/venues/:id", |_, ctx| async move {
            let id = ctx.param("id").cloned().unwrap_or_default();
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            let storage = ctx.data.storage.clone();
            handlers::venue::get_venue(id, repo, storage).await
        })
        .post_async("/venues", |req, ctx| async move {
            let auth = handlers::auth::get_authenticated_user(&req, &ctx.data.token_provider)?;
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            let storage = ctx.data.storage.clone();
            handlers::venue::create_venue(req, repo, storage, auth.user_id).await
        })
        .post_async("/images/upload-url", |req, ctx| async move {
            let _auth = handlers::auth::get_authenticated_user(&req, &ctx.data.token_provider)?;
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            let storage = ctx.data.storage.clone();
            handlers::image::get_upload_url(req, repo, storage).await
        })
        .run(req, env)
        .await
}
