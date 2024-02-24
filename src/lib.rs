#![doc = include_str!("../README.md")]
#![deny(unsafe_code, missing_docs)]
#![no_std]

#[allow(unused_imports)]
#[cfg(feature = "no-std")]
use micromath::F32Ext;
#[cfg(not(feature = "no-std"))]
extern crate std;

/// The temperature unit to use in the measurements.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum TemperatureUnit {
    #[default]
    /// Temperature in °C.
    Celcius,
    /// Temperature in °F.
    Farenheit,
}

/// The combination of the temperature and the relative humidity.
#[derive(Clone, Copy, Debug, Default)]
pub struct TemperatureAndRelativeHumidity {
    /// The relative humidity (in %).
    pub relative_humidity: f32,
    /// The temperature (either in °C or °F according to the temperature unit given in the
    /// [temperature_unit](TemperatureAndRelativeHumidity::temperature_unit) field).
    pub temperature: f32,
    /// The temperature unit used for the measurement.
    pub temperature_unit: TemperatureUnit,
}

/// The combination of the temperature and the barometric pressure.
#[derive(Clone, Copy, Debug, Default)]
pub struct TemperatureAndPressure {
    /// The barometric pressure (in hPa).
    pub pressure: f32,
    /// The temperature (either in °C or °F according to the temperature unit given in the
    /// [temperature_unit](TemperatureAndPressure::temperature_unit) field).
    pub temperature: f32,
    /// The temperature unit used for the measurement.
    pub temperature_unit: TemperatureUnit,
}

/// Computes the absolute humidity value (in g/m³), given the temperature and
/// the relative humidity.
pub fn compute_absolute_humidity(measurement: TemperatureAndRelativeHumidity) -> f32 {
    let temperature = match measurement.temperature_unit {
        TemperatureUnit::Celcius => measurement.temperature,
        TemperatureUnit::Farenheit => convert_farenheit_to_celcius(measurement.temperature),
    };
    (6.112
        * ((17.67 * temperature) / (temperature + 243.5)).exp()
        * measurement.relative_humidity
        * 2.1674)
        / (273.15 + temperature)
}

/// Compute the altitude (in m), given the barometric pressure and the
/// temperature.
pub fn compute_altitude(measurement: TemperatureAndPressure) -> f32 {
    let temperature = match measurement.temperature_unit {
        TemperatureUnit::Celcius => measurement.temperature,
        TemperatureUnit::Farenheit => convert_farenheit_to_celcius(measurement.temperature),
    };
    ((1_013.25 / measurement.pressure).powf(1.0 / 5.257) - 1.0) * (temperature + 273.15) / 0.0065
}

/// Converts a temperature in °C to °F.
pub fn convert_celcius_to_farenheit(temperature: f32) -> f32 {
    temperature * 1.8 + 32.0
}

/// Converts a temperature in °F to °C.
pub fn convert_farenheit_to_celcius(temperature: f32) -> f32 {
    (temperature - 32.0) * 0.55555
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn compute_absolute_humidity() {
        assert!(
            (crate::compute_absolute_humidity(TemperatureAndRelativeHumidity {
                relative_humidity: 45.59,
                temperature: 21.18,
                temperature_unit: TemperatureUnit::Celcius
            }) - 8.43)
                .abs()
                < 0.01
        );
        assert!(
            (crate::compute_absolute_humidity(TemperatureAndRelativeHumidity {
                relative_humidity: 45.59,
                temperature: 70.12,
                temperature_unit: TemperatureUnit::Farenheit
            }) - 8.43)
                .abs()
                < 0.01
        );
        assert!(
            (crate::compute_absolute_humidity(TemperatureAndRelativeHumidity {
                relative_humidity: 34.71,
                temperature: 2.93,
                temperature_unit: TemperatureUnit::Celcius
            }) - 2.06)
                .abs()
                < 0.01
        );
        assert!(
            (crate::compute_absolute_humidity(TemperatureAndRelativeHumidity {
                relative_humidity: 74.91,
                temperature: 107.7,
                temperature_unit: TemperatureUnit::Farenheit
            }) - 42.49)
                .abs()
                < 0.01
        );
    }

    #[test]
    fn compute_altitude() {
        assert!(
            (crate::compute_altitude(TemperatureAndPressure {
                pressure: 991.32,
                temperature: 20.55,
                temperature_unit: TemperatureUnit::Celcius
            }) - 188.46)
                .abs()
                < 0.01
        );
        assert!(
            (crate::compute_altitude(TemperatureAndPressure {
                pressure: 1013.25,
                temperature: 17.93,
                temperature_unit: TemperatureUnit::Celcius
            }) - 0.0)
                .abs()
                < 0.01
        );
        assert!(
            (crate::compute_altitude(TemperatureAndPressure {
                pressure: 1013.25,
                temperature: 37.5,
                temperature_unit: TemperatureUnit::Celcius
            }) - 0.0)
                .abs()
                < 0.01
        );
        assert!(
            (crate::compute_altitude(TemperatureAndPressure {
                pressure: 962.81,
                temperature: 19.37,
                temperature_unit: TemperatureUnit::Celcius
            }) - 439.25)
                .abs()
                < 0.01
        );
    }

    #[test]
    fn convert_celcius_to_farenheit() {
        assert!((crate::convert_celcius_to_farenheit(0.0) - 32.0).abs() < 0.01);
        assert!((crate::convert_celcius_to_farenheit(15.73) - 60.31).abs() < 0.01);
        assert!((crate::convert_celcius_to_farenheit(-7.49) - 18.52).abs() < 0.01);
        assert!((crate::convert_celcius_to_farenheit(37.5) - 99.5).abs() < 0.01);
    }

    #[test]
    fn convert_farenheit_to_celcius() {
        assert!((crate::convert_farenheit_to_celcius(32.0) - 0.0).abs() < 0.01);
        assert!((crate::convert_farenheit_to_celcius(60.31) - 15.73).abs() < 0.01);
        assert!((crate::convert_farenheit_to_celcius(18.52) - -7.49).abs() < 0.01);
        assert!((crate::convert_farenheit_to_celcius(99.5) - 37.5).abs() < 0.01);
    }
}
