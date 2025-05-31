use rand::Rng;

/// Returns a random number between 0 and 1
pub fn random_double() -> f64 {
    let mut rand = rand::rng();
    rand.random()
}

pub fn random_between(min: f64, max: f64) -> f64 {
     let mut rand = rand::rng();
    rand.random_range(min..max)
}