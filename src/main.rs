mod config;

fn main() {
    println!(
        "{} -> {} -> {}",
        config::info::homed().display(),
        config::info::greet(),
        config::info::qas_config_home().display()
    );
}
