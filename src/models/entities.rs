use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VenueEntity {
    pub id: String,
    pub name: String,
    pub location: String,
    pub capacity: i32,
    pub owner_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VenueImageEntity {
    pub id: String,
    pub venue_id: String,
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookingEntity {
    pub id: String,
    pub venue_id: String,
    pub user_id: String,
    pub start_time: String,
    pub end_time: String,
}
