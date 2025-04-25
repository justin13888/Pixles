rootProject.name = "Pixles"
enableFeaturePreview("TYPESAFE_PROJECT_ACCESSORS")

pluginManagement {
    repositories {
        google {
            mavenContent {
                includeGroupAndSubgroups("androidx")
                includeGroupAndSubgroups("com.android")
                includeGroupAndSubgroups("com.google")
            }
        }
        mavenCentral()
        gradlePluginPortal()
    }
}

dependencyResolutionManagement {
    repositories {
        google {
            mavenContent {
                includeGroupAndSubgroups("androidx")
                includeGroupAndSubgroups("com.android")
                includeGroupAndSubgroups("com.google")
            }
        }
        mavenCentral()
    }
}


include(":android")
project(":android").projectDir = file("pixles-android")
//include(":pixles-core-kotlin")
include(":core")
project(":core").projectDir = file("pixles-core-kotlin")
include(":cli")
project(":cli").projectDir = file("pixles-cli")
