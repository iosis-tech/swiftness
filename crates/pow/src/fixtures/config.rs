use crate::config::Config;

pub fn get() -> Config {
    Config { n_bits: 30 }
}
