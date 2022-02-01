use clap::Parser;
use clap::Subcommand;
use std::process::*;

pub mod install;
pub mod config;

#[derive(Parser)]
#[clap(version, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    Game
    {
        option: String,
    },

}

fn main() {
    let args = Cli::parse();

    match args.command
    {
        Commands::Game { option } => 
        { 
            if option == "install"
            {
                if install::GeometryDashExists()
                {
                    println!("Success!");
                    exit(0);
                }
                else
                {
                    exit(1);
                }
            }
        },
    }
}
