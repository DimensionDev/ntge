#
# Be sure to run `pod lib lint NtgeCore.podspec' to ensure this is a
# valid spec before submitting.
#
# Any lines starting with a # are optional, but their use is encouraged
# To learn more about a Podspec see https://guides.cocoapods.org/syntax/podspec.html
#

Pod::Spec.new do |s|
  s.name             = 'NtgeCore'
  s.version          = '0.1.0'
  s.summary          = 'A short description of NtgeCore.'

# This description is used to generate tags and improve search results.
#   * Think: What does it do? Why did you write it? What is the focus?
#   * Try to keep it short, snappy and to the point.
#   * Write the description between the DESC delimiters below.
#   * Finally, don't worry about the indent, CocoaPods strips it!

  s.description      = <<-DESC
TODO: Add long description of the pod here.
                       DESC

  s.homepage         = 'https://github.com/mainasuk/NtgeCore'
  # s.screenshots     = 'www.example.com/screenshots_1', 'www.example.com/screenshots_2'
  s.license          = { :type => 'MIT', :file => 'LICENSE' }
  s.author           = { 'mainasuk' => 'cirno.mainasuk@gmail.com' }
  s.source           = { :git => 'https://github.com/mainasuk/NtgeCore.git', :tag => s.version.to_s }
  # s.social_media_url = 'https://twitter.com/<TWITTER_USERNAME>'

  s.ios.deployment_target = '8.0'

  s.prepare_command = <<-CMD
    BASEPATH="${PWD}"
    echo "Building rust library..."
    cd ../ntge-core
    cargo build --target x86_64-apple-ios --release --lib
    RUSTFLAGS="-Z embed-bitcode" cargo +ios-arm64 build --target aarch64-apple-ios --release --lib
    echo "lipo bitcode lib"
    cd ..
    echo $(PWD)
    lipo -create target/aarch64-apple-ios/release/libntge_core.a target/x86_64-apple-ios/release/libntge_core.a -output target/libntge_core.a
    cd "${BASEPATH}"
    mkdir -p lib
    cp ../target/*.a lib/
    rm -rf NtgeCore/Classes/
    mkdir -p NtgeCore/Classes/include
    cp -r ../ntge-core/include NtgeCore/Classes/
    touch NtgeCore/Classes/dummy.c
  CMD

  s.pod_target_xcconfig = {
    'SWIFT_INCLUDE_PATHS' => '${PODS_ROOT}',
    'HEADER_SEARCH_PATHS' => '"${PODS_ROOT}/NtgeCore/Classes"',
  }

  s.source_files = 'NtgeCore/Classes/**/*'
  s.public_header_files = 'NtgeCore/Classes/include/*.h'
  s.vendored_libraries = "lib/libntge_core.a"
  
  s.static_framework = true
  
  # s.resource_bundles = {
  #   'NtgeCore' => ['NtgeCore/Assets/*.png']
  # }

  # s.public_header_files = 'Pod/Classes/**/*.h'
  # s.frameworks = 'UIKit', 'MapKit'
  # s.dependency 'AFNetworking', '~> 2.3'
end
