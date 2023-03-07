mod cli;
mod config;

fn main() {
    let args = cli::cli_parse();

    if args.grab {
        config::info::filenames();
    }

    if args.archive.is_some() {
        println!("archiving {}", args.archive.expect("fool"));
    }

    if args.filter.is_some() {
        println!("{}", config::info::home().display());
    }
}
