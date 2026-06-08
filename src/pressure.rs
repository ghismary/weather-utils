#[allow(unused_imports)]
#[cfg(feature = "no-std")]
use micromath::F32Ext;
#[cfg(not(feature = "no-std"))]
extern crate std;

use crate::{Celsius, Fahrenheit, Temperature};

/// The barometric pressure type (in hPa).
pub type BarometricPressure = f32;

/// The altitude type (in m).
pub type Altitude = f32;

/// The combination of the temperature and the barometric pressure.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TemperatureAndBarometricPressure<T: Temperature> {
    /// The temperature (either in °C or °F).
    pub temperature: T,
    /// The barometric pressure (in hPa).
    pub barometric_pressure: BarometricPressure,
}

impl<T: Temperature> TemperatureAndBarometricPressure<T> {
    /// Compute the altitude (in m).
    pub fn altitude(&self) -> Altitude {
        ((1_013.25 / self.barometric_pressure).powf(1.0 / 5.257) - 1.0)
            * (self.temperature.celsius().value() + 273.15)
            / 0.0065
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
    #[case(TemperatureAndBarometricPressure{ temperature: Celsius(20.55), barometric_pressure: 991.32 }, 188.46)]
    #[case(TemperatureAndBarometricPressure{ temperature: Celsius(17.93), barometric_pressure: 1013.25 }, 0.0)]
    #[case(TemperatureAndBarometricPressure{ temperature: Celsius(37.5), barometric_pressure: 1013.25 }, 0.0)]
    #[case(TemperatureAndBarometricPressure{ temperature: Celsius(19.37), barometric_pressure: 962.81 }, 439.25)]
    #[case(TemperatureAndBarometricPressure{ temperature: Fahrenheit(99.5), barometric_pressure: 1013.25 }, 0.0)]
    fn test_altitude_computation<T: Temperature>(
        #[case] input: TemperatureAndBarometricPressure<T>,
        #[case] expected_altitude: Altitude,
    ) {
        assert_relative_eq!(input.altitude(), expected_altitude, epsilon = 0.01);
    }

    #[rstest]
    #[case(TemperatureAndBarometricPressure{ temperature: Celsius(21.18), barometric_pressure: 991.32 }, TemperatureAndBarometricPressure{ temperature: Fahrenheit(70.12), barometric_pressure: 991.32 })]
    #[case(TemperatureAndBarometricPressure{ temperature: Celsius(37.5), barometric_pressure: 1013.25 }, TemperatureAndBarometricPressure{ temperature: Fahrenheit(99.5), barometric_pressure: 1013.25 })]
    fn test_temperature_and_barometric_pressure_celsius_to_fahrenheit_conversion(
        #[case] input: TemperatureAndBarometricPressure<Celsius>,
        #[case] expected: TemperatureAndBarometricPressure<Fahrenheit>,
    ) {
        let value: TemperatureAndBarometricPressure<Fahrenheit> = input.into();
        assert_eq!(value, expected);
    }

    #[rstest]
    #[case(TemperatureAndBarometricPressure{ temperature: Fahrenheit(70.12), barometric_pressure: 991.32 }, TemperatureAndBarometricPressure{ temperature: Celsius(21.18), barometric_pressure: 991.32 })]
    #[case(TemperatureAndBarometricPressure{ temperature: Fahrenheit(99.5), barometric_pressure: 1013.25 }, TemperatureAndBarometricPressure{ temperature: Celsius(37.5), barometric_pressure: 1013.25 })]
    fn test_temperature_and_barometric_pressure_fahrenheit_to_celsius_conversion(
        #[case] input: TemperatureAndBarometricPressure<Fahrenheit>,
        #[case] expected: TemperatureAndBarometricPressure<Celsius>,
    ) {
        let value: TemperatureAndBarometricPressure<Celsius> = input.into();
        assert_eq!(value, expected);
    }
}
