#![allow(async_fn_in_trait)]
use crate::models::VenueEntity;
use worker::Result;

pub trait VenueRepository {
    async fn save_venue(&self, venue: VenueEntity) -> Result<()>;
    async fn list_venues(&self) -> Result<Vec<VenueEntity>>;
}
