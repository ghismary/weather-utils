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
    /// Get the temperature value in degrees Celcius (°C).
    pub fn celcius(&self) -> f32 {
        self.value.celcius()
    }

    /// Get the temperature value in degrees Farenheit (°F).
    pub fn farenheit(&self) -> f32 {
        self.value.farenheit()
    }
}

impl Temperature<unit::Celcius> {
    /// Create a Celcius temperature.
    pub fn new(value: f32) -> Temperature<unit::Celcius> {
        Temperature {
            value: unit::Celcius { value },
        }
    }
}

impl From<Temperature<unit::Farenheit>> for Temperature<unit::Celcius> {
    fn from(value: Temperature<unit::Farenheit>) -> Self {
        Self::new(value.celcius())
    }
}

impl Temperature<unit::Farenheit> {
    /// Create a Farenheit temperature.
    pub fn new(value: f32) -> Temperature<unit::Farenheit> {
        Temperature {
            value: unit::Farenheit { value },
        }
    }
}

impl From<Temperature<unit::Celcius>> for Temperature<unit::Farenheit> {
    fn from(value: Temperature<unit::Celcius>) -> Self {
        Self::new(value.farenheit())
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
        let temperature = self.temperature.celcius();
        (6.112
            * ((17.67 * temperature) / (temperature + 243.5)).exp()
            * self.relative_humidity
            * 2.1674)
            / (273.15 + temperature)
    }
}

impl TemperatureAndRelativeHumidity<unit::Celcius> {
    /// Create a combination of Celcius temperature and relative humidity.
    pub fn new(
        temperature: f32,
        relative_humidity: f32,
    ) -> TemperatureAndRelativeHumidity<unit::Celcius> {
        TemperatureAndRelativeHumidity {
            relative_humidity,
            temperature: Temperature::<unit::Celcius>::new(temperature),
        }
    }
}

impl From<TemperatureAndRelativeHumidity<unit::Farenheit>>
    for TemperatureAndRelativeHumidity<unit::Celcius>
{
    fn from(value: TemperatureAndRelativeHumidity<unit::Farenheit>) -> Self {
        Self::new(value.temperature.celcius(), value.relative_humidity)
    }
}

impl TemperatureAndRelativeHumidity<unit::Farenheit> {
    /// Create a combination of Farenheit temperature and relative humidity.
    pub fn new(
        temperature: f32,
        relative_humidity: f32,
    ) -> TemperatureAndRelativeHumidity<unit::Farenheit> {
        TemperatureAndRelativeHumidity {
            relative_humidity,
            temperature: Temperature::<unit::Farenheit>::new(temperature),
        }
    }
}

impl From<TemperatureAndRelativeHumidity<unit::Celcius>>
    for TemperatureAndRelativeHumidity<unit::Farenheit>
{
    fn from(value: TemperatureAndRelativeHumidity<unit::Celcius>) -> Self {
        Self::new(value.temperature.farenheit(), value.relative_humidity)
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
        let temperature = self.temperature.celcius();
        ((1_013.25 / self.barometric_pressure).powf(1.0 / 5.257) - 1.0) * (temperature + 273.15)
            / 0.0065
    }
}

impl TemperatureAndBarometricPressure<unit::Celcius> {
    /// Create a combination of Celcius temperature and barometric pressure.
    pub fn new(
        temperature: f32,
        barometric_pressure: f32,
    ) -> TemperatureAndBarometricPressure<unit::Celcius> {
        TemperatureAndBarometricPressure {
            barometric_pressure,
            temperature: Temperature::<unit::Celcius>::new(temperature),
        }
    }
}

impl From<TemperatureAndBarometricPressure<unit::Farenheit>>
    for TemperatureAndBarometricPressure<unit::Celcius>
{
    fn from(value: TemperatureAndBarometricPressure<unit::Farenheit>) -> Self {
        Self::new(value.temperature.celcius(), value.barometric_pressure)
    }
}

