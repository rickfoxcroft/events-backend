#![allow(async_fn_in_trait)]
use crate::models::{VenueEntity, VenueImageEntity};
use worker::Result;

pub trait VenueRepository {
    async fn save_venue(&self, venue: VenueEntity) -> Result<()>;
    async fn list_venues(&self) -> Result<Vec<(VenueEntity, Vec<VenueImageEntity>)>>;
    async fn get_venue_with_images(&self, id: String) -> Result<Option<(VenueEntity, Vec<VenueImageEntity>)>>;
    async fn save_venue_image(&self, image: VenueImageEntity) -> Result<()>;
}
