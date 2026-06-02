#![allow(async_fn_in_trait)]
use crate::models::{UserEntity, VenueEntity, VenueId, VenueImageEntity};
use worker::Result;

pub trait VenueRepository {
    async fn save_venue(&self, venue: VenueEntity) -> Result<()>;
    async fn list_venues(&self) -> Result<Vec<(VenueEntity, Vec<VenueImageEntity>)>>;
    async fn get_venue_with_images(
        &self,
        id: VenueId,
    ) -> Result<Option<(VenueEntity, Vec<VenueImageEntity>)>>;
    async fn save_venue_image(&self, image: VenueImageEntity) -> Result<()>;
}

pub trait UserRepository {
    async fn get_user_by_provider_id(&self, provider_id: &str) -> Result<Option<UserEntity>>;
    async fn save_user(&self, user: UserEntity) -> Result<()>;
}