impl TemperatureAndBarometricPressure<unit::Farenheit> {
    /// Create a combination of Farenheit temperature and barometric pressure.
    pub fn new(
        temperature: f32,
        barometric_pressure: f32,
    ) -> TemperatureAndBarometricPressure<unit::Farenheit> {
        TemperatureAndBarometricPressure {
            barometric_pressure,
            temperature: Temperature::<unit::Farenheit>::new(temperature),
        }
    }
}

impl From<TemperatureAndBarometricPressure<unit::Celcius>>
    for TemperatureAndBarometricPressure<unit::Farenheit>
{
    fn from(value: TemperatureAndBarometricPressure<unit::Celcius>) -> Self {
        Self::new(value.temperature.farenheit(), value.barometric_pressure)
    }
}

#[cfg(test)]
mod tests {
    use crate::unit::*;
    use crate::*;

    #[test]
    fn compute_absolute_humidity() {
        assert!(
            (TemperatureAndRelativeHumidity::<Celcius>::new(21.18, 45.59).absolute_humidity()
                - 8.43)
                .abs()
                < 0.01
        );
        assert!(
            (TemperatureAndRelativeHumidity::<Farenheit>::new(70.12, 45.59).absolute_humidity()
                - 8.43)
                .abs()
                < 0.01
        );
        assert!(
            (TemperatureAndRelativeHumidity::<Celcius>::new(2.93, 34.71).absolute_humidity()
                - 2.06)
                .abs()
                < 0.01
        );
        assert!(
            (TemperatureAndRelativeHumidity::<Farenheit>::new(107.7, 74.91).absolute_humidity()
                - 42.49)
                .abs()
                < 0.01
        );
    }

    #[test]
    fn compute_altitude() {
        assert!(
            (TemperatureAndBarometricPressure::<Celcius>::new(20.55, 991.32).altitude() - 188.46)
                .abs()
                < 0.01
        );
        assert!(
            (TemperatureAndBarometricPressure::<Celcius>::new(17.93, 1013.25).altitude() - 0.0)
                .abs()
                < 0.01
        );
        assert!(
            (TemperatureAndBarometricPressure::<Celcius>::new(37.5, 1013.25).altitude() - 0.0)
                .abs()
                < 0.01
        );
        assert!(
            (TemperatureAndBarometricPressure::<Celcius>::new(19.37, 962.81).altitude() - 439.25)
                .abs()
                < 0.01
        );
    }

    #[test]
    fn convert_temperatures() {
        assert!(
            (Temperature::<Farenheit>::from(Temperature::<Celcius>::new(0.0))
                .value
                .value
                - 32.0)
                .abs()
                < 0.01
        );
        assert!(
            (Temperature::<Farenheit>::from(Temperature::<Celcius>::new(15.73))
                .value
                .value
                - 60.31)
                .abs()
                < 0.01
        );
        assert!(
            (Temperature::<Farenheit>::from(Temperature::<Celcius>::new(-7.49))
                .value
                .value
                - 18.52)
                .abs()
                < 0.01
        );
        assert!(
            (Temperature::<Farenheit>::from(Temperature::<Celcius>::new(37.5))
                .value
                .value
                - 99.5)
                .abs()
                < 0.01
        );

        assert!(
            (Temperature::<Celcius>::from(Temperature::<Farenheit>::new(32.0))
                .value
                .value
                - 0.0)
                .abs()
                < 0.01
        );
        assert!(
            (Temperature::<Celcius>::from(Temperature::<Farenheit>::new(60.31))
                .value
                .value
                - 15.73)
                .abs()
                < 0.01
        );
        assert!(
            (Temperature::<Celcius>::from(Temperature::<Farenheit>::new(18.52))
                .value
                .value
                - -7.49)
                .abs()
                < 0.01
        );
        assert!(
            (Temperature::<Celcius>::from(Temperature::<Farenheit>::new(99.5))
                .value
                .value
                - 37.5)
                .abs()
                < 0.01
        );
    }
}
