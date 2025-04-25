package com.justin13888.pixles

import androidx.compose.ui.graphics.vector.ImageVector
import kotlinx.coroutines.CoroutineDispatcher

expect class PlatformStorableImage

expect val ioDispatcher: CoroutineDispatcher

expect val isShareFeatureSupported: Boolean
