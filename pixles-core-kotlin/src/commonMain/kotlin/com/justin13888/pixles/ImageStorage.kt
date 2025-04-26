package com.justin13888.pixles

import androidx.compose.ui.graphics.ImageBitmap
import com.justin13888.pixles.models.PictureData
import com.justin13888.pixles.PlatformStorableImage

interface ImageStorage {
    fun saveImage(picture: PictureData.Camera, image: PlatformStorableImage)
    fun delete(picture: PictureData.Camera)
    fun rewrite(picture: PictureData.Camera)
    suspend fun getThumbnail(picture: PictureData.Camera): ImageBitmap
    suspend fun getImage(picture: PictureData.Camera): ImageBitmap
}