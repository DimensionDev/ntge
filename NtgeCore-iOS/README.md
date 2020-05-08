# NtgeCore

[![iOS](https://github.com/DimensionDev/ntge/workflows/iOS/badge.svg)](https://github.com/DimensionDev/ntge/actions?query=workflow%3AiOS)
[![Version](https://img.shields.io/cocoapods/v/NtgeCore.svg?style=flat)](https://cocoapods.org/pods/NtgeCore)
[![License](https://img.shields.io/cocoapods/l/NtgeCore.svg?style=flat)](https://cocoapods.org/pods/NtgeCore)
[![Platform](https://img.shields.io/cocoapods/p/NtgeCore.svg?style=flat)](https://cocoapods.org/pods/NtgeCore)

## Example

To run the example project, clone the repo, and run `pod install` from the Example directory first.

## Requirements
- iOS 13.0+
- Xcode 11.4+

## Setup
Before use this pod. Install the [rust-bitcode](https://github.com/getditto/rust-bitcode) 

```zsh
$ brew install rustup
$ rustup-init
$ rustup target add aarch64-apple-ios x86_64-apple-ios

$ cargo install cargo-lipo
$ cargo install --force cbindgen

$ wget https://github.com/getditto/rust-bitcode/releases/download/v1.40.0/rust-ios-arm64-1.40.0.zip
$ unzip rust-ios-arm64-1.40.0.zip
$ cd rust-ios-arm64-1.40.0
$ ./install.sh
```

## Installation

NtgeCore is available through [CocoaPods](https://cocoapods.org). To install
it, simply add the following line to your Podfile:

```ruby
pod 'NtgeCore', '~> 0.1.0'
```

## Maintains

### CocoaPods
Bootstrap the project and run it. 

```bash
$ cd ./NtgeCore-iOS/Example

# sudo gem install cocoapods-clean
$ pod clean
$ pod install --verbose
$ open NtgeCore.xcworkspace
```

### Rebuild Pod
Change Rust code and rebuild Pod painless.

```bash
$ pwd
> ~/<path>/ntge

$ ./NtgeCore-iOS/build.sh
```

### cbindgen
Generate C header manually. Resolve the issue in the header generation process here.

```bash
$ cd ./ntge-core
$ touch build.rs
$ cargo build --features cbindgen-enable
```


## Author

- @MainasuK mainasuk@sujitech.com

## License

NtgeCore is available under the MIT license. See the LICENSE file for more info.
