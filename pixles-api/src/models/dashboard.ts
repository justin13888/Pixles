import { t } from "elysia";

export const Stats = t.Object({
    totalPhotos: t.Number(),
    totalAlbums: t.Number(),
    storageUsed: t.Number(),
})
