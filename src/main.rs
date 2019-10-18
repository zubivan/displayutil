use clap::{App, Arg};
use core_graphics::display::*;

struct DisplayLocation {
    id: u32,
    x: i32,
    y: i32,
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
                .takes_value(true)
                .value_name("CONFIG_NAME"),
        )
        .arg(
            Arg::with_name("restore")
                .long("restore")
                .takes_value(true)
                .value_name("CONFIG_NAME"),
        )
        .get_matches();

    let save_location = config.value_of("save");
    let restore_location = config.value_of("restore");

    match (save_location, restore_location) {
        (Some(_), Some(_)) => {
            eprintln!("Both save and restore commands were specified.");
        }
        (None, Some(config_name)) => restore(&config_name),
        (Some(config_name), None) => save(&config_name),
        (None, None) => eprintln!("Configuration is required."),
    }
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
                        let config = display.begin_configuration().unwrap();
                        display
                            .change_display_location(config, display_config.x, display_config.y)
                            .unwrap();

                        display
                            .complete_configuration(
                                &config,
                                CGConfigureOption::ConfigurePermanently,
                            )
                            .unwrap();
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

fn save(configName: &str) {}

pub trait CGDisplayExt {
    fn change_display_location(
        &self,
        config: CGDisplayConfigRef,
        x: i32,
        y: i32,
    ) -> Result<(), CGError>;
}

impl CGDisplayExt for CGDisplay {
    fn change_display_location(
        &self,
        config: CGDisplayConfigRef,
        x: i32,
        y: i32,
    ) -> Result<(), CGError> {
        let result = unsafe { CGConfigureDisplayOrigin(config, self.id, x, y) };

        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    pub fn CGConfigureDisplayOrigin(
        config: CGDisplayConfigRef,
        display: CGDirectDisplayID,
        x: i32,
        y: i32,
    ) -> CGError;
}
