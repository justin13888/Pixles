import { Static, t } from "elysia"
import { UserID } from "./user"
import { PhotoID } from "./photo"
import { AlbumID, AlbumName } from "./album"

export const PhotoUploadActivity = t.Object({
    date: t.Date(),
    type: t.Literal("photo"),
    action: t.Literal("upload"),
    photos: t.Array(PhotoID),
}, { description: "Photo Upload Activity"})

export const PhotoDeleteActivity = t.Object({
    date: t.Date(),
    type: t.Literal("photo"),
    action: t.Literal("delete"),
    photos: t.Array(PhotoID),
}, { description: "Photo Delete Activity"})

export const PhotoActivity = t.Union([
    PhotoUploadActivity,
    PhotoDeleteActivity,
], { description: "Photo Activity"})

export const AlbumCreateActivity = t.Object({
    date: t.Date(),
    type: t.Literal("album"),
    action: t.Literal("create"),
    albumId: AlbumID,
    albumName: AlbumName,
}, { description: "Album Create Activity"})

export const AlbumDeleteActivity = t.Object({
    date: t.Date(),
    type: t.Literal("album"),
    action: t.Literal("delete"),
    albumId: AlbumID,
    albumName: AlbumName,
}, { description: "Album Delete Activity"})

export const AlbumEditActivity = t.Object({
    date: t.Date(),
    type: t.Literal("album"),
    action: t.Literal("edit"),
    albumId: AlbumID,
    albumName: AlbumName,
    fields: t.Object({
        name: t.Optional(t.String({ description: "New album name" })),
        description: t.Optional(t.String({ description: "New album description" })),
    })
}, { description: "Album Edit Activity"})

export const AlbumShareActivity = t.Object({
    date: t.Date(),
    type: t.Literal("album"),
    action: t.Literal("share"),
    albumId: AlbumID,
    albumName: AlbumName,
    users: t.Array(UserID),
}, { description: "Album Share Activity"})

export const AlbumActivity = t.Union([
    AlbumCreateActivity,
    AlbumDeleteActivity,
    AlbumEditActivity,
    AlbumShareActivity,
], { description: "Album Activity"})

export const Activity = t.Union([
    PhotoActivity,
    AlbumActivity,
], { description: "Activity" })

export type ActivityType = Static<typeof Activity>
