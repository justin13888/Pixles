package com.justin13888.pixles

import android.app.Application
import com.justin13888.pixles.di.initKoin
import com.justin13888.pixles.screens.DetailViewModel
import com.justin13888.pixles.screens.ListViewModel
import org.koin.dsl.module

class PixlesApp : Application() {
    override fun onCreate() {
        super.onCreate()
        initKoin(
            listOf(
                module {
                    factory { ListViewModel(get()) }
                    factory { DetailViewModel(get()) }
                }
            )
        )
    }
}
