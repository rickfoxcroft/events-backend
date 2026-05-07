use crate::models::{VenueDTO, VenueEntity, VenueInputDTO};
use crate::ports::VenueRepository;
use worker::Result;

pub struct VenueService<R: VenueRepository> {
    repo: R,
}

impl<R: VenueRepository> VenueService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn list_venues(&self) -> Result<Vec<VenueDTO>> {
        let entities = self.repo.list_venues().await?;
        Ok(entities.into_iter().map(VenueDTO::from).collect())
    }

    pub async fn create_venue(&self, input: VenueInputDTO) -> Result<()> {
        // Business logic: logic for ID generation, owner assignment, etc.
        let entity = VenueEntity {
            id: "temp-id".to_string(), // In a real app, generate a UUID
            name: input.name,
            location: input.location,
            capacity: input.capacity,
            owner_id: "owner-1".to_string(), // In a real app, get from auth context
        };
        self.repo.save_venue(entity).await
    }
}
