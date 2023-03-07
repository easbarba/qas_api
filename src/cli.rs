use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about="Easily manage multiple FLOSS repositories", long_about = None)]
pub struct Actions {
    /// Archive projects.
    #[arg(short, long)]
    pub archive: Option<String>,

    /// filter project to run action
    #[arg(short, long)]
    pub filter: Option<String>,

    /// Grab all projects.
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub grab: bool,
}

pub fn cli_parse() -> Actions {
    Actions::parse()
}
