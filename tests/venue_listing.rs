use cucumber::{given, then, when, World};
use event_app_backend::adapters::database::MockVenueRepository;
use event_app_backend::adapters::storage::mock::MockImageStorage;
use event_app_backend::models::{ImageUploadCompleteDTO, VenueInputDTO};
use event_app_backend::services::VenueService;

#[derive(Debug, World)]
pub struct VenueWorld {
    repo: MockVenueRepository,
    storage: MockImageStorage,
    owner_id: Option<String>,
    last_venue_id: Option<String>,
    last_response_status: Option<u16>,
}

impl Default for VenueWorld {
    fn default() -> Self {
        Self {
            repo: MockVenueRepository::new(),
            storage: MockImageStorage::new(),
            owner_id: None,
            last_venue_id: None,
            last_response_status: None,
        }
    }
}

impl VenueWorld {
    fn service(&self) -> VenueService<MockVenueRepository, MockImageStorage> {
        VenueService::new(self.repo.clone(), self.storage.clone())
    }
}

// Ensure MockVenueRepository and MockImageStorage are Cloneable for ease of use in tests
// They are now updated with Arc.

#[given(expr = "I am a registered venue owner")]
async fn i_am_a_registered_owner(world: &mut VenueWorld) {
    world.owner_id = Some("owner-1".to_string());
}

#[when(expr = "I submit the following details for my new venue:")]
async fn i_submit_venue_details(world: &mut VenueWorld, step: &cucumber::gherkin::Step) {
    let table = step.table().expect("Step must have a table");

    // Skip header row
    for row in table.rows.iter().skip(1) {
        let name = &row[0];
        let location = &row[1];
        let capacity: i32 = row[2].parse().expect("Capacity must be a number");

        let input = VenueInputDTO {
            name: name.clone(),
            location: location.clone(),
            capacity,
        };

        let venue_id = world
            .service()
            .create_venue(input)
            .await
            .expect("Failed to create venue");

        world.last_venue_id = Some(venue_id.0);
        world.last_response_status = Some(201);
    }
}

#[when(expr = "I upload the following images:")]
async fn i_upload_images(world: &mut VenueWorld, step: &cucumber::gherkin::Step) {
    let table = step.table().expect("Step must have a table");
    let venue_id = world
        .last_venue_id
        .clone()
        .expect("No venue created to upload images to");

    for row in table.rows.iter().skip(1) {
        let filename = &row[0];

        let upload_resp = world
            .service()
            .get_upload_url(&venue_id)
            .await
            .expect("Failed to get upload url");

        let complete_data = ImageUploadCompleteDTO {
            image_id: upload_resp.image_id,
            filename: filename.clone(),
        };

        world
            .service()
            .complete_upload(&venue_id, complete_data)
            .await
            .expect("Failed to complete upload");
    }
}

#[then(expr = "my venue should be successfully listed")]
async fn venue_should_be_listed(world: &mut VenueWorld) {
    assert_eq!(world.last_response_status, Some(201));
}

#[then(expr = "I should see {string} in my list of venues")]
async fn i_should_see_venue_in_list(world: &mut VenueWorld, name: String) {
    let venues = world
        .service()
        .list_venues()
        .await
        .expect("Failed to list venues");
    let exists = venues.iter().any(|v| v.name == name);
    assert!(exists, "Venue '{}' not found in list", name);
}

#[then(expr = "the venue {string} should have {int} images attached")]
async fn venue_should_have_images(world: &mut VenueWorld, name: String, count: usize) {
    let venues = world
        .service()
        .list_venues()
        .await
        .expect("Failed to list venues");

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
    VenueWorld::run("features/list_venue.feature").await;
}
