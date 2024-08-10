import axios from "axios";
import sharp from "sharp";

/**
 * Streams a JPEG thumbnail of the original image
 * @param originalUrl Original image URL
 */
export const generateThumbnail = async function* (originalUrl: string) {
    const originalImageResponse = await axios({
        url: originalUrl,
        method: 'GET',
        responseType: 'stream',
    });

    const transform = sharp().resize(200, 200).jpeg({ quality: 80});

    originalImageResponse.data.pipe(transform);

    for await (const chunk of transform) {
        yield chunk;
    }
};
// TODO: Check it works
