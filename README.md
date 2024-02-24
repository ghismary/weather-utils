[![crates.io](https://img.shields.io/crates/v/weather-utils.svg)](https://crates.io/crates/weather-utils)
[![License](https://img.shields.io/crates/l/weather-utils.svg)](https://crates.io/crates/weather-utils)
[![Documentation](https://docs.rs/weather-utils/badge.svg)](https://docs.rs/weather-utils)

# weather-utils

This crate includes a set of common and useful weather-related computations,
such as the temperature unit conversions, computations of humidity/temperature
related values (absolute humidity, dew point, heat index), computations of
barometric pressure related values (altitude)...

If a computation you need is not present, please feel free to ask for it, or
even better contribute it ;-)

## Features

- [x] Conversion from °C to °F.
- [x] Conversion from °F to °C.
- [x] Computation of absolute humidity from temperature and relative humidity.
- [x] Computation of altitude from barometric pressure and temperature.
- [ ] Computation of dew point.
- [ ] Computation of heat index.

### Documentation:

- [Introduction to humidity](https://www.sensirion.com/media/documents/8AB2AD38/61642ADD/Sensirion_AppNotes_Humidity_Sensors_Introduction_to_Relative_Humidit.pdf)
- [Humidity at a glance](https://www.sensirion.com/media/documents/A419127A/6163F5FE/Sensirion_AppNotes_Humidity_Sensors_at_a_Glance.pdf)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
