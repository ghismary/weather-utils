use approx::relative_eq;

/// Trait defining the different ways to get a temperature.
pub trait TemperatureUnit {
    /// Get the temperature in degrees Celsius (°C).
    fn celsius(&self) -> f32;
    /// Get the temperature in degrees Fahrenheit (°F).
    fn fahrenheit(&self) -> f32;
}

/// The degrees Celsius temperature unit.
#[derive(Clone, Copy, Debug, Default)]
pub struct Celsius(pub f32);

impl TemperatureUnit for Celsius {
    fn celsius(&self) -> f32 {
        self.0
    }

    fn fahrenheit(&self) -> f32 {
        convert_celsius_to_fahrenheit(self.0)
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(value: Fahrenheit) -> Self {
        Self(convert_fahrenheit_to_celsius(value.0))
    }
}

impl PartialEq for Celsius {
    fn eq(&self, other: &Self) -> bool {
        relative_eq!(self.0, other.0, epsilon = 0.01)
    }
}

/// The degrees Fahrenheit temperature unit.
#[derive(Clone, Copy, Debug, Default)]
pub struct Fahrenheit(pub f32);

impl TemperatureUnit for Fahrenheit {
    fn celsius(&self) -> f32 {
        convert_fahrenheit_to_celsius(self.0)
    }

    fn fahrenheit(&self) -> f32 {
        self.0
    }
}

impl From<Celsius> for Fahrenheit {
    fn from(value: Celsius) -> Self {
        Self(convert_celsius_to_fahrenheit(value.0))
    }
}

impl PartialEq for Fahrenheit {
    fn eq(&self, other: &Self) -> bool {
        relative_eq!(self.0, other.0, epsilon = 0.01)
    }
}

/// Converts a temperature in °C to °F.
fn convert_celsius_to_fahrenheit(temperature: f32) -> f32 {
    temperature * 1.8 + 32.0
}

/// Converts a temperature in °F to °C.
fn convert_fahrenheit_to_celsius(temperature: f32) -> f32 {
    (temperature - 32.0) * 0.55555
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rstest::rstest;

    #[rstest]
    #[case(0.0, 32.0)]
    #[case(15.73, 60.31)]
    #[case(-7.49, 18.52)]
    #[case(37.5, 99.5)]
    fn test_celcius_to_fahrenheit_conversion(#[case] input: f32, #[case] expected_output: f32) {
        assert_relative_eq!(
            convert_celsius_to_fahrenheit(input),
            expected_output,
            epsilon = 0.01
        );
    }

    #[rstest]
    #[case(32.0, 0.0)]
    #[case(60.31, 15.73)]
    #[case(18.52, -7.49)]
    #[case(99.5, 37.5)]
    fn test_fahrenheit_to_celsius_conversion(#[case] input: f32, #[case] expected_output: f32) {
        assert_relative_eq!(
            convert_fahrenheit_to_celsius(input),
            expected_output,
            epsilon = 0.01
        )
    }

    #[rstest]
    #[case(0.0, 32.0)]
    #[case(15.73, 60.31)]
    #[case(-7.49, 18.52)]
    #[case(37.5, 99.5)]
    fn test_celsius(#[case] celsius: f32, #[case] expected_fahrenheit: f32) {
        let temperature = Celsius(celsius);
        assert_relative_eq!(temperature.celsius(), celsius, epsilon = f32::EPSILON);
        assert_relative_eq!(
            temperature.fahrenheit(),
            expected_fahrenheit,
            epsilon = 0.01
        );
    }

    #[rstest]
    #[case(32.0, 0.0)]
    #[case(60.31, 15.73)]
    #[case(18.52, -7.49)]
    #[case(99.5, 37.5)]
    fn test_fahrenheit(#[case] fahrenheit: f32, #[case] expected_celsius: f32) {
        let temperature = Fahrenheit(fahrenheit);
        assert_relative_eq!(temperature.fahrenheit(), fahrenheit, epsilon = f32::EPSILON);
        assert_relative_eq!(temperature.celsius(), expected_celsius, epsilon = 0.01);
    }

    #[rstest]
    #[case(0.0, 0.001)]
    #[case(0.004, 0.0)]
    #[case(15.73, 15.728)]
    fn test_celsius_eq(#[case] a: f32, #[case] b: f32) {
        assert_eq!(Celsius(a), Celsius(b));
    }

    #[rstest]
    #[case(0.0, 10.3)]
    #[case(0.0, 0.09)]
    #[case(37.5, 38.9)]
    fn test_celsius_ne(#[case] a: f32, #[case] b: f32) {
        assert_ne!(Celsius(a), Celsius(b))
    }

    #[rstest]
    #[case(32.0, 32.001)]
    #[case(32.004, 32.0)]
    #[case(60.31, 60.308)]
    fn test_fahrenheit_eq(#[case] a: f32, #[case] b: f32) {
        assert_eq!(Fahrenheit(a), Fahrenheit(b));
    }

    #[rstest]
    #[case(0.0, 10.3)]
    #[case(0.0, 0.09)]
    #[case(99.5, 100.9)]
    fn test_fahrenheit_ne(#[case] a: f32, #[case] b: f32) {
        assert_ne!(Fahrenheit(a), Fahrenheit(b))
    }
}
