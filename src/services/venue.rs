use crate::models::entities::{UserId, VenueId, VenueImageId};
use crate::models::{
    ImageUploadCompleteDTO, ImageUploadURLResponseDTO, VenueDTO, VenueEntity, VenueImageEntity,
    VenueInputDTO,
};
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
        let id = VenueId::new_v7();
        let entity = VenueEntity {
            id: id.clone(),
            name: input.name,
            location: input.location,
            capacity: input.capacity,
            // TODO: Get the actual owner_id from the authenticated user context
            owner_id: UserId("owner-1".to_string()),
        };
        self.repo.save_venue(entity).await?;
        Ok(id)
    }

    pub async fn get_upload_url(&self, venue_id: &str) -> Result<ImageUploadURLResponseDTO> {
        let image_id = VenueImageId::new_v7();
        let upload_url = self
            .storage
            .generate_upload_url(venue_id, &image_id.0)
            .await?;

        Ok(ImageUploadURLResponseDTO {
            upload_url,
            image_id: image_id.0,
        })
    }

    pub async fn complete_upload(
        &self,
        venue_id: &str,
        data: ImageUploadCompleteDTO,
    ) -> Result<()> {
        let public_url = self
            .storage
            .get_public_url(venue_id, &data.image_id)
            .await?;

        let image_entity = VenueImageEntity {
            id: VenueImageId(data.image_id),
            venue_id: VenueId(venue_id.to_string()),
            url: public_url,
        };

        self.repo.save_venue_image(image_entity).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::database::MockVenueRepository;
    use crate::adapters::storage::mock::MockImageStorage;
    use crate::models::{ImageUploadCompleteDTO, VenueInputDTO};

    #[tokio::test]
    async fn test_create_venue() {
        let repo = MockVenueRepository::new();
        let storage = MockImageStorage::new();
        let service = VenueService::new(repo, storage);

        let input = VenueInputDTO {
            name: "Test Venue".to_string(),
            location: "Test Location".to_string(),
            capacity: 100,
        };

        let result = service.create_venue(input).await;
        assert!(result.is_ok());

        let venues = service.list_venues().await.unwrap();
        assert_eq!(venues.len(), 1);
        assert_eq!(venues[0].name, "Test Venue");
        assert!(!venues[0].id.is_empty());
    }

    #[tokio::test]
    async fn test_get_upload_url() {
        let repo = MockVenueRepository::new();
        let storage = MockImageStorage::new();
        let service = VenueService::new(repo, storage);

        let result = service.get_upload_url("venue-1").await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.upload_url.contains("upload"));
        assert!(!response.image_id.is_empty());
    }

    #[tokio::test]
    async fn test_complete_upload() {
        let repo = MockVenueRepository::new();
        let storage = MockImageStorage::new();
        let service = VenueService::new(repo, storage);

        // First create a venue
        service
            .create_venue(VenueInputDTO {
                name: "Test Venue".to_string(),
                location: "Test Location".to_string(),
                capacity: 100,
            })
            .await
            .unwrap();

        let venues = service.list_venues().await.unwrap();
        let venue_id = &venues[0].id;

        let complete_data = ImageUploadCompleteDTO {
            image_id: "test-img-id".to_string(),
            filename: "test.jpg".to_string(),
        };

        let result = service.complete_upload(venue_id, complete_data).await;
        assert!(result.is_ok());

        let venues = service.list_venues().await.unwrap();
        assert_eq!(venues[0].images.len(), 1);
        assert!(venues[0].images[0].url.contains("public/test-img-id"));
    }
}
