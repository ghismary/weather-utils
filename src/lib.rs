#![doc = include_str!("../README.md")]
#![deny(unsafe_code, missing_docs)]
#![no_std]

/// The humidity types.
pub mod humidity;
/// The pressure types.
pub mod pressure;
/// The temperature types (Celsius and Fahrenheit).
pub mod temperature;

pub use humidity::{
    AbsoluteHumidity, Comfort, HeatIndex, RelativeHumidity, TemperatureAndRelativeHumidity,
};
pub use pressure::{Altitude, BarometricPressure, TemperatureAndBarometricPressure};
pub use temperature::{Celsius, Fahrenheit, Temperature};
