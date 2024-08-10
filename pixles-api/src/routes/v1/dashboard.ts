import Elysia, { t } from "elysia";
import { Stats } from "../../models/dashboard";

export const dashboardRoutes = () => new Elysia({
    detail: {
        tags: ['Dashboard']
    },
})
    .get('/recent-activity', () => {
        return null
    }, {
        detail: {
            description: 'Get recent activity',
        },
        response: t.Null(), // TODO: Define response
    })
    .get('/stats', () => {
        return {
            totalPhotos: 100,
            totalAlbums: 10,
            storageUsed: 1000000,
        }   
    }, {
        detail: {
            description: 'Get stats',
        },
        response: Stats,
    })
