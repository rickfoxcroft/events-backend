use crate::models::VenueEntity;
use crate::ports::VenueRepository;
use std::sync::RwLock;
use worker::Result;

#[derive(Debug)]
pub struct MockVenueRepository {
    venues: RwLock<Vec<VenueEntity>>,
}

impl MockVenueRepository {
    pub fn new() -> Self {
        Self {
            venues: RwLock::new(Vec::new()),
        }
    }
}

impl Default for MockVenueRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl VenueRepository for MockVenueRepository {
    async fn save_venue(&self, venue: VenueEntity) -> Result<()> {
        let mut venues = self
            .venues
            .write()
            .map_err(|_| worker::Error::from("Lock poisoned"))?;
        venues.push(venue);
        Ok(())
    }

    async fn list_venues(&self) -> Result<Vec<VenueEntity>> {
        let venues = self
            .venues
            .read()
            .map_err(|_| worker::Error::from("Lock poisoned"))?;
        Ok(venues.clone())
    }
}
