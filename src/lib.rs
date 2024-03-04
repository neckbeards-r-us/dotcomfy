use std::error::Error;
use std::fs;
use std::env;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    config: String
}
