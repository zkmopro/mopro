# Notes

This is the latest version of the AA circuits with in total 1,767,153 contraints.

### Setup

```
./scripts/prepare.sh
```

### Build iOS Bindgins

To build bindings for iOS, adjust settings in your config file (we recommend starting with `simulator` and `release`) and run:

```sh
./scripts/build_ios.sh anon-aadhaar-config.toml
```

#### Update Bindings

To update bindings, run

```sh
./scripts/update_bindings.sh anon-aadhaar-config.toml
```
