pub trait Convertible {
    fn  convert(&self, to: &str) -> Option<f64>; //Option = some or none
}