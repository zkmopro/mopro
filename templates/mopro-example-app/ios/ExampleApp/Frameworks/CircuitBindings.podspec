Pod::Spec.new do |spec|
    spec.name         = 'CircuitBindings'
    spec.version      = '0.1.0'
    spec.summary      = 'CircuitBindings XCFramework'
    spec.homepage     = 'https://github.com/oskarth/mopro'
    spec.license      = { :type => 'MIT/Apache-2.0', :file => 'LICENSE' }
    spec.author       = { 'Mopro' => 'mopro@dev.null' }
    spec.platform     = :ios, '13.0'
    spec.source       = { :path => 'CircuitBindings.xcframework' }
    spec.vendored_frameworks = 'CircuitBindings.xcframework'
  end
  