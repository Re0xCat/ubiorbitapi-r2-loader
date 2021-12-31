use std::{env, fs};

use once_cell::sync::Lazy;

use crate::consts::CONFIG_NAME;
use crate::models::config::Config;

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let path = env::current_dir().unwrap().join(CONFIG_NAME);
    let data = fs::read_to_string(path).unwrap();
    let config: Config = toml::from_str(&data).unwrap();

    config
});
