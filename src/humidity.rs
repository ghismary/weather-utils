use core::ops::Deref;

#[allow(unused_imports)]
#[cfg(feature = "no-std")]
use micromath::F32Ext;
#[cfg(not(feature = "no-std"))]
extern crate std;

use approx::relative_eq;

use crate::{Celsius, Fahrenheit, Temperature};

/// The absolute humidity type (in g/m³).
pub type AbsoluteHumidity = f32;

/// The relative humidity type (in %).
#[derive(Clone, Copy, Debug, Default)]
pub struct RelativeHumidity(f32);

impl RelativeHumidity {
    /// Create a RelativeHumidity, checking that the passed value is correct.
    pub fn new(value: f32) -> Result<Self, &'static str> {
        value.try_into()
    }

    /// Get the value of the relative humidity (between 0 and 100 %).
    pub fn value(&self) -> f32 {
        self.0
    }
}

impl TryFrom<f32> for RelativeHumidity {
    type Error = &'static str;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !(0.0..=100.0).contains(&value) {
            Err("Relative humidity must be between 0 and 100 %")
        } else {
            Ok(Self(value))
        }
    }
}

impl PartialEq for RelativeHumidity {
    fn eq(&self, other: &Self) -> bool {
        relative_eq!(self.0, other.0, epsilon = 0.01)
    }
}

/// The heat index.
///
/// The heat index indicates how the human body feels temperature. If relative humidity is low
/// human body cools itself by perspiration, dissipating heat from the body. At higher
/// relative humidity the evaporation rate from the human skin is lower. In that case, the
/// body cannot dissipate heat as easily as it is the case in dry air.
/// The heat index is based on subjective measurements and is only meaningful above 25°C and
/// 40% RH.
#[derive(Clone, Copy, Debug, Default)]
pub struct HeatIndex<T: Temperature>(T);

impl<T: Temperature> HeatIndex<T> {
    /// Get the relative human body comfort corresponding to the heat index.
    pub fn comfort(&self) -> Comfort {
        if self.celsius().value() < 30. {
            Comfort::NoDiscomfort
        } else if self.celsius().value() < 40. {
            Comfort::SomeDiscomfort
        } else if self.celsius().value() < 45. {
            Comfort::GreatDiscomfort
        } else if self.celsius().value() < 54. {
            Comfort::Dangerous
        } else {
            Comfort::HeatStrokeImminent
        }
    }
}

impl<T: Temperature> Deref for HeatIndex<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The relative human body comfort corresponding to a heat index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comfort {
    /// No discomfort: heat index below 30°C.
    NoDiscomfort,
    /// Some discomfort: heat index between 30°C and 40°C.
    SomeDiscomfort,
    /// Great discomfort: heat index between 40°C and 45°C.
    GreatDiscomfort,
    /// Dangerous: heat index between 45°C and 54°C.
    Dangerous,
    /// Heat stroke immiment: heat index above 54°C.
    HeatStrokeImminent,
}

/// The combination of the temperature and the relative humidity.
#[derive(Clone, Copy, Debug, Default)]
pub struct TemperatureAndRelativeHumidity<T: Temperature> {
    /// The temperature (either in °C or °F).
    pub temperature: T,
    /// The relative humidity (in %).
    pub relative_humidity: RelativeHumidity,
}

impl<T: Temperature> TemperatureAndRelativeHumidity<T> {
    /// Computes the absolute humidity value (in g/m³).
    /// The absolute humidity is defined by the mass of water vapor per humid air volume.
    pub fn absolute_humidity(&self) -> AbsoluteHumidity {
        (6.112
            * ((17.67 * self.temperature.celsius().value())
                / (self.temperature.celsius().value() + 243.5))
                .exp()
            * self.relative_humidity.value()
            * 2.1674)
            / (273.15 + self.temperature.celsius().value())
    }

    /// Computes the dew point temperature.
    /// The dew point temperature is defined as the temperature to which the quantity of air must
    /// be cooled down such that, at constant pressure, condensation occurs.
    pub fn dew_point(&self) -> T {
        const M: f32 = 17.62;
        const TN: f32 = 243.12;
        let val = f32::ln(self.relative_humidity.value() / 100.0)
            + ((M * self.temperature.celsius().value())
                / (TN + self.temperature.celsius().value()));
        T::from_celsius(Celsius((TN * val) / (M - val)))
    }

