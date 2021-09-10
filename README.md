# PSP Test App
Simple test app based on [rust-psp](https://github.com/overdrivenpotato/rust-psp). Demonstrating the usage of C libs.

## Build
Download and unzip the [prebuilt PSPSDK](https://github.com/doodlewind/psp-test-app/releases/download/sdk/mipsel-sony-psp.zip) (built from [clang-psp](https://github.com/pspdev/clang-psp)) into the project root, then build the app:

``` sh
cd app
./build.sh
```

The output app file is `app/target/mipsel-sony-psp/debug/EBOOT.PBP`.

## License
Public domain
