# mopro-ios

## Linker problems?

- Open Xcworkspace
- Pod > MoproKit target
- Build Settings
	- Library Search Paths
	- Other Linker Flags
	- Header search paths
- (Build Phases > Link Binary With Libraries `lmopro_ffi` (no lib no .a))
- Gitignore
