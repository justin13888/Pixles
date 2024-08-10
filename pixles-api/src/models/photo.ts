import { t } from "elysia"

export const Photo = t.Object({
    id: t.String(),
    albumId: t.String(),
    thumbnailUrl: t.String({ format: 'uri'}),
    originalUrl: t.String({ format: 'uri'}),
    timestamp: t.String({ format: 'date-time'}),
})

export const PhotoCollection = t.Array(Photo)
