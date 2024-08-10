import { t } from "elysia"

export const PhotoThumbnail = t.Object({
    id: t.String(),
    thumbnailUrl: t.String({ format: 'uri'}),
    originalUrl: t.String({ format: 'uri'}),
    timestamp: t.String({ format: 'date-time'}),
});

export const PhotoID = t.String({ description: "Photo ID" });
