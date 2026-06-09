use approx::relative_eq;

use core::cmp::Ordering;

/// Trait defining the different ways to get a temperature.
pub trait Temperature: Sized + Copy {
    /// Get the temperature in degrees Celsius (°C).
    fn celsius(&self) -> Celsius;
    /// Get the temperature in degrees Fahrenheit (°F).
    fn fahrenheit(&self) -> Fahrenheit;
    /// Get the raw value.
    fn value(&self) -> f32;

    /// Create a Temperature from a Celsius temperature.
    fn from_celsius(celsius: Celsius) -> Self;
}

/// The degrees Celsius temperature unit.
#[derive(Clone, Copy, Debug, Default)]
pub struct Celsius(pub f32);

impl Temperature for Celsius {
    fn celsius(&self) -> Celsius {
        *self
    }

    fn fahrenheit(&self) -> Fahrenheit {
        Fahrenheit(self.0 * 1.8 + 32.0)
    }

    fn value(&self) -> f32 {
        self.0
    }

    fn from_celsius(celsius: Celsius) -> Self {
        celsius
    }
}

impl From<f32> for Celsius {
    fn from(value: f32) -> Self {
        Celsius(value)
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(value: Fahrenheit) -> Self {
        value.celsius()
    }
}

impl PartialEq for Celsius {
    fn eq(&self, other: &Self) -> bool {
        relative_eq!(self.0, other.0, epsilon = 0.01)
    }
}

impl PartialOrd for Celsius {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

/// The degrees Fahrenheit temperature unit.
#[derive(Clone, Copy, Debug, Default)]
pub struct Fahrenheit(pub f32);

impl Temperature for Fahrenheit {
    fn celsius(&self) -> Celsius {
        Celsius((self.0 - 32.0) * 0.55555)
    }

    fn fahrenheit(&self) -> Fahrenheit {
        *self
    }

    fn value(&self) -> f32 {
        self.0
    }

    fn from_celsius(celsius: Celsius) -> Self {
        celsius.fahrenheit()
    }
}

impl From<f32> for Fahrenheit {
    fn from(value: f32) -> Self {
        Fahrenheit(value)
    }
}

impl From<Celsius> for Fahrenheit {
    fn from(value: Celsius) -> Self {
        value.fahrenheit()
    }
}

impl PartialEq for Fahrenheit {
    fn eq(&self, other: &Self) -> bool {
        relative_eq!(self.0, other.0, epsilon = 0.01)
    }
}

impl PartialOrd for Fahrenheit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

#[cfg(test)]
mod tests {
    use more_asserts::{assert_gt, assert_lt};
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0.0, Celsius(0.0))]
    #[case(15.73, Celsius(15.73))]
    #[case(-7.49, Celsius(-7.49))]
    #[case(37.5, Celsius(37.5))]
    fn test_celsius_from_f32(#[case] input: f32, #[case] expected_output: Celsius) {
        let celsius: Celsius = input.into();
        assert_eq!(celsius, expected_output);
        assert_eq!(celsius.celsius(), celsius);
    }

    #[rstest]
    #[case(32.0, Fahrenheit(32.0))]
    #[case(60.31, Fahrenheit(60.31))]
    #[case(18.52, Fahrenheit(18.52))]
    #[case(99.5, Fahrenheit(99.5))]
    fn test_fahrenheit_from_f32(#[case] input: f32, #[case] expected_output: Fahrenheit) {
        let fahrenheit: Fahrenheit = input.into();
        assert_eq!(fahrenheit, expected_output);
        assert_eq!(fahrenheit.fahrenheit(), fahrenheit);
    }

    #[rstest]
    #[case(Celsius(0.0), Fahrenheit(32.0))]
    #[case(Celsius(15.73), Fahrenheit(60.31))]
    #[case(Celsius(-7.49), Fahrenheit(18.52))]
    #[case(Celsius(37.5), Fahrenheit(99.5))]
    fn test_celcius_to_fahrenheit_conversion(
        #[case] input: Celsius,
        #[case] expected_output: Fahrenheit,
    ) {
        assert_eq!(input.fahrenheit(), expected_output);
        let fahrenheit: Fahrenheit = input.into();
        assert_eq!(fahrenheit, expected_output);
    }

    #[rstest]
    #[case(Fahrenheit(32.0), Celsius(0.0))]
    #[case(Fahrenheit(60.31), Celsius(15.73))]
    #[case(Fahrenheit(18.52), Celsius(-7.49))]
    #[case(Fahrenheit(99.5), Celsius(37.5))]
    fn test_fahrenheit_to_celsius_conversion(
        #[case] input: Fahrenheit,
        #[case] expected_output: Celsius,
    ) {
        assert_eq!(input.celsius(), expected_output);
        let celsius: Celsius = input.into();
        assert_eq!(celsius, expected_output);
    }

    #[rstest]
    #[case(Celsius(0.0), Celsius(0.001))]
    #[case(Celsius(0.004), Celsius(0.0))]
    #[case(Celsius(15.73), Celsius(15.728))]
    fn test_celsius_eq(#[case] a: Celsius, #[case] b: Celsius) {
        assert_eq!(a, b);
    }

    #[rstest]
    #[case(Celsius(0.0), Celsius(10.3))]
    #[case(Celsius(0.0), Celsius(0.09))]
    #[case(Celsius(37.5), Celsius(38.9))]
    fn test_celsius_ne(#[case] a: Celsius, #[case] b: Celsius) {
        assert_ne!(a, b);
    }

    #[rstest]
    #[case(Fahrenheit(32.0), Fahrenheit(32.001))]
    #[case(Fahrenheit(32.004), Fahrenheit(32.0))]
    #[case(Fahrenheit(60.31), Fahrenheit(60.308))]
    fn test_fahrenheit_eq(#[case] a: Fahrenheit, #[case] b: Fahrenheit) {
        assert_eq!(a, b);
    }

    #[rstest]
    #[case(Fahrenheit(0.0), Fahrenheit(10.3))]
    #[case(Fahrenheit(0.0), Fahrenheit(0.09))]
    #[case(Fahrenheit(99.5), Fahrenheit(100.9))]
    fn test_fahrenheit_ne(#[case] a: Fahrenheit, #[case] b: Fahrenheit) {
        assert_ne!(a, b);
    }

    #[rstest]
    #[case(Celsius(0.0), Celsius(-7.49))]
    #[case(Celsius(15.73), Celsius(0.0))]
    #[case(Celsius(37.5), Celsius(15.73))]
    fn test_celsius_ord(#[case] a: Celsius, #[case] b: Celsius) {
        assert_gt!(a, b);
        assert_lt!(b, a);
    }

    #[rstest]
    #[case(Fahrenheit(32.0), Fahrenheit(18.52))]
    #[case(Fahrenheit(60.31), Fahrenheit(32.0))]
    #[case(Fahrenheit(99.5), Fahrenheit(60.31))]
    fn test_fahrenheit_ord(#[case] a: Fahrenheit, #[case] b: Fahrenheit) {
        assert_gt!(a, b);
        assert_lt!(b, a);
    }
}
