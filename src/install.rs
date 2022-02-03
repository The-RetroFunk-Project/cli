use std::path;
use std::path::PathBuf;
use sysinfo::{ProcessExt, System, SystemExt};
use std::fs;
use std::io::{Result, Error, ErrorKind};

use crate::config;

pub fn GetGeometryDash() -> Result<PathBuf>
{
	let mut sys = System::new();
    sys.refresh_processes();

    let mut geometryDash;

    if cfg!(windows) {
    	geometryDash = sys.processes_by_exact_name("GeometryDash.exe");
    } else {
    	geometryDash = sys.processes_by_exact_name("Geometry Dash");
    }


    let gdProcess = match geometryDash.next() {
        Some(e) => e,
        None => return Err(Error::new(ErrorKind::Other, "Geometry Dash is not running! Make sure the game is opened and run the command again.")),
    };

    if geometryDash.next().is_some() { 
    	return Err(Error::new(
    		ErrorKind::Other,
    		"Looks like Geometry Dash is running twice in your device (for some reason)! Make sure there's only one instance of the game open."
    	));
    }

    let mut gamePath = PathBuf::from(gdProcess.exe()).parent().unwrap().to_path_buf();

    Ok(gamePath)
}

pub fn GeometryDashExists() -> bool
{
	config::create_config();

	if !config::is_gd_path_valid()
	{
		match GetGeometryDash()
	    {
	        Ok(result_path) => 
	        { 
	        	config::save_installation_path(Some(result_path).unwrap());
	        	return true; 
	        },
	        Err(err) => {
	            println!("[ERROR]: {}", err);
	            return false;
	        },
	    }
	} else { return true; }

	return false;
}