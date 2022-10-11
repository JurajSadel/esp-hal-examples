## E-paper example with ESP32S3-DevKitC-1

## Hardware used

ESP chip: [ESP32S3-DevKitC-1](https://docs.espressif.com/projects/esp-idf/en/latest/esp32s3/hw-reference/esp32s3/user-guide-devkitc-1.html)

e-paper display: [296x128, 2.9inch E-Ink display module](https://www.waveshare.com/product/2.9inch-e-paper-module.htm)

### Used pins

| E-paper |  ESP32S3            |
----------|---------------------|
| CS      | GPIO10              |
| DC      | GPIO21              |
| SCK     | GPIO12              |
| RST     | GPIO18              |
| BUSY    | GPIO5               |
| MOSI    | GPIO11              |
| VCC     | 3V3                 |
| GND     | GND                 |

**Warning**
Currently, with [1.64.0.0](https://github.com/esp-rs/rust-build/releases/tag/v1.64.0.0) it is required to use `opt-level = 1` in release profile!