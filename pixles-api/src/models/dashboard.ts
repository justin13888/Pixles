import { t } from "elysia";

export const Stats = t.Object({
    totalPhotos: t.Number({ description: "Total number of photos" }),
    totalAlbums: t.Number({ description: "Total number of albums" }),
    storageUsed: t.Number({ description: "Total storage used in bytes" }),
});
