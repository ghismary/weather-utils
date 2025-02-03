#![doc = include_str!("../README.md")]
#![deny(unsafe_code, missing_docs)]
#![no_std]

use approx::relative_eq;
#[allow(unused_imports)]
#[cfg(feature = "no-std")]
use micromath::F32Ext;
#[cfg(not(feature = "no-std"))]
extern crate std;

/// The units for the measures.
pub mod unit;

/// The temperature (either in °C, or in °F).
#[derive(Clone, Copy, Debug, Default)]
pub struct Temperature<U: unit::TemperatureUnit> {
    pub(crate) value: U,
}

impl<U: unit::TemperatureUnit> Temperature<U> {
    /// Get the temperature value in degrees Celsius (°C).
    pub fn celsius(&self) -> f32 {
        self.value.celsius()
    }

    /// Get the temperature value in degrees Fahrenheit (°F).
    pub fn fahrenheit(&self) -> f32 {
        self.value.fahrenheit()
    }
}

impl<U: unit::TemperatureUnit> PartialEq for Temperature<U> {
    fn eq(&self, other: &Self) -> bool {
        relative_eq!(self.celsius(), &other.celsius(), epsilon = 0.01)
    }
}

impl Temperature<unit::Celsius> {
    /// Create a Celsius temperature.
    pub fn new(value: f32) -> Temperature<unit::Celsius> {
        Temperature {
            value: unit::Celsius(value),
        }
    }
}

impl From<Temperature<unit::Fahrenheit>> for Temperature<unit::Celsius> {
    fn from(value: Temperature<unit::Fahrenheit>) -> Self {
        Self {
            value: value.value.into(),
        }
    }
}

impl Temperature<unit::Fahrenheit> {
    /// Create a Fahrenheit temperature.
    pub fn new(value: f32) -> Temperature<unit::Fahrenheit> {
        Temperature {
            value: unit::Fahrenheit(value),
        }
    }
}

