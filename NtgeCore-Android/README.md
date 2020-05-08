[![Android](https://github.com/DimensionDev/ntge/workflows/Android/badge.svg)](https://github.com/DimensionDev/ntge/actions?query=workflow%3AAndroid)

Android wrapper for Ntge

# Getting Start
Add it in your root build.gradle at the end of repositories:
```
maven {
    name = "Github"
    url = uri("https://maven.pkg.github.com/DimensionDev/Ntge")
    credentials {
        username = <Your-Github-UserName>
        password = <Your-Github-Token>
    }
}
```
Add the dependency
```
implementation "com.github.DimensionDev:ntge:<latest-version>"
```

# Develop
## Requirement
- JDK 1.8
- Android SDK
  - Android NDK
- Rust with Android targets installed  
```$ rustup target install armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android```

## Build
```
./gradlew build
```

# Author

Tlaster, tlaster@outlook.com

# License

Ntge is available under the MIT license. See the LICENSE file for more info.
