package com.justin13888.pixles

import androidx.compose.ui.graphics.ImageBitmap
import kotlinx.coroutines.Dispatchers

class DesktopStorableImage(
    val imageBitmap: ImageBitmap
)

actual typealias PlatformStorableImage = DesktopStorableImage

actual val ioDispatcher = Dispatchers.IO

actual val isShareFeatureSupported: Boolean = false