import Elysia, { NotFoundError, t } from "elysia";
import { PhotoThumbnail } from "../../models/photo";
import { db } from "@/db";
import { photos, selectPhotoSchema } from "@/db/schema";
import { eq } from "drizzle-orm";
import { generateThumbnail } from "@/lib/pipeline/preview";

export const photosRoutes = () =>
	new Elysia({
		detail: {
			tags: ["Photo"],
		},
	}).get(
		"/:photoId",
		async ({ params: { photoId } }) => {
			const photo = await db.query.photos.findFirst({
				columns: {
					thumbnailUrl: false,
					originalUrl: false,
				},
				where: eq(photos.id, photoId),
			})

			if (!photo) {
				throw new NotFoundError("Photo not found");
			}

			return photo;
		},
		{
			detail: {
				description: "Get photo from an album",
				responses: {
					"200": {
						description: "Photo",
					},
					"404": {
						description: "Photo not found",
						
					},
				},
			},
			params: t.Object({
				photoId: t.String({ description: 'Photo ID' }),
			}),
			response: t.Omit(selectPhotoSchema, ["thumbnailUrl", "originalUrl"]),
			error({ error }) {
				if (error instanceof NotFoundError) {
					return new Response("Photo not found", { status: 404 });
				}
			}
		},
	).get("/:photoId/preview", async function*({ params: { photoId }, set, redirect }) {
		const photo = await db.query.photos.findFirst({
			columns: {
				thumbnailUrl: true,
				originalUrl: true,
			},
			where: eq(photos.id, photoId),
		});

		if (!photo) {
			throw new NotFoundError("Photo not found");
		}

		if (photo.thumbnailUrl) {
			return redirect(photo.thumbnailUrl, 302);
		}

		// Stream photo preview
		set.headers["content-type"] = 'image/jpeg'
		set.headers["transfer-encoding"] = 'chunked'
		yield* generateThumbnail(photo.originalUrl);
	}, {
		detail: {
			summary: "Get photo preview",
			responses: {
				"200": {
					description: "Photo preview stream",
				},
				"302": {
					description: "Redirect to photo preview",
				},
				"404": {
					description: "Photo not found",
				},
			},
		},
	})
	.get("/:photoId/original", async ({ params: { photoId } }) => {
		const photo = await db.query.photos.findFirst({
			columns: {
				originalUrl: true,
			},
			where: eq(photos.id, photoId),
		});

		if (!photo) {
			throw new NotFoundError("Photo not found");
		}

		return photo.originalUrl;
	}, {
		detail: {
			summary: "Get original photo",
			responses: {
				"200": {
					description: "Original photo URL",
				},
				"404": {
					description: "Photo not found",
				},
			},
		},
		response: t.String({ description: "Original photo URL" }),
	})
// TODO: Check implementation
