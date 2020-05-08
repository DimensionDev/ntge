rootProject.name = "ntge-android" 
pluginManagement {
    repositories {
        gradlePluginPortal()
        jcenter()
        google()
    }
    resolutionStrategy {
        eachPlugin {
            if (requested.id.namespace == "com.android") {
                useModule("com.android.tools.build:gradle:${requested.version}")
            }
            if (requested.id.id == "androidx.benchmark") {
                useModule("androidx.benchmark:benchmark-gradle-plugin:${requested.version}")
            }
        }
    }
}
