use crate::models::{VenueEntity, VenueImageEntity};
use crate::ports::VenueRepository;
use std::sync::RwLock;
use worker::Result;

#[derive(Debug)]
pub struct MockVenueRepository {
    venues: RwLock<Vec<VenueEntity>>,
    images: RwLock<Vec<VenueImageEntity>>,
}

impl MockVenueRepository {
    pub fn new() -> Self {
        Self {
            venues: RwLock::new(Vec::new()),
            images: RwLock::new(Vec::new()),
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

    async fn list_venues(&self) -> Result<Vec<(VenueEntity, Vec<VenueImageEntity>)>> {
        let venues = self
            .venues
            .read()
            .map_err(|_| worker::Error::from("Lock poisoned"))?;
        let images = self
            .images
            .read()
            .map_err(|_| worker::Error::from("Lock poisoned"))?;

        let mut results = Vec::new();
        for venue in venues.iter() {
            let venue_images: Vec<VenueImageEntity> = images
                .iter()
                .filter(|img| img.venue_id == venue.id)
                .cloned()
                .collect();
            results.push((venue.clone(), venue_images));
        }
        Ok(results)
    }

    async fn get_venue_with_images(&self, id: String) -> Result<Option<(VenueEntity, Vec<VenueImageEntity>)>> {
        let venues = self
            .venues
            .read()
            .map_err(|_| worker::Error::from("Lock poisoned"))?;
        let images = self
            .images
            .read()
            .map_err(|_| worker::Error::from("Lock poisoned"))?;

        if let Some(venue) = venues.iter().find(|v| v.id == id) {
            let venue_images: Vec<VenueImageEntity> = images
                .iter()
                .filter(|img| img.venue_id == id)
                .cloned()
                .collect();
            Ok(Some((venue.clone(), venue_images)))
        } else {
            Ok(None)
        }
    }

    async fn save_venue_image(&self, image: VenueImageEntity) -> Result<()> {
        let mut images = self
            .images
            .write()
            .map_err(|_| worker::Error::from("Lock poisoned"))?;
        images.push(image);
        Ok(())
    }
}
