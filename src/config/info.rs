pub use std::path::{Path, PathBuf};

/// user's home
pub fn user_home() -> &'static Path {
    Path::new(env!("HOME"))
}

/// project main config folder
pub fn home() -> PathBuf {
    Path::new(user_home()).join(".config").join("qas")
}

/// do config folder exist?
pub fn exist() -> bool {
    home().exists()
}

/// is config folder empty?
pub fn empty() -> bool {
    home().read_dir().unwrap().next().is_none()
}

pub fn count() -> usize {
    home().read_dir().unwrap().count()
}

/// list all configurations file names
pub fn filenames() {
    if exist() && !empty() {
        println!("{}", count());
        // home().read_dir().unwrap().cycle()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_home_test() {
        assert_eq!(PathBuf::from(user_home().join(".config/qas")), home());
    }
    #[test]
    fn config_folder_exist() {
        assert!(exist());
    }

    #[test]
    fn config_folder_is_empty() {
        assert!(!empty());
    }

    #[test]
    fn config_folder_count_correctly() {
        assert_eq!(count(), 2);
    }
}
