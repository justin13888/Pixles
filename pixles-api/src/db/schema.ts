import { relations, sql } from "drizzle-orm";
import {
	int,
	mysqlEnum,
	mysqlTable,
	uniqueIndex,
	varchar,
	serial,
	datetime,
} from "drizzle-orm/mysql-core";
import { createInsertSchema, createSelectSchema } from "drizzle-typebox";
import { t } from "elysia";
import { nanoid } from "nanoid";

// declaring enum in database
export const albums = mysqlTable(
	"albums",
	{
		id: varchar("id", { length: 21 })
			.primaryKey()
			.$defaultFn(() => nanoid()),
		name: varchar("name", { length: 256 }),
		description: varchar("description", { length: 256 }),
		coverUrl: varchar("cover_url", { length: 2083 }),
		dateCreated: datetime("date_created")
			.notNull()
			.default(sql`CURRENT_TIMESTAMP`),
		dateModified: datetime("date_modified")
			.notNull()
			.default(sql`CURRENT_TIMESTAMP`)
			.$onUpdate(() => new Date()),
	},
	(albums) => ({
		nameIndex: uniqueIndex("name_idx").on(albums.name),
	}),
);

export const insertAlbumSchema = createInsertSchema(albums);
export const selectAlbumSchema = createSelectSchema(albums);

export const albumRelations = relations(albums, ({ many }) => ({
    photos: many(photos),
}));

export const photos = mysqlTable("photos", {
	id: varchar("id", { length: 21 })
		.primaryKey()
		.$defaultFn(() => nanoid()),
	albumId: varchar("album_id", { length: 21 }),
    thumbnailUrl: varchar("thumbnail_url", { length: 2083 }),
    originalUrl: varchar("original_url", { length: 2083 }).notNull(),
    dateCreated: datetime("date_created")
			.notNull()
			.default(sql`CURRENT_TIMESTAMP`),
    dateModified: datetime("date_modified")
        .notNull()
        .default(sql`CURRENT_TIMESTAMP`)
        .$onUpdate(() => new Date()),
}, (photos) => ({
	dateModifiedIndex: uniqueIndex("date_modified_idx").on(photos.dateModified),
}));

export const insertPhotoSchema = createInsertSchema(photos);
export const selectPhotoSchema = createSelectSchema(photos);

export const photoRelations = relations(photos, ({ one }) => ({
    album: one(albums, {
        fields: [photos.albumId],
        references: [albums.id],
    }),
}));

// TODO: Edit
