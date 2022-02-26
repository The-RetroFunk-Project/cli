use clap::Parser;
use clap::Subcommand;
use std::process::*;

pub mod install;
pub mod config;
pub mod resources;
pub mod binaries;

use std::cmp::min;
use std::fs::File;
use std::io::Write;
use std::env;
use std::fs;

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

    Update {},
    Ping {}
}

fn pause_terminal()
{
    let mut input = String::new();

    std::io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");
}

#[tokio::main]
async fn main() {

    if config::exists(config::get_config_path().as_str())
    {
        let update_path = format!("{}/rfproject-old.exe", binaries::get_safe_global_directory());
        if config::exists(update_path.as_str())
        {
            fs::remove_file(&update_path).expect("Oops");
        }
    }

    if env::args().len() == 1
    {
        let mut input = String::new();

        println!(r#"Welcome to The RetroFunk Project CLI! 
If you want to install it, type 'game install'.
If you want to switch to the original Geometry Dash, type 'game switch-to-gd'.
If you want to switch back to The RetroFunk Project, type 'game switch-to-rfp'.
For more information, type 'help'."#);

        std::io::stdin().read_line(&mut input)
            .ok()
            .expect("Couldn't read line");    

        let mut initCommand = String::new();

        if config::exists(config::get_config_path().as_str()) 
        { initCommand = "rfproject".to_string(); }
        else 
        { 
            initCommand = std::env::current_exe().unwrap().into_os_string().into_string().unwrap(); 
        }

        let len = input.len();
        input.truncate(len - 2);

        //println!("Executable: {} | Arguments: {}", initCommand, input);
        let rest: Vec<&str> = input.split(' ').collect::<Vec<&str>>();

        let output = Command::new(initCommand).args(rest).spawn().expect("Oops");
        print!("Press Enter or Return to Exit.");
        pause_terminal();
        exit(0);
    }
    else 
    {
        println!("There are {} arguments!", env::args().len());
    }

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
                    install::extract_retrofunk_project_zip();
                    resources::read_plist_file();
                    binaries::binary_installation();
                    install::installation_complete_message();
                    exit(0);
                }
                else
                {
                    exit(1);
                }
            }
            else if option == "switch-to-gd" { binaries::switch_to_gd(); exit(0); }
            else if option == "switch-to-rfp" { binaries::switch_to_rfp(); exit(0); }
        },
        Commands::Update {} =>
        {
            println!("Updating The RetroFunk Project...");
            let mut client = reqwest::Client::new();

            if cfg!(windows)
            {
                let url = "https://github.com/The-RetroFunk-Project/cli/releases/download/release/rfproject.exe";
                let path = format!("{}/rfproject.exe", binaries::get_safe_global_directory());
                let old_path = format!("{}/rfproject-old.exe", binaries::get_safe_global_directory());

                fs::rename(&path, &old_path).expect("Oops");

                download_file(&client, url, &path).await;
            }
            exit(0);
        },
        Commands::Ping {} => { print!("Pong!"); }
    }
}
