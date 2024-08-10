import Elysia, { type Static, t } from "elysia";
import { Stats } from "../../models/dashboard";
import { Activity } from "@/models/activity";

export const dashboardRoutes = () => new Elysia({
    detail: {
        tags: ['Dashboard']
    },
})
    .get('/recent-activity', () => {
        return [
            {
                date: new Date(),
                type: 'photo',
                action: 'upload',
                photos: ['photo1', 'photo2'],
            }
        ] as Static<typeof Activity>[]
    }, {
        detail: {
            description: 'Get recent activity',
        },
        response: t.Array(Activity),
    })
    .get('/stats', () => {
        return {
            totalPhotos: 100,
            totalAlbums: 10,
            storageUsed: 29*1024**3, // 1 GiB
        }
    }, {
        detail: {
            description: 'Get stats',
        },
        response: Stats,
    })
