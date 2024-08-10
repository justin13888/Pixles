import Elysia, { t } from "elysia";
import { Photo } from "../../models/photo";

export const photos = () =>
	new Elysia({
		detail: {
			tags: ["Photo"],
		},
	}).get(
		"/:photoId",
		({ params: { photoId } }) => {
			console.log(
				"Fetching photo with id:",
				photoId,
				"from album with id:",
				photoId,
			);
			return {
				id: "fdf",
				albumId: "1",
				thumbnailUrl: "https://via.placeholder.com/150",
				originalUrl: "https://via.placeholder.com/600",
				timestamp: new Date().toISOString(),
                
			};
		},
		{
			detail: {
				description: "Get photo from an album",
				responses: {
					"200": {
						description: "Oriign",
					},
				},
			},
            params: t.Object({
                photoId: t.String({ description: 'Photo ID' }),
            }),
			response: Photo,
		},
	);
