use crate::models::{ImageUploadCompleteDTO, ImageUploadURLResponseDTO, VenueDTO, VenueEntity, VenueImageEntity, VenueInputDTO};
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

    pub async fn create_venue(&self, input: VenueInputDTO) -> Result<()> {
        let entity = VenueEntity {
            id: "temp-id".to_string(),
            name: input.name,
            location: input.location,
            capacity: input.capacity,
            owner_id: "owner-1".to_string(),
        };
        self.repo.save_venue(entity).await
    }

    pub async fn get_upload_url(&self, venue_id: &str) -> Result<ImageUploadURLResponseDTO> {
        let image_id = "img-temp-id".to_string(); // In a real app, generate a UUID
        let upload_url = self.storage.generate_upload_url(venue_id, &image_id).await?;
        
        Ok(ImageUploadURLResponseDTO {
            upload_url,
            image_id,
        })
    }

    pub async fn complete_upload(&self, venue_id: &str, data: ImageUploadCompleteDTO) -> Result<()> {
        let public_url = self.storage.get_public_url(venue_id, &data.image_id).await?;
        
        let image_entity = VenueImageEntity {
            id: data.image_id,
            venue_id: venue_id.to_string(),
            url: public_url,
        };
        
        self.repo.save_venue_image(image_entity).await
    }
}