    /// Computes the heat index.
    ///
    /// See `HeatIndex`.
    pub fn heat_index(&self) -> HeatIndex<T> {
        const C1: f32 = -8.784_695;
        const C2: f32 = 1.611_394_2;
        const C3: f32 = 2.338_549;
        const C4: f32 = -0.146_116_05;
        const C5: f32 = -0.012_308_094;
        const C6: f32 = -0.016_424_827;
        const C7: f32 = 0.002_211_732;
        const C8: f32 = 0.000_725_46;
        const C9: f32 = -0.000_003_582;

        let temperature = self.temperature.celsius().value();
        let relative_humidity = self.relative_humidity.value();
        let mut heat_index = 1.1 * temperature + 5. * (0.047 * relative_humidity - 7.1) / 9.;
        if (heat_index + temperature) / 2. >= 26.7 {
            heat_index = C1
                + C2 * temperature
                + C3 * relative_humidity
                + C4 * temperature * relative_humidity
                + C5 * temperature * temperature
                + C6 * relative_humidity * relative_humidity
                + C7 * temperature * temperature * relative_humidity
                + C8 * temperature * relative_humidity * relative_humidity
                + C9 * temperature * temperature * relative_humidity * relative_humidity;
            if relative_humidity < 13. && temperature > 26.7 && temperature < 44.4 {
                heat_index += (5. / 36.)
                    * (relative_humidity - 13.)
                    * ((17. - (1.8 * temperature - 63.).abs()) / 17.).sqrt()
                    - 160. / 9.;
            }
            if relative_humidity > 85. && temperature > 26.7 && temperature < 30.6 {
                heat_index +=
                    5. * (relative_humidity - 85.) * (55. - 1.8 * temperature) / 450. - 160. / 9.;
            }
        }
        HeatIndex(T::from_celsius(Celsius(heat_index)))
    }
}

impl<T: Temperature> PartialEq for TemperatureAndRelativeHumidity<T> {
    fn eq(&self, other: &Self) -> bool {
        self.relative_humidity.eq(&other.relative_humidity)
            && relative_eq!(
                self.temperature.value(),
                other.temperature.value(),
                epsilon = 0.01
            )
    }
}

impl From<TemperatureAndRelativeHumidity<Fahrenheit>> for TemperatureAndRelativeHumidity<Celsius> {
    fn from(value: TemperatureAndRelativeHumidity<Fahrenheit>) -> Self {
        Self {
            temperature: value.temperature.celsius(),
            relative_humidity: value.relative_humidity,
        }
    }
}

