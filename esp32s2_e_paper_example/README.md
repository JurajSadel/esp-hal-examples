## E-paper example with ESP32S2-DevKitC-1

## Hardware used

ESP chip: [ESP32S2-Saola-1](https://docs.espressif.com/projects/esp-idf/en/latest/esp32s2/hw-reference/esp32s2/user-guide-saola-1-v1.2.html)

e-paper display: [296x128, 2.9inch E-Ink display module](https://www.waveshare.com/product/2.9inch-e-paper-module.htm)

### Used pins

| E-paper |  ESP32S2            |
----------|---------------------|
| CS      | GPIO34              |
| DC      | GPIO37              |
| SCK     | GPIO36              |
| RST     | GPIO38              |
| BUSY    | GPIO5               |
| MOSI    | GPIO35              |
| VCC     | 3V3                 |
| GND     | GND                 |

**Warning**
Currently, with [1.64.0.0](https://github.com/esp-rs/rust-build/releases/tag/v1.64.0.0) it is required to use `opt-level = 1` in release profile!