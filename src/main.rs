use clap::{Parser, Subcommand};
use std::env;
use std::process;

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
    Install { repo: Option<String> },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Install { repo } => {
            // TODO: Figure out how to grab value from Option and check to see if it's username or
            // repo
            let repo = Some(repo).unwrap_or_else(|| println!("No username/repo passed"));
            println!("{:?}", repo)
            /*
            if *repo.contains("https://") {
                println!("{repo}")
            } else {
                println!("Didn't pass in a repo or GH username!")
            }
            */
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
