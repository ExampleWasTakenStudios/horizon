import * as z from 'zod';

export const HorizonConfigSchema = z.object({
  test: z.string(),
});

export type HorizonConfig = z.infer<typeof HorizonConfigSchema>;
