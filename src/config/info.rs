pub use std::path::{Path, PathBuf};

pub fn greet() -> &'static str {
    "Hello, world!"
}

pub fn homed() -> &'static Path {
    return Path::new(env!("HOME"));
}

pub fn qas_config_home() -> PathBuf {
    return Path::new(homed()).join(".config").join("qas");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_test() {
        assert_eq!("Hello, world!", greet());
    }

    #[test]
    fn qas_config_home_test() {
        assert_eq!(
            PathBuf::from(homed().join(".config/qas")),
            qas_config_home()
        );
    }
}
