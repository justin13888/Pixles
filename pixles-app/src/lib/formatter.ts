import type { ActivityType } from '@backend/models/activity'

export const dateFormatter = new Intl.DateTimeFormat("en-US", {
    year: "numeric",
    month: "long",
    day: "numeric",
    weekday: "long",
    hour: "numeric",
    minute: "numeric",
    second: "numeric",
}); // TODO: Load locale from settings

export const formatDate = (date: Date): string => {
    return dateFormatter.format(date);
}

export const activityToDescription = (activity: ActivityType): string => {
    switch (activity.type) {
        case 'photo':
        switch (activity.action) {
            case 'upload':
            return `Uploaded ${activity.photos.length} photo${activity.photos.length > 1 ? 's' : ''}`
            case 'delete':
            return `Deleted ${activity.photos.length} photo${activity.photos.length > 1 ? 's' : ''}`
        }
        break; // TODO: Biome needs to fix this false positive
        case 'album':
        switch (activity.action) {
            case 'create':
            return `Created album "${activity.albumName}"`
            case 'delete':
            return `Deleted album "${activity.albumName}"`
            case 'edit':
            return `Edited album "${activity.albumName}"`
            case 'share':
            return `Shared album "${activity.albumName}" with ${activity.users.length} user${activity.users.length > 1 ? 's' : ''}`
        }
    }
};
