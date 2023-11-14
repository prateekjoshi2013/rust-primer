use std::ops::Deref;

#[derive(Debug, Clone)]
struct FinalizedConfig<T>(T);

impl<T: Copy> Copy for FinalizedConfig<T> {}

impl<T> Deref for FinalizedConfig<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[derive(Debug)]
struct Config {
    a: usize,
    b: String,
}

impl Config {
    fn new() -> Self {
        Self {
            a: 0,
            b: String::from("Hello"),
        }
    }

    fn build(self) -> FinalizedConfig<Config> {
        FinalizedConfig(self)
    }
}

pub fn main() {
    let mut my_config = Config::new();
    my_config.a = 6;
    let mut finalized_config = my_config.build();
    // finalized_config.a=66; //cannot assign to data in dereference of
}