impl From<TemperatureAndRelativeHumidity<Celsius>> for TemperatureAndRelativeHumidity<Fahrenheit> {
    fn from(value: TemperatureAndRelativeHumidity<Celsius>) -> Self {
        Self {
            temperature: value.temperature.fahrenheit(),
            relative_humidity: value.relative_humidity,
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(RelativeHumidity::new(32.0).unwrap(), RelativeHumidity::new(32.001).unwrap())]
    #[case(RelativeHumidity::new(32.004).unwrap(), RelativeHumidity::new(32.0).unwrap())]
    #[case(RelativeHumidity::new(60.31).unwrap(), RelativeHumidity::new(60.308).unwrap())]
    fn test_relative_humidity_eq(#[case] a: RelativeHumidity, #[case] b: RelativeHumidity) {
        assert_eq!(a, b);
    }

    #[rstest]
    #[case(RelativeHumidity::new(0.0).unwrap(), RelativeHumidity::new(10.3).unwrap())]
    #[case(RelativeHumidity::new(0.0).unwrap(), RelativeHumidity::new(0.09).unwrap())]
    #[case(RelativeHumidity::new(98.5).unwrap(), RelativeHumidity::new(99.9).unwrap())]
    fn test_relative_humidity_ne(#[case] a: RelativeHumidity, #[case] b: RelativeHumidity) {
        assert_ne!(a, b);
    }

    #[rstest]
    #[case(TemperatureAndRelativeHumidity{ temperature: Celsius(21.18), relative_humidity: RelativeHumidity::new(45.59).unwrap() }, 8.43)]
    #[case(TemperatureAndRelativeHumidity{ temperature: Fahrenheit(70.12), relative_humidity: RelativeHumidity::new(45.59).unwrap() }, 8.43)]
    #[case(TemperatureAndRelativeHumidity{ temperature: Celsius(2.93), relative_humidity: RelativeHumidity::new(34.71).unwrap() }, 2.06)]
    #[case(TemperatureAndRelativeHumidity{ temperature: Fahrenheit(107.7), relative_humidity: RelativeHumidity::new(74.91).unwrap() }, 42.49)]
    fn test_absolute_humidity_computation<T: Temperature>(
        #[case] input: TemperatureAndRelativeHumidity<T>,
        #[case] expected_absolute_humidity: AbsoluteHumidity,
    ) {
        assert_relative_eq!(
            input.absolute_humidity(),
            expected_absolute_humidity,
            epsilon = 0.01
        );
    }

    #[rstest]
    #[case(TemperatureAndRelativeHumidity{ temperature: Celsius(21.18), relative_humidity: RelativeHumidity::new(45.59).unwrap() }, Celsius(8.96))]
    #[case(TemperatureAndRelativeHumidity{ temperature: Fahrenheit(70.12), relative_humidity: RelativeHumidity::new(45.59).unwrap() }, Fahrenheit(48.13))]
    #[case(TemperatureAndRelativeHumidity{ temperature: Celsius(2.93), relative_humidity: RelativeHumidity::new(34.71).unwrap() }, Celsius(-11.16))]
    #[case(TemperatureAndRelativeHumidity{ temperature: Fahrenheit(107.7), relative_humidity: RelativeHumidity::new(74.91).unwrap() }, Fahrenheit(98.01))]
    fn test_dew_point_temperature_computation<T: Temperature>(
        #[case] input: TemperatureAndRelativeHumidity<T>,
        #[case] expected_dew_point: T,
    ) {
        assert_relative_eq!(
            input.dew_point().value(),
            expected_dew_point.value(),
            epsilon = 0.01
        );
    }

    #[rstest]
    #[case(TemperatureAndRelativeHumidity{ temperature: Celsius(27.), relative_humidity: RelativeHumidity::new(40.).unwrap() }, Celsius(26.86), Comfort::NoDiscomfort)]
    #[case(TemperatureAndRelativeHumidity{ temperature: Celsius(29.), relative_humidity: RelativeHumidity::new(50.).unwrap() }, Celsius(29.65), Comfort::NoDiscomfort)]
    #[case(TemperatureAndRelativeHumidity{ temperature: Celsius(31.), relative_humidity: RelativeHumidity::new(60.).unwrap() }, Celsius(34.84), Comfort::SomeDiscomfort)]
    #[case(TemperatureAndRelativeHumidity{ temperature: Celsius(32.), relative_humidity: RelativeHumidity::new(70.).unwrap() }, Celsius(40.41), Comfort::GreatDiscomfort)]
    #[case(TemperatureAndRelativeHumidity{ temperature: Celsius(34.), relative_humidity: RelativeHumidity::new(80.).unwrap() }, Celsius(52.2), Comfort::Dangerous)]
    #[case(TemperatureAndRelativeHumidity{ temperature: Celsius(36.), relative_humidity: RelativeHumidity::new(90.).unwrap() }, Celsius(69.2), Comfort::HeatStrokeImminent)]
    #[case(TemperatureAndRelativeHumidity{ temperature: Celsius(37.5), relative_humidity: RelativeHumidity::new(100.).unwrap() }, Celsius(88.71), Comfort::HeatStrokeImminent)]
    #[case(TemperatureAndRelativeHumidity{ temperature: Fahrenheit(80.6), relative_humidity: RelativeHumidity::new(40.).unwrap() }, Fahrenheit(80.346), Comfort::NoDiscomfort)]
    #[case(TemperatureAndRelativeHumidity{ temperature: Fahrenheit(89.6), relative_humidity: RelativeHumidity::new(70.).unwrap() }, Fahrenheit(104.738), Comfort::GreatDiscomfort)]
    #[case(TemperatureAndRelativeHumidity{ temperature: Fahrenheit(96.8), relative_humidity: RelativeHumidity::new(90.).unwrap() }, Fahrenheit(156.56), Comfort::HeatStrokeImminent)]
    fn test_heat_index_computation<T: Temperature>(
        #[case] input: TemperatureAndRelativeHumidity<T>,
        #[case] expected_heat_index: T,
        #[case] expected_comfort: Comfort,
    ) {
        let heat_index = input.heat_index();
        assert_relative_eq!(
            heat_index.value(),
            expected_heat_index.value(),
            epsilon = 0.01
        );
        assert_eq!(heat_index.comfort(), expected_comfort);
    }

    #[rstest]
    #[case(
        TemperatureAndRelativeHumidity{ temperature: Celsius(21.18), relative_humidity: RelativeHumidity::new(45.59).unwrap() },
        TemperatureAndRelativeHumidity{ temperature: Fahrenheit(70.12), relative_humidity: RelativeHumidity::new(45.59).unwrap()
    })]
    #[case(
        TemperatureAndRelativeHumidity{ temperature: Celsius(-7.49), relative_humidity: RelativeHumidity::new(73.19).unwrap() },
        TemperatureAndRelativeHumidity{ temperature: Fahrenheit(18.52), relative_humidity: RelativeHumidity::new(73.19).unwrap() }
    )]
    fn test_temperature_and_relative_humidity_celsius_to_fahrenheit_conversion(
        #[case] input: TemperatureAndRelativeHumidity<Celsius>,
        #[case] expected: TemperatureAndRelativeHumidity<Fahrenheit>,
    ) {
        let value: TemperatureAndRelativeHumidity<Fahrenheit> = input.into();
        assert_eq!(value, expected);
    }

    #[rstest]
    #[case(
        TemperatureAndRelativeHumidity{ temperature: Fahrenheit(70.12), relative_humidity: RelativeHumidity::new(45.59).unwrap() },
        TemperatureAndRelativeHumidity{ temperature: Celsius(21.18), relative_humidity: RelativeHumidity::new(45.59).unwrap() }
    )]
    #[case(
        TemperatureAndRelativeHumidity{ temperature: Fahrenheit(18.52), relative_humidity: RelativeHumidity::new(73.19).unwrap() },
        TemperatureAndRelativeHumidity{ temperature: Celsius(-7.49), relative_humidity: RelativeHumidity::new(73.19).unwrap() }
    )]
    fn test_temperature_and_relative_humidity_fahrenheit_to_celsius_conversion(
        #[case] input: TemperatureAndRelativeHumidity<Fahrenheit>,
        #[case] expected: TemperatureAndRelativeHumidity<Celsius>,
    ) {
        let value: TemperatureAndRelativeHumidity<Celsius> = input.into();
        assert_eq!(value, expected);
    }
}
