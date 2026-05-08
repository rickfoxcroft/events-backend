use cucumber::{given, then, when, World};
use event_app_backend::adapters::database::MockVenueRepository;
use event_app_backend::models::{VenueEntity, VenueImageEntity};
use event_app_backend::ports::VenueRepository;

#[derive(Debug, World)]
pub struct VenueWorld {
    repo: MockVenueRepository,
    owner_id: Option<String>,
    last_venue_id: Option<String>,
    last_response_status: Option<u16>,
}

impl Default for VenueWorld {
    fn default() -> Self {
        Self {
            repo: MockVenueRepository::new(),
            owner_id: None,
            last_venue_id: None,
            last_response_status: None,
        }
    }
}

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

        let venue_id = format!("id-{}", name.to_lowercase().replace(' ', "-"));
        let entity = VenueEntity {
            id: venue_id.clone(),
            name: name.clone(),
            location: location.clone(),
            capacity,
            owner_id: world.owner_id.clone().expect("Owner must be registered"),
        };

        world
            .repo
            .save_venue(entity)
            .await
            .expect("Failed to save venue");
        
        world.last_venue_id = Some(venue_id);
        world.last_response_status = Some(201);
    }
}

#[when(expr = "I upload the following images:")]
async fn i_upload_images(world: &mut VenueWorld, step: &cucumber::gherkin::Step) {
    let table = step.table().expect("Step must have a table");
    let venue_id = world.last_venue_id.clone().expect("No venue created to upload images to");

    for (i, row) in table.rows.iter().skip(1).enumerate() {
        let filename = &row[0];
        
        let image = VenueImageEntity {
            id: format!("img-{}", i),
            venue_id: venue_id.clone(),
            url: format!("https://example.com/{}", filename),
        };
        
        world.repo.save_venue_image(image).await.expect("Failed to save image");
    }
}

#[then(expr = "my venue should be successfully listed")]
async fn venue_should_be_listed(world: &mut VenueWorld) {
    assert_eq!(world.last_response_status, Some(201));
}

#[then(expr = "I should see {string} in my list of venues")]
async fn i_should_see_venue_in_list(world: &mut VenueWorld, name: String) {
    let venues = world
        .repo
        .list_venues()
        .await
        .expect("Failed to list venues");
    let exists = venues.iter().any(|(v, _)| v.name == name);
    assert!(exists, "Venue '{}' not found in list", name);
}

#[then(expr = "the venue {string} should have {int} images attached")]
async fn venue_should_have_n_images(world: &mut VenueWorld, name: String, count: usize) {
    let venues = world
        .repo
        .list_venues()
        .await
        .expect("Failed to list venues");
    
    let (_, images) = venues.iter().find(|(v, _)| v.name == name).expect("Venue not found");
    assert_eq!(images.len(), count, "Expected {} images, found {}", count, images.len());
}

#[tokio::test]
async fn test_venue_listing() {
    VenueWorld::run("features/list_venue.feature").await;
}
