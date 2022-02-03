use clap::Parser;
use clap::Subcommand;
use std::process::*;

pub mod install;
pub mod config;
pub mod resources;

use std::cmp::min;
use std::fs::File;
use std::io::Write;

use reqwest::Client;
use indicatif::{ProgressBar, ProgressStyle};
use futures_util::StreamExt;

pub async fn download_file(client: &Client, url: &str, path: &str) -> Result<(), String> {
    // Reqwest setup
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;
    
    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("█=>"));
    pb.set_message(&format!("Downloading {}", url));

    // download chunks
    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(&format!("Downloaded {} to {}", url, path));
    return Ok(());
}

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

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command
    {
        Commands::Game { option } => 
        { 
            if option == "install"
            {
                if install::GeometryDashExists()
                {
                    println!("Downloading The RetroFunk Project...");
                    let mut client = reqwest::Client::new();
                    let url = "https://github.com/The-RetroFunk-Project/game/releases/download/release/TRFP-Win64.zip";
                    let path = format!("{}/TRFP.zip", config::get_installation_path());
                    download_file(&client, url, &path).await;
                    resources::read_plist_file();
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
