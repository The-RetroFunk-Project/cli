use std::fs;
use std::env;
use std::path::*;
use dirs::*;
use crate::config;

pub fn binary_installation()
{
	if !config::exists(format!("{}/Game", config::get_installation_path()).as_str()) 
	{ fs::create_dir(format!("{}/Game", config::get_installation_path()).as_str()); }

	if !config::exists(format!("{}/Game/Binaries", config::get_installation_path()).as_str()) 
	{ fs::create_dir(format!("{}/Game/Binaries", config::get_installation_path()).as_str()); }
	else {
		// BE CAREFUL! THIS MEANS ITS INSTALLED! LET'S PUT BACK THE OG GEOMETRY DASH!
		switch_to_gd();
	}

	if cfg!(windows)
	{
		fs::copy(
			format!("{}/GeometryDash.exe", config::get_installation_path()).as_str(), 
			format!("{}/Game/Binaries/gd.exe", config::get_installation_path()).as_str()).expect("Oops");

		fs::copy(
			format!("{}/TheRetroFunkProject.exe", config::get_installation_path()).as_str(), 
			format!("{}/Game/Binaries/rfp.exe", config::get_installation_path()).as_str()).expect("Oops");

		fs::remove_file(format!("{}/GeometryDash.exe", config::get_installation_path()).as_str()).expect("Oops");
		fs::remove_file(format!("{}/TheRetroFunkProject.exe", config::get_installation_path()).as_str()).expect("Oops");
	}

	switch_to_rfp();
	set_binary_into_safe_global_directories();
}

pub fn switch_to_rfp()
{
	if cfg!(windows)
	{
		fs::copy(
				format!("{}/Game/Binaries/rfp.exe", config::get_installation_path()).as_str(), 
				format!("{}/GeometryDash.exe", config::get_installation_path()).as_str()).expect("Oops");
	}
}

pub fn switch_to_gd()
{
	if cfg!(windows)
	{
		fs::remove_file(format!("{}/GeometryDash.exe", config::get_installation_path()).as_str()).expect("Oops");

		fs::copy(
				format!("{}/Game/Binaries/gd.exe", config::get_installation_path()).as_str(), 
				format!("{}/GeometryDash.exe", config::get_installation_path()).as_str()).expect("Oops");
	}
}

pub fn set_binary_into_safe_global_directories()
{
	if cfg!(windows)
	{
		copy_cli_into_windows_apps();
	}
}

pub fn copy_cli_into_windows_apps()
{
	let exe_path = env::current_exe().unwrap().into_os_string().into_string().unwrap();
	if !exe_path.contains("AppData")
	{
		fs::copy(&exe_path, 
			format!("{}/AppData/Local/Microsoft/WindowsApps/{}", dirs::home_dir().unwrap().into_os_string().into_string().unwrap(),
			 Path::new(&exe_path).file_name().unwrap().to_os_string().into_string().unwrap())).expect("Oops");

		fs::copy(&config::get_config_path(), 
			format!("{}/AppData/Local/Microsoft/WindowsApps/{}", dirs::home_dir().unwrap().into_os_string().into_string().unwrap(),
			 Path::new(&config::get_config_path()).file_name().unwrap().to_os_string().into_string().unwrap())).expect("Oops");
	}
}

pub fn get_binary_filename() -> String
{
	return env::current_exe().unwrap().into_os_string().into_string().unwrap();
}

pub fn get_safe_global_directory() -> String
{
	if cfg!(windows)
	{
		return format!("{}/AppData/Local/Microsoft/WindowsApps", dirs::home_dir().unwrap().into_os_string().into_string().unwrap());
	}

	return "".to_string();
}