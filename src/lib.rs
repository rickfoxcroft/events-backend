use worker::*;

pub mod adapters;
pub mod handlers;
pub mod models;
pub mod ports;
pub mod services;

use adapters::database::D1VenueRepository;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("Event Venue API"))
        .get_async("/venues", |_, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            handlers::venue::list_venues(repo).await
        })
        .post_async("/venues", |req, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            handlers::venue::create_venue(req, repo).await
        })
        .run(req, env)
        .await
}
