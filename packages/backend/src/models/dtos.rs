use crate::models::entities::{BookingEntity, VenueEntity, VenueImageEntity};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VenueDTO {
    pub id: String,
    pub name: String,
    pub location: String,
    pub capacity: i32,
    pub price_per_hour: i32,
    pub owner_id: String,
    pub images: Vec<VenueImageDTO>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VenueImageDTO {
    pub id: String,
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VenueInputDTO {
    pub name: String,
    pub location: String,
    pub capacity: i32,
    pub price_per_hour: i32,
    pub image_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageUploadURLResponseDTO {
    pub upload_url: String,
    pub image_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookingDTO {
    pub id: String,
    pub venue_id: String,
    pub user_id: String,
    pub start_time: String,
    pub end_time: String,
}

// The Bridge: From Entity to DTO
impl From<(VenueEntity, Vec<VenueImageEntity>)> for VenueDTO {
    fn from(data: (VenueEntity, Vec<VenueImageEntity>)) -> Self {
        let (entity, images) = data;
        Self {
            id: entity.id.0,
            name: entity.name,
            location: entity.location,
            capacity: entity.capacity,
            price_per_hour: entity.price_per_hour,
            owner_id: entity.owner_id.0,
            images: images.into_iter().map(VenueImageDTO::from).collect(),
        }
    }
}

impl From<VenueImageEntity> for VenueImageDTO {
    fn from(entity: VenueImageEntity) -> Self {
        Self {
            id: entity.id.0,
            url: entity.url,
        }
    }
}

impl From<BookingEntity> for BookingDTO {
    fn from(entity: BookingEntity) -> Self {
        Self {
            id: entity.id.0,
            venue_id: entity.venue_id.0,
            user_id: entity.user_id.0,
            start_time: entity.start_time,
            end_time: entity.end_time,
        }
    }
}
