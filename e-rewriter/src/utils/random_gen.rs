
use rand::Rng;



// pub fn generate_random_float() -> f64 {
//     let mut rng = rand::thread_rng();
//     let random_float: f64 = rng.gen_range(0.0..0.5);
//     random_float
// }

pub fn generate_random_float1() -> f64 {
    let mut rng = rand::thread_rng();
    let random_float: f64 = rng.gen_range(0.0..1.0);
    random_float
}