use super::convertible::Convertible;

pub struct Celsius {
    pub value: f64,
}

pub struct Fahrenheit {
    pub value: f64,
}

pub struct Kelvin {
    pub value: f64,
}

impl Convertible for Celsius {
    fn convert(&self, to: &str) -> Option<f64> {
        match to {
            "fahrenheit" => Some(self.value * 9.0 / 5.0 + 32.0),
            "kelvin" => Some(self.value + 273.15),
            _ => None,
        }
    }
}

impl Convertible for Fahrenheit {
    fn convert(&self, to: &str) -> Option<f64> {
        match to {
            "celsius" => Some((self.value - 32.0) * 5.0 / 9.0),
            "kelvin" => Some((self.value - 32.0) * 5.0 / 9.0 + 273.15),
            _ => None,
        }
    }
}

impl Convertible for Kelvin {
    fn convert(&self, to: &str) -> Option<f64> {
        match to {
            "celsius" => Some(self.value - 273.15),
            "fahrenheit" => Some((self.value - 273.15) * 9.0 / 5.0 + 32.0),
            _ => None,
        }
    }
}