use rand::Rng;

// TODO check if this can return 1.0
pub fn random_double() -> f64 {
    let mut rand = rand::rng();
    rand.random::<f64>()
}