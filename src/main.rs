use clap::{App, Arg};
use core_graphics::display::CGDisplay;
use dirs::home_dir;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

mod cg_extensions;
use cg_extensions::change_display_location;

#[derive(Serialize, Deserialize)]
struct DisplayLocation {
    id: u32,
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize)]
struct ConfigurationElement {
    name: String,
    configuration: Vec<DisplayLocation>,
}

impl DisplayLocation {
    pub fn new(id: u32, x: i32, y: i32) -> DisplayLocation {
        DisplayLocation { id, x, y }
    }
}

type CommandResult = Result<String, Box<dyn Error>>;

const ARRANGEMENT_COMMAND: &str = "arrangement";
const ARRANGEMENT_COMMAND_SHORT: &str = "a";

const SAVE_COMMAND: &str = "save";
const RESTORE_COMMAND: &str = "restore";

fn main() {
    let config = App::new("Display util")
        .version("0.1.0")
        .author("Ivan Z. <zub.ivan@gmail.com>")
        .arg(
            Arg::with_name(ARRANGEMENT_COMMAND)                
                .long(ARRANGEMENT_COMMAND)
                .short(ARRANGEMENT_COMMAND_SHORT)
                .conflicts_with(RESTORE_COMMAND)
                .takes_value(true)
                .possible_values(&[SAVE_COMMAND, RESTORE_COMMAND])
                .required(true),
        )
        .get_matches();

    let selected_command = config.value_of(ARRANGEMENT_COMMAND);

    let execution_result = match selected_command {
        Some(command) => {
            match command {
                SAVE_COMMAND => save("default"),
                RESTORE_COMMAND => restore("default"),
                _ => Result::Err(Box::from("Configuration is invalid"))
            }
        },
        None => Result::Err(Box::from("Configuration is invalid")),
    };

    ::std::process::exit(match execution_result {
        Ok(message) => {
            println!("{}", message);
            0
        }
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}

fn save(config_name: &str) -> CommandResult {
    let current_state = get_active_displays();
    let configuration = ConfigurationElement {
        name: config_name.to_string(),
        configuration: current_state?,
    };
    let json_config = serde_json::to_string_pretty(&configuration);

    let config_path = get_config_file_location();

    let mut file = File::create(&config_path)?;
    let write_file = file.write_all(json_config?.as_bytes());
    match write_file {
        Result::Ok(_) => Result::Ok(format!("Configuration is saved to '{}'", config_path.to_str().unwrap())),
        Result::Err(err) => Result::Err(Box::from(err)),
    }
}

fn read_stored_config(config_name: &str) -> Result<Vec<DisplayLocation>, Box<dyn Error>> {
    let config_path = get_config_file_location();
    let file = File::open(config_path)?;
    let reader = BufReader::new(file);
    let store: ConfigurationElement = serde_json::from_reader(reader)?;

    if store.name == config_name {
        Ok(store.configuration)
    } else {
        Err(Box::from("Configuration not found"))
    }
}

fn get_config_file_location() -> PathBuf {
    const DEFAULT_CONFIG_NAME: &str = ".displayutil";

    let home_dir = home_dir().unwrap();
    let home_dir = home_dir.as_path();

    let config_location: PathBuf = home_dir.join(DEFAULT_CONFIG_NAME);
    config_location.to_owned()
}

fn get_active_displays() -> Result<Vec<DisplayLocation>, Box<dyn Error>> {
    let displays = CGDisplay::active_displays();
    let mut result: Vec<DisplayLocation> = Vec::new();
    match displays {
        Result::Ok(displays) => {
            for &id in &displays {
                let display = CGDisplay::new(id);
                let bounds = display.bounds();
                let origin = bounds.origin;
                result.push(DisplayLocation::new(id, origin.x as i32, origin.y as i32));
            }
            Result::Ok(result)
        }
        Result::Err(error_code) => Result::Err(Box::from(format!(
            "Cannot get current display configuration. Error code {}.",
            error_code
        ))),
    }
}

fn restore(config_name: &str) -> CommandResult {
    let stored_config = read_stored_config(config_name)?;

    let displays = CGDisplay::active_displays();
    match displays {
        Result::Ok(displays) => {
            for &id in &displays {
                let config = stored_config.iter().find(|x| x.id == id);
                match config {
                    Some(display_config) => {
                        let display = CGDisplay::new(id);
                        let bounds = display.bounds();
                        let origin = bounds.origin;

                        if origin.x as i32 == display_config.x
                            && origin.y as i32 == display_config.y
                        {
                            continue;
                        }

                        change_display_location(id, display_config.x, display_config.y);
                    }
                    None => {
                        continue;
                    }
                };
            }
            Result::Ok("Configuration finished.".to_string())
        }
        Result::Err(err) => Result::Err(Box::from(format!("Operation failed with code: {}", err))),
    }
}
