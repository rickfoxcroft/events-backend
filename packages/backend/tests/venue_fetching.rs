use cucumber::{given, then, when, StatsWriter, World};
use event_app_backend::models::{ImageUploadURLResponseDTO, VenueDTO, VenueInputDTO};
use std::env;

#[derive(Debug, World)]
pub struct FetchVenueWorld {
    client: reqwest::Client,
    base_url: String,
    fetched_venues: Vec<VenueDTO>,
}

impl Default for FetchVenueWorld {
    fn default() -> Self {
        let base_url =
            env::var("TEST_API_URL").unwrap_or_else(|_| "http://127.0.0.1:8787".to_string());
        Self {
            client: reqwest::Client::new(),
            base_url,
            fetched_venues: Vec::new(),
        }
    }
}

#[given(expr = "there are no venues listed on the platform")]
async fn there_are_no_venues(_world: &mut FetchVenueWorld) {
    // This is now handled by the before hook, but we keep the step for Gherkin compatibility
}

#[given(expr = "the following venues exist:")]
async fn the_following_venues_exist(world: &mut FetchVenueWorld, step: &cucumber::gherkin::Step) {
    let table = step.table().expect("Step must have a table");

    for row in table.rows.iter().skip(1) {
        let name = &row[0];
        let location = &row[1];
        let capacity: i32 = row[2].parse().unwrap();

        let input = VenueInputDTO {
            name: name.clone(),
            location: location.clone(),
            capacity,
            price_per_hour: 100, // Default for tests
            image_ids: Vec::new(),
        };

        let url = format!("{}/venues", world.base_url);
        world.client.post(&url).json(&input).send().await.unwrap();
    }
}

#[given(expr = "the following venues exist with images:")]
async fn the_following_venues_exist_with_images(
    world: &mut FetchVenueWorld,
    step: &cucumber::gherkin::Step,
) {
    let table = step.table().unwrap();

    for row in table.rows.iter().skip(1) {
        let name = &row[0];
        let location = &row[1];
        let capacity: i32 = row[2].parse().unwrap();
        let images_str = &row[3];

        let mut image_ids = Vec::new();
        for _ in images_str.split(',') {
            let url = format!("{}/images/upload-url", world.base_url);
            let resp = world.client.post(&url).send().await.unwrap();

            let upload_resp: ImageUploadURLResponseDTO = resp.json().await.unwrap();
            image_ids.push(upload_resp.image_id);
        }

        let input = VenueInputDTO {
            name: name.clone(),
            location: location.clone(),
            capacity,
            price_per_hour: 100, // Default for tests
            image_ids,
        };

        let url = format!("{}/venues", world.base_url);
        world.client.post(&url).json(&input).send().await.unwrap();
    }
}

#[when(expr = "I request the list of all venues")]
async fn i_request_all_venues(world: &mut FetchVenueWorld) {
    let url = format!("{}/venues", world.base_url);
    let resp = world.client.get(&url).send().await.unwrap();

    world.fetched_venues = resp.json().await.unwrap();
}

#[then(expr = "I should receive an empty list of venues")]
async fn i_should_receive_empty_list(world: &mut FetchVenueWorld) {
    assert!(world.fetched_venues.is_empty());
}

#[then(expr = "I should see {int} venues in the list")]
async fn i_should_see_n_venues(world: &mut FetchVenueWorld, count: usize) {
    assert_eq!(world.fetched_venues.len(), count);
}

#[then(expr = "I should see {string} in the list of venues")]
async fn i_should_see_venue_in_list(world: &mut FetchVenueWorld, name: String) {
    let exists = world.fetched_venues.iter().any(|v| v.name == name);
    assert!(exists, "Venue '{}' not found in list", name);
}

#[then(expr = "{string} should display its images")]
async fn venue_should_display_images(world: &mut FetchVenueWorld, name: String) {
    let venue = world
        .fetched_venues
        .iter()
        .find(|v| v.name == name)
        .unwrap();
    assert!(!venue.images.is_empty(), "Venue '{}' has no images", name);
}

#[tokio::test]
async fn test_venue_fetching() {
    let stats = FetchVenueWorld::cucumber()
        .max_concurrent_scenarios(1)
        .before(|_, _, _, _| {
            Box::pin(async move {
                // Ensure tables exist and then clear them
                let _ = std::process::Command::new("wrangler")
                    .args([
                        "d1",
                        "execute",
                        "event-app-db",
                        "--local",
                        "--command",
                        "CREATE TABLE IF NOT EXISTS venues (id TEXT PRIMARY KEY, name TEXT NOT NULL, location TEXT NOT NULL, capacity INTEGER NOT NULL, price_per_hour INTEGER NOT NULL DEFAULT 0, owner_id TEXT NOT NULL); CREATE TABLE IF NOT EXISTS venue_images (id TEXT PRIMARY KEY, venue_id TEXT NOT NULL, url TEXT NOT NULL, FOREIGN KEY (venue_id) REFERENCES venues(id) ON DELETE CASCADE); DELETE FROM venue_images; DELETE FROM venues;",
                        "--yes",
                    ])
                    .status();
            })
        })
        .run("features/fetch_venues.feature")
        .await;

    if stats.failed_steps() > 0 {
        panic!("Cucumber tests failed");
    }
}
