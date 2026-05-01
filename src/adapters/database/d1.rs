use crate::models::VenueEntity;
use crate::ports::VenueRepository;
use worker::d1::D1Database;
use worker::Result;

pub struct D1VenueRepository {
    db: D1Database,
}

impl D1VenueRepository {
    pub fn new(db: D1Database) -> Self {
        Self { db }
    }
}

impl VenueRepository for D1VenueRepository {
    async fn save_venue(&self, venue: VenueEntity) -> Result<()> {
        self.db
            .prepare("INSERT INTO venues (id, name, location, capacity, owner_id) VALUES (?, ?, ?, ?, ?)")
            .bind(&[
                venue.id.into(),
                venue.name.into(),
                venue.location.into(),
                venue.capacity.into(),
                venue.owner_id.into(),
            ])?
            .run()
            .await?;
        Ok(())
    }

    async fn list_venues(&self) -> Result<Vec<VenueEntity>> {
        let results = self.db.prepare("SELECT * FROM venues").all().await?;
        results.results::<VenueEntity>()
    }
}
