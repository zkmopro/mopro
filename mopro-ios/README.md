# mopro-ios

## Prepare

<!--TODO: If the monorepo is seperated, update this-->

Check the [Prepare](../README.md#prepare) and [Build Bindings](../README.md#build-bindings) steps in the root directory.

## Execute

Open the `MoproKit/Example/MoproKit.xcworkspace` in Xcode.
Use `command`+`R` to execute a simulator.

## Linker problems?

Add the following settings after

1. `MoproKit/Example/Pods/Target Support Files/MoproKit/MoproKit.debug.xcconfig`
2. `MoproKit/Example/Pods/Target Support Files/MoproKit/MoproKit.release.xcconfig`

files

```
LIBRARY_SEARCH_PATHS=${SRCROOT}/../../Libs
OTHER_LDFLAGS=-lmopro_ffi
USER_HEADER_SEARCH_PATHS=${SRCROOT}/../../include
```
