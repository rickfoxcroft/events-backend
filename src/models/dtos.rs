use crate::models::entities::{BookingEntity, VenueEntity};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VenueDTO {
    pub id: String,
    pub name: String,
    pub location: String,
    pub capacity: i32,
    pub owner_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VenueInputDTO {
    pub name: String,
    pub location: String,
    pub capacity: i32,
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
impl From<VenueEntity> for VenueDTO {
    fn from(entity: VenueEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            location: entity.location,
            capacity: entity.capacity,
            owner_id: entity.owner_id,
        }
    }
}

impl From<BookingEntity> for BookingDTO {
    fn from(entity: BookingEntity) -> Self {
        Self {
            id: entity.id,
            venue_id: entity.venue_id,
            user_id: entity.user_id,
            start_time: entity.start_time,
            end_time: entity.end_time,
        }
    }
}
