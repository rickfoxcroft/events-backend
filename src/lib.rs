use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("Event Venue API"))
        .get_async("/venues", |_, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let statement = d1.prepare("SELECT * FROM venues");
            let result = statement.all().await?;
            Response::from_json(&result.results::<serde_json::Value>()?)
        })
        .post_async("/venues", |mut req, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let body: serde_json::Value = req.json().await?;
            // Logic to insert into D1
            Response::ok("Venue created")
        })
        .run(req, env)
        .await
}
