use serde;
use serde::*;
use serde_json;
use serde_json::*;
use std::fs;
use std::path::*;
use std::env;
use std::io;

#[derive(Serialize, Deserialize, Clone)]
pub struct Configuration 
{
    pub InstallationPath: Option<PathBuf>,
    pub CurrentVersion: Option<String>
}

static mut CONFIG: Configuration = Configuration {
    InstallationPath: None,
    CurrentVersion: None
};

pub fn exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn create_config()
{
    unsafe
    {
        let exePath = std::env::current_exe().unwrap();
        let saveDirectory = exePath.parent().unwrap();
        let saveFile = saveDirectory.join("config.json");
    
        if !exists(exePath.into_os_string().into_string().unwrap().as_str())
        {
            let raw = serde_json::to_string(&CONFIG).unwrap();
            fs::write(saveFile, raw).expect("Oops!");
        }
    }
}

pub fn get_exe_directory() -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    Ok(dir)
}

pub fn get_config_from_json() -> Configuration
{
    let configPath = get_config_path();
    println!("Path: {}", configPath);

    let configString = fs::read_to_string(&configPath).expect("Oops");
    let mut configFile : Configuration = serde_json::from_str(configString.as_str()).unwrap();

    return configFile;
}

pub fn get_config_path() -> String
{
    return format!(r#"{}\config.json"#, get_exe_directory().unwrap().into_os_string().into_string().unwrap());
}

pub fn save_installation_path(loc: PathBuf)
{
    let mut configFile = get_config_from_json();

    configFile.InstallationPath = Some(loc);

    let newConfigData = serde_json::to_string(&configFile).unwrap();
    fs::write(get_config_path(), newConfigData).expect("Oops");
}

pub fn is_gd_path_valid() -> bool
{
    let mut configFile = get_config_from_json();

    if !configFile.InstallationPath.is_none()
    {
        return exists(configFile.InstallationPath.unwrap().into_os_string().into_string().unwrap().as_str());
    }

    return false;
}