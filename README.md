## Starting a new project with `esp32-camera` bindings

First setup `espup` and the toolchain, and be sure to set env vars before compiling:
`. ~/export-esp.sh`

```sh
git https://github.com/esp-rs/esp-idf-template cargo
cd <project_dir>
git clone git@github.com:espressif/esp32-camera.git
touch esp32-camera-bindings.h
echo '#include "esp32-camera/driver/include/esp_camera.h"' > esp32-camera-bindings.h
echo 'CONFIG_ESP32_SPIRAM_SUPPORT=y' >> sdkconfig.defaults
```

Edit `Cargo.toml` and set:

```toml
[package.metadata.esp-idf-sys]
extra_components = [
  { component_dirs = "./esp32-camera", bindings_header = "esp32-camera-bindings.h" }
]
```

At the time of writing, setting `bindings_module` was triggering compile errors related to `c_types`.
