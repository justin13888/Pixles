plugins {
    alias(libs.plugins.kotlinMultiplatform)
    alias(libs.plugins.kotlinxSerialization)
    alias(libs.plugins.ksp)
}

group = "com.justin13888.pixles.cli"
version = "0.1.0"

kotlin {
    val hostOs = System.getProperty("os.name")
    val isArm64 = System.getProperty("os.arch") == "aarch64"
    val isMingwX64 = hostOs.startsWith("Windows")
    val nativeTarget = when {
        hostOs == "Mac OS X" && isArm64 -> macosArm64("native")
        hostOs == "Mac OS X" && !isArm64 -> macosX64("native")
        hostOs == "Linux" && isArm64 -> linuxArm64("native")
        hostOs == "Linux" && !isArm64 -> linuxX64("native")
        isMingwX64 -> mingwX64("native")
        else -> throw GradleException("Host OS is not supported in Kotlin/Native.")
    }

    nativeTarget.apply {
        binaries {
            executable {
                entryPoint = "com.justin13888.pixles.cli.main"
                baseName = "pixles-cli"
                freeCompilerArgs
            }
        }
    }

//    val macosTargets = listOf(
//        macosX64("macosx64"),
//        macosArm64("macosarm64"),
//    )
//    val linuxTargets = listOf(
//        linuxArm64("linuxarm64"),
//        linuxX64("linuxx64"),
//    )
//    val windowsTargets = listOf(
//        mingwX64("windowsx64")
//    )

//    (macosTargets + linuxTargets + windowsTargets).forEach { target ->
//        target.binaries.executable {
//            entryPoint = "com.justin13888.pixles.cli.main"
//        }
//    }

    sourceSets {
        nativeMain.dependencies {
            implementation(libs.clikt)
        }
//        appleMain.dependencies {
//            implementation(libs.ktor.client.darwin)
//        }
//        commonMain.dependencies {
//            implementation(libs.kotlinx.datetime)
//            implementation(libs.ktor.client.core)
//            implementation(libs.ktor.client.content.negotiation)
//            implementation(libs.ktor.serialization.kotlinx.json)
//            // implementation(libs.koin.core)
//        }
//        commonTest.dependencies {
//            implementation(kotlin("test"))
//        }
    }
}
