use cucumber::{given, then, when, World};
use event_app_backend::adapters::database::MockVenueRepository;
use event_app_backend::adapters::storage::mock::MockImageStorage;
use event_app_backend::models::{VenueDTO, VenueInputDTO, ImageUploadCompleteDTO};
use event_app_backend::services::VenueService;

#[derive(Debug, World)]
pub struct FetchVenueWorld {
    repo: MockVenueRepository,
    storage: MockImageStorage,
    fetched_venues: Vec<VenueDTO>,
}

impl Default for FetchVenueWorld {
    fn default() -> Self {
        Self {
            repo: MockVenueRepository::new(),
            storage: MockImageStorage::new(),
            fetched_venues: Vec::new(),
        }
    }
}

impl FetchVenueWorld {
    fn service(&self) -> VenueService<MockVenueRepository, MockImageStorage> {
        VenueService::new(self.repo.clone(), self.storage.clone())
    }
}

#[given(expr = "there are no venues listed on the platform")]
async fn there_are_no_venues(_world: &mut FetchVenueWorld) {
    // Repository is empty by default
}

#[given(expr = "the following venues exist:")]
async fn the_following_venues_exist(world: &mut FetchVenueWorld, step: &cucumber::gherkin::Step) {
    let table = step.table().expect("Step must have a table");

    for row in table.rows.iter().skip(1) {
        let name = &row[0];
        let location = &row[1];
        let capacity: i32 = row[2].parse().expect("Capacity must be a number");

        let input = VenueInputDTO {
            name: name.clone(),
            location: location.clone(),
            capacity,
        };

        world
            .service()
            .create_venue(input)
            .await
            .expect("Failed to create venue");
    }
}

#[given(expr = "the following venues exist with images:")]
async fn the_following_venues_exist_with_images(
    world: &mut FetchVenueWorld,
    step: &cucumber::gherkin::Step,
) {
    let table = step.table().expect("Step must have a table");

    for row in table.rows.iter().skip(1) {
        let name = &row[0];
        let location = &row[1];
        let capacity: i32 = row[2].parse().expect("Capacity must be a number");
        let images_str = &row[3];

        let input = VenueInputDTO {
            name: name.clone(),
            location: location.clone(),
            capacity,
        };

        world
            .service()
            .create_venue(input)
            .await
            .expect("Failed to create venue");
        
        // Note: VenueService currently uses "temp-id"
        let venue_id = "temp-id";

        for img_name in images_str.split(',') {
            let upload_resp = world.service().get_upload_url(venue_id).await.expect("Failed to get upload url");
            
            let complete_data = ImageUploadCompleteDTO {
                image_id: upload_resp.image_id,
                filename: img_name.trim().to_string(),
            };
            
            world.service().complete_upload(venue_id, complete_data).await.expect("Failed to complete upload");
        }
    }
}

#[when(expr = "I request the list of all venues")]
async fn i_request_all_venues(world: &mut FetchVenueWorld) {
    world.fetched_venues = world
        .service()
        .list_venues()
        .await
        .expect("Failed to list venues");
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
        .expect("Venue not found");
    assert!(!venue.images.is_empty(), "Venue '{}' has no images", name);
}

#[tokio::test]
async fn test_venue_fetching() {
    FetchVenueWorld::run("features/fetch_venues.feature").await;
}
