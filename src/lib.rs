use worker::*;

pub mod adapters;
pub mod models;
pub mod ports;

use adapters::database::D1VenueRepository;
use models::*;
use ports::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("Event Venue API"))
        .get_async("/venues", |_, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            handle_list_venues(&repo).await
        })
        .post_async("/venues", |req, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let repo = D1VenueRepository::new(d1);
            handle_create_venue(req, &repo).await
        })
        .run(req, env)
        .await
}

async fn handle_list_venues<R: VenueRepository>(repo: &R) -> Result<Response> {
    let entities = repo.list_venues().await?;
    let dtos: Vec<VenueDTO> = entities.into_iter().map(VenueDTO::from).collect();
    Response::from_json(&dtos)
}

async fn handle_create_venue<R: VenueRepository>(mut req: Request, repo: &R) -> Result<Response> {
    let input: VenueInputDTO = req.json().await?;
    let entity = VenueEntity {
        id: "temp-id".to_string(),
        name: input.name,
        location: input.location,
        capacity: input.capacity,
        owner_id: "owner-1".to_string(),
    };
    repo.save_venue(entity).await?;
    Response::ok("Venue created")
}
