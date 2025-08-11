#[derive(Clone)]
pub struct Config {
    pub fish_age: u32,
    pub shark_age: u32,
    pub shark_energy: u32,
    pub chronon: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fish_age: 5,
            shark_age: 10,
            shark_energy: 3,
            chronon: 50,
        }
    }
}
