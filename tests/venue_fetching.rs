use cucumber::{given, then, when, World};
use event_app_backend::adapters::database::MockVenueRepository;
use event_app_backend::models::{VenueEntity, VenueImageEntity};
use event_app_backend::ports::VenueRepository;

#[derive(Debug, World)]
pub struct FetchVenueWorld {
    repo: MockVenueRepository,
    fetched_venues: Vec<(VenueEntity, Vec<VenueImageEntity>)>,
}

impl Default for FetchVenueWorld {
    fn default() -> Self {
        Self {
            repo: MockVenueRepository::new(),
            fetched_venues: Vec::new(),
        }
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

        let entity = VenueEntity {
            id: format!("id-{}", name.to_lowercase().replace(' ', "-")),
            name: name.clone(),
            location: location.clone(),
            capacity,
            owner_id: "owner-1".to_string(),
        };

        world
            .repo
            .save_venue(entity)
            .await
            .expect("Failed to save venue");
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

        let venue_id = format!("id-{}", name.to_lowercase().replace(' ', "-"));
        let entity = VenueEntity {
            id: venue_id.clone(),
            name: name.clone(),
            location: location.clone(),
            capacity,
            owner_id: "owner-1".to_string(),
        };

        world
            .repo
            .save_venue(entity)
            .await
            .expect("Failed to save venue");

        for (i, img_name) in images_str.split(',').enumerate() {
            let image = VenueImageEntity {
                id: format!("img-{}", i),
                venue_id: venue_id.clone(),
                url: format!("https://example.com/{}", img_name.trim()),
            };
            world
                .repo
                .save_venue_image(image)
                .await
                .expect("Failed to save image");
        }
    }
}

#[when(expr = "I request the list of all venues")]
async fn i_request_all_venues(world: &mut FetchVenueWorld) {
    world.fetched_venues = world
        .repo
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
    let exists = world.fetched_venues.iter().any(|(v, _)| v.name == name);
    assert!(exists, "Venue '{}' not found in list", name);
}

#[then(expr = "{string} should display its images")]
async fn venue_should_display_images(world: &mut FetchVenueWorld, name: String) {
    let (_, images) = world
        .fetched_venues
        .iter()
        .find(|(v, _)| v.name == name)
        .expect("Venue not found");
    assert!(!images.is_empty(), "Venue '{}' has no images", name);
}

#[tokio::test]
async fn test_venue_fetching() {
    FetchVenueWorld::run("features/fetch_venues.feature").await;
}
