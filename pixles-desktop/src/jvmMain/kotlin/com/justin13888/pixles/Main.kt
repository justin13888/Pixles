package com.justin13888.pixles

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.window.application
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.material3.Text
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.window.Window
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.key.Key
import androidx.compose.ui.input.key.KeyEventType
import androidx.compose.ui.input.key.key
import androidx.compose.ui.input.key.type
import androidx.compose.ui.unit.DpSize
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.ApplicationScope
import androidx.compose.ui.window.Window
import androidx.compose.ui.window.WindowPosition
import androidx.compose.ui.window.WindowState
import com.justin13888.pixles.*
//import com.justin13888.pixles.filter.PlatformContext
//import com.justin13888.pixles.model.PictureData
//import com.justin13888.pixles.style.ImageViewerTheme
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.channels.BufferOverflow
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.flow.asSharedFlow
import org.jetbrains.compose.resources.painterResource
import java.awt.Dimension
import java.awt.Toolkit

fun main() = application {
    DesktopView()
}

@Composable
fun ApplicationScope.DesktopView() {
    val ioScope = rememberCoroutineScope { ioDispatcher }
//    val toastState = remember { mutableStateOf<ToastState>(ToastState.Hidden) }
//    val externalNavigationEventBus = remember { ExternalNavigationEventBus() } // TODO: How do we idiomatically pass around mouse/keyboard events

    Window(
        onCloseRequest = ::exitApplication,
        title = "Image Viewer",
        state = WindowState(
            position = WindowPosition.Aligned(Alignment.Center),
            size = getPreferredWindowSize(720, 857)
        ),
//        icon = painterResource(Res.drawable.ic_imageviewer_round),
        // https://github.com/JetBrains/compose-jb/issues/2741
        onKeyEvent = {
//            if (it.type == KeyEventType.KeyUp) {
//                // TODO: do something
//                when (it.key) {
//                    Key.DirectionLeft -> externalNavigationEventBus.produceEvent(
//                        ExternalImageViewerEvent.Previous
//                    )
//
//                    Key.DirectionRight -> externalNavigationEventBus.produceEvent(
//                        ExternalImageViewerEvent.Next
//                    )
//
//                    Key.Escape -> externalNavigationEventBus.produceEvent(
//                        ExternalImageViewerEvent.ReturnBack
//                    )
//                }
//            }
            false
        }
    ) {
        Text("sdfd")
    }
}

private fun getPreferredWindowSize(desiredWidth: Int, desiredHeight: Int): DpSize {
    val screenSize: Dimension = Toolkit.getDefaultToolkit().screenSize
    val preferredWidth: Int = (screenSize.width * 0.8f).toInt()
    val preferredHeight: Int = (screenSize.height * 0.8f).toInt()
    val width: Int = if (desiredWidth < preferredWidth) desiredWidth else preferredWidth
    val height: Int = if (desiredHeight < preferredHeight) desiredHeight else preferredHeight
    return DpSize(width.dp, height.dp)
}
