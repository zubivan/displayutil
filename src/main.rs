use core_graphics::display::*;

fn main() {
    let displays = CGDisplay::active_displays();
    match displays {
        Result::Ok(displays) => {            
            for id in &displays {
                println!("Display id is {}", id);
                let display = CGDisplay::new(*id);
                let bounds = display.bounds();
                println!("Serial {}", display.serial_number());
                println!(
                    "Display origin is {}:{}",
                    bounds.origin.x, bounds.origin.y
                );
                println!(
                    "Display size is {}:{}",
                    bounds.size.width, bounds.size.height
                );
                println!("Starting display configuration");
                let config = display.begin_configuration().unwrap();

                display.change_display_location(config, 110, 110).unwrap();
                display
                    .complete_configuration(&config, CGConfigureOption::ConfigurePermanently)
                    .unwrap();
                
                println!("Finished display configuration");
            }
        }
        Result::Err(err) => panic!(err),
    }
}

pub trait CGDisplayExt {
    fn change_display_location(
        &self,
        config: CGDisplayConfigRef,
        x: u32,
        y: u32,
    ) -> Result<(), CGError>;
}

impl CGDisplayExt for CGDisplay {
    fn change_display_location(
        &self,
        config: CGDisplayConfigRef,
        x: u32,
        y: u32,
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
        x: u32,
        y: u32,
    ) -> CGError;
}
