import org.jetbrains.kotlin.gradle.ExperimentalKotlinGradlePluginApi
import org.jetbrains.kotlin.gradle.dsl.JvmTarget
//import org.jetbrains.kotlin.gradle.swiftexport.ExperimentalSwiftExportDsl

plugins {
    alias(libs.plugins.kotlinMultiplatform)
    alias(libs.plugins.androidLibrary)
    alias(libs.plugins.kotlinxSerialization)
    alias(libs.plugins.ksp)
    alias(libs.plugins.kmpNativeCoroutines)
}

kotlin {
    androidTarget {
        @OptIn(ExperimentalKotlinGradlePluginApi::class)
        compilerOptions {
            jvmTarget.set(JvmTarget.JVM_11)
            optIn.set(listOf(
                "kotlin.uuid.ExperimentalUuidApi",
            ))
        }
    }

    jvm {
        testRuns["test"].executionTask.configure {
            useJUnitPlatform()
        }
    }

    val macosTargets = listOf(
        macosX64(),
        macosArm64()
    )
    // linuxArm64("native") // on Linux
    // mingwX64("native")   // on Windows

    val iosTargets = listOf(
        iosX64(),
        iosArm64(),
        iosSimulatorArm64()
    )

    (iosTargets + macosTargets).forEach { target ->
        target.binaries.framework {
            baseName = "Shared"
            isStatic = true
        }
    }

//    @OptIn(ExperimentalSwiftExportDsl::class)
//    swiftExport {
//        // Root module name
//        moduleName = "shared"
//        flattenPackage = "com.justin13888.pixles"
//    }

    sourceSets {
        androidMain.dependencies {
            implementation(libs.ktor.client.okhttp)
        }
        appleMain.dependencies {
            implementation(libs.ktor.client.darwin)
        }
        commonMain.dependencies {
            implementation(libs.kotlinx.datetime)
            implementation(libs.ktor.client.core)
            implementation(libs.ktor.client.content.negotiation)
            implementation(libs.ktor.serialization.kotlinx.json)
            implementation(libs.koin.core)
            api(libs.kmp.observable.viewmodel)
        }
        commonTest.dependencies {
            implementation(kotlin("test"))
        }


        // Required by KMM-ViewModel
        all {
            languageSettings.optIn("kotlinx.cinterop.ExperimentalForeignApi")
            languageSettings.optIn("kotlin.experimental.ExperimentalObjCName")
        }
    }
}

android {
    namespace = "com.justin13888.pixles.shared"
    compileSdk = 35
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }
    defaultConfig {
        minSdk = 24
    }
}
