pub mod units;

use units::convertible::Convertible;
use units::degrees::{Celsius, Fahrenheit, Kelvin};

pub fn convert_temperature(value: f64, from: &str, to: &str) -> Option<f64> {
    let from = from.to_lowercase();
    let to = to.to_lowercase();

    match from.as_str() {
        "celsius" => Celsius { value }.convert(&to),
        "fahrenheit" => Fahrenheit { value }.convert(&to),
        "kelvin" => Kelvin { value }.convert(&to),
        _ => None,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(convert_temperature(0.0, "Celsius", "Fahrenheit"), Some(32.0));
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(convert_temperature(32.0, "Fahrenheit", "Celsius"), Some(0.0));
    }

    #[test]
    fn test_kelvin_to_celsius() {
        assert_eq!(convert_temperature(273.15, "Kelvin", "Celsius"), Some(0.0));
    }

    #[test]
    fn test_invalid_unit() {
        assert_eq!(convert_temperature(100.0, "Celsius", "Banane"), None);
    }
}