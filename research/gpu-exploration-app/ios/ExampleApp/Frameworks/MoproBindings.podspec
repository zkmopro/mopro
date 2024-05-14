Pod::Spec.new do |spec|
  spec.name         = 'MoproBindings'
  spec.version      = '0.1.0'
  spec.summary      = 'MoproBindings XCFramework'
  spec.homepage     = 'https://github.com/oskarth/mopro'
  spec.license      = { :type => 'MIT/Apache-2.0', :file => 'LICENSE' }
  spec.author       = { 'Mopro' => 'mopro@dev.null' }
  spec.platform     = :ios, '13.0'
  spec.source       = { :path => 'MoproBindings.xcframework' }
  spec.vendored_frameworks = 'MoproBindings.xcframework'
end
