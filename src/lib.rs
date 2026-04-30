use worker::*;

mod models;
use models::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("Event Venue API"))
        .get_async("/venues", |_, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let statement = d1.prepare("SELECT * FROM venues");
            let entities = statement.all().await?
                .results::<VenueEntity>()?;
            
            // Bridge: Convert Entities to DTOs
            let dtos: Vec<VenueDTO> = entities.into_iter().map(VenueDTO::from).collect();
            
            Response::from_json(&dtos)
        })
        .post_async("/venues", |mut req, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let input: VenueInputDTO = req.json().await?;
            
            // Logic to insert into D1 (omitted for brevity, would typically involve creating an entity)
            // Example:
            // let id = uuid::Uuid::new_v4().to_string();
            // d1.prepare("INSERT INTO venues (id, name, location, capacity, owner_id) VALUES (?, ?, ?, ?, ?)")
            //   .bind(&[id.into(), input.name.into(), input.location.into(), input.capacity.into(), "owner-1".into()])?
            //   .run().await?;

            Response::ok("Venue created")
        })
        .run(req, env)
        .await
}
