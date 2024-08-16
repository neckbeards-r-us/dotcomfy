use clap::{Parser, Subcommand};

use dotcomfy::{install::install_repo, parse::parse_config};

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
    /// Install dotfiles from a git repo
    Install {
        /// repo_url can be a full url of a git repo, or just a GitHub username
        repo_url: Option<String>,
        /// Path to install dotfiles
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Parse config file, but do nothing
    Parse {
        #[arg(short, long, default_value = "comfy.toml")]
        file_name: String
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Install { repo_url, path } => install_repo(repo_url, path),
        Commands::Parse { file_name } => parse_config(file_name),
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
