import org.jetbrains.kotlin.gradle.internal.ensureParentDirsCreated

val kotlinVersion = "1.3.71"

repositories {
    google()
    jcenter()
    mavenCentral()
    maven("https://jitpack.io")
}

plugins {
    id("com.android.library") version "3.6.1"
    kotlin("android") version "1.3.71"
    id("org.mozilla.rust-android-gradle.rust-android") version "0.8.3"
}

android {
    compileSdkVersion(29)
    defaultConfig {
        minSdkVersion(21)
        targetSdkVersion(29)
        versionCode = 1
        versionName = "1.0"
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    buildTypes {
        getByName("release") {
            isMinifyEnabled = false
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
        }
    }
}

cargo {
    module = "../ntge-core"
    targets = listOf("arm", "x86", "arm64", "x86_64")
    libname = "ntge-core"
    profile = "release"
}

dependencies {
    implementation(kotlin("stdlib-jdk8", kotlinVersion))
    testImplementation("junit:junit:4.12")
    androidTestImplementation("androidx.test.ext:junit:1.1.1")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.2.0")
}

afterEvaluate {
    // The `cargoBuild` task isn't available until after evaluation.
    task("copyNtgeNativeLib") {
        val libName = "libntge_core.so"
        val target = mapOf(
                "aarch64-linux-android" to "arm64-v8a",
                "armv7-linux-androideabi" to "armeabi-v7a",
                "i686-linux-android" to "x86",
                "x86_64-linux-android" to "x86_64"
        )
        target.forEach {
            val targetFile = File("${projectDir.absolutePath}/src/main/jniLibs/${it.value}/${libName}")
            targetFile.ensureParentDirsCreated()
            if (!targetFile.exists()) {
                targetFile.createNewFile()
            }
            File(projectDir.parentFile, "target/${it.key}/release/${libName}").copyTo(targetFile, overwrite = true)
        }
    }.dependsOn(tasks["cargoBuild"])

    android.libraryVariants.forEach { variant ->
        var productFlavor = ""
        variant.productFlavors.forEach {
            productFlavor += it.name.capitalize()
        }
        val buildType = variant.buildType.name.capitalize()
        tasks["generate${productFlavor}${buildType}Assets"].dependsOn(tasks["copyNtgeNativeLib"])
    }
}