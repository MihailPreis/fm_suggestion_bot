use std::env;

use crate::utils::result_utils::FatalValueMapper;

pub fn get_env_key(key: &str) -> String {
    env::var(key).map_value_or_exit(format!("Can not get value for key {}, exiting", key))
}

pub fn try_get_env(key: &str) -> Option<String> {
    env::var(key).ok()
}
