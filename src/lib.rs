#![doc = include_str!("../README.md")]
#![deny(unsafe_code, missing_docs)]
#![no_std]

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

impl Temperature<unit::Celsius> {
    /// Create a Celsius temperature.
    pub fn new(value: f32) -> Temperature<unit::Celsius> {
        Temperature {
            value: unit::Celsius { value },
        }
    }
}

impl From<Temperature<unit::Fahrenheit>> for Temperature<unit::Celsius> {
    fn from(value: Temperature<unit::Fahrenheit>) -> Self {
        Self::new(value.celsius())
    }
}

impl Temperature<unit::Fahrenheit> {
    /// Create a Fahrenheit temperature.
    pub fn new(value: f32) -> Temperature<unit::Fahrenheit> {
        Temperature {
            value: unit::Fahrenheit { value },
        }
    }
}

impl From<Temperature<unit::Celsius>> for Temperature<unit::Fahrenheit> {
    fn from(value: Temperature<unit::Celsius>) -> Self {
        Self::new(value.fahrenheit())
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

impl<U: unit::TemperatureUnit> TemperatureAndRelativeHumidity<U> {
    /// Computes the absolute humidity value (in g/m³).
    pub fn absolute_humidity(&self) -> AbsoluteHumidity {
        let temperature = self.temperature.celsius();
        (6.112
            * ((17.67 * temperature) / (temperature + 243.5)).exp()
            * self.relative_humidity
            * 2.1674)
            / (273.15 + temperature)
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
#[derive(Clone, Copy, Debug, Default)]
pub struct TemperatureAndBarometricPressure<U: unit::TemperatureUnit> {
    /// The barometric pressure (in hPa).
    pub barometric_pressure: BarometricPressure,
    /// The temperature (either in °C or °F).
    pub temperature: Temperature<U>,
}

impl<U: unit::TemperatureUnit> TemperatureAndBarometricPressure<U> {
    /// Compute the altitude (in m).
    pub fn altitude(&self) -> Altitude {
        let temperature = self.temperature.celsius();
        ((1_013.25 / self.barometric_pressure).powf(1.0 / 5.257) - 1.0) * (temperature + 273.15)
            / 0.0065
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
    use crate::unit::*;
    use crate::*;

    #[test]
    fn compute_absolute_humidity() {
        assert!(
            (TemperatureAndRelativeHumidity::<Celsius>::new(21.18, 45.59).absolute_humidity()
                - 8.43)
                .abs()
                < 0.01
        );
        assert!(
            (TemperatureAndRelativeHumidity::<Fahrenheit>::new(70.12, 45.59).absolute_humidity()
                - 8.43)
                .abs()
                < 0.01
        );
        assert!(
            (TemperatureAndRelativeHumidity::<Celsius>::new(2.93, 34.71).absolute_humidity()
                - 2.06)
                .abs()
                < 0.01
        );
        assert!(
            (TemperatureAndRelativeHumidity::<Fahrenheit>::new(107.7, 74.91).absolute_humidity()
                - 42.49)
                .abs()
                < 0.01
        );
    }

    #[test]
    fn compute_altitude() {
        assert!(
            (TemperatureAndBarometricPressure::<Celsius>::new(20.55, 991.32).altitude() - 188.46)
                .abs()
                < 0.01
        );
        assert!(
            (TemperatureAndBarometricPressure::<Celsius>::new(17.93, 1013.25).altitude() - 0.0)
                .abs()
                < 0.01
        );
        assert!(
            (TemperatureAndBarometricPressure::<Celsius>::new(37.5, 1013.25).altitude() - 0.0)
                .abs()
                < 0.01
        );
        assert!(
            (TemperatureAndBarometricPressure::<Celsius>::new(19.37, 962.81).altitude() - 439.25)
                .abs()
                < 0.01
        );
    }

    #[test]
    fn convert_temperatures() {
        assert!(
            (Temperature::<Fahrenheit>::from(Temperature::<Celsius>::new(0.0))
                .value
                .value
                - 32.0)
                .abs()
                < 0.01
        );
        assert!(
            (Temperature::<Fahrenheit>::from(Temperature::<Celsius>::new(15.73))
                .value
                .value
                - 60.31)
                .abs()
                < 0.01
        );
        assert!(
            (Temperature::<Fahrenheit>::from(Temperature::<Celsius>::new(-7.49))
                .value
                .value
                - 18.52)
                .abs()
                < 0.01
        );
        assert!(
            (Temperature::<Fahrenheit>::from(Temperature::<Celsius>::new(37.5))
                .value
                .value
                - 99.5)
                .abs()
                < 0.01
        );

        assert!(
            (Temperature::<Celsius>::from(Temperature::<Fahrenheit>::new(32.0))
                .value
                .value
                - 0.0)
                .abs()
                < 0.01
        );
        assert!(
            (Temperature::<Celsius>::from(Temperature::<Fahrenheit>::new(60.31))
                .value
                .value
                - 15.73)
                .abs()
                < 0.01
        );
        assert!(
            (Temperature::<Celsius>::from(Temperature::<Fahrenheit>::new(18.52))
                .value
                .value
                - -7.49)
                .abs()
                < 0.01
        );
        assert!(
            (Temperature::<Celsius>::from(Temperature::<Fahrenheit>::new(99.5))
                .value
                .value
                - 37.5)
                .abs()
                < 0.01
        );
    }
}
