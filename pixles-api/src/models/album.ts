import { t } from "elysia";

export const Album = t.Object({
    id: t.String(),
    name: t.String(),
    description: t.Optional(t.String()),
    coverUrl: t.String({ format: 'uri'}),
    photoCount: t.Number(),
})
