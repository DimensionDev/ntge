import com.android.build.gradle.internal.tasks.factory.dependsOn

val kotlinVersion = "1.3.72"

plugins {
    id("com.android.library") version "3.6.3"
    id("androidx.benchmark") version "1.0.0"
    kotlin("android") version "1.3.72"
    id("org.mozilla.rust-android-gradle.rust-android") version "0.8.3"
    id("digital.wup.android-maven-publish") version "3.6.2"
}

android {
    compileSdkVersion(29)
    ndkVersion = "21.2.6472646"
    defaultConfig {
        minSdkVersion(21)
        targetSdkVersion(29)
        versionCode = 1
        versionName = getConfiguration("versionName", "0.0.0")
        testInstrumentationRunner = "androidx.benchmark.junit4.AndroidBenchmarkRunner"
        testInstrumentationRunnerArgument(
                "androidx.benchmark.suppressErrors",
                listOf(
                        "EMULATOR",
                        "UNLOCKED"
                ).joinToString(",")
        )
    }
    testBuildType = "release"
    buildTypes {
        getByName("release") {
            isDefault.set(true)
            isMinifyEnabled = true
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
            signingConfig = signingConfigs.getByName("debug")
        }
    }
}

tasks.withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
    kotlinOptions.jvmTarget = "1.8"
}

cargo {
    module = "./src/main/rust"
    targets = listOf("arm", "x86", "arm64", "x86_64")
    libname = "ntgedroid"
    profile = "release"
}

repositories {
    google()
    jcenter()
    mavenCentral()
    maven("https://jitpack.io")
}

dependencies {
    implementation(kotlin("stdlib-jdk8", kotlinVersion))
    testImplementation("junit:junit:4.13")
    androidTestImplementation("androidx.benchmark:benchmark-junit4:1.0.0")
    androidTestImplementation("androidx.test.ext:junit:1.1.1")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.2.0")
}

tasks.register<Exec>("cargoClean") {
    executable("cargo")
    args("clean")
    workingDir("$projectDir/${cargo.module}")
}
tasks.preBuild.dependsOn("cargoBuild")
tasks.clean.dependsOn("cargoClean")

publishing {
    repositories {
        maven {
            name = "Github"
            url = uri(getConfiguration("source", ""))
            credentials {
                username = getConfiguration("user", "")
                password = getConfiguration("token", "")
            }
        }
    }
    publications {
        create<MavenPublication>("mavenAar") {
            groupId = "com.dimension"
            artifactId = "ntge"
            version = getConfiguration("versionName", "0.0.0")
            from(components["android"])
        }
    }
}

inline fun <reified T : Any> Project.getConfiguration(name: String, defaultValue: T): T {
    return (if (project.hasProperty(name)) {
        val property = project.property(name)
        if (property == null) {
            defaultValue
        } else {
            when (defaultValue) {
                is String -> property
                is Boolean -> property.toString().toBoolean()
                is Int -> property.toString().toInt()
                is Double -> property.toString().toDouble()
                else -> property
            }
        }
    } else {
        defaultValue
    }) as T
}