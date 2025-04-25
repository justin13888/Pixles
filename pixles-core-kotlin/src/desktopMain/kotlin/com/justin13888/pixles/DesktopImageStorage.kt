package com.justin13888.pixles

import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.toAwtImage
import androidx.compose.ui.graphics.toComposeImageBitmap
//import com.justin13888.pixles.filter.scaleBitmapAspectRatio
import com.justin13888.pixles.models.PictureData
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch
import kotlin.uuid.ExperimentalUuidApi
import kotlin.uuid.Uuid

private const val maxStorableImageSizePx = 2000
private const val storableThumbnailSizePx = 200

class DesktopImageStorage(
    private val ioScope: CoroutineScope
) : ImageStorage {
    private val largeImages = mutableMapOf<Uuid, ImageBitmap>()

    private val thumbnails = mutableMapOf<Uuid, ImageBitmap>()

    override fun saveImage(picture: PictureData.Camera, image: PlatformStorableImage) {
        if (image.imageBitmap.width == 0 || image.imageBitmap.height == 0) {
            return
        }
        ioScope.launch {
            // TODO: May want to ensure image.imageBitmap is appropriate size/not too big
            largeImages[picture.id] = image.imageBitmap
            thumbnails[picture.id] = image.imageBitmap
        }
    }

    override fun delete(picture: PictureData.Camera) {
        largeImages.remove(picture.id)
        thumbnails.remove(picture.id)
    }

    override fun rewrite(picture: PictureData.Camera) {
        // For now, on Desktop pictures saving in memory. We don't need additional rewrite logic.
    }

    override suspend fun getThumbnail(picture: PictureData.Camera): ImageBitmap {
        return thumbnails[picture.id]!!
    }

    override suspend fun getImage(picture: PictureData.Camera): ImageBitmap {
        return largeImages[picture.id]!!
    }
}

//private fun ImageBitmap.fitInto(px: Int): ImageBitmap {
//    val targetScale = maxOf(
//        px.toFloat() / width,
//        px.toFloat() / height
//    )
//    return if (targetScale < 1.0) {
//        scaleBitmapAspectRatio(
//            toAwtImage(),
//            width = (width * targetScale).toInt(),
//            height = (height * targetScale).toInt()
//        ).toComposeImageBitmap()
//    } else {
//        this
//    }
//}
