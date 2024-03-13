use clap::{ArgMatches, Parser};
use std::env;
use std::process;

use dotcomfy::Config;

#[derive(Parser, Debug)]
#[command(
    version,
    author="Ethan Harmon, Stephen Reaves",
    about="A simple tool for managing your dotfiles",
    long_about = None
)]
pub struct Args {
    #[arg(short, long)]
    install: String, // needs to be optional
    repo: String, // needs to be a sub command
                  // TODO: Am I supposed to write these options with annotations or the format on the website??
                  // TODO: Use argmatches to see if they passed in a github username, or a repo URL
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = dotcomfy::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
