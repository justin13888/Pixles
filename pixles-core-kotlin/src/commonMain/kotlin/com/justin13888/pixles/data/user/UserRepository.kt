package com.justin13888.pixles.data.user

import com.justin13888.pixles.data.MuseumApi
import com.justin13888.pixles.data.MuseumObject
import com.justin13888.pixles.data.MuseumStorage

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.launch

// TODO: Do some sort of optimized, native caching
class UserRepository(
//    private val museumStorage: MuseumStorage,
) {
//    private val scope = CoroutineScope(SupervisorJob())

    fun initialize() {
//        scope.launch {
//            refresh()
//        }
    }

//    suspend fun refresh() {
//        museumStorage.saveObjects(museumApi.getData())
//    }

    // TODO
    suspend fun getContext(): UserContext? = UserContext("123", null)

    // TODO
    fun getContextBlocking(): UserContext? = UserContext("123", null)
}