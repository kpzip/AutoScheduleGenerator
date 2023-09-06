pub trait Preference {
    fn evaluate(&self) -> f64;
}
