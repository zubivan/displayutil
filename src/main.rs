use core_graphics::display::CGDisplay;

fn main() {
    let displays = CGDisplay::active_displays();
    match displays {
        Result::Ok(displays) => {            
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
