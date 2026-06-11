#[allow(unused_imports)]
#[cfg(feature = "no-std")]
use micromath::F32Ext;
#[cfg(not(feature = "no-std"))]
extern crate std;

use approx::relative_eq;

use crate::{Celsius, Fahrenheit, Temperature};

/// The barometric pressure type (in hPa).
#[derive(Clone, Copy, Debug, Default)]
pub struct BarometricPressure(pub f32);

impl BarometricPressure {
    /// Get the value of the barometric pressure.
    pub fn value(&self) -> f32 {
        self.0
    }
}

impl From<f32> for BarometricPressure {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl PartialEq for BarometricPressure {
    fn eq(&self, other: &Self) -> bool {
        relative_eq!(self.0, other.0, epsilon = 0.01)
    }
}

/// The altitude type (in m).
#[derive(Clone, Copy, Debug, Default)]
pub struct Altitude(pub f32);

impl Altitude {
    /// Get the value of the altitude.
    pub fn value(&self) -> f32 {
        self.0
    }
}

impl From<f32> for Altitude {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl PartialEq for Altitude {
    fn eq(&self, other: &Self) -> bool {
        relative_eq!(self.0, other.0, epsilon = 0.01)
    }
}

/// The combination of the temperature and the barometric pressure.
#[derive(Clone, Copy, Debug, Default)]
pub struct TemperatureAndBarometricPressure<T: Temperature> {
    /// The temperature (either in °C or °F).
    pub temperature: T,
    /// The barometric pressure (in hPa).
    pub barometric_pressure: BarometricPressure,
}

impl<T: Temperature> TemperatureAndBarometricPressure<T> {
    /// Compute the altitude (in m).
    pub fn altitude(&self) -> Altitude {
        Altitude(
            ((1_013.25 / self.barometric_pressure.value()).powf(1.0 / 5.257) - 1.0)
                * (self.temperature.celsius().value() + 273.15)
                / 0.0065,
        )
    }
}

impl<T: Temperature + PartialEq> PartialEq for TemperatureAndBarometricPressure<T> {
    fn eq(&self, other: &Self) -> bool {
        self.barometric_pressure.eq(&other.barometric_pressure)
            && self.temperature.eq(&other.temperature)
    }
}

impl From<TemperatureAndBarometricPressure<Fahrenheit>>
    for TemperatureAndBarometricPressure<Celsius>
{
    fn from(value: TemperatureAndBarometricPressure<Fahrenheit>) -> Self {
        Self {
            temperature: value.temperature.celsius(),
            barometric_pressure: value.barometric_pressure,
        }
    }
}

impl From<TemperatureAndBarometricPressure<Celsius>>
    for TemperatureAndBarometricPressure<Fahrenheit>
{
    fn from(value: TemperatureAndBarometricPressure<Celsius>) -> Self {
        Self {
            temperature: value.temperature.fahrenheit(),
            barometric_pressure: value.barometric_pressure,
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(TemperatureAndBarometricPressure{ temperature: Celsius(20.55), barometric_pressure: BarometricPressure(991.32) }, Altitude(188.46))]
    #[case(TemperatureAndBarometricPressure{ temperature: Celsius(17.93), barometric_pressure: 1013.25.into() }, 0.0.into())]
    #[case(TemperatureAndBarometricPressure{ temperature: Celsius(37.5), barometric_pressure: BarometricPressure(1013.25) }, Altitude(0.0))]
    #[case(TemperatureAndBarometricPressure{ temperature: Celsius(19.37), barometric_pressure: 962.81.into() }, 439.25.into())]
    #[case(TemperatureAndBarometricPressure{ temperature: Fahrenheit(99.5), barometric_pressure: BarometricPressure(1013.25) }, Altitude(0.0))]
    fn test_altitude_computation<T: Temperature>(
        #[case] input: TemperatureAndBarometricPressure<T>,
        #[case] expected_altitude: Altitude,
    ) {
        assert_eq!(input.altitude(), expected_altitude);
        assert_relative_eq!(
            input.altitude().value(),
            expected_altitude.value(),
            epsilon = 0.01
        );
    }

    #[rstest]
    #[case(
        TemperatureAndBarometricPressure{ temperature: Celsius(21.18), barometric_pressure: BarometricPressure(991.32) },
        TemperatureAndBarometricPressure{ temperature: Fahrenheit(70.12), barometric_pressure: 991.32.into() }
    )]
    #[case(
        TemperatureAndBarometricPressure{ temperature: Celsius(37.5), barometric_pressure: 1013.25.into() },
        TemperatureAndBarometricPressure{ temperature: Fahrenheit(99.5), barometric_pressure: BarometricPressure(1013.25) }
    )]
    fn test_temperature_and_barometric_pressure_celsius_to_fahrenheit_conversion(
        #[case] input: TemperatureAndBarometricPressure<Celsius>,
        #[case] expected: TemperatureAndBarometricPressure<Fahrenheit>,
    ) {
        let value: TemperatureAndBarometricPressure<Fahrenheit> = input.into();
        assert_eq!(value, expected);
    }

    #[rstest]
    #[case(
        TemperatureAndBarometricPressure{ temperature: Fahrenheit(70.12), barometric_pressure: BarometricPressure(991.32) },
        TemperatureAndBarometricPressure{ temperature: Celsius(21.18), barometric_pressure: 991.32.into() }
    )]
    #[case(
        TemperatureAndBarometricPressure{ temperature: Fahrenheit(99.5), barometric_pressure: 1013.25.into() },
        TemperatureAndBarometricPressure{ temperature: Celsius(37.5), barometric_pressure: BarometricPressure(1013.25) }
    )]
    fn test_temperature_and_barometric_pressure_fahrenheit_to_celsius_conversion(
        #[case] input: TemperatureAndBarometricPressure<Fahrenheit>,
        #[case] expected: TemperatureAndBarometricPressure<Celsius>,
    ) {
        let value: TemperatureAndBarometricPressure<Celsius> = input.into();
        assert_eq!(value, expected);
    }
}
