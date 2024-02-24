#![doc = include_str!("../README.md")]
#![deny(unsafe_code, missing_docs)]
#![no_std]

#[allow(unused_imports)]
#[cfg(feature = "no-std")]
use micromath::F32Ext;
#[cfg(not(feature = "no-std"))]
extern crate std;

/// Trait defining the different ways to get a temperature.
pub trait TemperatureUnit {
    /// Get the temperature in degrees Celcius (°C).
    fn celcius(&self) -> f32;
    /// Get the temperature in degrees Farenheit (°F).
    fn farenheit(&self) -> f32;
}

/// The degrees Celcius temperature unit.
pub struct Celcius {
    value: f32,
}

impl TemperatureUnit for Celcius {
    fn celcius(&self) -> f32 {
        self.value
    }

    fn farenheit(&self) -> f32 {
        convert_celcius_to_farenheit(self.value)
    }
}

/// The degrees Farenheit temperature unit.
pub struct Farenheit {
    value: f32,
}

impl TemperatureUnit for Farenheit {
    fn celcius(&self) -> f32 {
        convert_farenheit_to_celcius(self.value)
    }

    fn farenheit(&self) -> f32 {
        self.value
    }
}

/// The temperature (either in °C, or in °F).
#[derive(Clone, Copy, Debug, Default)]
pub struct Temperature<U: TemperatureUnit> {
    value: U,
}

impl<U: TemperatureUnit> Temperature<U> {
    /// Get the temperature value in degrees Celcius (°C).
    pub fn celcius(&self) -> f32 {
        self.value.celcius()
    }

    /// Get the temperature value in degrees Farenheit (°F).
    pub fn farenheit(&self) -> f32 {
        self.value.farenheit()
    }
}

impl Temperature<Celcius> {
    /// Create a Celcius temperature.
    pub fn new(value: f32) -> Temperature<Celcius> {
        Temperature {
            value: Celcius { value },
        }
    }
}

impl Temperature<Farenheit> {
    /// Create a Farenheit temperature.
    pub fn new(value: f32) -> Temperature<Farenheit> {
        Temperature {
            value: Farenheit { value },
        }
    }
}

/// The relative humidity type (in %).
pub type RelativeHumidity = f32;

/// The absolute humidity type (in g/m³).
pub type AbsoluteHumidity = f32;

/// The barometric pressure type (in hPa).
pub type BarometricPressure = f32;

/// The altitude type (in m).
pub type Altitude = f32;

/// The combination of the temperature and the relative humidity.
#[derive(Clone, Copy, Debug, Default)]
pub struct TemperatureAndRelativeHumidity<U: TemperatureUnit> {
    /// The relative humidity (in %).
    pub relative_humidity: RelativeHumidity,
    /// The temperature (either in °C or °F).
    pub temperature: Temperature<U>,
}

/// The combination of the temperature and the barometric pressure.
#[derive(Clone, Copy, Debug, Default)]
pub struct TemperatureAndPressure<U: TemperatureUnit> {
    /// The barometric pressure (in hPa).
    pub pressure: BarometricPressure,
    /// The temperature (either in °C or °F).
    pub temperature: Temperature<U>,
}

/// Computes the absolute humidity value (in g/m³), given the temperature and
/// the relative humidity.
pub fn compute_absolute_humidity<U: TemperatureUnit>(
    measurement: TemperatureAndRelativeHumidity<U>,
) -> AbsoluteHumidity {
    let temperature = measurement.temperature.celcius();
    (6.112
        * ((17.67 * temperature) / (temperature + 243.5)).exp()
        * measurement.relative_humidity
        * 2.1674)
        / (273.15 + temperature)
}

/// Compute the altitude (in m), given the barometric pressure and the
/// temperature.
pub fn compute_altitude<U: TemperatureUnit>(measurement: TemperatureAndPressure<U>) -> Altitude {
    let temperature = measurement.temperature.celcius();
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
                temperature: Temperature::<Celcius>::new(21.18)
            }) - 8.43)
                .abs()
                < 0.01
        );
        assert!(
            (crate::compute_absolute_humidity(TemperatureAndRelativeHumidity {
                relative_humidity: 45.59,
                temperature: Temperature::<Farenheit>::new(70.12)
            }) - 8.43)
                .abs()
                < 0.01
        );
        assert!(
            (crate::compute_absolute_humidity(TemperatureAndRelativeHumidity {
                relative_humidity: 34.71,
                temperature: Temperature::<Celcius>::new(2.93)
            }) - 2.06)
                .abs()
                < 0.01
        );
        assert!(
            (crate::compute_absolute_humidity(TemperatureAndRelativeHumidity {
                relative_humidity: 74.91,
                temperature: Temperature::<Farenheit>::new(107.7)
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
                temperature: Temperature::<Celcius>::new(20.55)
            }) - 188.46)
                .abs()
                < 0.01
        );
        assert!(
            (crate::compute_altitude(TemperatureAndPressure {
                pressure: 1013.25,
                temperature: Temperature::<Celcius>::new(17.93)
            }) - 0.0)
                .abs()
                < 0.01
        );
        assert!(
            (crate::compute_altitude(TemperatureAndPressure {
                pressure: 1013.25,
                temperature: Temperature::<Celcius>::new(37.5)
            }) - 0.0)
                .abs()
                < 0.01
        );
        assert!(
            (crate::compute_altitude(TemperatureAndPressure {
                pressure: 962.81,
                temperature: Temperature::<Celcius>::new(19.37)
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
