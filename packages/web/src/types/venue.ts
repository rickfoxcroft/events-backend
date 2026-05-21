import { z } from 'zod';
import { schemas } from './api-zod';

export type VenueDTO = z.infer<typeof schemas.VenueDTO>;
export type VenueInputDTO = z.infer<typeof schemas.VenueInputDTO>;
export type VenueImageDTO = z.infer<typeof schemas.VenueImageDTO>;

/**
 * Frontend-specific Venue type, often used for display logic
 * or when we need to transform the raw API DTO.
 */
export interface Venue {
  id: string;
  name: string;
  location: string;
  capacity: number;
  imageUrl: string;
  pricePerHour: number;
}
