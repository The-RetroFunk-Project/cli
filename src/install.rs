use std::path;
use std::path::PathBuf;
use sysinfo::{ProcessExt, System, SystemExt};
use std::fs;
use std::io::{Result, Error, ErrorKind};
use zip_extensions::read::*;

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
    gdProcess.kill();
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

pub fn extract_retrofunk_project_zip()
{
	zip_extensions::read::zip_extract(
    	&PathBuf::from(format!("{}/TRFP.zip", config::get_installation_path()).as_str()),
    	&PathBuf::from(config::get_installation_path().as_str()),
	);
}

pub fn installation_complete_message()
{
	println!(r#"
========================
"The RetroFunk Project is now installed!"

Open Geometry Dash now to see the changes!

If you wanna go back to the original Geometry Dash,
type the command "rfproject game switch-to-gd" in
your command prompt.

And if you wanna go back to The RetroFunk Project,
use the command "rfproject game switch-to-rfp".

You can check out the command "rfproject help" 
for more info.
========================
	"#);
}