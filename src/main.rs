use clap::{Parser, Subcommand};
use git2::Repository;

use dotcomfy::install_repo;

#[derive(Parser)]
#[command(
    version,
    author="Ethan Harmon, Stephen Reaves",
    about="A simple tool for managing your dotfiles",
    long_about = None
)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install the repo
    Install {
        /// repo_url can be a full url, or just a username, or left empty
        repo_url: Option<String>,
        /// path to install dotfiles. relative to the current folder
        path: Option<String>,
    },
}

fn main() {
    let args = Args::parse();

    let dot_file_path = "/tmp/dotfiles";

    match &args.command {
        Commands::Install { repo_url, path } => install_repo(repo_url, path),
    }
    /*
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    */

    /*
    if let Err(e) = dotcomfy::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    */
}
