package com.justin13888.pixles

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.IO
import platform.UIKit.UIImage

class IosStorableImage(
    val rawValue: UIImage
)

actual typealias PlatformStorableImage = IosStorableImage

actual val ioDispatcher = Dispatchers.IO

actual val isShareFeatureSupported: Boolean = true
