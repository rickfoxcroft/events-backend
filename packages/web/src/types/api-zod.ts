import { z } from 'zod';

const ImageUploadURLResponseDTO = z
  .object({ upload_url: z.string(), image_id: z.string().uuid() })
  .passthrough();
const VenueImageDTO = z
  .object({ id: z.string().uuid(), url: z.string() })
  .passthrough();
const VenueDTO = z
  .object({
    id: z.string().uuid(),
    name: z.string(),
    location: z.string(),
    capacity: z.number().int(),
    price_per_hour: z.number().int(),
    owner_id: z.string().uuid(),
    images: z.array(VenueImageDTO),
  })
  .passthrough();
const VenueInputDTO = z
  .object({
    name: z.string(),
    location: z.string(),
    capacity: z.number().int(),
    price_per_hour: z.number().int(),
    image_ids: z.array(z.string()),
  })
  .passthrough();

export const schemas = {
  ImageUploadURLResponseDTO,
  VenueImageDTO,
  VenueDTO,
  VenueInputDTO,
};
