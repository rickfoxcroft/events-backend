use crate::models::entities::{UserId, VenueId, VenueImageEntity, VenueImageId};
use crate::models::{ImageUploadURLResponseDTO, VenueDTO, VenueEntity, VenueInputDTO};
use crate::ports::{ImageStorage, VenueRepository};
use worker::Result;

pub struct VenueService<R: VenueRepository, S: ImageStorage> {
    repo: R,
    storage: S,
}

impl<R: VenueRepository, S: ImageStorage> VenueService<R, S> {
    pub fn new(repo: R, storage: S) -> Self {
        Self { repo, storage }
    }

    pub async fn list_venues(&self) -> Result<Vec<VenueDTO>> {
        let items = self.repo.list_venues().await?;
        Ok(items.into_iter().map(VenueDTO::from).collect())
    }

    pub async fn create_venue(&self, input: VenueInputDTO) -> Result<VenueId> {
        let venue_id = VenueId::new_v7();
        let entity = VenueEntity {
            id: venue_id.clone(),
            name: input.name,
            location: input.location,
            capacity: input.capacity,
            // TODO: Get the actual owner_id from the authenticated user context
            owner_id: UserId("owner-1".to_string()),
        };
        self.repo.save_venue(entity).await?;

        // Atomic attachment of pre-uploaded images
        for image_id in input.image_ids {
            let public_url = self.storage.get_public_url(&venue_id.0, &image_id).await?;
            let image_entity = VenueImageEntity {
                id: VenueImageId(image_id),
                venue_id: venue_id.clone(),
                url: public_url,
            };
            self.repo.save_venue_image(image_entity).await?;
        }

        Ok(venue_id)
    }

    pub async fn get_upload_url(&self) -> Result<ImageUploadURLResponseDTO> {
        let image_id = VenueImageId::new_v7();
        let upload_url = self.storage.generate_upload_url(&image_id.0).await?;

        Ok(ImageUploadURLResponseDTO {
            upload_url,
            image_id: image_id.0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::database::MockVenueRepository;
    use crate::adapters::storage::mock::MockImageStorage;
    use crate::models::VenueInputDTO;

    #[tokio::test]
    async fn test_create_venue() {
        let repo = MockVenueRepository::new();
        let storage = MockImageStorage::new();
        let service = VenueService::new(repo, storage);

        let input = VenueInputDTO {
            name: "Test Venue".to_string(),
            location: "Test Location".to_string(),
            capacity: 100,
            image_ids: vec!["img-1".to_string(), "img-2".to_string()],
        };

        let venue_id = service.create_venue(input).await.unwrap();

        let venues = service.list_venues().await.unwrap();
        assert_eq!(venues.len(), 1);
        assert_eq!(venues[0].id, venue_id.0);
        assert_eq!(venues[0].images.len(), 2);
        assert!(venues[0].images[0].url.contains("public/img-1"));
    }

    #[tokio::test]
    async fn test_get_upload_url() {
        let repo = MockVenueRepository::new();
        let storage = MockImageStorage::new();
        let service = VenueService::new(repo, storage);

        let result = service.get_upload_url().await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.upload_url.contains("upload"));
        assert!(!response.image_id.is_empty());
    }
}
