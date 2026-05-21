import { z } from 'zod';

const configSchema = z.object({
  publicApiUrl: z
    .string()
    .url({ message: 'PUBLIC_API_URL must be a valid URL' }),
});

export type Config = z.infer<typeof configSchema>;

export const config = configSchema.parse({
  publicApiUrl: import.meta.env.PUBLIC_API_URL,
});
