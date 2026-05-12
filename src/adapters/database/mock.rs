use crate::models::{VenueEntity, VenueId, VenueImageEntity};
use crate::ports::VenueRepository;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use worker::Result;

#[derive(Clone, Default, Debug)]
pub struct MockVenueRepository {
    venues: Arc<RwLock<HashMap<VenueId, VenueEntity>>>,
    venue_images: Arc<RwLock<HashMap<VenueId, Vec<VenueImageEntity>>>>,
}

impl MockVenueRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl VenueRepository for MockVenueRepository {
    async fn save_venue(&self, venue: VenueEntity) -> Result<()> {
        let mut venues = self
            .venues
            .write()
            .map_err(|_| worker::Error::from("Lock poisoned"))?;
        venues.insert(venue.id.clone(), venue);
        Ok(())
    }

    async fn list_venues(&self) -> Result<Vec<(VenueEntity, Vec<VenueImageEntity>)>> {
        let venues = self
            .venues
            .read()
            .map_err(|_| worker::Error::from("Lock poisoned"))?;
        let images = self
            .venue_images
            .read()
            .map_err(|_| worker::Error::from("Lock poisoned"))?;

        let mut results = Vec::new();
        for venue in venues.values() {
            let venue_images = images.get(&venue.id).cloned().unwrap_or_default();
            results.push((venue.clone(), venue_images));
        }

        Ok(results)
    }

    async fn get_venue_with_images(
        &self,
        id: VenueId,
    ) -> Result<Option<(VenueEntity, Vec<VenueImageEntity>)>> {
        let venues = self
            .venues
            .read()
            .map_err(|_| worker::Error::from("Lock poisoned"))?;
        let images = self
            .venue_images
            .read()
            .map_err(|_| worker::Error::from("Lock poisoned"))?;

        if let Some(venue) = venues.get(&id) {
            let venue_images = images.get(&id).cloned().unwrap_or_default();
            Ok(Some((venue.clone(), venue_images)))
        } else {
            Ok(None)
        }
    }

    async fn save_venue_image(&self, image: VenueImageEntity) -> Result<()> {
        let mut images = self
            .venue_images
            .write()
            .map_err(|_| worker::Error::from("Lock poisoned"))?;
        let venue_images = images.entry(image.venue_id.clone()).or_default();
        venue_images.push(image);
        Ok(())
    }
}
