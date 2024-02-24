/// Trait defining the different ways to get a temperature.
pub trait TemperatureUnit {
    /// Get the temperature in degrees Celcius (°C).
    fn celcius(&self) -> f32;
    /// Get the temperature in degrees Farenheit (°F).
    fn farenheit(&self) -> f32;
}

/// The degrees Celcius temperature unit.
pub struct Celcius {
    pub(crate) value: f32,
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
    pub(crate) value: f32,
}

impl TemperatureUnit for Farenheit {
    fn celcius(&self) -> f32 {
        convert_farenheit_to_celcius(self.value)
    }

    fn farenheit(&self) -> f32 {
        self.value
    }
}

/// Converts a temperature in °C to °F.
fn convert_celcius_to_farenheit(temperature: f32) -> f32 {
    temperature * 1.8 + 32.0
}

/// Converts a temperature in °F to °C.
fn convert_farenheit_to_celcius(temperature: f32) -> f32 {
    (temperature - 32.0) * 0.55555
}

#[cfg(test)]
mod tests {
    #[test]
    fn convert_celcius_to_farenheit() {
        assert!((super::convert_celcius_to_farenheit(0.0) - 32.0).abs() < 0.01);
        assert!((super::convert_celcius_to_farenheit(15.73) - 60.31).abs() < 0.01);
        assert!((super::convert_celcius_to_farenheit(-7.49) - 18.52).abs() < 0.01);
        assert!((super::convert_celcius_to_farenheit(37.5) - 99.5).abs() < 0.01);
    }

    #[test]
    fn convert_farenheit_to_celcius() {
        assert!((super::convert_farenheit_to_celcius(32.0) - 0.0).abs() < 0.01);
        assert!((super::convert_farenheit_to_celcius(60.31) - 15.73).abs() < 0.01);
        assert!((super::convert_farenheit_to_celcius(18.52) - -7.49).abs() < 0.01);
        assert!((super::convert_farenheit_to_celcius(99.5) - 37.5).abs() < 0.01);
    }
}
