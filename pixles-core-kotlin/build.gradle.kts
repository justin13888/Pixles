import org.jetbrains.kotlin.gradle.ExperimentalKotlinGradlePluginApi
import org.jetbrains.kotlin.gradle.dsl.JvmTarget
//import org.jetbrains.kotlin.gradle.swiftexport.ExperimentalSwiftExportDsl

plugins {
    alias(libs.plugins.kotlinMultiplatform)
    alias(libs.plugins.androidLibrary)
    alias(libs.plugins.composeCompiler)
    alias(libs.plugins.composeMultiplatform)
    alias(libs.plugins.kotlinxSerialization)
    alias(libs.plugins.ksp)
    alias(libs.plugins.kmpNativeCoroutines)
}

version = "0.1.0"

kotlin {
    androidTarget {
        @OptIn(ExperimentalKotlinGradlePluginApi::class)
        compilerOptions {
            jvmTarget.set(JvmTarget.JVM_17)
        }
    }

    jvm("desktop") {
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

    applyDefaultHierarchyTemplate()

//    @OptIn(ExperimentalSwiftExportDsl::class)
//    swiftExport {
//        // Root module name
//        moduleName = "shared"
//        flattenPackage = "com.justin13888.pixles"
//    }

    sourceSets {
        all {
            languageSettings.optIn("kotlin.uuid.ExperimentalUuidApi")
            languageSettings.optIn("org.jetbrains.compose.resources.ExperimentalResourceApi")
            
            // Required by KMM-ViewModel
            languageSettings.optIn("kotlinx.cinterop.ExperimentalForeignApi")
            languageSettings.optIn("kotlin.experimental.ExperimentalObjCName")
        }

        commonMain.dependencies {
            implementation(libs.kotlinx.datetime)
            implementation(libs.ktor.client.core)
            implementation(libs.ktor.client.content.negotiation)
            implementation(libs.ktor.serialization.kotlinx.json)
            implementation(libs.koin.core)
            api(libs.kmp.observable.viewmodel)

             implementation(compose.runtime)
             implementation(compose.foundation)
             implementation(compose.material)
             implementation(compose.components.resources)
             implementation("org.jetbrains.compose.material:material-icons-core:1.6.11")
             implementation(libs.kotlinx.serialization.core)
             implementation(libs.kotlinx.serialization.json)
             implementation(libs.kotlinx.datetime)
             implementation(libs.kotlinx.coroutines.core)
        }

        commonTest.dependencies {
            implementation(kotlin("test"))
        }

        androidMain.dependencies {
            implementation(libs.ktor.client.okhttp)

             api("androidx.activity:activity-compose:1.8.2")
             api("androidx.appcompat:appcompat:1.6.1")
             api("androidx.core:core-ktx:1.12.0")
             implementation("androidx.camera:camera-camera2:1.3.1")
             implementation("androidx.camera:camera-lifecycle:1.3.1")
             implementation("androidx.camera:camera-view:1.3.1")
             implementation("com.google.accompanist:accompanist-permissions:0.29.2-rc")
             implementation("com.google.android.gms:play-services-maps:18.2.0")
             implementation("com.google.android.gms:play-services-location:21.1.0")
             implementation("com.google.maps.android:maps-compose:2.11.2")
        }

        appleMain.dependencies {
            implementation(libs.ktor.client.darwin)
        }

        val desktopMain by getting
        desktopMain.dependencies {
            implementation(compose.desktop.common)
        }
        val desktopTest by getting
//        desktopTest.dependencies {
//            implementation(compose.desktop.currentOs)
//            implementation(compose.desktop.uiTestJUnit4)
//        }
    }
}

android {
    namespace = "com.justin13888.pixles.shared"
    compileSdk = 35
    sourceSets["main"].manifest.srcFile("src/androidMain/AndroidManifest.xml")
    sourceSets["main"].res.srcDirs("src/androidMain/res")

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    defaultConfig {
        minSdk = 26
    }
    kotlin {
        jvmToolchain(17)
    }
}
