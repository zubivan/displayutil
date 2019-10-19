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

type CommandResult = Result<(), Box<dyn Error>>;

fn main() {
    let config = App::new("DisplayKeeper")
        .version("0.1")
        .author("Ivan Z. <zub.ivan@gmail.com>")
        .arg(
            Arg::with_name("save")
                .long("save")
                .conflicts_with("restore")
                .required(true),
        )
        .arg(
            Arg::with_name("restore")
                .long("restore")
                .conflicts_with("save")
                .required(true),
        )
        .get_matches();

    let save_location = config.is_present("save");
    let restore_location = config.is_present("restore");

    let execution_result = match (save_location, restore_location) {
        (true, true) => Result::Err(Box::from("Both 'save' and 'restore' options are specified")),
        (false, true) => {
            println!("Restoring configuration");
            restore("default")
        }
        (true, false) => {
            println!("Saving current configuration");
            save("default")
        }
        _ => Result::Err(Box::from("Configuration is invalid")),
    };

    ::std::process::exit(match execution_result {
        Ok(_) => {
            println!("Done");
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

    let mut file = File::create(config_path)?;
    let write_file = file.write_all(json_config?.as_bytes());
    match write_file {
        Result::Ok(_) => Result::Ok(()),
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
    const DEFAULT_CONFIG_LOCATION: &str = ".displaykeeper";

    let home_dir = home_dir().unwrap();
    let home_dir = home_dir.as_path();

    let config_location: PathBuf = home_dir.join(DEFAULT_CONFIG_LOCATION);
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
            "Cannot get dispaly current display configuration. Error code {}.",
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
                        println!("Configuring display id: {}", id);
                        println!("- model number {}", display.model_number());
                        println!("- unit number {}", display.unit_number());
                        println!("- vendor number {}", display.vendor_number());

                        println!("- current display origin is {}:{}", origin.x, origin.y);
                        println!(
                            "- cached display origin is {}:{}",
                            display_config.x, display_config.y
                        );

                        if origin.x as i32 == display_config.x
                            && origin.y as i32 == display_config.y
                        {
                            println!("- already in the right position");
                            continue;
                        }

                        println!("- starting display configuration");

                        change_display_location(id, display_config.x, display_config.y);
                        println!("- finished display configuration");
                    }
                    None => {
                        println!("No cached config exists");
                        continue;
                    }
                };
            }
            Result::Ok(())
        }
        Result::Err(err) => Result::Err(Box::from(format!("Operation failed with code: {}", err))),
    }
}
