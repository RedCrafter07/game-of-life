use std::env;

pub fn get_arg_or_default(key: &str, value: String) -> String {
    env::var(key).unwrap_or(value)
}
