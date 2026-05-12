use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VenueId(pub String);

impl VenueId {
    pub fn new_v7() -> Self {
        Self(Uuid::now_v7().to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VenueImageId(pub String);

impl VenueImageId {
    pub fn new_v7() -> Self {
        Self(Uuid::now_v7().to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserId(pub String);

impl UserId {
    pub fn new_v7() -> Self {
        Self(Uuid::now_v7().to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BookingId(pub String);

impl BookingId {
    pub fn new_v7() -> Self {
        Self(Uuid::now_v7().to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VenueEntity {
    pub id: VenueId,
    pub name: String,
    pub location: String,
    pub capacity: i32,
    pub owner_id: UserId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VenueImageEntity {
    pub id: VenueImageId,
    pub venue_id: VenueId,
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookingEntity {
    pub id: BookingId,
    pub venue_id: VenueId,
    pub user_id: UserId,
    pub start_time: String,
    pub end_time: String,
}
