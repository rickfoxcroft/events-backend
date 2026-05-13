use cucumber::{given, then, when, World};
use event_app_backend::models::{ImageUploadURLResponseDTO, VenueDTO, VenueInputDTO};
use std::env;

#[derive(Debug, World)]
pub struct VenueWorld {
    client: reqwest::Client,
    base_url: String,
    owner_id: Option<String>,
    uploaded_image_ids: Vec<String>,
    last_venue_id: Option<String>,
    last_response_status: Option<u16>,
}

impl Default for VenueWorld {
    fn default() -> Self {
        let base_url = env::var("TEST_API_URL").unwrap_or_else(|_| "http://127.0.0.1:8787".to_string());
        Self {
            client: reqwest::Client::new(),
            base_url,
            owner_id: None,
            uploaded_image_ids: Vec::new(),
            last_venue_id: None,
            last_response_status: None,
        }
    }
}

#[given(expr = "I am a registered venue owner")]
async fn i_am_a_registered_owner(world: &mut VenueWorld) {
    world.owner_id = Some("owner-1".to_string());
}

#[when(expr = "I upload the following images:")]
async fn i_upload_images(world: &mut VenueWorld, step: &cucumber::gherkin::Step) {
    let table = step.table().expect("Step must have a table");

    for _ in table.rows.iter().skip(1) {
        let url = format!("{}/images/upload-url", world.base_url);
        let resp = world
            .client
            .post(&url)
            .send()
            .await
            .expect("Failed to get upload url");
        
        assert_eq!(resp.status(), 200);
        let upload_resp: ImageUploadURLResponseDTO = resp.json().await.expect("Failed to parse upload url response");
        world.uploaded_image_ids.push(upload_resp.image_id);
    }
}

#[when(expr = "I submit the following details for my new venue:")]
async fn i_submit_venue_details(world: &mut VenueWorld, step: &cucumber::gherkin::Step) {
    let table = step.table().expect("Step must have a table");

    for row in table.rows.iter().skip(1) {
        let name = &row[0];
        let location = &row[1];
        let capacity: i32 = row[2].parse().expect("Capacity must be a number");

        let input = VenueInputDTO {
            name: name.clone(),
            location: location.clone(),
            capacity,
            image_ids: world.uploaded_image_ids.drain(..).collect(),
        };

        let url = format!("{}/venues", world.base_url);
        let resp = world
            .client
            .post(&url)
            .json(&input)
            .send()
            .await
            .expect("Failed to create venue");

        world.last_response_status = Some(resp.status().as_u16());
        
        // Since we don't return the ID in the response body yet (just 201), 
        // we might need to find it in the list if we want to track it, 
        // but for these tests, name is used as the identifier in subsequent steps.
    }
}

#[then(expr = "my venue should be successfully listed")]
async fn venue_should_be_listed(world: &mut VenueWorld) {
    assert_eq!(world.last_response_status, Some(201));
}

#[then(expr = "I should see {string} in my list of venues")]
async fn i_should_see_venue_in_list(world: &mut VenueWorld, name: String) {
    let url = format!("{}/venues", world.base_url);
    let resp = world
        .client
        .get(&url)
        .send()
        .await
        .expect("Failed to list venues");
    
    assert_eq!(resp.status(), 200);
    let venues: Vec<VenueDTO> = resp.json().await.expect("Failed to parse venues list");
    let exists = venues.iter().any(|v| v.name == name);
    assert!(exists, "Venue '{}' not found in list", name);
}

#[then(expr = "the venue {string} should have {int} images attached")]
async fn venue_should_have_images(world: &mut VenueWorld, name: String, count: usize) {
    let url = format!("{}/venues", world.base_url);
    let resp = world
        .client
        .get(&url)
        .send()
        .await
        .expect("Failed to list venues");
    
    assert_eq!(resp.status(), 200);
    let venues: Vec<VenueDTO> = resp.json().await.expect("Failed to parse venues list");

    let venue = venues
        .iter()
        .find(|v| v.name == name)
        .expect("Venue not found");
    assert_eq!(
        venue.images.len(),
        count,
        "Expected {} images, found {}",
        count,
        venue.images.len()
    );
}

#[tokio::test]
async fn test_venue_listing() {
    VenueWorld::cucumber()
        .max_concurrent_scenarios(1)
        .before(|_, _, _, _| {
            Box::pin(async move {
                let _ = std::process::Command::new("wrangler")
                    .args(&[
                        "d1",
                        "execute",
                        "event-app-db",
                        "--local",
                        "--command",
                        "DELETE FROM venue_images; DELETE FROM venues;",
                        "--yes",
                    ])
                    .status();
            })
        })
        .run("features/list_venue.feature")
        .await;
}
