use clap::{Parser, Subcommand};
use git2::Repository;
// use std::env;
// use std::process;

// use dotcomfy::Config;

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
    Install { repo_url: Option<String> },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Install { repo_url } => {
            // TODO: Figure out how to grab value from Option and check to see if it's username or
            // repo_url
            let repo: Repository;

            if let Some(repo_url) = &repo_url {
                if repo_url.contains("https://") && repo_url.contains("/dotfiles.git") {
                    repo = match Repository::clone(repo_url, "/tmp/dotfiles") {
                        Ok(repo) => repo,
                        Err(e) => panic!("Failed to clone: {}", e),
                    };
                } else {
                    let repo_url = format!("https://github.com/{}/dotfiles.git", repo_url);
                    repo = match Repository::clone(&repo_url, "/tmp/dotfiles") {
                        Ok(repo) => repo,
                        Err(e) => panic!("Failed to clone: {}", e),
                    };
                }
            }

            let repo_head = repo.checkout_head({});
            // println!("{:?}", repo_url.unwrap())
        }
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
