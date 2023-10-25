use std::env;

pub fn ensure_is_not_empty(value: String) -> String {
    if value.is_empty() {
        panic!("Value must not be empty");
    }

    value
}

pub fn env_or_default(key: &str, default: &str) -> String {
    ensure_is_not_empty(env::var(key).unwrap_or(default.to_string()))
}

pub fn env_or_panic(key: &str) -> String {
    ensure_is_not_empty(env::var(key).expect(&format!("{} must be set", key)))
}
