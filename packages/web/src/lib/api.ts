import createClient from 'openapi-fetch';
import type { paths } from '../types/api';
import { config } from '../config';

/**
 * Type-safe API client for the Event App.
 *
 * This client is strictly typed against the auto-generated 'paths' interface
 * from our TypeSpec contract. It prevents calling invalid paths or using
 * incorrect HTTP methods at compile time.
 *
 * Usage:
 * const { data, error } = await api.GET('/venues');
 */
export const api = createClient<paths>({
  baseUrl: config.publicApiUrl,
});
