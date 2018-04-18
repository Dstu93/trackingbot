
use bot::configuration::config::ApplicationConfig;

use std::io::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use serde_json;

    /// loads the base Configuration of this Application
pub fn applicationconfig() -> Result<ApplicationConfig, Error>{
        
    let mut json = String::new();
    let config_path = Path::new("config.json");

    let mut file: File = match File::open(&config_path){
        Ok(file) => {file},
        Err(error) => {panic!("Could not load config: {}", error)}
    };

    //read from config File to the json String
    file.read_to_string(&mut json).expect("could not read from config file");
    let config: ApplicationConfig = serde_json::from_str(&json)?;
    Ok(config)
}