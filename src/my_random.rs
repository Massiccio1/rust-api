use rand::Rng;

pub fn get_random_int(max: u32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..max as f32).floor() as i32
}
pub fn get_random_bool() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0) >= 0.5
}