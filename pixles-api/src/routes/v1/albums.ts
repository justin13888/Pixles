import Elysia, { t } from "elysia";
import { PhotoThumbnail } from "../../models/photo";
import { selectAlbumSchema } from "@/db/schema";

// TODO: Enforce bearer token

export const albumsRoutes = () => new Elysia({
    detail: {
        tags: ['Album']
    },
})
    .get('/', () => {
        return [
            {
                id: "1",
                name: "Album 1",
                description: "Description 1",
                coverUrl: "https://via.placeholder.com/150",
                dateCreated: new Date(),
                dateModified: new Date(),
            },
            {
                id: "2",
                name: "Album 2",
                description: "Description 2",
                coverUrl: "https://via.placeholder.com/150",
                dateCreated: new Date(),
                dateModified: new Date()
            }
        ]
    }, {
        detail: {
            summary: 'Get all albums',
            responses: {
                '200': {
                    description: 'List of albums',
                }
            }
        },
        response: t.Array(selectAlbumSchema),
    })
    .post('/', () => {
        console.log('Creating album')
        return {
            id: "1234",
        }
    }, {
        detail: {
            summary: 'Create an album',
            responses: {
                '200': {
                    description: 'Album created',
                }
            }
        },
        response: t.Object({
            id: t.String({ description: 'Album ID' }),
        })
    })
    .group('/:id', {
        params: t.Object({
            id: t.String({ description: 'Album ID' }),
        }),
    }, (app) => app
        .get('', ({ params: { id } }) => { // TODO: Add query params for cursor and limit
            console.log('Fetching album with id:', id)
            return [
                {
                    id: "1",
                    thumbnailUrl: "https://via.placeholder.com/150",
                    originalUrl: "https://via.placeholder.com/600",
                    timestamp: new Date().toISOString(),
                },
                {
                    id: "2",
                    thumbnailUrl: "https://via.placeholder.com/150",
                    originalUrl: "https://via.placeholder.com/600",
                    timestamp: new Date().toISOString(),
                }
            ]
        }, {
            detail: {
                summary: 'Get photos for an album',
                responses: {
                    '200': {
                        description: 'Photos for the album',
                    }
                },
            },
            response: t.Array(PhotoThumbnail),
        })
        .put('', ({ params: { id } }) => {
            console.log('Uploading to album with id:', id)
            return "FKjdfkjsdfkjds" // TODO: Implement upload
        }, {
            detail: {
                summary: 'Upload photos to an album',
                responses: {
                    '200': {
                        description: 'Upload task ID',
                    }
                }
            },
            response: t.String(),
        })
        .delete('', ({ params: { id }}) => {
            console.log('Deleting album with id:', id)
        }, {
            detail: {
                summary: 'Delete album',
                description: 'Marks the album as deleted',
                responses: {
                    '200': {
                        description: 'Album updated',
                    }
                }
            },
            response: t.Void(),
        })
    )
