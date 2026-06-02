import createClient from 'openapi-fetch';
import type { paths } from '../types/api';
import { config } from '../config';

export const api = createClient<paths>({
  baseUrl: config.publicApiUrl,
});

/**
 * Creates an authenticated API client.
 * In SSR, pass the token from cookies.
 * In Client, it will attempt to read from document.cookie if no token provided.
 */
export const getAuthenticatedClient = (token?: string) => {
  let authToken = token;

  if (!authToken && typeof document !== 'undefined') {
    const match = document.cookie.match(/auth_token=([^;]+)/);
    if (match) authToken = match[1];
  }

  return createClient<paths>({
    baseUrl: config.publicApiUrl,
    headers: authToken
      ? {
          Authorization: `Bearer ${authToken}`,
        }
      : {},
  });
};
