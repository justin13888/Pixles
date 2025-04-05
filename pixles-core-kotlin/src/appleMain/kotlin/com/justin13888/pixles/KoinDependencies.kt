package com.justin13888.pixles

import com.justin13888.pixles.data.MuseumRepository
import com.justin13888.pixles.data.user.UserRepository
import org.koin.core.component.KoinComponent
import org.koin.core.component.inject

class KoinDependencies : KoinComponent {
    val userRepository: UserRepository by inject()
    val museumRepository: MuseumRepository by inject()
}
