# Flutter Setup

After completing the [Rust setup](rust-setup.md) and setting up either [iOS setup](ios-setup.md) or [Android setup](android-setup.md), you're ready to create a cross-platform project using [Flutter](https://flutter.dev/). <br/>
Flutter is a framework for building natively compiled, multi-platform applications from a single codebase.

## Flutter plugin example

You can now clone the repository from [zkmopro/flutter-app](https://github.com/zkmopro/flutter-app) with

```sh
git clone https://github.com/zkmopro/flutter-app
```

Once cloned, follow the instructions in the README file to run the example app for the plugin.

<div style={{ display: 'flex', justifyContent: 'center', gap: '10px' }}>
    <img src="/img/flutter-android.png" alt="First Image" width="250"/>
    <img src="/img/flutter-ios.png" alt="Second Image" width="250"/>
</div>

## Use the flutter plugin

To use the plugin, update your `pubspec.yaml` file by adding the following dependency:

```yml
dependencies:
  mopro_flutter:
    # When depending on this package from a real application you should use:
    #   mopro_flutter: ^x.y.z
    # See https://dart.dev/tools/pub/dependencies#version-constraints
    # The example app is bundled with the plugin so we use a path dependency on
    # the parent directory to use the current plugin's version.
    path: <THE_PATH_TO_MOPRO_PLUGIN>
    # e.g. path: ../flutter-app
```