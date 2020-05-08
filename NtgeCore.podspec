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
  s.summary          = 'Not That Good Encryption is a general-purpose rust-based encryption tool.'

  s.description      = <<-DESC
Not That Good Encryption is a general-purpose rust-based encryption tool. Our main goal is to learn rust in a project-based way for a real on-hand practice for the team. We are also willing to integrate this tool/lib to our new encryption input method mobile app. If our implementation works well enough, we will definitely give it a try.
                       DESC

  s.homepage         = 'https://github.com/DimensionDev/ntge'
  s.license          = { :type => 'MIT', :file => 'LICENSE' }
  s.author           = { 'mainasuk' => 'mainasuk@sujitech.com' }
  s.source           = { :git => 'https://github.com/DimensionDev/ntge.git', :tag => s.version.to_s }
  s.swift_versions   = '5.0'

  s.ios.deployment_target = '13.0'

  s.prepare_command = <<-CMD
    ./NtgeCore-iOS/build.sh
  CMD

  s.pod_target_xcconfig = {
    'SWIFT_INCLUDE_PATHS' => '${PODS_ROOT}',
    'HEADER_SEARCH_PATHS' => '"${PODS_ROOT}/NtgeCore/Classes"',
  }

  s.source_files = 'NtgeCore-iOS/NtgeCore/Classes/**/*'
  s.public_header_files = 'NtgeCore-iOS/NtgeCore/Classes/include/*.h'
  s.vendored_libraries = "NtgeCore-iOS/lib/libntge_core.a"

  s.test_spec 'Tests' do |test_spec|
    test_spec.source_files = 'NtgeCore-iOS/NtgeCore/Tests/**'
  end
  
  s.static_framework = true
  
  end
