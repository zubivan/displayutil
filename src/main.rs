use clap::{App, Arg};
use core_graphics::display::{CGDisplay};
use dirs::home_dir;
use std::path::{Path, PathBuf};
use std::io::Write;
use std::fs::File;

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

fn main() {
    let config = App::new("DisplayKeeper")
        .version("0.1")
        .author("Ivan Z. <zub.ivan@gmail.com>")
        .arg(
            Arg::with_name("save")
                .long("save")
                .conflicts_with("restore")
                .required(true)
                .takes_value(true)
                .default_value("default")
                .value_name("CONFIG_NAME"),
        )
        .arg(
            Arg::with_name("restore")
                .long("restore")
                .conflicts_with("save")
                .required(true)
                .takes_value(true)
                .value_name("CONFIG_NAME"),
        )
        .get_matches();

    let save_location = config.value_of("save");
    let restore_location = config.value_of("restore");

    match (save_location, restore_location) {
        (Some("default"), Some("default")) => {
            eprintln!("Both save and restore commands were specified.");
        }
        (None, Some(config_name)) => restore(&config_name),
        (Some(config_name), None) => save(&config_name),
        (_, _) => eprintln!("Configuration is invalid."),
    }
}

fn save(config_name: &str) {
    const DEFAULT_CONFIG_LOCATION: &str = ".displaykeeper";

    let home_dir = home_dir().unwrap();
    let home_dir = home_dir.as_path();

    let config_path = home_dir.join(DEFAULT_CONFIG_LOCATION);

    let current_state = get_active_displays();
    let configuration = ConfigurationElement {
        name: config_name.to_string(),
        configuration: current_state.unwrap(),
    };
    let json_config = serde_json::to_string_pretty(&configuration);

    let mut file = File::create(config_path).unwrap();
    file.write_all(json_config.unwrap().as_bytes()).unwrap();
}

fn get_active_displays() -> Option<Vec<DisplayLocation>> {
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
        }
        Result::Err(e) => panic!(e),
    };

    Some(result)
}

fn restore(config_name: &str) {
    let stored_config = vec![
        DisplayLocation::new(731409289, -1714, -1440),
        DisplayLocation::new(69733382, 0, 0),
        DisplayLocation::new(731409290, 846, -1440),
    ];

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

                        if origin.x == (display_config.x as f64)
                            && origin.y == (display_config.y as f64)
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
        }
    }
        }
        Result::Err(err) => panic!(err),
    }
}
}
