# esp-hal-examples

Various examples demonstrating the use of [esp-hal](https://github.com/esp-rs/esp-hal).

## Contributing

If you would like to contribute an example to this repository, we ask that you please follow these conventions:

- Packages should be prefixed with the device they are targeting, followed by the name of the example, using underscores as separators
  - eg.) An example for the ESP32-C3 called **My Example** would be `esp32c3_my_example`
- Each example should be its own binary package placed within the root of this repository
  - eg.) `cargo new --bin esp32c3_my_example`
- The example packages may optionally include a `README` (left up to the discretion of the contributor), however must include all other configuration files required to build the example

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