impl From<Temperature<unit::Celsius>> for Temperature<unit::Fahrenheit> {
    fn from(value: Temperature<unit::Celsius>) -> Self {
        Self {
            value: value.value.into(),
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
pub struct TemperatureAndRelativeHumidity<U: unit::TemperatureUnit> {
    /// The relative humidity (in %).
    pub relative_humidity: RelativeHumidity,
    /// The temperature (either in °C or °F).
    pub temperature: Temperature<U>,
}

fn calculate_absolute_humidity(temperature: f32, relative_humidity: f32) -> f32 {
    (6.112 * ((17.67 * temperature) / (temperature + 243.5)).exp() * relative_humidity * 2.1674)
        / (273.15 + temperature)
}

impl<U: unit::TemperatureUnit> TemperatureAndRelativeHumidity<U> {
    /// Computes the absolute humidity value (in g/m³).
    pub fn absolute_humidity(&self) -> AbsoluteHumidity {
        calculate_absolute_humidity(self.temperature.celsius(), self.relative_humidity)
    }
}

impl<U: unit::TemperatureUnit> PartialEq for TemperatureAndRelativeHumidity<U> {
    fn eq(&self, other: &Self) -> bool {
        relative_eq!(self.relative_humidity, other.relative_humidity)
            && self.temperature.eq(&other.temperature)
    }
}

impl TemperatureAndRelativeHumidity<unit::Celsius> {
    /// Create a combination of Celsius temperature and relative humidity.
    pub fn new(
        temperature: f32,
        relative_humidity: f32,
    ) -> TemperatureAndRelativeHumidity<unit::Celsius> {
        TemperatureAndRelativeHumidity {
            relative_humidity,
            temperature: Temperature::<unit::Celsius>::new(temperature),
        }
    }
}

impl From<TemperatureAndRelativeHumidity<unit::Fahrenheit>>
    for TemperatureAndRelativeHumidity<unit::Celsius>
{
    fn from(value: TemperatureAndRelativeHumidity<unit::Fahrenheit>) -> Self {
        Self::new(value.temperature.celsius(), value.relative_humidity)
    }
}

impl TemperatureAndRelativeHumidity<unit::Fahrenheit> {
    /// Create a combination of Fahrenheit temperature and relative humidity.
    pub fn new(
        temperature: f32,
        relative_humidity: f32,
    ) -> TemperatureAndRelativeHumidity<unit::Fahrenheit> {
        TemperatureAndRelativeHumidity {
            relative_humidity,
            temperature: Temperature::<unit::Fahrenheit>::new(temperature),
        }
    }
}

impl From<TemperatureAndRelativeHumidity<unit::Celsius>>
    for TemperatureAndRelativeHumidity<unit::Fahrenheit>
{
    fn from(value: TemperatureAndRelativeHumidity<unit::Celsius>) -> Self {
        Self::new(value.temperature.fahrenheit(), value.relative_humidity)
    }
}

/// The combination of the temperature and the barometric pressure.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TemperatureAndBarometricPressure<U: unit::TemperatureUnit> {
    /// The barometric pressure (in hPa).
    pub barometric_pressure: BarometricPressure,
    /// The temperature (either in °C or °F).
    pub temperature: Temperature<U>,
}

fn calculate_altitude(temperature: f32, barometric_pressure: f32) -> f32 {
    ((1_013.25 / barometric_pressure).powf(1.0 / 5.257) - 1.0) * (temperature + 273.15) / 0.0065
}

impl<U: unit::TemperatureUnit> TemperatureAndBarometricPressure<U> {
    /// Compute the altitude (in m).
    pub fn altitude(&self) -> Altitude {
        calculate_altitude(self.temperature.celsius(), self.barometric_pressure)
    }
}

impl TemperatureAndBarometricPressure<unit::Celsius> {
    /// Create a combination of Celsius temperature and barometric pressure.
    pub fn new(
        temperature: f32,
        barometric_pressure: f32,
    ) -> TemperatureAndBarometricPressure<unit::Celsius> {
        TemperatureAndBarometricPressure {
            barometric_pressure,
            temperature: Temperature::<unit::Celsius>::new(temperature),
        }
    }
}

impl From<TemperatureAndBarometricPressure<unit::Fahrenheit>>
    for TemperatureAndBarometricPressure<unit::Celsius>
{
    fn from(value: TemperatureAndBarometricPressure<unit::Fahrenheit>) -> Self {
        Self::new(value.temperature.celsius(), value.barometric_pressure)
    }
}

impl TemperatureAndBarometricPressure<unit::Fahrenheit> {
    /// Create a combination of Fahrenheit temperature and barometric pressure.
    pub fn new(
        temperature: f32,
        barometric_pressure: f32,
    ) -> TemperatureAndBarometricPressure<unit::Fahrenheit> {
        TemperatureAndBarometricPressure {
            barometric_pressure,
            temperature: Temperature::<unit::Fahrenheit>::new(temperature),
        }
    }
}

impl From<TemperatureAndBarometricPressure<unit::Celsius>>
    for TemperatureAndBarometricPressure<unit::Fahrenheit>
{
    fn from(value: TemperatureAndBarometricPressure<unit::Celsius>) -> Self {
        Self::new(value.temperature.fahrenheit(), value.barometric_pressure)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::{Celsius, Fahrenheit, TemperatureUnit};
    use approx::assert_relative_eq;
    use rstest::rstest;

    #[rstest]
    #[case(TemperatureAndRelativeHumidity::<Celsius>::new(21.18, 45.59), 8.43)]
    #[case(TemperatureAndRelativeHumidity::<Fahrenheit>::new(70.12, 45.59), 8.43)]
    #[case(TemperatureAndRelativeHumidity::<Celsius>::new(2.93, 34.71), 2.06)]
    #[case(TemperatureAndRelativeHumidity::<Fahrenheit>::new(107.7, 74.91), 42.49)]
    fn test_absolute_humidity_computation<U: TemperatureUnit>(
        #[case] input: TemperatureAndRelativeHumidity<U>,
        #[case] expected_absolute_humidity: AbsoluteHumidity,
    ) {
        assert_relative_eq!(
            input.absolute_humidity(),
            expected_absolute_humidity,
            epsilon = 0.01
        );
    }

    #[rstest]
    #[case(TemperatureAndBarometricPressure::<Celsius>::new(20.55, 991.32), 188.46)]
    #[case(TemperatureAndBarometricPressure::<Celsius>::new(17.93, 1013.25), 0.0)]
    #[case(TemperatureAndBarometricPressure::<Celsius>::new(37.5, 1013.25), 0.0)]
    #[case(TemperatureAndBarometricPressure::<Celsius>::new(19.37, 962.81), 439.25)]
    #[case(TemperatureAndBarometricPressure::<Fahrenheit>::new(99.5, 1013.25), 0.0)]
    fn test_altitude_computation<U: TemperatureUnit>(
        #[case] input: TemperatureAndBarometricPressure<U>,
        #[case] expected_altitude: Altitude,
    ) {
        assert_relative_eq!(input.altitude(), expected_altitude, epsilon = 0.01);
    }

    #[rstest]
    #[case(0.0, 32.0)]
    #[case(15.73, 60.31)]
    #[case(-7.49, 18.52)]
    #[case(37.5, 99.5)]
    fn test_celsius_to_fahrenheit_temperature_conversion(
        #[case] input: f32,
        #[case] expected_fahrenheit: f32,
    ) {
        let temperature: Temperature<Fahrenheit> = Temperature::<Celsius>::new(input).into();
        assert_relative_eq!(temperature.value.0, expected_fahrenheit, epsilon = 0.01);
        assert_relative_eq!(
            temperature.fahrenheit(),
            expected_fahrenheit,
            epsilon = 0.01
        );
        assert_relative_eq!(temperature.celsius(), input, epsilon = 0.01);
    }

    #[rstest]
    #[case(32.0, 0.0)]
    #[case(60.31, 15.73)]
    #[case(18.52, -7.49)]
    #[case(99.5, 37.5)]
    fn test_fahrenheit_to_celsius_temperature_conversion(
        #[case] input: f32,
        #[case] expected_celsius: f32,
    ) {
        let temperature: Temperature<Celsius> = Temperature::<Fahrenheit>::new(input).into();
        assert_relative_eq!(temperature.value.0, expected_celsius, epsilon = 0.01);
        assert_relative_eq!(temperature.celsius(), expected_celsius, epsilon = 0.01);
        assert_relative_eq!(temperature.fahrenheit(), input, epsilon = 0.01);
    }

    #[rstest]
    #[case(TemperatureAndRelativeHumidity::<Celsius>::new(21.18, 45.59), TemperatureAndRelativeHumidity::<Fahrenheit>::new(70.12, 45.59))]
    #[case(TemperatureAndRelativeHumidity::<Celsius>::new(-7.49, 73.19), TemperatureAndRelativeHumidity::<Fahrenheit>::new(18.52, 73.19))]
    fn test_temperature_and_relative_humidity_celsius_to_fahrenheit_conversion(
        #[case] input: TemperatureAndRelativeHumidity<Celsius>,
        #[case] expected: TemperatureAndRelativeHumidity<Fahrenheit>,
    ) {
        let value: TemperatureAndRelativeHumidity<Fahrenheit> = input.into();
        assert_eq!(value, expected);
    }

    #[rstest]
    #[case(TemperatureAndRelativeHumidity::<Fahrenheit>::new(70.12, 45.59), TemperatureAndRelativeHumidity::<Celsius>::new(21.18, 45.59))]
    #[case(TemperatureAndRelativeHumidity::<Fahrenheit>::new(18.52, 73.19), TemperatureAndRelativeHumidity::<Celsius>::new(-7.49, 73.19))]
    fn test_temperature_and_relative_humidity_fahrenheit_to_celsius_conversion(
        #[case] input: TemperatureAndRelativeHumidity<Fahrenheit>,
        #[case] expected: TemperatureAndRelativeHumidity<Celsius>,
    ) {
        let value: TemperatureAndRelativeHumidity<Celsius> = input.into();
        assert_eq!(value, expected);
    }

    #[rstest]
    #[case(TemperatureAndBarometricPressure::<Celsius>::new(21.18, 991.32), TemperatureAndBarometricPressure::<Fahrenheit>::new(70.12, 991.32))]
    #[case(TemperatureAndBarometricPressure::<Celsius>::new(37.5, 1013.25), TemperatureAndBarometricPressure::<Fahrenheit>::new(99.5, 1013.25))]
    fn test_temperature_and_barometric_pressure_celsius_to_fahrenheit_conversion(
        #[case] input: TemperatureAndBarometricPressure<Celsius>,
        #[case] expected: TemperatureAndBarometricPressure<Fahrenheit>,
    ) {
        let value: TemperatureAndBarometricPressure<Fahrenheit> = input.into();
        assert_eq!(value, expected);
    }

    #[rstest]
    #[case(TemperatureAndBarometricPressure::<Fahrenheit>::new(70.12, 991.32), TemperatureAndBarometricPressure::<Celsius>::new(21.18, 991.32))]
    #[case(TemperatureAndBarometricPressure::<Fahrenheit>::new(99.5, 1013.25), TemperatureAndBarometricPressure::<Celsius>::new(37.5, 1013.25))]
    fn test_temperature_and_barometric_pressure_fahrenheit_to_celsius_conversion(
        #[case] input: TemperatureAndBarometricPressure<Fahrenheit>,
        #[case] expected: TemperatureAndBarometricPressure<Celsius>,
    ) {
        let value: TemperatureAndBarometricPressure<Celsius> = input.into();
        assert_eq!(value, expected);
    }
}
