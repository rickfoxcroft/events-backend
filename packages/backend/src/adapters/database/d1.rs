use crate::models::{VenueEntity, VenueId, VenueImageEntity};
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
                venue.id.0.into(),
                venue.name.into(),
                venue.location.into(),
                venue.capacity.into(),
                venue.owner_id.0.into(),
            ])?
            .run()
            .await?;
        Ok(())
    }

    async fn list_venues(&self) -> Result<Vec<(VenueEntity, Vec<VenueImageEntity>)>> {
        let venues = self
            .db
            .prepare("SELECT * FROM venues")
            .all()
            .await?
            .results::<VenueEntity>()?;
        let mut results = Vec::new();

        for venue in venues {
            let images = self
                .db
                .prepare("SELECT * FROM venue_images WHERE venue_id = ?")
                .bind(&[venue.id.0.clone().into()])?
                .all()
                .await?
                .results::<VenueImageEntity>()?;
            results.push((venue, images));
        }

        Ok(results)
    }

    async fn get_venue_with_images(
        &self,
        id: VenueId,
    ) -> Result<Option<(VenueEntity, Vec<VenueImageEntity>)>> {
        let venue = self
            .db
            .prepare("SELECT * FROM venues WHERE id = ?")
            .bind(&[id.0.clone().into()])?
            .first::<VenueEntity>(None)
            .await?;

        if let Some(venue) = venue {
            let images = self
                .db
                .prepare("SELECT * FROM venue_images WHERE venue_id = ?")
                .bind(&[id.0.into()])?
                .all()
                .await?
                .results::<VenueImageEntity>()?;
            Ok(Some((venue, images)))
        } else {
            Ok(None)
        }
    }

    async fn save_venue_image(&self, image: VenueImageEntity) -> Result<()> {
        self.db
            .prepare("INSERT INTO venue_images (id, venue_id, url) VALUES (?, ?, ?)")
            .bind(&[image.id.0.into(), image.venue_id.0.into(), image.url.into()])?
            .run()
            .await?;
        Ok(())
    }
}
